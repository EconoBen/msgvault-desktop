//! View logic (UI rendering)
//!
//! The View in the MVU pattern.
//! Renders the UI based on current application state.

pub mod widgets;

use crate::message::Message;
use crate::model::{AppState, ConnectionStatus};
use iced::widget::{button, center, column, container, row, text, text_input, Space};
use iced::{Element, Length, Theme};

/// Render the application view based on current state
pub fn render(state: &AppState) -> Element<'_, Message> {
    let content = if state.first_run || !state.is_connected() {
        // Show connection/setup view
        connection_view(state)
    } else {
        // Show main application view (placeholder for Phase 2)
        connected_view(state)
    };

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(20)
        .into()
}

/// Connection setup view - shown on first run or when disconnected
fn connection_view(state: &AppState) -> Element<'_, Message> {
    let title = text("msgvault").size(32);

    let subtitle = text("Connect to your msgvault server").size(16);

    let url_input = text_input(
        "Server URL (e.g., http://localhost:8080)",
        &state.server_url,
    )
    .on_input(Message::ServerUrlChanged)
    .padding(10)
    .width(Length::Fixed(400.0));

    let api_key_input = text_input("API Key (optional)", &state.api_key)
        .on_input(Message::ApiKeyChanged)
        .padding(10)
        .width(Length::Fixed(400.0))
        .secure(true);

    let connect_button = button(text("Connect"))
        .padding([10, 20])
        .on_press(Message::CheckHealth);

    let status_text = match &state.connection_status {
        ConnectionStatus::Unknown => text(""),
        ConnectionStatus::Connecting => text("Connecting..."),
        ConnectionStatus::Connected => text("Connected!").style(|_theme: &Theme| text::Style {
            color: Some(iced::Color::from_rgb(0.0, 0.6, 0.0)),
        }),
        ConnectionStatus::Failed(err) => text(format!("Failed: {}", truncate_error(err, 50)))
            .style(|_theme: &Theme| text::Style {
                color: Some(iced::Color::from_rgb(0.8, 0.0, 0.0)),
            }),
    };

    let form = column![
        title,
        Space::with_height(10),
        subtitle,
        Space::with_height(30),
        url_input,
        Space::with_height(10),
        api_key_input,
        Space::with_height(20),
        connect_button,
        Space::with_height(10),
        status_text,
    ]
    .spacing(5)
    .align_x(iced::Alignment::Center);

    center(form).into()
}

/// Main connected view - placeholder for Phase 2
fn connected_view(state: &AppState) -> Element<'_, Message> {
    let header = row![
        text("msgvault").size(24),
        Space::with_width(Length::Fill),
        text(format!("Connected to: {}", &state.server_url)).size(12),
    ]
    .padding(10);

    let content = center(
        column![
            text("Connected to msgvault server!").size(20),
            Space::with_height(20),
            text("Navigation and stats coming in Phase 2...").size(14),
        ]
        .align_x(iced::Alignment::Center),
    );

    column![header, content]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

/// Truncate error messages for display
fn truncate_error(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len])
    }
}
