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
pub mod theme;
mod update;
mod view;

use app::MsgVaultApp;
use config::Settings;
use iced::theme::{Custom, Palette};
use iced::{Color, Theme};

/// Create the msgvault custom dark theme inspired by Zed One Dark
fn msgvault_theme() -> Theme {
    Theme::Custom(std::sync::Arc::new(Custom::new(
        "msgvault Dark".to_string(),
        Palette {
            background: theme::colors::BG_BASE,
            text: theme::colors::TEXT_PRIMARY,
            primary: theme::colors::ACCENT_PRIMARY,
            success: theme::colors::ACCENT_SUCCESS,
            danger: theme::colors::ACCENT_ERROR,
        },
    )))
}

fn main() -> iced::Result {
    // Load configuration
    let settings = Settings::load().unwrap_or_else(|e| {
        eprintln!("Warning: Could not load config: {}", e);
        Settings::default()
    });

    // Run the Iced application with custom theme
    iced::application(MsgVaultApp::title, MsgVaultApp::update, MsgVaultApp::view)
        .subscription(MsgVaultApp::subscription)
        .theme(|_| msgvault_theme())
        .run_with(|| MsgVaultApp::new(settings))
}
