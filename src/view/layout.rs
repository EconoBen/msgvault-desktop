//! Three-panel email client layout
//!
//! Provides the main application layout with sidebar, message list, and detail pane.

use crate::message::Message;
use crate::theme::{colors, spacing};
use iced::widget::{container, row};
use iced::{Background, Border, Element, Length};

/// Minimum message list width
pub const MESSAGE_LIST_MIN_WIDTH: f32 = 350.0;

/// Create a three-panel layout
pub fn three_panel_layout<'a>(
    sidebar: Element<'a, Message>,
    message_list: Element<'a, Message>,
    detail: Option<Element<'a, Message>>,
) -> Element<'a, Message> {
    let sidebar_container = container(sidebar)
        .width(Length::Fixed(spacing::SIDEBAR_WIDTH))
        .height(Length::Fill)
        .style(|_| container::Style {
            background: Some(Background::Color(colors::BG_DEEP)),
            border: Border {
                width: 0.0,
                ..Default::default()
            },
            ..Default::default()
        });

    let list_container = container(message_list)
        .width(Length::FillPortion(2))
        .height(Length::Fill)
        .style(|_| container::Style {
            background: Some(Background::Color(colors::BG_SURFACE)),
            border: Border {
                color: colors::BORDER_SUBTLE,
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        });

    let content = if let Some(detail_view) = detail {
        let detail_container = container(detail_view)
            .width(Length::FillPortion(3))
            .height(Length::Fill)
            .style(|_| container::Style {
                background: Some(Background::Color(colors::BG_BASE)),
                ..Default::default()
            });

        row![sidebar_container, list_container, detail_container]
    } else {
        row![sidebar_container, list_container]
    };

    content
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

/// Create a two-panel layout (sidebar + content)
pub fn two_panel_layout<'a>(
    sidebar: Element<'a, Message>,
    content: Element<'a, Message>,
) -> Element<'a, Message> {
    let sidebar_container = container(sidebar)
        .width(Length::Fixed(spacing::SIDEBAR_WIDTH))
        .height(Length::Fill)
        .style(|_| container::Style {
            background: Some(Background::Color(colors::BG_DEEP)),
            ..Default::default()
        });

    let content_container = container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_| container::Style {
            background: Some(Background::Color(colors::BG_SURFACE)),
            ..Default::default()
        });

    row![sidebar_container, content_container]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
