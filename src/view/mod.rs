//! View logic (UI rendering)
//!
//! The View in the MVU pattern.
//! Renders the UI based on current application state.

pub mod dashboard;
pub mod widgets;

use crate::message::Message;
use crate::model::{AppState, ConnectionStatus, LoadingState, ViewLevel};
use dashboard::dashboard;
use iced::widget::{button, center, column, container, row, text, text_input, Space};
use iced::{Element, Length, Theme};
use widgets::{breadcrumb, error, loading};

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

/// Main connected view with navigation and content
fn connected_view(state: &AppState) -> Element<'_, Message> {
    // Header with app title and breadcrumb
    let header = header_view(state);

    // Main content based on loading state and current view
    let content = match &state.loading {
        LoadingState::Loading => loading("Loading..."),
        LoadingState::Error(msg) => error(msg),
        LoadingState::Idle => view_content(state),
    };

    column![header, content]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

/// Render the header with breadcrumb navigation
fn header_view(state: &AppState) -> Element<'_, Message> {
    let breadcrumbs = state.navigation.breadcrumbs();

    let title = text("msgvault").size(24);

    let breadcrumb_bar = if !breadcrumbs.is_empty() {
        breadcrumb(breadcrumbs)
    } else {
        row![].into()
    };

    let server_info = text(format!("Connected: {}", &state.server_url))
        .size(11)
        .style(|theme: &Theme| {
            let palette = theme.palette();
            text::Style {
                color: Some(iced::Color {
                    a: 0.6,
                    ..palette.text
                }),
            }
        });

    column![
        row![title, Space::with_width(Length::Fill), server_info]
            .align_y(iced::Alignment::Center)
            .padding([10, 20]),
        container(breadcrumb_bar).padding([0, 20]),
    ]
    .spacing(5)
    .into()
}

/// Render content based on current view level
fn view_content(state: &AppState) -> Element<'_, Message> {
    match state.navigation.current() {
        ViewLevel::Dashboard => {
            // Show dashboard with stats if loaded
            if let Some(stats) = &state.stats {
                dashboard(stats)
            } else {
                loading("Loading statistics...")
            }
        }
        ViewLevel::Aggregates { view_type } => {
            // Placeholder for Phase 3
            center(
                column![
                    text(format!("Aggregates: {}", view_type.display_name())).size(20),
                    Space::with_height(10),
                    text("Coming in Phase 3...").size(14),
                ]
                .align_x(iced::Alignment::Center),
            )
            .into()
        }
        ViewLevel::SubAggregates {
            parent_key,
            view_type,
            ..
        } => {
            // Placeholder for Phase 3
            center(
                column![
                    text(format!("{} → {}", parent_key, view_type.display_name())).size(20),
                    Space::with_height(10),
                    text("Coming in Phase 3...").size(14),
                ]
                .align_x(iced::Alignment::Center),
            )
            .into()
        }
        ViewLevel::Messages { filter_description } => {
            // Placeholder for Phase 4
            center(
                column![
                    text(format!("Messages: {}", filter_description)).size(20),
                    Space::with_height(10),
                    text("Coming in Phase 4...").size(14),
                ]
                .align_x(iced::Alignment::Center),
            )
            .into()
        }
        ViewLevel::MessageDetail { message_id } => {
            // Placeholder for Phase 4
            center(
                column![
                    text(format!("Message #{}", message_id)).size(20),
                    Space::with_height(10),
                    text("Coming in Phase 4...").size(14),
                ]
                .align_x(iced::Alignment::Center),
            )
            .into()
        }
    }
}

/// Truncate error messages for display
fn truncate_error(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len])
    }
}
