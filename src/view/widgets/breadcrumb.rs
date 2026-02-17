//! Breadcrumb navigation widget
//!
//! Shows the navigation path and allows clicking to jump back.

use crate::message::Message;
use crate::model::BreadcrumbEntry;
use iced::widget::{button, row, text, Row};
use iced::Element;

/// Render a breadcrumb navigation bar
///
/// Takes ownership of entries so the labels can live in the returned Element.
pub fn breadcrumb(entries: Vec<BreadcrumbEntry>) -> Element<'static, Message> {
    if entries.is_empty() {
        return row![].into();
    }

    let len = entries.len();
    let mut items: Vec<Element<'static, Message>> = Vec::new();

    for (i, entry) in entries.into_iter().enumerate() {
        let is_last = i == len - 1;

        if is_last {
            // Current page - not clickable
            items.push(text(entry.label).size(14).into());
        } else {
            // Previous pages - clickable
            items.push(
                button(text(entry.label.clone()).size(14))
                    .on_press(Message::JumpToBreadcrumb(i))
                    .padding([2, 6])
                    .style(button::text)
                    .into(),
            );

            // Separator
            items.push(text(" › ").size(14).into());
        }
    }

    Row::with_children(items)
        .spacing(0)
        .align_y(iced::Alignment::Center)
        .into()
}
