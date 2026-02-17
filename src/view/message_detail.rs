//! Message detail view
//!
//! Displays a single message in full detail with header, body, and attachments.

use crate::api::types::MessageDetail;
use crate::message::Message;
use crate::theme::{colors, components, spacing, typography};
use crate::view::widgets::{avatar, format_bytes};
use chrono::{DateTime, Local, Utc};
use iced::widget::{column, container, row, scrollable, text, Space};
use iced::{Background, Border, Element, Length};

/// Render the message detail view
pub fn message_detail_view<'a>(message: &'a MessageDetail) -> Element<'a, Message> {
    // Header section
    let header = header_section(message);

    // Body section (scrollable)
    let body = body_section(&message.body);

    // Attachments section (if any)
    let attachments = if !message.attachments.is_empty() {
        attachments_section(message)
    } else {
        column![].into()
    };

    // Keyboard hints
    let hints = text("Esc: back | ←/→: prev/next message")
        .size(typography::SIZE_XS)
        .style(components::text_muted);

    column![
        header,
        Space::with_height(spacing::LG),
        body,
        Space::with_height(spacing::MD),
        attachments,
        Space::with_height(Length::Fill),
        hints,
    ]
    .spacing(spacing::XS)
    .padding(spacing::LG)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

/// Render the message header section
fn header_section<'a>(message: &'a MessageDetail) -> Element<'a, Message> {
    // Get sender name from email
    let sender_name = extract_name(&message.from_addr);

    // Avatar
    let avatar_widget = avatar(&sender_name, 48);

    // Subject (large)
    let subject = text(&message.subject)
        .size(typography::SIZE_LG)
        .style(components::text_primary);

    // From
    let from_label = text("From")
        .size(typography::SIZE_XS)
        .style(components::text_muted);
    let from_value = text(&message.from_addr)
        .size(typography::SIZE_SM)
        .style(components::text_secondary);

    // To
    let to_label = text("To")
        .size(typography::SIZE_XS)
        .style(components::text_muted);
    let to_value = text(message.to.join(", "))
        .size(typography::SIZE_SM)
        .style(components::text_secondary);

    // CC (if present)
    let cc_section: Element<'a, Message> = if !message.cc.is_empty() {
        let cc_label = text("CC")
            .size(typography::SIZE_XS)
            .style(components::text_muted);
        let cc_value = text(message.cc.join(", "))
            .size(typography::SIZE_SM)
            .style(components::text_secondary);
        column![cc_label, cc_value].spacing(2).into()
    } else {
        Space::with_height(0).into()
    };

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
                        .style(components::text_accent)
                )
                .padding([2, spacing::SM])
                .style(|_| container::Style {
                    background: Some(Background::Color(colors::with_alpha(colors::ACCENT_PRIMARY, 0.15))),
                    border: Border {
                        radius: 4.0.into(),
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
            column![from_label, from_value].spacing(2),
            Space::with_height(spacing::XS),
            column![to_label, to_value].spacing(2),
            cc_section,
            Space::with_height(spacing::SM),
            labels_section,
        ]
        .width(Length::Fill),
        Space::with_width(spacing::MD),
        date_text,
    ]
    .align_y(iced::Alignment::Start);

    // Header container with subtle background
    container(header_content)
        .width(Length::Fill)
        .padding(spacing::LG)
        .style(|_| container::Style {
            background: Some(Background::Color(colors::BG_ELEVATED)),
            border: Border {
                radius: 8.0.into(),
                ..Default::default()
            },
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

/// Render the attachments section
fn attachments_section<'a>(message: &'a MessageDetail) -> Element<'a, Message> {
    let title = text("Attachments")
        .size(typography::SIZE_SM)
        .style(components::text_primary);

    let attachment_rows: Vec<Element<'a, Message>> = message
        .attachments
        .iter()
        .map(|att| {
            let icon = get_file_icon(&att.filename);
            let filename = text(&att.filename)
                .size(typography::SIZE_SM)
                .style(components::text_secondary);
            let size = text(format!("({})", format_bytes(att.size_bytes)))
                .size(typography::SIZE_XS)
                .style(components::text_muted);

            container(
                row![
                    text(icon).size(typography::SIZE_MD),
                    Space::with_width(spacing::SM),
                    filename,
                    Space::with_width(spacing::SM),
                    size,
                ]
                .align_y(iced::Alignment::Center)
            )
            .padding([spacing::XS, spacing::SM])
            .style(|_| container::Style {
                background: Some(Background::Color(colors::BG_SURFACE)),
                border: Border {
                    radius: 4.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .into()
        })
        .collect();

    container(
        column![
            title,
            Space::with_height(spacing::SM),
        ]
        .push(column(attachment_rows).spacing(spacing::XS)),
    )
    .width(Length::Fill)
    .padding(spacing::MD)
    .style(|_| container::Style {
        background: Some(Background::Color(colors::BG_ELEVATED)),
        border: Border {
            radius: 6.0.into(),
            width: 1.0,
            color: colors::BORDER_SUBTLE,
        },
        ..Default::default()
    })
    .into()
}

/// Get file icon based on extension
fn get_file_icon(filename: &str) -> &'static str {
    let extension = filename.rsplit('.').next().unwrap_or("").to_lowercase();
    match extension.as_str() {
        "pdf" => "📄",
        "doc" | "docx" => "📝",
        "xls" | "xlsx" => "📊",
        "ppt" | "pptx" => "📽️",
        "png" | "jpg" | "jpeg" | "gif" | "webp" => "🖼️",
        "zip" | "tar" | "gz" | "rar" => "📦",
        "mp3" | "wav" | "m4a" => "🎵",
        "mp4" | "mov" | "avi" => "🎬",
        _ => "📎",
    }
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
