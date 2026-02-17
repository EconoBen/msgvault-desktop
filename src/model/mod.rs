//! Application state modules

pub mod compose;
mod navigation;
mod state;

pub use compose::{format_quoted_body, AttachmentDraft, ComposeMode, ComposeState};
pub use navigation::{BreadcrumbEntry, ViewLevel};
pub use state::{AppState, ConnectionStatus, LoadingState, SettingsTab, WizardStep};
