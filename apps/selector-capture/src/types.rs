use serde::{Deserialize, Serialize};

// ── Single-capture model (used by `single` subcommand) ────

/// A single node in the UI Automation tree path
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathNode {
    pub control_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_id: Option<String>,
}

/// The flat optimal selector for the target element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BestSelector {
    pub control_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_id: Option<String>,
}

/// A single capture record (single mode)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capture {
    pub id: String,
    pub timestamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub full_path: Vec<PathNode>,
    pub best_selector: BestSelector,
}

/// Root output JSON for single captures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureOutput {
    pub tool: String,
    pub version: String,
    #[serde(default)]
    pub captures: Vec<Capture>,
}

// ── Series-recording model (used by `series` subcommand) ──

/// A UIA element as captured during action recording
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapturedElement {
    pub full_path: Vec<PathNode>,
    pub best_selector: BestSelector,
}

/// A single recorded action in a series session
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action")]
pub enum Action {
    /// User clicked a mouse button on an element
    Click {
        button: String,
        element: CapturedElement,
    },
    /// User typed text into an element (accumulated between clicks)
    Input {
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        element: Option<CapturedElement>,
    },
}

/// Root output JSON for a series recording session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeriesRecording {
    pub tool: String,
    pub version: String,
    pub timestamp_start: String,
    pub timestamp_end: String,
    pub actions: Vec<Action>,
}
