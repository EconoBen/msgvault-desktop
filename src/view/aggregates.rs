//! Aggregates list view
//!
//! Displays a scrollable list of aggregate rows (senders, domains, labels, etc.)
//! with column headers and keyboard hints.

use crate::api::types::{AggregateRow, SortDirection, SortField, ViewType};
use crate::message::Message;
use crate::view::widgets::format_bytes;
use iced::widget::{column, container, row, scrollable, text, Space};
use iced::{Element, Length};

/// Render the aggregates list view
pub fn aggregates_view<'a>(
    view_type: &ViewType,
    aggregates: &'a [AggregateRow],
    selected_index: usize,
    sort_field: SortField,
    sort_dir: SortDirection,
) -> Element<'a, Message> {
    // Header with view type and sort info
    let header = header_row(view_type, sort_field, sort_dir);

    // Column headers
    let column_headers = column_header_row();

    // Scrollable list of aggregate rows
    let list_content: Element<'a, Message> = if aggregates.is_empty() {
        container(text("No data available").size(14))
            .padding(20)
            .into()
    } else {
        let rows: Vec<Element<'a, Message>> = aggregates
            .iter()
            .enumerate()
            .map(|(i, agg)| aggregate_row(agg, i == selected_index))
            .collect();

        scrollable(column(rows).spacing(2))
            .height(Length::Fill)
            .into()
    };

    // Keyboard hints
    let hints = text("Up/Down: navigate | Enter: select | Tab: switch view | s: toggle sort | Esc: back")
        .size(12);

    column![
        header,
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

/// Header row showing current view type and sort info
fn header_row<'a>(
    view_type: &ViewType,
    sort_field: SortField,
    sort_dir: SortDirection,
) -> Element<'a, Message> {
    let title = text(view_type.display_name()).size(20);

    let sort_indicator = match sort_dir {
        SortDirection::Desc => "v",
        SortDirection::Asc => "^",
    };

    let sort_info = text(format!(
        "Sorted by: {} {}",
        sort_field.as_str(),
        sort_indicator
    ))
    .size(12);

    row![title, Space::with_width(Length::Fill), sort_info]
        .align_y(iced::Alignment::Center)
        .into()
}

/// Column header row
fn column_header_row<'a>() -> Element<'a, Message> {
    let name_header = text("Name").size(12).width(Length::FillPortion(3));
    let count_header = text("Count").size(12).width(Length::FillPortion(1));
    let size_header = text("Size").size(12).width(Length::FillPortion(1));
    let attachments_header = text("Attachments").size(12).width(Length::FillPortion(1));

    container(
        row![name_header, count_header, size_header, attachments_header]
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

/// Single aggregate row
fn aggregate_row<'a>(agg: &'a AggregateRow, is_selected: bool) -> Element<'a, Message> {
    let name = text(&agg.key).size(14).width(Length::FillPortion(3));
    let count = text(format!("{}", agg.count))
        .size(14)
        .width(Length::FillPortion(1));
    let size = text(format_bytes(agg.total_size))
        .size(14)
        .width(Length::FillPortion(1));
    let attachments = text(format!("{}", agg.attachment_count))
        .size(14)
        .width(Length::FillPortion(1));

    let row_content = row![name, count, size, attachments]
        .spacing(10)
        .padding([8, 10]);

    if is_selected {
        container(row_content)
            .style(|theme: &iced::Theme| {
                let palette = theme.palette();
                container::Style {
                    background: Some(iced::Background::Color(palette.primary)),
                    text_color: Some(iced::Color::WHITE),
                    ..Default::default()
                }
            })
            .width(Length::Fill)
            .into()
    } else {
        container(row_content).width(Length::Fill).into()
    }
}
