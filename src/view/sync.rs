//! Sync status view
//!
//! Displays sync status for all configured email accounts with
//! last sync time, next sync time, and manual sync trigger buttons.

use crate::api::types::{AccountSyncStatus, SyncState};
use crate::message::Message;
use iced::widget::{button, column, container, row, scrollable, text, Space};
use iced::{Element, Length};

/// Render the sync status view
pub fn sync_view<'a>(
    accounts: &'a [AccountSyncStatus],
    is_loading: bool,
    syncing_account: Option<&'a str>,
) -> Element<'a, Message> {
    // Header
    let title = text("Sync Status").size(24);
    let refresh_button = button(text("Refresh").size(14))
        .padding([8, 16])
        .on_press(Message::FetchSyncStatus);

    let header = row![title, Space::with_width(Length::Fill), refresh_button]
        .align_y(iced::Alignment::Center);

    // Loading indicator or content
    let content: Element<'a, Message> = if is_loading && accounts.is_empty() {
        container(text("Loading sync status...").size(14))
            .padding(20)
            .into()
    } else if accounts.is_empty() {
        container(text("No accounts configured").size(14))
            .padding(20)
            .into()
    } else {
        let account_rows: Vec<Element<'a, Message>> = accounts
            .iter()
            .map(|account| account_row(account, syncing_account))
            .collect();

        scrollable(column(account_rows).spacing(10))
            .height(Length::Fill)
            .into()
    };

    // Keyboard hints
    let hints = text("y: refresh | Esc: back").size(12);

    column![
        header,
        Space::with_height(20),
        content,
        Space::with_height(10),
        hints,
    ]
    .spacing(5)
    .padding(20)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

/// Render a single account row
fn account_row<'a>(
    account: &'a AccountSyncStatus,
    syncing_account: Option<&'a str>,
) -> Element<'a, Message> {
    let is_syncing = syncing_account == Some(&account.email)
        || account.status == SyncState::Running;

    // Account name/email
    let name = account
        .display_name
        .as_ref()
        .filter(|n| !n.is_empty())
        .unwrap_or(&account.email);
    let account_name = text(name).size(16);
    let account_email = text(&account.email).size(12).style(|theme: &iced::Theme| {
        let palette = theme.palette();
        iced::widget::text::Style {
            color: Some(iced::Color {
                a: 0.6,
                ..palette.text
            }),
        }
    });

    // Status indicator
    let status_color = match account.status {
        SyncState::Idle => iced::Color::from_rgb(0.4, 0.7, 0.4),
        SyncState::Running => iced::Color::from_rgb(0.2, 0.5, 0.9),
        SyncState::Paused => iced::Color::from_rgb(0.8, 0.7, 0.2),
        SyncState::Error => iced::Color::from_rgb(0.8, 0.2, 0.2),
    };
    let status_text = text(account.status.display_name())
        .size(12)
        .style(move |_theme: &iced::Theme| iced::widget::text::Style {
            color: Some(status_color),
        });

    // Sync times
    let last_sync = account
        .last_sync_at
        .as_ref()
        .map(|t| format!("Last: {}", format_time(t)))
        .unwrap_or_else(|| "Never synced".to_string());
    let next_sync = account
        .next_sync_at
        .as_ref()
        .map(|t| format!("Next: {}", format_time(t)))
        .unwrap_or_default();

    let times = column![
        text(last_sync).size(12),
        text(next_sync).size(12),
    ]
    .spacing(2);

    // Sync button
    let sync_button: Element<'a, Message> = if is_syncing {
        text("Syncing...").size(14).into()
    } else {
        button(text("Sync Now").size(14))
            .padding([8, 16])
            .on_press(Message::TriggerSync(account.email.clone()))
            .into()
    };

    // Error message if any
    let error_row: Element<'a, Message> = if let Some(ref err) = account.error {
        text(format!("Error: {}", err))
            .size(12)
            .style(|_theme: &iced::Theme| iced::widget::text::Style {
                color: Some(iced::Color::from_rgb(0.8, 0.2, 0.2)),
            })
            .into()
    } else {
        Space::new(0, 0).into()
    };

    // Progress info
    let progress_info: Element<'a, Message> = if let Some(count) = account.messages_synced {
        text(format!("{} messages synced", count)).size(12).into()
    } else {
        Space::new(0, 0).into()
    };

    let left_col = column![
        account_name,
        account_email,
        Space::with_height(5),
        status_text,
        progress_info,
        error_row,
    ]
    .spacing(2)
    .width(Length::FillPortion(3));

    let right_col = column![times, Space::with_height(10), sync_button]
        .spacing(5)
        .width(Length::FillPortion(2))
        .align_x(iced::Alignment::End);

    let row_content = row![left_col, right_col]
        .spacing(20)
        .padding(15);

    container(row_content)
        .style(|theme: &iced::Theme| {
            let palette = theme.palette();
            container::Style {
                background: Some(iced::Background::Color(iced::Color {
                    a: 0.05,
                    ..palette.text
                })),
                border: iced::Border {
                    radius: 8.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }
        })
        .width(Length::Fill)
        .into()
}

/// Format a timestamp for display
fn format_time(timestamp: &str) -> String {
    // Try to parse and format nicely, fall back to raw string
    // In a real app, we'd parse the ISO timestamp properly
    if timestamp.len() > 16 {
        // Truncate to "YYYY-MM-DD HH:MM"
        timestamp[..16].replace('T', " ")
    } else {
        timestamp.to_string()
    }
}
