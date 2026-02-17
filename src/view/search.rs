//! Search view
//!
//! Displays a search interface with input, mode toggle, and results list.

use crate::api::types::MessageSummary;
use crate::message::Message;
use crate::view::widgets::format_bytes;
use chrono::{DateTime, Datelike, Local, Utc};
use iced::widget::{button, column, container, row, scrollable, text, text_input, Space};
use iced::{Element, Length};
use std::collections::HashSet;

/// Render the search view
pub fn search_view<'a>(
    query: &'a str,
    is_deep: bool,
    results: &'a [MessageSummary],
    selected_index: usize,
    total: i64,
    is_searching: bool,
    selected_messages: &'a HashSet<i64>,
) -> Element<'a, Message> {
    // Search input bar
    let search_input = text_input("Search messages...", query)
        .on_input(Message::SearchQueryChanged)
        .padding(10)
        .width(Length::Fill);

    // Mode toggle buttons
    let fast_button = if !is_deep {
        button(text("Fast").size(14))
            .padding([8, 16])
            .style(|theme: &iced::Theme, _status| {
                let palette = theme.palette();
                button::Style {
                    background: Some(iced::Background::Color(palette.primary)),
                    text_color: iced::Color::WHITE,
                    ..Default::default()
                }
            })
            .on_press(Message::ToggleSearchMode)
    } else {
        button(text("Fast").size(14))
            .padding([8, 16])
            .on_press(Message::ToggleSearchMode)
    };

    let deep_button = if is_deep {
        button(text("Deep").size(14))
            .padding([8, 16])
            .style(|theme: &iced::Theme, _status| {
                let palette = theme.palette();
                button::Style {
                    background: Some(iced::Background::Color(palette.primary)),
                    text_color: iced::Color::WHITE,
                    ..Default::default()
                }
            })
            .on_press(Message::ToggleSearchMode)
    } else {
        button(text("Deep").size(14))
            .padding([8, 16])
            .on_press(Message::ToggleSearchMode)
    };

    let mode_toggle = row![fast_button, deep_button].spacing(5);

    let search_bar = row![search_input, Space::with_width(10), mode_toggle]
        .align_y(iced::Alignment::Center);

    // Results count
    let results_count = text(format!("{} results", total)).size(12);

    // Column headers
    let column_headers = column_header_row();

    // Selection count
    let selection_info = if !selected_messages.is_empty() {
        text(format!("{} selected", selected_messages.len()))
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

    // Results list content
    let list_content: Element<'a, Message> = if is_searching {
        container(text("Searching...").size(14))
            .padding(20)
            .into()
    } else if results.is_empty() {
        container(text("No results").size(14))
            .padding(20)
            .into()
    } else {
        let rows: Vec<Element<'a, Message>> = results
            .iter()
            .enumerate()
            .map(|(i, msg)| message_row(msg, i == selected_index, selected_messages.contains(&msg.id)))
            .collect();

        scrollable(column(rows).spacing(2))
            .height(Length::Fill)
            .into()
    };

    // Keyboard hints
    let hints = text("Enter: open | Tab: toggle mode | Space: select | A: all | x: clear | d: delete").size(12);

    column![
        search_bar,
        Space::with_height(10),
        row![results_count, Space::with_width(Length::Fill), selection_info],
        Space::with_height(10),
        column_headers,
        Space::with_height(5),
        list_content,
        Space::with_height(10),
        hints,
    ]
    .spacing(5)
    .padding(20)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

/// Column header row
fn column_header_row<'a>() -> Element<'a, Message> {
    let select_header = text("").size(12).width(Length::Fixed(24.0));
    let subject_header = text("Subject").size(12).width(Length::FillPortion(4));
    let from_header = text("From").size(12).width(Length::FillPortion(3));
    let date_header = text("Date").size(12).width(Length::FillPortion(2));
    let size_header = text("Size").size(12).width(Length::FillPortion(1));
    let attach_header = text("").size(12).width(Length::Fixed(20.0));

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

/// Single message row (reused pattern from messages.rs)
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
        container(row_content)
    };

    style.width(Length::Fill).into()
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
