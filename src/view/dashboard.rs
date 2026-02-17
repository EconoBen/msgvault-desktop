//! Dashboard view
//!
//! Shows archive statistics and quick navigation.

use crate::api::types::{StatsResponse, ViewType};
use crate::message::Message;
use crate::model::ViewLevel;
use crate::theme::{colors, components, spacing, typography};
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
    .spacing(spacing::LG)
    .width(Length::Fill);

    // Quick navigation section
    let nav_title = text("Browse by")
        .size(typography::SIZE_MD)
        .style(components::text_primary);

    let nav_buttons = row![
        nav_button("Senders", ViewType::Senders),
        nav_button("Domains", ViewType::Domains),
        nav_button("Labels", ViewType::Labels),
        nav_button("Time", ViewType::Time),
    ]
    .spacing(spacing::SM);

    // Keyboard hints
    let hints = text("Press Tab to cycle views • Esc to go back • ? for help")
        .size(typography::SIZE_XS)
        .style(components::text_muted);

    column![
        stats_row,
        Space::with_height(spacing::XXL),
        nav_title,
        Space::with_height(spacing::SM),
        nav_buttons,
        Space::with_height(Length::Fill),
        hints,
    ]
    .spacing(spacing::SM)
    .padding(spacing::XL)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

/// Navigation button for quick access to views
fn nav_button(label: &str, view_type: ViewType) -> Element<'_, Message> {
    button(text(label).size(typography::SIZE_SM))
        .padding([spacing::SM, spacing::XL])
        .style(components::button_secondary)
        .on_press(Message::NavigateTo(ViewLevel::Aggregates { view_type }))
        .into()
}
