//! Sync status view
//!
//! Displays sync status for all configured email accounts with
//! last sync time, next sync time, and manual sync trigger buttons.

use crate::api::types::{AccountSyncStatus, SyncState};
use crate::message::Message;
use crate::theme::{colors, components, spacing, typography};
use iced::widget::{button, column, container, row, scrollable, text, Space};
use iced::{Background, Border, Color, Element, Length};

/// Render the sync status view
pub fn sync_view<'a>(
    accounts: &'a [AccountSyncStatus],
    is_loading: bool,
    syncing_account: Option<&'a str>,
) -> Element<'a, Message> {
    // Header
    let title = text("Sync Status")
        .size(typography::SIZE_XL)
        .font(typography::FONT_MEDIUM)
        .style(components::text_primary);

    let refresh_button = button(text("Refresh").size(typography::SIZE_SM))
        .padding([spacing::SM, spacing::LG])
        .style(components::button_secondary)
        .on_press(Message::FetchSyncStatus);

    let header = row![title, Space::with_width(Length::Fill), refresh_button]
        .align_y(iced::Alignment::Center);

    // Loading indicator or content
    let content: Element<'a, Message> = if is_loading && accounts.is_empty() {
        container(
            text("Loading sync status...")
                .size(typography::SIZE_SM)
                .style(components::text_muted),
        )
        .padding(spacing::XL)
        .into()
    } else if accounts.is_empty() {
        container(
            text("No accounts configured")
                .size(typography::SIZE_SM)
                .style(components::text_muted),
        )
        .padding(spacing::XL)
        .into()
    } else {
        let account_rows: Vec<Element<'a, Message>> = accounts
            .iter()
            .map(|account| account_row(account, syncing_account))
            .collect();

        scrollable(column(account_rows).spacing(spacing::SM))
            .height(Length::Fill)
            .into()
    };

    // Keyboard hints in FONT_MONO
    let hints = text("y: refresh | Esc: back")
        .size(typography::SIZE_2XS)
        .font(typography::FONT_MONO)
        .style(components::text_muted);

    column![
        header,
        Space::with_height(spacing::XL),
        content,
        Space::with_height(spacing::SM),
        hints,
    ]
    .spacing(spacing::XS)
    .padding(spacing::XL)
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
    let account_name = text(name)
        .size(typography::SIZE_MD)
        .font(typography::FONT_MEDIUM)
        .style(components::text_primary);

    let account_email = text(&account.email)
        .size(typography::SIZE_XS)
        .style(components::text_secondary);

    // Status indicator with semantic colors
    let (status_color, status_icon) = match account.status {
        SyncState::Idle => (colors::ACCENT_SUCCESS, icons_dot()),
        SyncState::Running => (colors::ACCENT_INFO, icons_dot()),
        SyncState::Paused => (colors::ACCENT_WARNING, icons_dot()),
        SyncState::Error => (colors::ACCENT_ERROR, icons_dot()),
    };

    let status_badge = container(
        row![
            text(status_icon)
                .size(typography::SIZE_2XS)
                .style(move |_: &iced::Theme| iced::widget::text::Style {
                    color: Some(status_color),
                }),
            Space::with_width(spacing::XS),
            text(account.status.display_name())
                .size(typography::SIZE_XS)
                .style(move |_: &iced::Theme| iced::widget::text::Style {
                    color: Some(status_color),
                }),
        ]
        .align_y(iced::Alignment::Center),
    )
    .padding([spacing::SPACE_1, spacing::SM])
    .style(move |_| container::Style {
        background: Some(Background::Color(Color {
            a: 0.12,
            ..status_color
        })),
        border: Border {
            radius: spacing::RADIUS_SM.into(),
            ..Default::default()
        },
        ..Default::default()
    });

    // Sync times in FONT_MONO
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
        text(last_sync)
            .size(typography::SIZE_XS)
            .font(typography::FONT_MONO)
            .style(components::text_muted),
        text(next_sync)
            .size(typography::SIZE_XS)
            .font(typography::FONT_MONO)
            .style(components::text_muted),
    ]
    .spacing(spacing::SPACE_1);

    // Sync button
    let sync_button: Element<'a, Message> = if is_syncing {
        text("Syncing...")
            .size(typography::SIZE_SM)
            .style(components::text_accent)
            .into()
    } else {
        button(text("Sync Now").size(typography::SIZE_SM))
            .padding([spacing::SM, spacing::LG])
            .style(components::button_primary)
            .on_press(Message::TriggerSync(account.email.clone()))
            .into()
    };

    // Error message if any
    let error_row: Element<'a, Message> = if let Some(ref err) = account.error {
        text(format!("Error: {}", err))
            .size(typography::SIZE_XS)
            .style(components::text_error)
            .into()
    } else {
        Space::new(0, 0).into()
    };

    // Progress info
    let progress_info: Element<'a, Message> = if let Some(count) = account.messages_synced {
        text(format!("{} messages synced", count))
            .size(typography::SIZE_XS)
            .style(components::text_secondary)
            .into()
    } else {
        Space::new(0, 0).into()
    };

    let left_col = column![
        account_name,
        account_email,
        Space::with_height(spacing::XS),
        status_badge,
        progress_info,
        error_row,
    ]
    .spacing(spacing::SPACE_1)
    .width(Length::FillPortion(3));

    let right_col = column![times, Space::with_height(spacing::SM), sync_button]
        .spacing(spacing::XS)
        .width(Length::FillPortion(2))
        .align_x(iced::Alignment::End);

    let row_content = row![left_col, right_col]
        .spacing(spacing::XL)
        .padding(spacing::LG);

    container(row_content)
        .style(components::card_style)
        .width(Length::Fill)
        .into()
}

/// Status dot indicator
fn icons_dot() -> &'static str {
    crate::theme::icons::DOT_FILLED
}

/// Format a timestamp for display
fn format_time(timestamp: &str) -> String {
    // Try to parse and format nicely, fall back to raw string
    if timestamp.len() > 16 {
        // Truncate to "YYYY-MM-DD HH:MM"
        timestamp[..16].replace('T', " ")
    } else {
        timestamp.to_string()
    }
}
