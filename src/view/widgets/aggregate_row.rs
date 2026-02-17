//! Aggregate row widget
//!
//! Displays a single row in an aggregate view with key, message count,
//! total size, and attachment count.

use crate::api::types::AggregateRow;
use crate::message::Message;
use iced::widget::{button, container, row, text};
use iced::{Element, Length, Theme};

use super::stats_card::{format_bytes, format_number};

/// Render an aggregate row with key, count, size, and attachments
pub fn aggregate_row<'a>(
    row_data: &'a AggregateRow,
    index: usize,
    is_selected: bool,
) -> Element<'a, Message> {
    let selected_style = |theme: &Theme| {
        let palette = theme.palette();
        container::Style {
            background: Some(iced::Background::Color(iced::Color {
                a: 0.2,
                ..palette.primary
            })),
            border: iced::Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    };

    let normal_style = |_theme: &Theme| container::Style {
        background: None,
        border: iced::Border {
            radius: 4.0.into(),
            ..Default::default()
        },
        ..Default::default()
    };

    let key_text = text(&row_data.key).size(14).width(Length::FillPortion(4));

    let count_text = text(format_number(row_data.count))
        .size(14)
        .width(Length::FillPortion(1));

    let size_text = text(format_bytes(row_data.total_size))
        .size(14)
        .width(Length::FillPortion(1));

    let attachment_text = text(format_number(row_data.attachment_count))
        .size(14)
        .width(Length::FillPortion(1));

    let row_content = row![key_text, count_text, size_text, attachment_text]
        .spacing(10)
        .align_y(iced::Alignment::Center);

    let styled_container = container(row_content)
        .width(Length::Fill)
        .padding([8, 12])
        .style(if is_selected {
            selected_style
        } else {
            normal_style
        });

    button(styled_container)
        .on_press(Message::SelectAggregate(index))
        .padding(0)
        .style(button::text)
        .width(Length::Fill)
        .into()
}
