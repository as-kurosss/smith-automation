//! selector-capture – Windows UI element selector recording utility
//!
//! Two modes:
//! - `single`  – press CTRL alone to capture the element under cursor.
//! - `series`  – automatically record every mouse click & text input;
//!               press Ctrl+Shift+F2 to stop.
//!
//! Outputs structured JSON with full tree paths and best-effort flat selectors.

#![cfg_attr(not(windows), allow(unused))]

use clap::{Parser, Subcommand};

mod types;

#[cfg(windows)]
mod capture;
#[cfg(windows)]
mod recorder;

/// Windows-only utility – compile on `x86_64-pc-windows-msvc`.
#[cfg(not(windows))]
compile_error!("selector-capture requires a Windows compilation target");

// ── CLI ───────────────────────────────────────────────────

#[derive(Parser)]
#[command(name = "selector-capture", version, about = "Capture Windows UI element selectors via hotkey")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Capture a single element and exit
    Single {
        /// Optional description for the capture
        #[arg(short, long)]
        description: Option<String>,

        /// Output JSON file
        #[arg(short, long, default_value = "selectors.json")]
        output: String,
    },

    /// Automatically record all clicks & inputs in a session; Ctrl+Shift+F2 to stop
    Series {
        /// Output JSON file
        #[arg(short, long, default_value = "selectors.json")]
        output: String,
    },
}

// ── Entry point ───────────────────────────────────────────

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Single { description, output } => {
            recorder::run_single_mode(&output, description)?;
        }
        Commands::Series { output } => {
            recorder::run_series_mode(&output)?;
        }
    }

    Ok(())
}
