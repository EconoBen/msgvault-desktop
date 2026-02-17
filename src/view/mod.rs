//! View logic (UI rendering)
//!
//! The View in the MVU pattern.
//! Renders the UI based on current application state.

pub mod accounts;
pub mod aggregates;
pub mod dashboard;
pub mod message_detail;
pub mod messages;
pub mod search;
pub mod sync;
pub mod widgets;

pub use accounts::accounts_view;
pub use aggregates::aggregates_view;
pub use message_detail::message_detail_view;
pub use messages::messages_view;
pub use search::search_view;
pub use sync::sync_view;

use crate::message::Message;
use crate::model::{AppState, ConnectionStatus, LoadingState, ViewLevel};
use dashboard::dashboard;
use iced::widget::{button, center, column, container, row, stack, text, text_input, Space};
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

    let main_view: Element<'_, Message> = column![header, content]
        .width(Length::Fill)
        .height(Length::Fill)
        .into();

    // Overlay delete modal if showing
    if state.show_delete_modal {
        stack![
            main_view,
            delete_confirmation_modal(state.selected_messages.len())
        ]
        .into()
    } else {
        main_view
    }
}

/// Delete confirmation modal overlay
fn delete_confirmation_modal(count: usize) -> Element<'static, Message> {
    // Semi-transparent backdrop
    let backdrop = container(Space::new(Length::Fill, Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_theme: &Theme| container::Style {
            background: Some(iced::Background::Color(iced::Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.5,
            })),
            ..Default::default()
        });

    // Modal dialog
    let title = text("Confirm Delete").size(20);
    let message = text(format!(
        "Are you sure you want to stage {} message{} for deletion?",
        count,
        if count == 1 { "" } else { "s" }
    ))
    .size(14);

    let cancel_button = button(text("Cancel").size(14))
        .padding([8, 16])
        .on_press(Message::HideDeleteModal);

    let confirm_button = button(text("Delete").size(14))
        .padding([8, 16])
        .style(|theme: &iced::Theme, _status| {
            button::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgb(0.8, 0.2, 0.2))),
                text_color: iced::Color::WHITE,
                border: iced::Border {
                    radius: 4.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }
        })
        .on_press(Message::ConfirmDelete);

    let buttons = row![cancel_button, Space::with_width(10), confirm_button]
        .align_y(iced::Alignment::Center);

    let dialog_content = column![
        title,
        Space::with_height(15),
        message,
        Space::with_height(20),
        buttons,
    ]
    .spacing(5)
    .padding(20)
    .align_x(iced::Alignment::Center);

    let dialog = container(dialog_content)
        .style(|theme: &Theme| {
            let palette = theme.palette();
            container::Style {
                background: Some(iced::Background::Color(palette.background)),
                border: iced::Border {
                    radius: 8.0.into(),
                    width: 1.0,
                    color: iced::Color {
                        a: 0.3,
                        ..palette.text
                    },
                },
                ..Default::default()
            }
        })
        .padding(10);

    // Center the dialog on the backdrop
    stack![
        backdrop,
        center(dialog)
    ]
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
            // Show aggregate list view
            aggregates_view(
                view_type,
                &state.aggregates,
                state.selected_index,
                state.sort_field,
                state.sort_dir,
            )
        }
        ViewLevel::SubAggregates {
            parent_key,
            view_type,
            ..
        } => {
            // Placeholder for sub-aggregates (will use same aggregates_view with different data)
            center(
                column![
                    text(format!("{} → {}", parent_key, view_type.display_name())).size(20),
                    Space::with_height(10),
                    text("Sub-aggregates coming soon...").size(14),
                ]
                .align_x(iced::Alignment::Center),
            )
            .into()
        }
        ViewLevel::Messages { filter_description } => {
            // Show message list view
            messages_view(
                filter_description,
                &state.messages,
                state.message_selected_index,
                state.messages_offset,
                state.messages_total,
                &state.selected_messages,
            )
        }
        ViewLevel::MessageDetail { .. } => {
            // Show message detail view
            if let Some(detail) = &state.current_message {
                message_detail_view(detail)
            } else {
                loading("Loading message...")
            }
        }
        ViewLevel::Search => {
            // Show search view
            search_view(
                &state.search_query,
                state.search_deep_mode,
                &state.search_results,
                state.search_selected_index,
                state.search_total,
                state.is_searching,
                &state.selected_messages,
            )
        }
        ViewLevel::Sync => {
            // Show sync status view
            sync_view(
                &state.sync_accounts,
                state.sync_loading,
                state.syncing_account.as_deref(),
            )
        }
        ViewLevel::Accounts => {
            // Show accounts management view
            accounts_view(
                &state.sync_accounts,
                state.sync_loading,
                &state.add_account_email,
                state.adding_account,
                state.oauth_response.as_ref(),
                state.show_remove_modal,
                state.removing_account.as_deref(),
            )
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
