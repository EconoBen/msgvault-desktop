//! Aggregates list view
//!
//! Displays a scrollable list of aggregate rows (senders, domains, labels, etc.)
//! with column headers and keyboard hints.

use crate::api::types::{AggregateRow, SortDirection, SortField, ViewType};
use crate::message::Message;
use crate::theme::{colors, components, icons, spacing, typography};
use crate::view::widgets::format_bytes;
use iced::widget::{column, container, row, scrollable, text, Space};
use iced::{Background, Border, Element, Length};

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
        container(
            text("No data available")
                .size(typography::SIZE_SM)
                .style(components::text_muted),
        )
        .padding(spacing::XL)
        .into()
    } else {
        let rows: Vec<Element<'a, Message>> = aggregates
            .iter()
            .enumerate()
            .map(|(i, agg)| aggregate_row(agg, i == selected_index))
            .collect();

        scrollable(column(rows).spacing(spacing::SPACE_1))
            .height(Length::Fill)
            .into()
    };

    // Keyboard hints in FONT_MONO
    let hints = text("Up/Down: navigate | Enter: select | Tab: switch view | s: toggle sort | Esc: back")
        .size(typography::SIZE_2XS)
        .font(typography::FONT_MONO)
        .style(components::text_muted);

    column![
        header,
        Space::with_height(spacing::MD),
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

/// Header row showing current view type and sort info
fn header_row<'a>(
    view_type: &ViewType,
    sort_field: SortField,
    sort_dir: SortDirection,
) -> Element<'a, Message> {
    let title = text(view_type.display_name())
        .size(typography::SIZE_LG)
        .font(typography::FONT_MEDIUM)
        .style(components::text_primary);

    let sort_indicator = match sort_dir {
        SortDirection::Desc => icons::ARROW_DOWN,
        SortDirection::Asc => icons::ARROW_UP,
    };

    let sort_info = text(format!(
        "Sorted by: {} {}",
        sort_field.as_str(),
        sort_indicator
    ))
    .size(typography::SIZE_XS)
    .style(components::text_muted);

    row![title, Space::with_width(Length::Fill), sort_info]
        .align_y(iced::Alignment::Center)
        .into()
}

/// Column header row
fn column_header_row<'a>() -> Element<'a, Message> {
    let name_header = text("Name")
        .size(typography::SIZE_XS)
        .font(typography::FONT_MEDIUM)
        .style(components::text_muted)
        .width(Length::FillPortion(3));
    let count_header = text("Count")
        .size(typography::SIZE_XS)
        .font(typography::FONT_MEDIUM)
        .style(components::text_muted)
        .width(Length::FillPortion(1));
    let size_header = text("Size")
        .size(typography::SIZE_XS)
        .font(typography::FONT_MEDIUM)
        .style(components::text_muted)
        .width(Length::FillPortion(1));
    let attachments_header = text("Attachments")
        .size(typography::SIZE_XS)
        .font(typography::FONT_MEDIUM)
        .style(components::text_muted)
        .width(Length::FillPortion(1));

    container(
        row![name_header, count_header, size_header, attachments_header]
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

/// Single aggregate row with hover/focus states
fn aggregate_row<'a>(agg: &'a AggregateRow, is_selected: bool) -> Element<'a, Message> {
    let name = text(&agg.key)
        .size(typography::SIZE_SM)
        .style(components::text_primary)
        .width(Length::FillPortion(3));
    let count = text(format!("{}", agg.count))
        .size(typography::SIZE_SM)
        .font(typography::FONT_MONO)
        .style(components::text_secondary)
        .width(Length::FillPortion(1));
    let size = text(format_bytes(agg.total_size))
        .size(typography::SIZE_SM)
        .font(typography::FONT_MONO)
        .style(components::text_secondary)
        .width(Length::FillPortion(1));
    let attachments = text(format!("{}", agg.attachment_count))
        .size(typography::SIZE_SM)
        .font(typography::FONT_MONO)
        .style(components::text_secondary)
        .width(Length::FillPortion(1));

    let row_content = row![name, count, size, attachments]
        .spacing(spacing::SM)
        .padding([spacing::SM, spacing::SM]);

    if is_selected {
        container(row_content)
            .style(components::selected_row_style)
            .width(Length::Fill)
            .into()
    } else {
        container(row_content)
            .style(|_| container::Style {
                border: Border {
                    radius: spacing::RADIUS_MD.into(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .width(Length::Fill)
            .into()
    }
}
