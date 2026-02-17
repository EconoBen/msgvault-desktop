//! Message types for the Iced application
//!
//! All user interactions and async events are represented as Messages.
//! The update function processes these to modify application state.

use crate::api::types::{
    AggregateResponse, DeviceFlowStatus, HealthResponse, MessageDetail, MessageListResponse,
    OAuthInitResponse, RemoveAccountResponse, SchedulerStatus, SearchResponse, StatsResponse,
    SyncTriggerResponse, ViewType,
};
use crate::config::DiscoveryResult;
use crate::error::AppError;
use crate::model::{SettingsTab, ViewLevel};

/// All possible messages in the application
#[derive(Debug, Clone)]
pub enum Message {
    // === Discovery ===
    /// Start server discovery
    StartDiscovery,
    /// Discovery completed
    DiscoveryComplete(DiscoveryResult),
    /// User confirmed discovered server
    ConfirmDiscoveredServer,
    /// User chose manual entry
    ChooseManualEntry,
    /// Wizard server URL changed
    WizardServerUrlChanged(String),
    /// Wizard API key changed
    WizardApiKeyChanged(String),
    /// User finished wizard
    FinishWizard,

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

    // === Threading ===
    /// View full thread for current message
    ViewThread(String),
    /// Thread messages loaded
    ThreadLoaded(Result<Vec<MessageDetail>, AppError>),
    /// Toggle expand/collapse of a thread message
    ToggleThreadMessage(usize),
    /// Expand all messages in thread
    ExpandAllThread,
    /// Collapse all messages in thread
    CollapseAllThread,
    /// Focus previous message in thread
    ThreadFocusPrevious,
    /// Focus next message in thread
    ThreadFocusNext,

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

    // === Attachments ===
    /// Download an attachment
    DownloadAttachment {
        message_id: i64,
        attachment_idx: usize,
        filename: String,
    },
    /// Download progress update
    DownloadProgress {
        message_id: i64,
        attachment_idx: usize,
        progress: f32,
    },
    /// Download completed successfully
    DownloadComplete {
        message_id: i64,
        attachment_idx: usize,
        path: std::path::PathBuf,
    },
    /// Download failed
    DownloadFailed {
        message_id: i64,
        attachment_idx: usize,
        error: String,
    },
    /// Open a downloaded file
    OpenFile(std::path::PathBuf),

    // === Compose ===
    /// Open compose for new email
    OpenCompose,
    /// Open compose as reply to message
    OpenReply(i64),
    /// Open compose as reply-all to message
    OpenReplyAll(i64),
    /// Open compose as forward of message
    OpenForward(i64),
    /// To field input changed
    ComposeToChanged(String),
    /// Add recipient to To field
    ComposeAddTo,
    /// Remove recipient from To field
    ComposeRemoveTo(usize),
    /// CC field input changed
    ComposeCcChanged(String),
    /// Add recipient to CC field
    ComposeAddCc,
    /// Remove recipient from CC field
    ComposeRemoveCc(usize),
    /// BCC field input changed
    ComposeBccChanged(String),
    /// Add recipient to BCC field
    ComposeAddBcc,
    /// Remove recipient from BCC field
    ComposeRemoveBcc(usize),
    /// Subject changed
    ComposeSubjectChanged(String),
    /// Body changed
    ComposeBodyChanged(String),
    /// From account changed
    ComposeFromChanged(String),
    /// Toggle CC/BCC visibility
    ComposeToggleCcBcc,
    /// Add attachment
    ComposeAddAttachment,
    /// Attachment file selected
    ComposeAttachmentSelected(std::path::PathBuf),
    /// Remove attachment
    ComposeRemoveAttachment(usize),
    /// Send the email
    ComposeSend,
    /// Email sent result
    ComposeSent(Result<(), crate::error::AppError>),
    /// Save as draft
    ComposeSaveDraft,
    /// Draft saved result
    ComposeDraftSaved(Result<i64, crate::error::AppError>),
    /// Discard and close compose
    ComposeDiscard,
    /// Close compose (with confirmation if dirty)
    ComposeClose,

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
