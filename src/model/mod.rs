//! Application state modules

pub mod compose;
pub mod downloads;
mod navigation;
mod state;
pub mod thread;

pub use compose::{format_quoted_body, AttachmentDraft, ComposeMode, ComposeState};
pub use downloads::{DownloadState, DownloadTracker};
pub use navigation::{BreadcrumbEntry, ViewLevel};
pub use state::{AppState, ConnectionStatus, LoadingState, SettingsTab, WizardStep};
pub use thread::ThreadState;
