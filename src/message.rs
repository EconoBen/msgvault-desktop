//! Message types for the Iced application
//!
//! All user interactions and async events are represented as Messages.
//! The update function processes these to modify application state.

use crate::api::types::{
    AggregateResponse, DeviceFlowStatus, HealthResponse, MessageDetail, MessageListResponse,
    OAuthInitResponse, RemoveAccountResponse, SchedulerStatus, SearchResponse, StatsResponse,
    SyncTriggerResponse, ViewType,
};
use crate::error::AppError;
use crate::model::{SettingsTab, ViewLevel};

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

    // === Search ===
    /// Open search view
    OpenSearch,
    /// Search query changed (debounced)
    SearchQueryChanged(String),
    /// Execute the search
    ExecuteSearch,
    /// Search results loaded
    SearchLoaded(Result<SearchResponse, AppError>),
    /// Toggle between fast/deep search mode
    ToggleSearchMode,
    /// Select a search result
    SelectSearchResult(usize),
    /// Open selected search result
    OpenSearchResult,

    // === Sync ===
    /// Open sync status view
    OpenSync,
    /// Fetch scheduler status
    FetchSyncStatus,
    /// Scheduler status loaded
    SyncStatusLoaded(Result<SchedulerStatus, AppError>),
    /// Trigger manual sync for an account
    TriggerSync(String),
    /// Manual sync triggered response
    SyncTriggered(Result<SyncTriggerResponse, AppError>),
    /// Refresh sync status (polling)
    RefreshSyncStatus,

    // === Account Management ===
    /// Open accounts view
    OpenAccounts,
    /// Email input changed for add account
    AddAccountEmailChanged(String),
    /// Start add account flow (initiate OAuth)
    StartAddAccount,
    /// OAuth initiation response
    OAuthInitiated(Result<OAuthInitResponse, AppError>),
    /// Open browser for OAuth
    OpenOAuthBrowser(String),
    /// Poll device flow status
    PollDeviceFlow,
    /// Device flow status received
    DeviceFlowStatusReceived(Result<DeviceFlowStatus, AppError>),
    /// Cancel add account flow
    CancelAddAccount,
    /// Show remove account confirmation
    ShowRemoveAccountModal(String),
    /// Hide remove account modal
    HideRemoveAccountModal,
    /// Confirm remove account
    ConfirmRemoveAccount,
    /// Account removed response
    AccountRemoved(Result<RemoveAccountResponse, AppError>),

    // === Help ===
    /// Show keyboard shortcuts help
    ShowHelp,
    /// Hide help modal
    HideHelp,

    // === Settings ===
    /// Open settings view
    OpenSettings,
    /// Switch settings tab
    SwitchSettingsTab(SettingsTab),
    /// Settings server URL changed
    SettingsServerUrlChanged(String),
    /// Settings API key changed
    SettingsApiKeyChanged(String),
    /// Test connection
    TestConnection,
    /// Connection test result
    ConnectionTested(Result<HealthResponse, AppError>),
    /// Save settings
    SaveSettings,
    /// Settings saved
    SettingsSaved(Result<(), String>),

    // === Selection ===
    /// Toggle selection of current message (Space key)
    ToggleSelection,
    /// Select all visible messages (Shift+A key)
    SelectAll,
    /// Clear all selections (x key)
    ClearSelection,
    /// Show delete confirmation modal (d key)
    ShowDeleteModal,
    /// Hide delete confirmation modal
    HideDeleteModal,
    /// Confirm deletion of selected messages
    ConfirmDelete,
    /// Stage selected messages for deletion
    StageForDeletion,

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
