//! Hotkey-based recorders for UI element selectors
//!
//! ## Single mode
//! CTRL alone → capture element under cursor. ESC to cancel.
//!
//! ## Series mode (action recorder)
//! Automatically records every mouse click and text input.
//! `Ctrl+Shift+F2` → stop recording and save.

use std::fs;
use std::io::{self, Write};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

use chrono::Utc;
use rdev::{listen, EventType, Key};

use crate::capture;
use crate::types::{
    Action, BestSelector, CapturedElement, Capture, CaptureOutput, SeriesRecording,
};

// ──────────────────────────────────────────────────────────
//  SINGLE MODE
// ──────────────────────────────────────────────────────────

/// Run single-capture mode.
pub fn run_single_mode(output: &str, description: Option<String>) -> anyhow::Result<()> {
    eprintln!("=== Selector Capture: Single Mode ===");
    eprintln!("Place cursor over a UI element and press CTRL (alone) to capture.");
    eprintln!("Press ESC to cancel.");

    let (tx, rx) = mpsc::channel();
    spawn_single_listener(tx);

    print!("\n  Waiting for CTRL… ");
    io::stdout().flush()?;

    match wait_single_capture(&rx, description)? {
        Some(cap) => {
            let label = label(&cap.best_selector);
            println!("\n  ✓ Captured: {label}");
            append_capture(output, cap)?;
            println!("  ✓ Saved to {output}");
        }
        None => println!("\n  Cancelled."),
    }
    Ok(())
}

fn wait_single_capture(
    rx: &Receiver<SingleEvent>,
    description: Option<String>,
) -> anyhow::Result<Option<Capture>> {
    loop {
        match rx.recv()? {
            SingleEvent::Stop => return Ok(None),
            SingleEvent::Trigger => {
                let (x, y) = capture::cursor_position();
                match capture::capture_at_point(x, y) {
                    Ok((path, best)) => {
                        return Ok(Some(Capture {
                            id: format!("capture-{:016x}", timestamp_id()),
                            timestamp: Utc::now().to_rfc3339(),
                            description,
                            full_path: path,
                            best_selector: best,
                        }));
                    }
                    Err(e) => {
                        eprintln!("\n  ⚠ capture: {e}");
                        continue;
                    }
                }
            }
        }
    }
}

enum SingleEvent {
    Trigger,
    Stop,
}

fn spawn_single_listener(tx: Sender<SingleEvent>) {
    std::thread::spawn(move || {
        let mut ctrl_down = false;
        let mut blocked = false;

        let _ = listen(move |event| {
            match event.event_type {
                // CTRL press
                EventType::KeyPress(ref k)
                    if !ctrl_down && matches!(k, Key::ControlLeft | Key::ControlRight) =>
                {
                    ctrl_down = true;
                    blocked = false;
                }
                // Non-modifier while CTRL held → combo
                EventType::KeyPress(ref k) if ctrl_down && !is_modifier(k) => {
                    blocked = true;
                }
                // CTRL released
                EventType::KeyRelease(Key::ControlLeft) | EventType::KeyRelease(Key::ControlRight) => {
                    if ctrl_down && !blocked {
                        let _ = tx.send(SingleEvent::Trigger);
                    }
                    ctrl_down = false;
                    blocked = false;
                }
                // ESC
                EventType::KeyPress(Key::Escape) if !ctrl_down => {
                    let _ = tx.send(SingleEvent::Stop);
                }
                _ => {}
            }
        });
    });
}

// ──────────────────────────────────────────────────────────
//  SERIES MODE  — automatic action recorder
// ──────────────────────────────────────────────────────────

/// Run series (action recorder) mode.
///
/// Every mouse click and every text input is recorded automatically.
/// Press `Ctrl+Shift+F2` to stop.
pub fn run_series_mode(output: &str) -> anyhow::Result<()> {
    eprintln!("=== Selector Capture: Series Mode (Action Recorder) ===");
    eprintln!("• Mouse clicks  → element selector captured at click position");
    eprintln!("• Keyboard     → empty Input actions recorded (developer fills text)");
    eprintln!("• Press Ctrl+Shift+F2 to stop recording");
    eprintln!();

    let (tx, rx) = mpsc::channel();
    spawn_series_listener(tx);

    let started = Utc::now();
    let mut actions: Vec<Action> = Vec::new();
    let mut pending_input = false;

    loop {
        match rx.recv()? {
            SeriesEvent::Stop => {
                flush_input(&mut actions, &mut pending_input);
                break;
            }

            SeriesEvent::MouseDown { button } => {
                flush_input(&mut actions, &mut pending_input);

                let (x, y) = capture::cursor_position();
                if let Ok((path, best)) = capture::capture_at_point(x, y) {
                    actions.push(Action::Click {
                        button,
                        element: CapturedElement {
                            full_path: path,
                            best_selector: best,
                        },
                    });
                } else {
                    eprintln!("  ⚠ could not capture element at ({x:.0}, {y:.0})");
                }
            }

            SeriesEvent::Input => {
                pending_input = true;
            }
        }
    }

    let finished = Utc::now();

    println!("\n  Recording stopped — {} actions", actions.len());

    if !actions.is_empty() {
        let rec = SeriesRecording {
            tool: "selector-capture".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            timestamp_start: started.to_rfc3339(),
            timestamp_end: finished.to_rfc3339(),
            actions,
        };
        let json = serde_json::to_string_pretty(&rec)?;
        fs::write(output, json)?;
        println!("  ✓ Saved {output}");
    }

    Ok(())
}

/// Push a single `Input` action if keys were pressed since the last click.
fn flush_input(actions: &mut Vec<Action>, pending: &mut bool) {
    if *pending {
        actions.push(Action::Input {
            text: String::new(),
            element: None,
        });
        *pending = false;
    }
}

// ── Listener events for series mode ──────────────────────

enum SeriesEvent {
    MouseDown { button: String },
    Input,
    Stop,
}

/// Spawns a background thread that captures all keyboard & mouse events.
/// Mouse position is obtained via `GetCursorPos` on the main thread at
/// event processing time (to avoid coordinate system mismatch with UIA).
fn spawn_series_listener(tx: Sender<SeriesEvent>) {
    std::thread::spawn(move || {
        let mut shift = false;
        let mut ctrl = false;
        let mut alt = false;

        let _ = listen(move |event| {
            // ── update modifier states on every event ──
            match event.event_type {
                EventType::KeyPress(Key::ShiftLeft) | EventType::KeyPress(Key::ShiftRight) => {
                    shift = true
                }
                EventType::KeyRelease(Key::ShiftLeft) | EventType::KeyRelease(Key::ShiftRight) => {
                    shift = false
                }
                EventType::KeyPress(Key::ControlLeft) | EventType::KeyPress(Key::ControlRight) => {
                    ctrl = true
                }
                EventType::KeyRelease(Key::ControlLeft)
                | EventType::KeyRelease(Key::ControlRight) => ctrl = false,
                EventType::KeyPress(Key::Alt) | EventType::KeyPress(Key::AltGr) => alt = true,
                EventType::KeyRelease(Key::Alt) | EventType::KeyRelease(Key::AltGr) => alt = false,
                _ => {}
            }

            match event.event_type {
                // ── Stop combo: Ctrl + Shift + F2 ──
                EventType::KeyPress(Key::F2) if ctrl && shift => {
                    let _ = tx.send(SeriesEvent::Stop);
                }

                // ── Mouse clicks ──
                EventType::ButtonPress(button) => {
                    let _ = tx.send(SeriesEvent::MouseDown {
                        button: format!("{button:?}"),
                    });
                }

                // ── Input detected (any printable key, Ctrl/Alt not held) ──
                EventType::KeyPress(ref key) if !ctrl && !alt && is_printable_key(key) => {
                    let _ = tx.send(SeriesEvent::Input);
                }

                _ => {}
            }
        });
    });
}

// ──────────────────────────────────────────────────────────
//  Shared helpers
// ──────────────────────────────────────────────────────────

fn is_modifier(k: &Key) -> bool {
    matches!(
        k,
        Key::ControlLeft
            | Key::ControlRight
            | Key::ShiftLeft
            | Key::ShiftRight
            | Key::Alt
            | Key::AltGr
            | Key::MetaLeft
            | Key::MetaRight
    )
}

fn timestamp_id() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64
}

fn label(sel: &BestSelector) -> &str {
    sel.name.as_deref().unwrap_or(sel.control_type.as_str())
}

/// Append a single capture to a JSON file (create or read+append).
fn append_capture(output: &str, new_cap: Capture) -> anyhow::Result<()> {
    let mut obj: CaptureOutput = fs::read_to_string(output)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or(CaptureOutput {
            tool: "selector-capture".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            captures: Vec::new(),
        });
    obj.captures.push(new_cap);
    fs::write(output, serde_json::to_string_pretty(&obj)?)?;
    Ok(())
}

/// Returns `true` for keys that normally produce text input
/// (letters, numbers, symbols, space, enter, numpad).
fn is_printable_key(key: &Key) -> bool {
    matches!(
        key,
        Key::KeyA | Key::KeyB | Key::KeyC | Key::KeyD | Key::KeyE
            | Key::KeyF | Key::KeyG | Key::KeyH | Key::KeyI | Key::KeyJ
            | Key::KeyK | Key::KeyL | Key::KeyM | Key::KeyN | Key::KeyO
            | Key::KeyP | Key::KeyQ | Key::KeyR | Key::KeyS | Key::KeyT
            | Key::KeyU | Key::KeyV | Key::KeyW | Key::KeyX | Key::KeyY
            | Key::KeyZ
            | Key::Num0 | Key::Num1 | Key::Num2 | Key::Num3 | Key::Num4
            | Key::Num5 | Key::Num6 | Key::Num7 | Key::Num8 | Key::Num9
            | Key::Minus | Key::Equal | Key::LeftBracket | Key::RightBracket
            | Key::SemiColon | Key::Quote | Key::Comma | Key::Dot
            | Key::Slash | Key::BackSlash | Key::BackQuote
            | Key::Space | Key::Return
            | Key::Kp0 | Key::Kp1 | Key::Kp2 | Key::Kp3 | Key::Kp4
            | Key::Kp5 | Key::Kp6 | Key::Kp7 | Key::Kp8 | Key::Kp9
            | Key::KpPlus | Key::KpMinus | Key::KpMultiply
            | Key::KpDivide | Key::KpReturn
    )
}
