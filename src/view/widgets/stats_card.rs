//! Stats card widget
//!
//! Displays a single statistic with label and value.

use crate::message::Message;
use crate::theme::{colors, components, spacing, typography};
use iced::widget::{column, container, text};
use iced::{Background, Border, Element, Length, Theme};

/// Render a stats card with label and value
pub fn stats_card<'a>(label: &'a str, value: impl ToString) -> Element<'a, Message> {
    let card_style = |_theme: &Theme| {
        container::Style {
            background: Some(Background::Color(colors::BG_SURFACE)),
            border: Border {
                radius: 8.0.into(),
                width: 1.0,
                color: colors::BORDER_SUBTLE,
            },
            ..Default::default()
        }
    };

    container(
        column![
            text(value.to_string())
                .size(typography::SIZE_XL)
                .style(components::text_primary),
            text(label)
                .size(typography::SIZE_XS)
                .style(components::text_muted),
        ]
        .spacing(spacing::XS)
        .align_x(iced::Alignment::Center),
    )
    .width(Length::Fill)
    .padding(spacing::XL)
    .style(card_style)
    .into()
}

/// Format bytes as human-readable size
pub fn format_bytes(bytes: i64) -> String {
    const KB: i64 = 1024;
    const MB: i64 = KB * 1024;
    const GB: i64 = MB * 1024;

    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Format large numbers with commas
pub fn format_number(n: i64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.insert(0, ',');
        }
        result.insert(0, c);
    }
    result
}
