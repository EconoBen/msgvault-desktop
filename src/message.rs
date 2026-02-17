//! Message types for the Iced application
//!
//! All user interactions and async events are represented as Messages.
//! The update function processes these to modify application state.

use crate::api::types::{
    AggregateResponse, HealthResponse, MessageDetail, MessageListResponse, StatsResponse, ViewType,
};
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

    // === Aggregates ===
    /// Fetch aggregates for current view type
    FetchAggregates(ViewType),
    /// Aggregates loaded
    AggregatesLoaded(Result<AggregateResponse, AppError>),
    /// Select an aggregate row by index
    SelectAggregate(usize),
    /// Move selection up
    SelectPrevious,
    /// Move selection down
    SelectNext,
    /// Drill down into selected aggregate
    DrillDown,
    /// Toggle sort field (name -> count -> size)
    ToggleSortField,
    /// Toggle sort direction
    ToggleSortDirection,

    // === Messages ===
    /// Fetch messages with filter
    FetchMessages {
        filter_type: String,
        filter_value: String,
    },
    /// Messages loaded
    MessagesLoaded(Result<MessageListResponse, AppError>),
    /// Select a message in the list
    SelectMessage(usize),
    /// Open the selected message (view detail)
    OpenMessage,
    /// Message detail loaded
    MessageDetailLoaded(Result<MessageDetail, AppError>),
    /// Go to next page of messages
    NextPage,
    /// Go to previous page of messages
    PreviousPage,
    /// Navigate to previous message in list
    PreviousMessage,
    /// Navigate to next message in list
    NextMessage,

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
