//! Application state
//!
//! The Model in the MVU (Model-View-Update) pattern.
//! Contains all application state that determines what to render.

use crate::api::types::{
    AggregateRow, MessageDetail, MessageSummary, SortDirection, SortField, StatsResponse,
};
use crate::config::Settings;
use crate::model::navigation::NavigationStack;
use std::collections::HashSet;

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

    // === Aggregates ===
    /// Current aggregate data
    pub aggregates: Vec<AggregateRow>,
    /// Currently selected row index
    pub selected_index: usize,
    /// Current sort field
    pub sort_field: SortField,
    /// Current sort direction
    pub sort_dir: SortDirection,

    // === Messages ===
    /// Current message list
    pub messages: Vec<MessageSummary>,
    /// Selected message index in list
    pub message_selected_index: usize,
    /// Current message detail (when viewing single message)
    pub current_message: Option<MessageDetail>,
    /// Pagination offset
    pub messages_offset: i64,
    /// Total messages matching filter
    pub messages_total: i64,
    /// Messages per page
    pub messages_limit: i64,
    /// Current filter type (sender, domain, label, etc.)
    pub filter_type: String,
    /// Current filter value
    pub filter_value: String,

    // === Search ===
    /// Current search query
    pub search_query: String,
    /// Whether deep search mode is enabled
    pub search_deep_mode: bool,
    /// Search results
    pub search_results: Vec<MessageSummary>,
    /// Selected result index
    pub search_selected_index: usize,
    /// Total matching results
    pub search_total: i64,
    /// Whether a search is in progress
    pub is_searching: bool,

    // === Selection ===
    /// Set of selected message IDs
    pub selected_messages: HashSet<i64>,
    /// Whether the delete confirmation modal is showing
    pub show_delete_modal: bool,
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

            // Aggregates
            aggregates: Vec::new(),
            selected_index: 0,
            sort_field: SortField::Count,
            sort_dir: SortDirection::Desc,

            // Messages
            messages: Vec::new(),
            message_selected_index: 0,
            current_message: None,
            messages_offset: 0,
            messages_total: 0,
            messages_limit: 50,
            filter_type: String::new(),
            filter_value: String::new(),

            // Search
            search_query: String::new(),
            search_deep_mode: false,
            search_results: Vec::new(),
            search_selected_index: 0,
            search_total: 0,
            is_searching: false,

            // Selection
            selected_messages: HashSet::new(),
            show_delete_modal: false,
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
