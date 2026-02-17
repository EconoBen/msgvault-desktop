//! Badge widget
//!
//! Displays small badges for labels, counts, and status indicators.

use crate::message::Message;
use crate::theme::{colors, spacing, typography};
use iced::widget::{container, text};
use iced::{Background, Border, Element, Length};

/// Badge style variants
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BadgeStyle {
    /// Default subtle badge
    Default,
    /// Primary accent badge
    Primary,
    /// Success (green) badge
    Success,
    /// Warning (yellow) badge
    Warning,
    /// Error (red) badge
    Error,
    /// Muted (very subtle) badge
    Muted,
}

/// Create a badge with text
pub fn badge(content: &str, style: BadgeStyle) -> Element<'static, Message> {
    let (bg_color, text_color) = match style {
        BadgeStyle::Default => (
            colors::with_alpha(colors::TEXT_PRIMARY, 0.15),
            colors::TEXT_SECONDARY,
        ),
        BadgeStyle::Primary => (
            colors::with_alpha(colors::ACCENT_PRIMARY, 0.2),
            colors::ACCENT_PRIMARY,
        ),
        BadgeStyle::Success => (
            colors::with_alpha(colors::ACCENT_SUCCESS, 0.2),
            colors::ACCENT_SUCCESS,
        ),
        BadgeStyle::Warning => (
            colors::with_alpha(colors::ACCENT_WARNING, 0.2),
            colors::ACCENT_WARNING,
        ),
        BadgeStyle::Error => (
            colors::with_alpha(colors::ACCENT_ERROR, 0.2),
            colors::ACCENT_ERROR,
        ),
        BadgeStyle::Muted => (
            colors::with_alpha(colors::TEXT_MUTED, 0.1),
            colors::TEXT_MUTED,
        ),
    };

    let badge_text = text(content.to_string())
        .size(typography::SIZE_XS)
        .style(move |_| iced::widget::text::Style {
            color: Some(text_color),
        });

    container(badge_text)
        .padding([2, spacing::XS])
        .style(move |_| container::Style {
            background: Some(Background::Color(bg_color)),
            border: Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .into()
}

/// Create a count badge (circular with number)
pub fn count_badge(count: i64) -> Element<'static, Message> {
    let display = if count > 99 {
        "99+".to_string()
    } else {
        count.to_string()
    };

    let badge_text = text(display)
        .size(typography::SIZE_XS)
        .style(|_| iced::widget::text::Style {
            color: Some(colors::TEXT_MUTED),
        });

    container(badge_text)
        .padding([2, spacing::XS])
        .style(|_| container::Style {
            background: Some(Background::Color(colors::with_alpha(colors::TEXT_MUTED, 0.1))),
            border: Border {
                radius: 10.0.into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .into()
}

/// Create an unread indicator dot
pub fn unread_dot() -> Element<'static, Message> {
    container(text(""))
        .width(Length::Fixed(8.0))
        .height(Length::Fixed(8.0))
        .style(|_| container::Style {
            background: Some(Background::Color(colors::ACCENT_PRIMARY)),
            border: Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .into()
}

/// Create an attachment indicator
pub fn attachment_indicator(count: i32) -> Element<'static, Message> {
    if count == 0 {
        return text("").into();
    }

    let label = if count == 1 {
        "📎".to_string()
    } else {
        format!("📎 {}", count)
    };

    text(label)
        .size(typography::SIZE_XS)
        .style(components::text_muted)
        .into()
}

// Import for attachment_indicator
use crate::theme::components;
