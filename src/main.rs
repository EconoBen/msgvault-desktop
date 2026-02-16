//! msgvault-desktop: Native desktop app for msgvault email archive
//!
//! This application provides a graphical interface to browse, search, and manage
//! your email archive. It communicates with the msgvault server via HTTP API.

mod api;
mod app;
mod config;
mod error;
mod message;
mod model;
mod update;
mod view;

use app::MsgVaultApp;
use config::Settings;

fn main() -> iced::Result {
    // Load configuration
    let settings = Settings::load().unwrap_or_else(|e| {
        eprintln!("Warning: Could not load config: {}", e);
        Settings::default()
    });

    // Run the Iced application
    iced::application(MsgVaultApp::title, MsgVaultApp::update, MsgVaultApp::view)
        .subscription(MsgVaultApp::subscription)
        .run_with(|| MsgVaultApp::new(settings))
}
