//! Application state modules

mod navigation;
mod state;

pub use navigation::{BreadcrumbEntry, ViewLevel};
pub use state::{AppState, ConnectionStatus, LoadingState, SettingsTab};
