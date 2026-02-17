//! API response types
//!
//! These types mirror the Go server's JSON responses.
//! See msgvault/internal/query/models.go for the source definitions.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Health check response
#[derive(Debug, Clone, Deserialize)]
pub struct HealthResponse {
    pub status: String,
}

/// Archive statistics response from /api/v1/stats
#[derive(Debug, Clone, Deserialize)]
pub struct StatsResponse {
    pub total_messages: i64,
    pub total_threads: i64,
    pub total_accounts: i64,
    pub total_labels: i64,
    pub total_attachments: i64,
    pub database_size_bytes: i64,
}

/// Single row in an aggregate view
#[derive(Debug, Clone, Deserialize)]
pub struct AggregateRow {
    pub key: String,
    pub count: i64,
    pub total_size: i64,
    pub attachment_size: i64,
    pub attachment_count: i64,
    pub total_unique: i64,
}

/// Aggregate response from /api/v1/aggregates
#[derive(Debug, Clone, Deserialize)]
pub struct AggregateResponse {
    pub view_type: String,
    pub rows: Vec<AggregateRow>,
}

/// Message summary for list views
#[derive(Debug, Clone, Deserialize)]
pub struct MessageSummary {
    pub id: i64,
    pub subject: String,
    pub snippet: String,
    #[serde(rename = "from")]
    pub from_email: String,
    #[serde(default)]
    pub from_name: Option<String>,
    pub sent_at: DateTime<Utc>,
    pub size_bytes: i64,
    pub has_attachments: bool,
    #[serde(default)]
    pub labels: Vec<String>,
}

/// Email address with optional name
#[derive(Debug, Clone, Deserialize)]
pub struct Address {
    pub email: String,
    #[serde(default)]
    pub name: Option<String>,
}

/// Attachment metadata
#[derive(Debug, Clone, Deserialize)]
pub struct Attachment {
    pub filename: String,
    pub mime_type: String,
    pub size_bytes: i64,
}

/// Full message detail from /api/v1/messages/{id}
#[derive(Debug, Clone, Deserialize)]
pub struct MessageDetail {
    pub id: i64,
    pub subject: String,
    #[serde(rename = "from")]
    pub from_addr: String,
    pub to: Vec<String>,
    #[serde(default)]
    pub cc: Vec<String>,
    #[serde(default)]
    pub bcc: Vec<String>,
    pub sent_at: DateTime<Utc>,
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub labels: Vec<String>,
    #[serde(default)]
    pub attachments: Vec<Attachment>,
}

/// Paginated message list response
#[derive(Debug, Clone, Deserialize)]
pub struct MessageListResponse {
    pub total: i64,
    pub page: i32,
    pub page_size: i32,
    pub messages: Vec<MessageSummary>,
}

/// Search response from /api/v1/search/*
#[derive(Debug, Clone, Deserialize)]
pub struct SearchResponse {
    pub query: String,
    pub total: i64,
    #[serde(default)]
    pub messages: Vec<MessageSummary>,
}

/// Account info from /api/v1/accounts
#[derive(Debug, Clone, Deserialize)]
pub struct AccountInfo {
    pub email: String,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub last_sync_at: Option<String>,
    #[serde(default)]
    pub next_sync_at: Option<String>,
    #[serde(default)]
    pub schedule: Option<String>,
    pub enabled: bool,
}

/// Accounts list response
#[derive(Debug, Clone, Deserialize)]
pub struct AccountsResponse {
    pub accounts: Vec<AccountInfo>,
}

/// Scheduler status for all accounts
#[derive(Debug, Clone, Deserialize)]
pub struct SchedulerStatus {
    pub accounts: Vec<AccountSyncStatus>,
}

/// Sync status for a single account
#[derive(Debug, Clone, Deserialize)]
pub struct AccountSyncStatus {
    pub email: String,
    #[serde(default)]
    pub display_name: Option<String>,
    pub status: SyncState,
    #[serde(default)]
    pub last_sync_at: Option<String>,
    #[serde(default)]
    pub next_sync_at: Option<String>,
    #[serde(default)]
    pub messages_synced: Option<i64>,
    #[serde(default)]
    pub error: Option<String>,
}

/// Possible sync states
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncState {
    Idle,
    Running,
    Paused,
    Error,
}

impl SyncState {
    pub fn display_name(&self) -> &'static str {
        match self {
            SyncState::Idle => "Idle",
            SyncState::Running => "Syncing",
            SyncState::Paused => "Paused",
            SyncState::Error => "Error",
        }
    }
}

/// Response from triggering a sync
#[derive(Debug, Clone, Deserialize)]
pub struct SyncTriggerResponse {
    pub message: String,
}

/// Response from initiating OAuth flow
#[derive(Debug, Clone, Deserialize)]
pub struct OAuthInitResponse {
    /// URL to open in browser for OAuth
    pub auth_url: String,
    /// Whether this is a device flow (show code instead of browser)
    #[serde(default)]
    pub device_flow: bool,
    /// User code for device flow
    #[serde(default)]
    pub user_code: Option<String>,
    /// Verification URL for device flow
    #[serde(default)]
    pub verification_url: Option<String>,
    /// Interval to poll for device flow completion (seconds)
    #[serde(default)]
    pub poll_interval: Option<i32>,
}

/// Device flow status
#[derive(Debug, Clone, Deserialize)]
pub struct DeviceFlowStatus {
    pub status: DeviceFlowState,
    #[serde(default)]
    pub error: Option<String>,
}

/// Device flow states
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeviceFlowState {
    Pending,
    Complete,
    Expired,
    Error,
}

/// Response from removing an account
#[derive(Debug, Clone, Deserialize)]
pub struct RemoveAccountResponse {
    pub message: String,
}

/// View types for aggregation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ViewType {
    Senders,
    SenderNames,
    Recipients,
    RecipientNames,
    Domains,
    Labels,
    Time,
}

impl ViewType {
    /// API parameter value
    pub fn as_str(&self) -> &'static str {
        match self {
            ViewType::Senders => "senders",
            ViewType::SenderNames => "sender_names",
            ViewType::Recipients => "recipients",
            ViewType::RecipientNames => "recipient_names",
            ViewType::Domains => "domains",
            ViewType::Labels => "labels",
            ViewType::Time => "time",
        }
    }

    /// Human-readable display name
    pub fn display_name(&self) -> &'static str {
        match self {
            ViewType::Senders => "Senders",
            ViewType::SenderNames => "Sender Names",
            ViewType::Recipients => "Recipients",
            ViewType::RecipientNames => "Recipient Names",
            ViewType::Domains => "Domains",
            ViewType::Labels => "Labels",
            ViewType::Time => "Time",
        }
    }

    /// Get the next view type (for Tab cycling)
    pub fn next(&self) -> Self {
        match self {
            ViewType::Senders => ViewType::SenderNames,
            ViewType::SenderNames => ViewType::Recipients,
            ViewType::Recipients => ViewType::RecipientNames,
            ViewType::RecipientNames => ViewType::Domains,
            ViewType::Domains => ViewType::Labels,
            ViewType::Labels => ViewType::Time,
            ViewType::Time => ViewType::Senders,
        }
    }

    /// Get the previous view type (for Shift+Tab)
    pub fn previous(&self) -> Self {
        match self {
            ViewType::Senders => ViewType::Time,
            ViewType::SenderNames => ViewType::Senders,
            ViewType::Recipients => ViewType::SenderNames,
            ViewType::RecipientNames => ViewType::Recipients,
            ViewType::Domains => ViewType::RecipientNames,
            ViewType::Labels => ViewType::Domains,
            ViewType::Time => ViewType::Labels,
        }
    }

    /// All view types in order
    pub fn all() -> &'static [ViewType] {
        &[
            ViewType::Senders,
            ViewType::SenderNames,
            ViewType::Recipients,
            ViewType::RecipientNames,
            ViewType::Domains,
            ViewType::Labels,
            ViewType::Time,
        ]
    }
}

/// Sort fields for aggregates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SortField {
    #[default]
    Count,
    Size,
    AttachmentSize,
    Name,
}

impl SortField {
    pub fn as_str(&self) -> &'static str {
        match self {
            SortField::Count => "count",
            SortField::Size => "size",
            SortField::AttachmentSize => "attachment_size",
            SortField::Name => "name",
        }
    }
}

/// Sort direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SortDirection {
    #[default]
    Desc,
    Asc,
}

impl SortDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            SortDirection::Desc => "desc",
            SortDirection::Asc => "asc",
        }
    }
}
