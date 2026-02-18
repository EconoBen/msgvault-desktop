//! Search view
//!
//! Displays a search interface with input, mode toggle, and results list.

use crate::api::types::MessageSummary;
use crate::message::Message;
use crate::theme::{colors, components, icons, spacing, typography};
use crate::view::widgets::format_bytes;
use chrono::{DateTime, Datelike, Local, Utc};
use iced::widget::{button, column, container, row, scrollable, text, text_input, Space};
use iced::{Background, Border, Element, Length};
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
        .padding(spacing::MD)
        .width(Length::Fill)
        .style(components::text_input_style);

    // Mode toggle buttons
    let fast_button = if !is_deep {
        button(
            text("Fast")
                .size(typography::SIZE_SM)
                .font(typography::FONT_MEDIUM),
        )
        .padding([spacing::SM, spacing::LG])
        .style(components::button_primary)
        .on_press(Message::ToggleSearchMode)
    } else {
        button(text("Fast").size(typography::SIZE_SM))
            .padding([spacing::SM, spacing::LG])
            .style(components::button_ghost)
            .on_press(Message::ToggleSearchMode)
    };

    let deep_button = if is_deep {
        button(
            text("Deep")
                .size(typography::SIZE_SM)
                .font(typography::FONT_MEDIUM),
        )
        .padding([spacing::SM, spacing::LG])
        .style(components::button_primary)
        .on_press(Message::ToggleSearchMode)
    } else {
        button(text("Deep").size(typography::SIZE_SM))
            .padding([spacing::SM, spacing::LG])
            .style(components::button_ghost)
            .on_press(Message::ToggleSearchMode)
    };

    let mode_toggle = row![fast_button, deep_button].spacing(spacing::XS);

    let search_bar = row![search_input, Space::with_width(spacing::MD), mode_toggle]
        .align_y(iced::Alignment::Center);

    // Results count in TEXT_MUTED
    let results_count = text(format!("{} results", total))
        .size(typography::SIZE_XS)
        .style(components::text_muted);

    // Column headers
    let column_headers = column_header_row();

    // Selection count
    let selection_info = if !selected_messages.is_empty() {
        text(format!("{} selected", selected_messages.len()))
            .size(typography::SIZE_XS)
            .style(components::text_accent)
    } else {
        text("").size(typography::SIZE_XS)
    };

    // Results list content
    let list_content: Element<'a, Message> = if is_searching {
        container(
            column![
                text(icons::DOTS)
                    .size(typography::SIZE_XL)
                    .style(components::text_muted),
                text("Searching...")
                    .size(typography::SIZE_SM)
                    .font(typography::FONT_MEDIUM)
                    .style(components::text_muted),
            ]
            .spacing(spacing::SM)
            .align_x(iced::Alignment::Center),
        )
        .width(Length::Fill)
        .center_x(Length::Fill)
        .padding(spacing::XXL)
        .into()
    } else if results.is_empty() && !query.is_empty() {
        container(
            column![
                text(icons::SEARCH)
                    .size(typography::SIZE_XL)
                    .style(components::text_muted),
                text("No results found")
                    .size(typography::SIZE_MD)
                    .font(typography::FONT_SEMIBOLD)
                    .style(components::text_secondary),
                text("Try different search terms")
                    .size(typography::SIZE_SM)
                    .style(components::text_muted),
            ]
            .spacing(spacing::SM)
            .align_x(iced::Alignment::Center),
        )
        .width(Length::Fill)
        .center_x(Length::Fill)
        .padding(spacing::XXL)
        .into()
    } else if results.is_empty() {
        container(
            column![
                text(icons::SEARCH)
                    .size(typography::SIZE_XL)
                    .style(components::text_muted),
                text("Search your archive")
                    .size(typography::SIZE_MD)
                    .font(typography::FONT_SEMIBOLD)
                    .style(components::text_secondary),
                text("Type to search across all messages")
                    .size(typography::SIZE_SM)
                    .style(components::text_muted),
            ]
            .spacing(spacing::SM)
            .align_x(iced::Alignment::Center),
        )
        .width(Length::Fill)
        .center_x(Length::Fill)
        .padding(spacing::XXL)
        .into()
    } else {
        let rows: Vec<Element<'a, Message>> = results
            .iter()
            .enumerate()
            .map(|(i, msg)| message_row(msg, i == selected_index, selected_messages.contains(&msg.id)))
            .collect();

        scrollable(column(rows).spacing(spacing::SPACE_1))
            .height(Length::Fill)
            .into()
    };

    // Keyboard hints in FONT_MONO
    let hints = text("Enter: open | Tab: toggle mode | Space: select | A: all | x: clear | d: delete")
        .size(typography::SIZE_2XS)
        .font(typography::FONT_MONO)
        .style(components::text_muted);

    column![
        search_bar,
        Space::with_height(spacing::MD),
        row![results_count, Space::with_width(Length::Fill), selection_info],
        Space::with_height(spacing::SM),
        column_headers,
        Space::with_height(spacing::XS),
        list_content,
        Space::with_height(spacing::SM),
        hints,
    ]
    .spacing(spacing::XS)
    .padding(spacing::XL)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

/// Column header row
fn column_header_row<'a>() -> Element<'a, Message> {
    let select_header = text("")
        .size(typography::SIZE_XS)
        .width(Length::Fixed(24.0));
    let subject_header = text("Subject")
        .size(typography::SIZE_XS)
        .font(typography::FONT_MEDIUM)
        .style(components::text_muted)
        .width(Length::FillPortion(4));
    let from_header = text("From")
        .size(typography::SIZE_XS)
        .font(typography::FONT_MEDIUM)
        .style(components::text_muted)
        .width(Length::FillPortion(3));
    let date_header = text("Date")
        .size(typography::SIZE_XS)
        .font(typography::FONT_MEDIUM)
        .style(components::text_muted)
        .width(Length::FillPortion(2));
    let size_header = text("Size")
        .size(typography::SIZE_XS)
        .font(typography::FONT_MEDIUM)
        .style(components::text_muted)
        .width(Length::FillPortion(1));
    let attach_header = text("")
        .size(typography::SIZE_XS)
        .width(Length::Fixed(20.0));

    container(
        row![
            select_header,
            subject_header,
            from_header,
            date_header,
            size_header,
            attach_header
        ]
        .spacing(spacing::SM)
        .padding([spacing::XS, spacing::SM]),
    )
    .style(|_| container::Style {
        background: Some(Background::Color(colors::BG_ELEVATED)),
        border: Border {
            radius: spacing::RADIUS_MD.into(),
            ..Default::default()
        },
        ..Default::default()
    })
    .width(Length::Fill)
    .into()
}

/// Single message row (reused pattern from messages.rs)
fn message_row<'a>(msg: &'a MessageSummary, is_cursor: bool, is_checked: bool) -> Element<'a, Message> {
    // Selection checkbox indicator
    let checkbox_indicator = if is_checked { icons::CHECK } else { icons::DOT_EMPTY };
    let checkbox = text(checkbox_indicator)
        .size(typography::SIZE_SM)
        .style(if is_checked {
            components::text_accent
        } else {
            components::text_muted
        })
        .width(Length::Fixed(24.0));

    let subject = text(truncate_string(&msg.subject, 50))
        .size(typography::SIZE_SM)
        .style(components::text_primary)
        .width(Length::FillPortion(4));

    let from_display = msg
        .from_name
        .as_ref()
        .filter(|n| !n.is_empty())
        .map(|n| n.as_str())
        .unwrap_or(&msg.from_email);
    let from = text(truncate_string(from_display, 30))
        .size(typography::SIZE_SM)
        .style(components::text_secondary)
        .width(Length::FillPortion(3));

    let date = text(format_date(&msg.sent_at))
        .size(typography::SIZE_XS)
        .font(typography::FONT_MONO)
        .style(components::text_muted)
        .width(Length::FillPortion(2));

    let size = text(format_bytes(msg.size_bytes))
        .size(typography::SIZE_XS)
        .style(components::text_muted)
        .width(Length::FillPortion(1));

    let attachment_indicator = if msg.has_attachments { icons::ATTACH } else { "" };
    let attach = text(attachment_indicator)
        .size(typography::SIZE_SM)
        .style(components::text_muted)
        .width(Length::Fixed(20.0));

    let row_content = row![checkbox, subject, from, date, size, attach]
        .spacing(spacing::SM)
        .padding([spacing::SM, spacing::SM]);

    // Style based on cursor position and selection state
    let style = if is_cursor {
        container(row_content).style(components::selected_row_style)
    } else if is_checked {
        container(row_content).style(|_| container::Style {
            background: Some(Background::Color(colors::SELECTION_BG)),
            border: Border {
                radius: spacing::RADIUS_MD.into(),
                ..Default::default()
            },
            ..Default::default()
        })
    } else {
        container(row_content).style(|_| container::Style {
            border: Border {
                radius: spacing::RADIUS_MD.into(),
                ..Default::default()
            },
            ..Default::default()
        })
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
