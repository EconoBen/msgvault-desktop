//! Message types for the Iced application
//!
//! All user interactions and async events are represented as Messages.
//! The update function processes these to modify application state.

use crate::api::types::{HealthResponse, StatsResponse};
use crate::error::AppError;
use crate::model::ViewLevel;

/// All possible messages in the application
#[derive(Debug, Clone)]
pub enum Message {
    // === Connection ===
    /// Check server health on startup
    CheckHealth,
    /// Health check completed
    HealthChecked(Result<HealthResponse, AppError>),

    // === Stats ===
    /// Fetch archive statistics
    FetchStats,
    /// Stats loaded
    StatsLoaded(Result<StatsResponse, AppError>),

    // === Navigation ===
    /// Navigate to a specific view
    NavigateTo(ViewLevel),
    /// Go back to previous view
    GoBack,
    /// Jump to a breadcrumb index
    JumpToBreadcrumb(usize),
    /// Cycle to next aggregate view type (Tab key)
    NextViewType,
    /// Cycle to previous aggregate view type (Shift+Tab)
    PreviousViewType,

    // === User Input ===
    /// Server URL changed in settings
    ServerUrlChanged(String),
    /// API key changed in settings
    ApiKeyChanged(String),
    /// Retry connection button pressed
    RetryConnection,

    // === Keyboard ===
    /// A key was pressed
    KeyPressed(iced::keyboard::Key, iced::keyboard::Modifiers),

    // === No-op ===
    /// Message that does nothing (used for unhandled events)
    None,
}
