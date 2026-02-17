//! Message detail view
//!
//! Displays a single message in full detail with header, body, and attachments.

use crate::api::types::MessageDetail;
use crate::message::Message;
use crate::view::widgets::format_bytes;
use chrono::{DateTime, Local, Utc};
use iced::widget::{column, container, row, scrollable, text, Space};
use iced::{Element, Length, Theme};

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
    let hints = text("Esc: back | \u{2190}/\u{2192}: prev/next message")
        .size(12)
        .style(|theme: &Theme| {
            let palette = theme.palette();
            text::Style {
                color: Some(iced::Color {
                    a: 0.6,
                    ..palette.text
                }),
            }
        });

    column![
        header,
        Space::with_height(15),
        body,
        Space::with_height(10),
        attachments,
        Space::with_height(Length::Fill),
        hints,
    ]
    .spacing(5)
    .padding(20)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

/// Render the message header section
fn header_section<'a>(message: &'a MessageDetail) -> Element<'a, Message> {
    // Subject (large, bold)
    let subject = text(&message.subject).size(24);

    // From
    let from_label = text("From:").size(13).style(label_style);
    let from_value = text(&message.from_addr).size(13);
    let from_row = row![from_label, Space::with_width(10), from_value]
        .align_y(iced::Alignment::Center);

    // To
    let to_label = text("To:").size(13).style(label_style);
    let to_value = text(message.to.join(", ")).size(13);
    let to_row = row![to_label, Space::with_width(10), to_value]
        .align_y(iced::Alignment::Center);

    // CC (if present)
    let cc_row: Element<'a, Message> = if !message.cc.is_empty() {
        let cc_label = text("CC:").size(13).style(label_style);
        let cc_value = text(message.cc.join(", ")).size(13);
        row![cc_label, Space::with_width(10), cc_value]
            .align_y(iced::Alignment::Center)
            .into()
    } else {
        row![].into()
    };

    // Date (formatted nicely)
    let date_label = text("Date:").size(13).style(label_style);
    let date_value = text(format_date(&message.sent_at)).size(13);
    let date_row = row![date_label, Space::with_width(10), date_value]
        .align_y(iced::Alignment::Center);

    // Labels (if present)
    let labels_row: Element<'a, Message> = if !message.labels.is_empty() {
        let labels_label = text("Labels:").size(13).style(label_style);
        let labels_value = text(message.labels.join(", ")).size(13);
        row![labels_label, Space::with_width(10), labels_value]
            .align_y(iced::Alignment::Center)
            .into()
    } else {
        row![].into()
    };

    // Header container with subtle background
    let header_style = |theme: &Theme| {
        let palette = theme.palette();
        container::Style {
            background: Some(iced::Background::Color(iced::Color {
                a: 0.05,
                ..palette.primary
            })),
            border: iced::Border {
                radius: 8.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    };

    container(
        column![subject, Space::with_height(10), from_row, to_row, cc_row, date_row, labels_row,]
            .spacing(4),
    )
    .width(Length::Fill)
    .padding(15)
    .style(header_style)
    .into()
}

/// Render the scrollable body section
fn body_section<'a>(body: &'a str) -> Element<'a, Message> {
    let body_text = if body.is_empty() {
        text("(No message body)").size(14).style(|theme: &Theme| {
            let palette = theme.palette();
            text::Style {
                color: Some(iced::Color {
                    a: 0.5,
                    ..palette.text
                }),
            }
        })
    } else {
        text(body).size(14)
    };

    scrollable(
        container(body_text)
            .width(Length::Fill)
            .padding([10, 0]),
    )
    .height(Length::FillPortion(3))
    .into()
}

/// Render the attachments section
fn attachments_section<'a>(message: &'a MessageDetail) -> Element<'a, Message> {
    let title = text("Attachments").size(14).style(label_style);

    let attachment_rows: Vec<Element<'a, Message>> = message
        .attachments
        .iter()
        .map(|att| {
            let filename = text(&att.filename).size(13);
            let size = text(format!("({})", format_bytes(att.size_bytes)))
                .size(12)
                .style(|theme: &Theme| {
                    let palette = theme.palette();
                    text::Style {
                        color: Some(iced::Color {
                            a: 0.6,
                            ..palette.text
                        }),
                    }
                });

            row![text("\u{1F4CE}").size(12), Space::with_width(5), filename, Space::with_width(5), size]
                .align_y(iced::Alignment::Center)
                .into()
        })
        .collect();

    let attachments_style = |theme: &Theme| {
        let palette = theme.palette();
        container::Style {
            background: Some(iced::Background::Color(iced::Color {
                a: 0.03,
                ..palette.primary
            })),
            border: iced::Border {
                radius: 6.0.into(),
                width: 1.0,
                color: iced::Color {
                    a: 0.1,
                    ..palette.text
                },
            },
            ..Default::default()
        }
    };

    container(
        column![title, Space::with_height(8)]
            .push(column(attachment_rows).spacing(4)),
    )
    .width(Length::Fill)
    .padding(12)
    .style(attachments_style)
    .into()
}

/// Style for field labels (From:, To:, etc.)
fn label_style(theme: &Theme) -> text::Style {
    let palette = theme.palette();
    text::Style {
        color: Some(iced::Color {
            a: 0.7,
            ..palette.text
        }),
    }
}

/// Format a DateTime for display
fn format_date(dt: &DateTime<Utc>) -> String {
    let local: DateTime<Local> = dt.with_timezone(&Local);
    local.format("%a, %b %d, %Y at %I:%M %p").to_string()
}
