//! Loading indicator widget
//!
//! Shows a simple loading state while data is being fetched.

use crate::message::Message;
use iced::widget::{center, column, text};
use iced::Element;

/// Render a loading indicator
pub fn loading<'a>(message: &'a str) -> Element<'a, Message> {
    center(
        column![
            text("⏳").size(32),
            text(message).size(14),
        ]
        .spacing(10)
        .align_x(iced::Alignment::Center),
    )
    .into()
}

/// Render an error state with message
pub fn error<'a>(error_message: &'a str) -> Element<'a, Message> {
    center(
        column![
            text("⚠️").size(32),
            text("Error loading data").size(16),
            text(error_message).size(12),
        ]
        .spacing(10)
        .align_x(iced::Alignment::Center),
    )
    .into()
}
