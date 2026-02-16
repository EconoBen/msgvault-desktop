//! Application state
//!
//! The Model in the MVU (Model-View-Update) pattern.
//! Contains all application state that determines what to render.

use crate::config::Settings;

/// Connection status with the msgvault server
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionStatus {
    /// Haven't tried connecting yet
    Unknown,
    /// Currently attempting to connect
    Connecting,
    /// Successfully connected to server
    Connected,
    /// Connection failed with error message
    Failed(String),
}

/// Root application state
#[derive(Debug, Clone)]
pub struct AppState {
    /// Current connection status
    pub connection_status: ConnectionStatus,

    /// Server URL (from config or user input)
    pub server_url: String,

    /// API key for authentication
    pub api_key: String,

    /// Whether this is the first run (no config exists)
    pub first_run: bool,
}

impl AppState {
    /// Create initial state from settings
    pub fn new(settings: &Settings) -> Self {
        Self {
            connection_status: ConnectionStatus::Unknown,
            server_url: settings.server_url.clone(),
            api_key: settings.api_key.clone(),
            first_run: settings.server_url.is_empty(),
        }
    }

    /// Check if we're currently connected
    pub fn is_connected(&self) -> bool {
        matches!(self.connection_status, ConnectionStatus::Connected)
    }

    /// Check if we're currently connecting
    pub fn is_connecting(&self) -> bool {
        matches!(self.connection_status, ConnectionStatus::Connecting)
    }
}
