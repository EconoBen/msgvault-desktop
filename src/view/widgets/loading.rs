//! Loading and error state widgets
//!
//! Consistent, styled empty states using the Foundry Dark design system.

use crate::message::Message;
use crate::theme::{colors, components, icons, spacing, typography};
use iced::widget::{center, column, text, Space};
use iced::Element;

/// Render a loading indicator with animated dots
pub fn loading<'a>(message: &'a str) -> Element<'a, Message> {
    center(
        column![
            text(icons::DOTS)
                .size(typography::SIZE_2XL)
                .style(components::text_muted),
            Space::with_height(spacing::SM),
            text(message)
                .size(typography::SIZE_SM)
                .font(typography::FONT_MEDIUM)
                .style(components::text_muted),
        ]
        .spacing(spacing::XS)
        .align_x(iced::Alignment::Center),
    )
    .into()
}

/// Render an error state with message
pub fn error<'a>(error_message: &'a str) -> Element<'a, Message> {
    center(
        column![
            text(icons::CROSS)
                .size(typography::SIZE_2XL)
                .style(components::text_error),
            Space::with_height(spacing::SM),
            text("Something went wrong")
                .size(typography::SIZE_MD)
                .font(typography::FONT_SEMIBOLD)
                .style(components::text_primary),
            Space::with_height(spacing::XS),
            text(error_message)
                .size(typography::SIZE_SM)
                .font(typography::FONT_MONO)
                .style(components::text_muted),
        ]
        .spacing(spacing::XS)
        .align_x(iced::Alignment::Center),
    )
    .into()
}

/// Render an empty state (e.g., no messages, no search results)
pub fn empty_state<'a>(icon: &'a str, title: &'a str, subtitle: &'a str) -> Element<'a, Message> {
    center(
        column![
            text(icon)
                .size(typography::SIZE_3XL)
                .style(components::text_muted),
            Space::with_height(spacing::MD),
            text(title)
                .size(typography::SIZE_MD)
                .font(typography::FONT_SEMIBOLD)
                .style(components::text_secondary),
            Space::with_height(spacing::XS),
            text(subtitle)
                .size(typography::SIZE_SM)
                .style(components::text_muted),
        ]
        .spacing(spacing::XS)
        .align_x(iced::Alignment::Center),
    )
    .into()
}
