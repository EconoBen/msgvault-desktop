//! Messages list view
//!
//! Displays a scrollable list of message summaries with pagination.
//! Each row shows subject, sender, date, size, and attachment indicator.

use crate::api::types::MessageSummary;
use crate::message::Message;
use crate::view::widgets::format_bytes;
use chrono::{DateTime, Datelike, Local, Utc};
use iced::widget::{column, container, row, scrollable, text, Space};
use iced::{Element, Length};
use std::collections::HashSet;

/// Render the messages list view
pub fn messages_view<'a>(
    filter_description: &'a str,
    messages: &'a [MessageSummary],
    selected_index: usize,
    offset: i64,
    total: i64,
    selected_messages: &'a HashSet<i64>,
) -> Element<'a, Message> {
    // Header with filter description and selection count
    let header = header_row(filter_description, offset, messages.len(), total, selected_messages.len());

    // Column headers
    let column_headers = column_header_row();

    // Scrollable list of message rows
    let list_content: Element<'a, Message> = if messages.is_empty() {
        container(text("No messages found").size(14))
            .padding(20)
            .into()
    } else {
        let rows: Vec<Element<'a, Message>> = messages
            .iter()
            .enumerate()
            .map(|(i, msg)| message_row(msg, i == selected_index, selected_messages.contains(&msg.id)))
            .collect();

        scrollable(column(rows).spacing(2))
            .height(Length::Fill)
            .into()
    };

    // Pagination info
    let pagination = pagination_info(offset, messages.len(), total);

    // Keyboard hints
    let hints =
        text("Up/Down: navigate | Enter: open | Space: select | A: all | x: clear | d: delete").size(12);

    column![
        header,
        Space::with_height(10),
        column_headers,
        Space::with_height(5),
        list_content,
        Space::with_height(10),
        pagination,
        Space::with_height(5),
        hints,
    ]
    .spacing(5)
    .padding(20)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

/// Header row showing filter description
fn header_row<'a>(
    filter_description: &'a str,
    offset: i64,
    page_count: usize,
    total: i64,
    selection_count: usize,
) -> Element<'a, Message> {
    let title = text(filter_description).size(20);

    let selection_info = if selection_count > 0 {
        text(format!("{} selected", selection_count))
            .size(12)
            .style(|theme: &iced::Theme| {
                let palette = theme.palette();
                iced::widget::text::Style {
                    color: Some(palette.primary),
                }
            })
    } else {
        text("").size(12)
    };

    let count_info = text(format!(
        "Showing {}-{} of {}",
        offset + 1,
        offset + page_count as i64,
        total
    ))
    .size(12);

    row![title, Space::with_width(Length::Fill), selection_info, Space::with_width(20), count_info]
        .align_y(iced::Alignment::Center)
        .into()
}

/// Column header row
fn column_header_row<'a>() -> Element<'a, Message> {
    let select_header = text("").size(12).width(Length::Fixed(24.0)); // Selection checkbox
    let subject_header = text("Subject").size(12).width(Length::FillPortion(4));
    let from_header = text("From").size(12).width(Length::FillPortion(3));
    let date_header = text("Date").size(12).width(Length::FillPortion(2));
    let size_header = text("Size").size(12).width(Length::FillPortion(1));
    let attach_header = text("").size(12).width(Length::Fixed(20.0)); // Attachment indicator

    container(
        row![
            select_header,
            subject_header,
            from_header,
            date_header,
            size_header,
            attach_header
        ]
        .spacing(10)
        .padding([5, 10]),
    )
    .style(|theme: &iced::Theme| {
        let palette = theme.palette();
        container::Style {
            background: Some(iced::Background::Color(iced::Color {
                a: 0.1,
                ..palette.text
            })),
            ..Default::default()
        }
    })
    .width(Length::Fill)
    .into()
}

/// Single message row
fn message_row<'a>(msg: &'a MessageSummary, is_cursor: bool, is_checked: bool) -> Element<'a, Message> {
    // Selection checkbox indicator
    let checkbox_indicator = if is_checked { "[x]" } else { "[ ]" };
    let checkbox = text(checkbox_indicator)
        .size(14)
        .width(Length::Fixed(24.0));

    let subject = text(truncate_string(&msg.subject, 50))
        .size(14)
        .width(Length::FillPortion(4));

    let from_display = msg
        .from_name
        .as_ref()
        .filter(|n| !n.is_empty())
        .map(|n| n.as_str())
        .unwrap_or(&msg.from_email);
    let from = text(truncate_string(from_display, 30))
        .size(14)
        .width(Length::FillPortion(3));

    let date = text(format_date(&msg.sent_at))
        .size(14)
        .width(Length::FillPortion(2));

    let size = text(format_bytes(msg.size_bytes))
        .size(14)
        .width(Length::FillPortion(1));

    let attachment_indicator = if msg.has_attachments { "📎" } else { "" };
    let attach = text(attachment_indicator)
        .size(14)
        .width(Length::Fixed(20.0));

    let row_content = row![checkbox, subject, from, date, size, attach]
        .spacing(10)
        .padding([8, 10]);

    // Style based on cursor position and selection state
    let style = if is_cursor {
        // Cursor position - highlighted
        container(row_content)
            .style(|theme: &iced::Theme| {
                let palette = theme.palette();
                container::Style {
                    background: Some(iced::Background::Color(palette.primary)),
                    text_color: Some(iced::Color::WHITE),
                    ..Default::default()
                }
            })
    } else if is_checked {
        // Selected but not cursor - subtle highlight
        container(row_content)
            .style(|theme: &iced::Theme| {
                let palette = theme.palette();
                container::Style {
                    background: Some(iced::Background::Color(iced::Color {
                        a: 0.2,
                        ..palette.primary
                    })),
                    ..Default::default()
                }
            })
    } else {
        // Normal row
        container(row_content)
    };

    style.width(Length::Fill).into()
}

/// Pagination info row
fn pagination_info<'a>(offset: i64, page_count: usize, total: i64) -> Element<'a, Message> {
    let start = offset + 1;
    let end = offset + page_count as i64;

    let info = if total == 0 {
        "No messages".to_string()
    } else {
        format!("Showing {} - {} of {} messages", start, end, total)
    };

    text(info).size(12).into()
}

/// Format a datetime for display
fn format_date(dt: &DateTime<Utc>) -> String {
    let local: DateTime<Local> = dt.with_timezone(&Local);
    let now = Local::now();

    // If today, show time only
    if local.date_naive() == now.date_naive() {
        return local.format("%H:%M").to_string();
    }

    // If this year, show month and day
    if local.year() == now.year() {
        return local.format("%b %d").to_string();
    }

    // Otherwise show full date
    local.format("%Y-%m-%d").to_string()
}

/// Truncate a string to a maximum length, adding ellipsis if needed
fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}
