//! Dashboard view
//!
//! Shows archive statistics and quick navigation.

use crate::api::types::{StatsResponse, ViewType};
use crate::message::Message;
use crate::model::ViewLevel;
use crate::theme::{colors, components, spacing, typography};
use crate::view::widgets::{format_bytes, format_number};
use iced::widget::{button, column, container, row, text, Space};
use iced::{Background, Border, Element, Length};

/// Render the dashboard view
pub fn dashboard<'a>(stats: &StatsResponse) -> Element<'a, Message> {
    // Hero stat: Total messages with special treatment
    let hero_stat = hero_stat_card("Total Messages", format_number(stats.total_messages));

    // Secondary stats row
    let secondary_row = row![
        secondary_stat_card("Threads", format_number(stats.total_threads)),
        secondary_stat_card("Accounts", format_number(stats.total_accounts)),
        secondary_stat_card("Labels", format_number(stats.total_labels)),
        secondary_stat_card("Attachments", format_number(stats.total_attachments)),
        secondary_stat_card("Database Size", format_bytes(stats.database_size_bytes)),
    ]
    .spacing(spacing::MD)
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
    let hints = text("Press Tab to cycle views \u{2022} Esc to go back \u{2022} ? for help")
        .size(typography::SIZE_XS)
        .font(typography::FONT_MONO)
        .style(components::text_muted);

    column![
        hero_stat,
        Space::with_height(spacing::LG),
        secondary_row,
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

/// Hero stat card: large copper number, centered, special card
fn hero_stat_card<'a>(label: &'a str, value: String) -> Element<'a, Message> {
    container(
        column![
            text(value)
                .size(typography::SIZE_3XL)
                .font(typography::FONT_SEMIBOLD)
                .style(components::text_accent),
            text(label)
                .size(typography::SIZE_SM)
                .style(components::text_muted),
        ]
        .spacing(spacing::XS)
        .align_x(iced::Alignment::Center),
    )
    .width(Length::Fill)
    .padding([spacing::XXL, spacing::XL])
    .center_x(Length::Fill)
    .style(|_| container::Style {
        background: Some(Background::Color(colors::BG_SURFACE)),
        border: Border {
            radius: spacing::RADIUS_LG.into(),
            width: 1.0,
            color: colors::BORDER_SUBTLE,
        },
        ..Default::default()
    })
    .into()
}

/// Secondary stat card: smaller, SIZE_XL number, SIZE_XS label
fn secondary_stat_card<'a>(label: &'a str, value: String) -> Element<'a, Message> {
    container(
        column![
            text(value)
                .size(typography::SIZE_XL)
                .font(typography::FONT_MONO)
                .style(components::text_primary),
            text(label)
                .size(typography::SIZE_XS)
                .style(components::text_muted),
        ]
        .spacing(spacing::XS)
        .align_x(iced::Alignment::Center),
    )
    .width(Length::Fill)
    .padding(spacing::LG)
    .style(|_| container::Style {
        background: Some(Background::Color(colors::BG_SURFACE)),
        border: Border {
            radius: spacing::RADIUS_MD.into(),
            width: 1.0,
            color: colors::BORDER_SUBTLE,
        },
        ..Default::default()
    })
    .into()
}

/// Navigation button for quick access to views
fn nav_button(label: &str, view_type: ViewType) -> Element<'_, Message> {
    button(text(label).size(typography::SIZE_SM))
        .padding([spacing::SM, spacing::XXL])
        .style(components::button_secondary)
        .on_press(Message::NavigateTo(ViewLevel::Aggregates { view_type }))
        .into()
}
