//! Messages list view
//!
//! Displays a polished scrollable list of message summaries with avatars,
//! unread indicators, and modern styling.

use crate::api::types::MessageSummary;
use crate::message::Message;
use crate::theme::{colors, components, spacing, typography};
use crate::view::widgets::{avatar, format_bytes};
use chrono::{DateTime, Datelike, Local, Utc};
use iced::widget::{column, container, row, scrollable, text, Space};
use iced::{Background, Border, Element, Length};
use std::collections::HashSet;

/// Render the messages list view
pub fn messages_view<'a>(
    filter_description: String,
    messages: &'a [MessageSummary],
    selected_index: usize,
    offset: i64,
    total: i64,
    selected_messages: &'a HashSet<i64>,
) -> Element<'a, Message> {
    // Header with filter description and counts
    let header = header_section(filter_description, offset, messages.len(), total, selected_messages.len());

    // Message list
    let list_content: Element<'a, Message> = if messages.is_empty() {
        empty_state()
    } else {
        let rows: Vec<Element<'a, Message>> = messages
            .iter()
            .enumerate()
            .map(|(i, msg)| {
                message_row(
                    msg,
                    i == selected_index,
                    selected_messages.contains(&msg.id),
                )
            })
            .collect();

        scrollable(column(rows).spacing(1))
            .height(Length::Fill)
            .into()
    };

    // Pagination and hints
    let footer = footer_section(offset, messages.len(), total);

    column![
        header,
        Space::with_height(spacing::MD),
        list_content,
        Space::with_height(spacing::MD),
        footer,
    ]
    .spacing(spacing::XS)
    .padding(spacing::LG)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

/// Header section with title and counts
fn header_section(
    filter_description: String,
    offset: i64,
    page_count: usize,
    total: i64,
    selection_count: usize,
) -> Element<'static, Message> {
    let title = text(filter_description)
        .size(typography::SIZE_LG)
        .style(components::text_primary);

    let selection_badge: Element<'static, Message> = if selection_count > 0 {
        container(
            text(format!("{} selected", selection_count))
                .size(typography::SIZE_XS)
                .style(components::text_accent)
        )
        .padding([2, spacing::SM])
        .style(|_| container::Style {
            background: Some(Background::Color(colors::with_alpha(colors::ACCENT_PRIMARY, 0.15))),
            border: Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .into()
    } else {
        Space::with_width(0).into()
    };

    let count_text = text(format!(
        "{}-{} of {}",
        offset + 1,
        (offset + page_count as i64).min(total),
        total
    ))
    .size(typography::SIZE_XS)
    .style(components::text_muted);

    row![
        title,
        Space::with_width(Length::Fill),
        selection_badge,
        Space::with_width(spacing::MD),
        count_text,
    ]
    .align_y(iced::Alignment::Center)
    .into()
}

/// Empty state when no messages
fn empty_state<'a>() -> Element<'a, Message> {
    container(
        column![
            text("No messages").size(typography::SIZE_MD).style(components::text_secondary),
            Space::with_height(spacing::XS),
            text("Try adjusting your filters")
                .size(typography::SIZE_SM)
                .style(components::text_muted),
        ]
        .align_x(iced::Alignment::Center)
    )
    .width(Length::Fill)
    .padding(spacing::XXL)
    .into()
}

/// Single message row with avatar and modern styling
fn message_row<'a>(msg: &'a MessageSummary, is_focused: bool, is_selected: bool) -> Element<'a, Message> {
    // Determine display name
    let display_name = msg
        .from_name
        .as_ref()
        .filter(|n| !n.is_empty())
        .map(|n| n.as_str())
        .unwrap_or(&msg.from_email);

    // Avatar
    let avatar_widget = avatar(display_name, 40);

    // Unread indicator (placeholder - would need is_unread field)
    // TODO: Add unread indicator when is_unread field is available
    let _unread_indicator: Element<'a, Message> = Space::with_width(spacing::XS).into();

    // Selection checkbox
    let checkbox: Element<'a, Message> = if is_selected {
        container(text("✓").size(typography::SIZE_XS))
            .width(Length::Fixed(20.0))
            .height(Length::Fixed(20.0))
            .style(|_| container::Style {
                background: Some(Background::Color(colors::ACCENT_PRIMARY)),
                border: Border {
                    radius: 4.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .into()
    } else {
        container(text("").size(typography::SIZE_XS))
            .width(Length::Fixed(20.0))
            .height(Length::Fixed(20.0))
            .style(|_| container::Style {
                background: Some(Background::Color(colors::BG_ELEVATED)),
                border: Border {
                    radius: 4.0.into(),
                    width: 1.0,
                    color: colors::BORDER_SUBTLE,
                },
                ..Default::default()
            })
            .into()
    };

    // Main content
    let sender_name = text(truncate_string(display_name, 25))
        .size(typography::SIZE_SM)
        .style(components::text_primary);

    let subject_text = text(truncate_string(&msg.subject, 60))
        .size(typography::SIZE_SM)
        .style(if is_focused {
            components::text_primary
        } else {
            components::text_secondary
        });

    let date_text = text(format_relative_time(&msg.sent_at))
        .size(typography::SIZE_XS)
        .style(components::text_muted);

    // Attachment indicator
    let attachment: Element<'a, Message> = if msg.has_attachments {
        text("📎")
            .size(typography::SIZE_XS)
            .style(components::text_muted)
            .into()
    } else {
        Space::with_width(0).into()
    };

    // Size
    let size_text = text(format_bytes(msg.size_bytes))
        .size(typography::SIZE_XS)
        .style(components::text_muted);

    // Content layout
    let content = column![
        row![
            sender_name,
            Space::with_width(Length::Fill),
            date_text,
        ].align_y(iced::Alignment::Center),
        Space::with_height(2),
        row![
            subject_text,
            Space::with_width(Length::Fill),
            attachment,
            Space::with_width(spacing::SM),
            size_text,
        ].align_y(iced::Alignment::Center),
    ]
    .width(Length::Fill);

    // Row layout
    let row_content = row![
        checkbox,
        Space::with_width(spacing::SM),
        avatar_widget,
        Space::with_width(spacing::MD),
        content,
    ]
    .align_y(iced::Alignment::Center)
    .padding([spacing::SM, spacing::MD]);

    // Apply styling based on state
    let bg_color = if is_focused {
        colors::SELECTION_BG
    } else if is_selected {
        colors::with_alpha(colors::ACCENT_PRIMARY, 0.08)
    } else {
        colors::BG_SURFACE
    };

    container(row_content)
        .width(Length::Fill)
        .style(move |_| container::Style {
            background: Some(Background::Color(bg_color)),
            border: Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .into()
}

/// Footer with pagination and keyboard hints
fn footer_section(offset: i64, _page_count: usize, total: i64) -> Element<'static, Message> {
    let pagination = text(format!(
        "Page {} of {}",
        (offset / 50) + 1,
        (total / 50) + 1
    ))
    .size(typography::SIZE_XS)
    .style(components::text_muted);

    let hints = text("j/k: navigate • Enter: open • Space: select • d: delete • n/p: pages")
        .size(typography::SIZE_XS)
        .style(components::text_muted);

    row![
        pagination,
        Space::with_width(Length::Fill),
        hints,
    ]
    .align_y(iced::Alignment::Center)
    .into()
}

/// Format datetime as relative time (Today, Yesterday, or date)
fn format_relative_time(dt: &DateTime<Utc>) -> String {
    let local: DateTime<Local> = dt.with_timezone(&Local);
    let now = Local::now();

    // If today, show time
    if local.date_naive() == now.date_naive() {
        return local.format("%H:%M").to_string();
    }

    // If yesterday
    let yesterday = now.date_naive().pred_opt().unwrap_or(now.date_naive());
    if local.date_naive() == yesterday {
        return "Yesterday".to_string();
    }

    // If this week (within 7 days)
    let days_ago = (now.date_naive() - local.date_naive()).num_days();
    if days_ago < 7 {
        return local.format("%A").to_string(); // Day name
    }

    // If this year
    if local.year() == now.year() {
        return local.format("%b %d").to_string();
    }

    // Otherwise full date
    local.format("%b %d, %Y").to_string()
}

/// Truncate a string with ellipsis
fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}
