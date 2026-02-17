//! Dashboard view
//!
//! Shows archive statistics and quick navigation.

use crate::api::types::{StatsResponse, ViewType};
use crate::message::Message;
use crate::model::ViewLevel;
use crate::view::widgets::{format_bytes, format_number, stats_card};
use iced::widget::{button, column, row, text, Space};
use iced::{Element, Length};

/// Render the dashboard view
pub fn dashboard<'a>(stats: &StatsResponse) -> Element<'a, Message> {
    // Stats cards row
    let stats_row = row![
        stats_card("Messages", format_number(stats.total_messages)),
        stats_card("Threads", format_number(stats.total_threads)),
        stats_card("Accounts", format_number(stats.total_accounts)),
        stats_card("Labels", format_number(stats.total_labels)),
        stats_card("Attachments", format_number(stats.total_attachments)),
        stats_card("Database Size", format_bytes(stats.database_size_bytes)),
    ]
    .spacing(15)
    .width(Length::Fill);

    // Quick navigation section
    let nav_title = text("Browse by").size(16);

    let nav_buttons = row![
        nav_button("Senders", ViewType::Senders),
        nav_button("Domains", ViewType::Domains),
        nav_button("Labels", ViewType::Labels),
        nav_button("Time", ViewType::Time),
    ]
    .spacing(10);

    // Keyboard hints
    let hints = text("Press Tab to cycle views • Esc to go back • ? for help")
        .size(12);

    column![
        stats_row,
        Space::with_height(30),
        nav_title,
        Space::with_height(10),
        nav_buttons,
        Space::with_height(Length::Fill),
        hints,
    ]
    .spacing(10)
    .padding(20)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

/// Navigation button for quick access to views
fn nav_button(label: &str, view_type: ViewType) -> Element<'_, Message> {
    button(text(label).size(14))
        .padding([10, 20])
        .on_press(Message::NavigateTo(ViewLevel::Aggregates { view_type }))
        .into()
}
