//! Messages list view
//!
//! Displays a 3-line message list with Foundry Dark styling.
//! Each row shows sender + time, subject + attachment, and snippet.

use crate::api::types::MessageSummary;
use crate::message::Message;
use crate::theme::{colors, components, icons, spacing, typography};
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
    let header = header_section(
        filter_description,
        offset,
        messages.len(),
        total,
        selected_messages.len(),
    );

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

    column![header, Space::with_height(spacing::SM), list_content, footer,]
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
        .font(typography::FONT_MEDIUM)
        .style(components::text_primary);

    let selection_badge: Element<'static, Message> = if selection_count > 0 {
        container(
            text(format!("{} selected", selection_count))
                .size(typography::SIZE_XS)
                .style(components::text_accent),
        )
        .padding([2, spacing::SM])
        .style(|_| container::Style {
            background: Some(Background::Color(colors::with_alpha(
                colors::ACCENT_PRIMARY,
                0.15,
            ))),
            border: Border {
                radius: spacing::RADIUS_SM.into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .into()
    } else {
        Space::with_width(0).into()
    };

    let count_text = text(format!(
        "{}\u{2013}{} of {}",
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
            text(icons::DIAMOND_SM)
                .size(typography::SIZE_2XL)
                .style(components::text_muted),
            Space::with_height(spacing::MD),
            text("No messages")
                .size(typography::SIZE_MD)
                .font(typography::FONT_SEMIBOLD)
                .style(components::text_secondary),
            Space::with_height(spacing::XS),
            text("Try adjusting your filters or browsing a different view")
                .size(typography::SIZE_SM)
                .style(components::text_muted),
        ]
        .spacing(spacing::XS)
        .align_x(iced::Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .padding(spacing::XXL)
    .into()
}

/// Single message row — 3-line layout with focus/selection states
fn message_row<'a>(
    msg: &'a MessageSummary,
    is_focused: bool,
    is_selected: bool,
) -> Element<'a, Message> {
    // Determine display name
    let display_name = msg
        .from_name
        .as_ref()
        .filter(|n| !n.is_empty())
        .map(|n| n.as_str())
        .unwrap_or(&msg.from_email);

    // Avatar (36px — slightly smaller for denser rows)
    let avatar_widget = avatar(display_name, 36);

    // Selection checkbox — only rendered when selected
    let checkbox: Element<'a, Message> = if is_selected {
        container(
            text(icons::CHECK)
                .size(typography::SIZE_XS)
                .style(|_| iced::widget::text::Style {
                    color: Some(iced::Color::WHITE),
                }),
        )
        .width(Length::Fixed(18.0))
        .height(Length::Fixed(18.0))
        .center_x(Length::Fixed(18.0))
        .center_y(Length::Fixed(18.0))
        .style(|_| container::Style {
            background: Some(Background::Color(colors::ACCENT_PRIMARY)),
            border: Border {
                radius: spacing::RADIUS_SM.into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .into()
    } else {
        // No visible checkbox when not selected — just a spacer
        Space::with_width(18).into()
    };

    // --- Line 1: Sender name + right-aligned time ---
    let sender_name = text(truncate_string(display_name, 30))
        .size(typography::SIZE_SM)
        .font(typography::FONT_MEDIUM)
        .style(components::text_primary);

    let time_text = text(format_relative_time(&msg.sent_at))
        .size(typography::SIZE_XS)
        .style(components::text_muted);

    let line1 = row![
        sender_name,
        Space::with_width(Length::Fill),
        time_text,
    ]
    .align_y(iced::Alignment::Center);

    // --- Line 2: Subject + right-aligned attachment icon ---
    let subject_text = text(truncate_string(&msg.subject, 55))
        .size(typography::SIZE_SM)
        .style(components::text_secondary);

    let attachment_and_size: Element<'a, Message> = if msg.has_attachments {
        row![
            text(icons::ATTACH)
                .size(typography::SIZE_XS)
                .style(components::text_muted),
            Space::with_width(spacing::XS),
            text(format_bytes(msg.size_bytes))
                .size(typography::SIZE_XS)
                .style(components::text_muted),
        ]
        .align_y(iced::Alignment::Center)
        .into()
    } else {
        text(format_bytes(msg.size_bytes))
            .size(typography::SIZE_XS)
            .style(components::text_muted)
            .into()
    };

    let line2 = row![
        subject_text,
        Space::with_width(Length::Fill),
        attachment_and_size,
    ]
    .align_y(iced::Alignment::Center);

    // --- Line 3: Snippet (truncated to ~80 chars) ---
    let snippet_str = if msg.snippet.is_empty() {
        "\u{00A0}".to_string() // non-breaking space to preserve row height
    } else {
        truncate_string(&msg.snippet, 80)
    };

    let line3 = text(snippet_str)
        .size(typography::SIZE_XS)
        .style(components::text_muted);

    // 3-line content column
    let content = column![line1, line2, line3,]
        .spacing(1)
        .width(Length::Fill);

    // Row layout: checkbox + avatar + content
    let row_content = row![
        checkbox,
        Space::with_width(spacing::SM),
        avatar_widget,
        Space::with_width(spacing::SM),
        content,
    ]
    .align_y(iced::Alignment::Center)
    .padding([spacing::SPACE_3, spacing::MD]);

    // --- Styling based on state ---
    // Focused: copper left border + selection bg
    // Selected: subtle copper tint (8% alpha)
    // Default: surface bg
    let bg_color = if is_focused {
        colors::SELECTION_BG
    } else if is_selected {
        colors::with_alpha(colors::ACCENT_PRIMARY, 0.08)
    } else {
        colors::BG_SURFACE
    };

    let left_border_width: f32 = if is_focused { 2.0 } else { 0.0 };
    let left_border_color = if is_focused {
        colors::ACCENT_PRIMARY
    } else {
        iced::Color::TRANSPARENT
    };

    // Use a nested container approach: outer provides left border, inner provides content
    if is_focused {
        // Focused row: left copper border + SELECTION_BG
        container(
            container(row_content)
                .width(Length::Fill)
                .style(move |_| container::Style {
                    background: Some(Background::Color(bg_color)),
                    ..Default::default()
                }),
        )
        .width(Length::Fill)
        .style(move |_| container::Style {
            border: Border {
                radius: spacing::RADIUS_MD.into(),
                width: left_border_width,
                color: left_border_color,
            },
            ..Default::default()
        })
        .into()
    } else {
        container(row_content)
            .width(Length::Fill)
            .style(move |_| container::Style {
                background: Some(Background::Color(bg_color)),
                border: Border {
                    radius: spacing::RADIUS_MD.into(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .into()
    }
}

/// Footer with pagination and keyboard hints
fn footer_section(offset: i64, _page_count: usize, total: i64) -> Element<'static, Message> {
    let pagination = text(format!(
        "Page {} of {}",
        (offset / 50) + 1,
        ((total.max(1) - 1) / 50) + 1
    ))
    .size(typography::SIZE_XS)
    .style(components::text_muted);

    let hints = text("j/k navigate  Enter open  Space select  d delete  n/p pages")
        .size(typography::SIZE_2XS)
        .font(typography::FONT_MONO)
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
    if s.chars().count() <= max_len {
        s.to_string()
    } else {
        let truncated: String = s.chars().take(max_len.saturating_sub(1)).collect();
        format!("{}\u{2026}", truncated)
    }
}
