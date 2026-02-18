//! Message detail view
//!
//! Displays a single message in full detail with header, body, and attachments.

use crate::api::types::MessageDetail;
use crate::message::Message;
use crate::model::downloads::DownloadTracker;
use crate::theme::{colors, components, icons, spacing, typography};
use crate::view::attachments::attachments_section;
use crate::view::widgets::avatar;
use chrono::{DateTime, Local, Utc};
use iced::widget::{button, column, container, row, scrollable, text, Space};
use iced::{Background, Border, Element, Length};

/// Render the message detail view
pub fn message_detail_view<'a>(
    message: &'a MessageDetail,
    downloads: &'a DownloadTracker,
) -> Element<'a, Message> {
    // Action bar at top
    let action_bar = action_bar_section(message.id);

    // Header section
    let header = header_section(message);

    // Horizontal divider between header and body
    let divider_top = horizontal_divider();

    // Body section (scrollable)
    let body = body_section(&message.body);

    // Horizontal divider between body and attachments
    let divider_bottom = horizontal_divider();

    // Attachments section with download support
    let attachments = attachments_section(message.id, &message.attachments, downloads);

    // Keyboard hints
    let hints = text("Esc: back | \u{2190}/\u{2192}: prev/next message")
        .size(typography::SIZE_XS)
        .font(typography::FONT_MONO)
        .style(components::text_muted);

    column![
        action_bar,
        header,
        divider_top,
        body,
        divider_bottom,
        attachments,
        Space::with_height(Length::Fill),
        hints,
    ]
    .spacing(spacing::SM)
    .padding(spacing::LG)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

/// Render the action bar with back, reply, forward, compose buttons
fn action_bar_section(message_id: i64) -> Element<'static, Message> {
    let back_btn = button(
        text(icons::ARROW_LEFT)
            .size(typography::SIZE_MD),
    )
    .padding([spacing::XS, spacing::SM])
    .style(components::button_ghost)
    .on_press(Message::GoBack);

    let reply_btn = button(
        text(icons::REPLY)
            .size(typography::SIZE_SM),
    )
    .padding([spacing::XS, spacing::SM])
    .style(components::button_ghost)
    .on_press(Message::OpenReply(message_id));

    let reply_all_btn = button(
        text(icons::REPLY_ALL)
            .size(typography::SIZE_SM),
    )
    .padding([spacing::XS, spacing::SM])
    .style(components::button_ghost)
    .on_press(Message::OpenReplyAll(message_id));

    let forward_btn = button(
        text(icons::FORWARD)
            .size(typography::SIZE_SM),
    )
    .padding([spacing::XS, spacing::SM])
    .style(components::button_ghost)
    .on_press(Message::OpenForward(message_id));

    let compose_btn = button(
        text(icons::COMPOSE)
            .size(typography::SIZE_SM),
    )
    .padding([spacing::XS, spacing::SM])
    .style(components::button_ghost)
    .on_press(Message::OpenCompose);

    row![
        back_btn,
        Space::with_width(Length::Fill),
        reply_btn,
        reply_all_btn,
        forward_btn,
        compose_btn,
    ]
    .spacing(spacing::XS)
    .align_y(iced::Alignment::Center)
    .width(Length::Fill)
    .into()
}

/// Render the message header section
fn header_section<'a>(message: &'a MessageDetail) -> Element<'a, Message> {
    // Get sender name from email
    let sender_name = extract_name(&message.from_addr);

    // Avatar
    let avatar_widget = avatar(&sender_name, 48);

    // Subject (SIZE_XL, FONT_SEMIBOLD, TEXT_PRIMARY)
    let subject = text(&message.subject)
        .size(typography::SIZE_XL)
        .font(typography::FONT_SEMIBOLD)
        .style(components::text_primary);

    // Recipient line: "from -> to, cc" compact single line
    let recipient_line = build_recipient_line(message);

    // Date (formatted nicely)
    let date_text = text(format_date(&message.sent_at))
        .size(typography::SIZE_XS)
        .style(components::text_muted);

    // Labels (if present)
    let labels_section: Element<'a, Message> = if !message.labels.is_empty() {
        let labels_row: Vec<Element<'a, Message>> = message
            .labels
            .iter()
            .map(|label| {
                container(
                    text(label.clone())
                        .size(typography::SIZE_XS)
                        .style(components::text_accent),
                )
                .padding([2, spacing::SM])
                .style(|_| container::Style {
                    background: Some(Background::Color(colors::with_alpha(
                        colors::ACCENT_PRIMARY,
                        0.15,
                    ))),
                    border: Border {
                        radius: spacing::RADIUS_SM.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .into()
            })
            .collect();

        row(labels_row).spacing(spacing::XS).into()
    } else {
        Space::with_height(0).into()
    };

    // Header layout
    let header_content = row![
        avatar_widget,
        Space::with_width(spacing::MD),
        column![
            subject,
            Space::with_height(spacing::SM),
            recipient_line,
            Space::with_height(spacing::SM),
            labels_section,
        ]
        .width(Length::Fill),
        Space::with_width(spacing::MD),
        date_text,
    ]
    .align_y(iced::Alignment::Start);

    // Header container with RADIUS_LG
    container(header_content)
        .width(Length::Fill)
        .padding(spacing::LG)
        .style(|_| container::Style {
            background: Some(Background::Color(colors::BG_ELEVATED)),
            border: Border {
                radius: spacing::RADIUS_LG.into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .into()
}

/// Build compact recipient line: "from -> to, cc"
fn build_recipient_line<'a>(message: &'a MessageDetail) -> Element<'a, Message> {
    let mut parts = String::new();

    // From
    parts.push_str(&extract_name(&message.from_addr));
    parts.push_str(" \u{2192} ");

    // To
    let to_names: Vec<String> = message.to.iter().map(|t| extract_name(t)).collect();
    parts.push_str(&to_names.join(", "));

    // CC (if present)
    if !message.cc.is_empty() {
        let cc_names: Vec<String> = message.cc.iter().map(|c| extract_name(c)).collect();
        parts.push_str(", ");
        parts.push_str(&cc_names.join(", "));
    }

    text(parts)
        .size(typography::SIZE_SM)
        .style(components::text_secondary)
        .into()
}

/// Render a horizontal divider (1px line)
fn horizontal_divider<'a>() -> Element<'a, Message> {
    container(Space::with_height(0))
        .width(Length::Fill)
        .height(Length::Fixed(1.0))
        .style(|_| container::Style {
            background: Some(Background::Color(colors::BORDER_SUBTLE)),
            ..Default::default()
        })
        .into()
}

/// Render the scrollable body section
fn body_section<'a>(body: &'a str) -> Element<'a, Message> {
    let body_text = if body.is_empty() {
        text("(No message body)")
            .size(typography::SIZE_SM)
            .style(components::text_muted)
    } else {
        text(body)
            .size(typography::SIZE_SM)
            .style(components::text_secondary)
    };

    scrollable(
        container(body_text)
            .width(Length::Fill)
            .padding([spacing::MD, 0]),
    )
    .height(Length::FillPortion(3))
    .into()
}

/// Extract name from email address
fn extract_name(email: &str) -> String {
    // Try to extract display name from "Name <email>" format
    if let Some(idx) = email.find('<') {
        let name = email[..idx].trim().trim_matches('"');
        if !name.is_empty() {
            return name.to_string();
        }
    }
    // Fall back to email local part
    email.split('@').next().unwrap_or(email).to_string()
}

/// Format a DateTime for display
fn format_date(dt: &DateTime<Utc>) -> String {
    let local: DateTime<Local> = dt.with_timezone(&Local);
    local.format("%a, %b %d, %Y at %I:%M %p").to_string()
}
