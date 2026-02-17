//! Application state
//!
//! The Model in the MVU (Model-View-Update) pattern.
//! Contains all application state that determines what to render.

use crate::api::types::StatsResponse;
use crate::config::Settings;
use crate::model::navigation::NavigationStack;

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

/// Loading state for async operations
#[derive(Debug, Clone, PartialEq, Default)]
pub enum LoadingState {
    #[default]
    Idle,
    Loading,
    Error(String),
}

impl LoadingState {
    pub fn is_loading(&self) -> bool {
        matches!(self, LoadingState::Loading)
    }
}

/// Root application state
#[derive(Debug, Clone)]
pub struct AppState {
    // === Connection ===
    /// Current connection status
    pub connection_status: ConnectionStatus,
    /// Server URL (from config or user input)
    pub server_url: String,
    /// API key for authentication
    pub api_key: String,
    /// Whether this is the first run (no config exists)
    pub first_run: bool,

    // === Navigation ===
    /// Navigation stack (breadcrumbs, history)
    pub navigation: NavigationStack,

    // === Data ===
    /// Archive statistics (loaded on connect)
    pub stats: Option<StatsResponse>,
    /// Loading state for current data fetch
    pub loading: LoadingState,
}

impl AppState {
    /// Create initial state from settings
    pub fn new(settings: &Settings) -> Self {
        Self {
            // Connection
            connection_status: ConnectionStatus::Unknown,
            server_url: settings.server_url.clone(),
            api_key: settings.api_key.clone(),
            first_run: settings.server_url.is_empty(),

            // Navigation
            navigation: NavigationStack::new(),

            // Data
            stats: None,
            loading: LoadingState::Idle,
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

    /// Get window title based on current state
    pub fn window_title(&self) -> String {
        if !self.is_connected() {
            return "msgvault".to_string();
        }

        let view_title = self.navigation.current().title();
        format!("msgvault - {}", view_title)
    }
}
