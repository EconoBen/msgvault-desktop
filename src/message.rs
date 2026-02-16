//! Message types for the Iced application
//!
//! All user interactions and async events are represented as Messages.
//! The update function processes these to modify application state.

use crate::api::types::HealthResponse;
use crate::error::AppError;

/// All possible messages in the application
#[derive(Debug, Clone)]
pub enum Message {
    // === Connection ===
    /// Check server health on startup
    CheckHealth,
    /// Health check completed
    HealthChecked(Result<HealthResponse, AppError>),

    // === Navigation (placeholder for Phase 2) ===
    // NavigateTo(ViewLevel),
    // GoBack,

    // === User Input ===
    /// Server URL changed in settings
    ServerUrlChanged(String),
    /// API key changed in settings
    ApiKeyChanged(String),
    /// Retry connection button pressed
    RetryConnection,

    // === Keyboard ===
    /// A key was pressed
    KeyPressed(iced::keyboard::Key),

    // === No-op ===
    /// Message that does nothing (used for unhandled events)
    None,
}
