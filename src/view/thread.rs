//! Thread/conversation view
//!
//! Displays all messages in a thread with expand/collapse capability.

use crate::api::types::MessageDetail;
use crate::message::Message;
use crate::model::ThreadState;
use crate::theme::{colors, components, spacing, typography};
use crate::view::widgets::{avatar, format_bytes};
use chrono::{DateTime, Local, Utc};
use iced::widget::{button, column, container, row, scrollable, text, Space};
use iced::{Background, Border, Element, Length};

/// Render the thread/conversation view
pub fn thread_view(thread_state: &ThreadState) -> Element<'_, Message> {
    if thread_state.is_loading {
        return loading_view();
    }

    if thread_state.messages.is_empty() {
        return empty_view();
    }

    // Header with thread info
    let header = thread_header(thread_state);

    // Message list
    let messages_col = thread_state
        .messages
        .iter()
        .enumerate()
        .fold(column![].spacing(spacing::SM), |col, (idx, msg)| {
            let is_expanded = thread_state.is_expanded(idx);
            let is_focused = idx == thread_state.focused_index;
            col.push(thread_message_card(msg, idx, is_expanded, is_focused))
        });

    // Action buttons at the bottom
    let actions = action_buttons(thread_state);

    // Keyboard hints
    let hints = text("e: expand all | E: collapse all | Enter: toggle focused | j/k: navigate | Esc: back")
        .size(typography::SIZE_XS)
        .style(components::text_muted);

    column![
        header,
        Space::with_height(spacing::MD),
        scrollable(messages_col.padding(spacing::SM))
            .height(Length::FillPortion(5)),
        Space::with_height(spacing::MD),
        actions,
        Space::with_height(Length::Fill),
        hints,
    ]
    .spacing(spacing::XS)
    .padding(spacing::LG)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

/// Thread header with message count and subject
fn thread_header(thread_state: &ThreadState) -> Element<'_, Message> {
    let msg_count = thread_state.message_count();
    let subject = thread_state
        .messages
        .first()
        .map(|m| m.subject.as_str())
        .unwrap_or("(No subject)");

    let title = text(subject)
        .size(typography::SIZE_LG)
        .style(components::text_primary);

    let count_text = text(format!("{} messages in conversation", msg_count))
        .size(typography::SIZE_SM)
        .style(components::text_muted);

    // Expand/collapse all buttons
    let expand_btn = button(text("Expand All").size(typography::SIZE_XS))
        .padding([spacing::XS, spacing::SM])
        .style(components::button_ghost)
        .on_press(Message::ExpandAllThread);

    let collapse_btn = button(text("Collapse All").size(typography::SIZE_XS))
        .padding([spacing::XS, spacing::SM])
        .style(components::button_ghost)
        .on_press(Message::CollapseAllThread);

    container(
        column![
            title,
            Space::with_height(spacing::XS),
            row![
                count_text,
                Space::with_width(Length::Fill),
                expand_btn,
                collapse_btn,
            ]
            .align_y(iced::Alignment::Center),
        ]
    )
    .width(Length::Fill)
    .padding(spacing::MD)
    .style(|_| container::Style {
        background: Some(Background::Color(colors::BG_ELEVATED)),
        border: Border {
            radius: 6.0.into(),
            ..Default::default()
        },
        ..Default::default()
    })
    .into()
}

/// Single message card in the thread (collapsed or expanded)
fn thread_message_card(
    message: &MessageDetail,
    index: usize,
    is_expanded: bool,
    is_focused: bool,
) -> Element<'_, Message> {
    // Different styling for focused vs non-focused
    let border_color = if is_focused {
        colors::ACCENT_PRIMARY
    } else {
        colors::BORDER_SUBTLE
    };

    let bg_color = if is_focused {
        colors::with_alpha(colors::ACCENT_PRIMARY, 0.08)
    } else {
        colors::BG_SURFACE
    };

    if is_expanded {
        // Expanded view - full message
        expanded_message_view(message, index, is_focused, border_color, bg_color)
    } else {
        // Collapsed view - just header
        collapsed_message_view(message, index, is_focused, border_color, bg_color)
    }
}

/// Collapsed message header (clickable to expand)
fn collapsed_message_view(
    message: &MessageDetail,
    index: usize,
    is_focused: bool,
    border_color: iced::Color,
    bg_color: iced::Color,
) -> Element<'_, Message> {
    let sender_name = extract_name(&message.from_addr);
    let date_str = format_date(&message.sent_at);

    let avatar_widget = avatar(&sender_name, 32);

    let sender = text(sender_name)
        .size(typography::SIZE_SM)
        .style(if is_focused {
            components::text_accent
        } else {
            components::text_primary
        });

    let date = text(date_str)
        .size(typography::SIZE_XS)
        .style(components::text_muted);

    let expand_hint = text("Click to expand")
        .size(typography::SIZE_XS)
        .style(components::text_muted);

    let content = row![
        avatar_widget,
        Space::with_width(spacing::SM),
        column![sender, date].spacing(2),
        Space::with_width(Length::Fill),
        expand_hint,
    ]
    .align_y(iced::Alignment::Center)
    .padding(spacing::MD);

    button(content)
        .width(Length::Fill)
        .style(move |_theme, status| {
            let hover_bg = match status {
                button::Status::Hovered => colors::BG_ELEVATED,
                _ => bg_color,
            };
            button::Style {
                background: Some(Background::Color(hover_bg)),
                border: Border {
                    radius: 6.0.into(),
                    width: if is_focused { 2.0 } else { 1.0 },
                    color: border_color,
                },
                ..Default::default()
            }
        })
        .on_press(Message::ToggleThreadMessage(index))
        .into()
}

/// Expanded message view (full body)
fn expanded_message_view(
    message: &MessageDetail,
    index: usize,
    is_focused: bool,
    border_color: iced::Color,
    bg_color: iced::Color,
) -> Element<'_, Message> {
    let sender_name = extract_name(&message.from_addr);
    let avatar_widget = avatar(&sender_name, 40);

    // Header row
    let sender = text(sender_name)
        .size(typography::SIZE_SM)
        .style(if is_focused {
            components::text_accent
        } else {
            components::text_primary
        });

    let from_email = text(&message.from_addr)
        .size(typography::SIZE_XS)
        .style(components::text_secondary);

    let date = text(format_date(&message.sent_at))
        .size(typography::SIZE_XS)
        .style(components::text_muted);

    let to_label = text("To:")
        .size(typography::SIZE_XS)
        .style(components::text_muted);

    let to_list = text(message.to.join(", "))
        .size(typography::SIZE_XS)
        .style(components::text_secondary);

    // Collapse button
    let collapse_btn = button(text("Collapse").size(typography::SIZE_XS))
        .padding([spacing::XS, spacing::SM])
        .style(components::button_ghost)
        .on_press(Message::ToggleThreadMessage(index));

    let header = row![
        avatar_widget,
        Space::with_width(spacing::SM),
        column![
            sender,
            from_email,
            row![to_label, Space::with_width(spacing::XS), to_list].align_y(iced::Alignment::Center),
        ]
        .spacing(2),
        Space::with_width(Length::Fill),
        column![date, collapse_btn].align_x(iced::Alignment::End),
    ]
    .align_y(iced::Alignment::Start);

    // CC section (if present)
    let cc_section: Element<'_, Message> = if !message.cc.is_empty() {
        row![
            text("CC:").size(typography::SIZE_XS).style(components::text_muted),
            Space::with_width(spacing::XS),
            text(message.cc.join(", ")).size(typography::SIZE_XS).style(components::text_secondary),
        ]
        .align_y(iced::Alignment::Center)
        .into()
    } else {
        Space::with_height(0).into()
    };

    // Body
    let body_text = if message.body.is_empty() {
        text("(No message body)")
            .size(typography::SIZE_SM)
            .style(components::text_muted)
    } else {
        text(&message.body)
            .size(typography::SIZE_SM)
            .style(components::text_secondary)
    };

    // Attachments (if any)
    let attachments_section: Element<'_, Message> = if !message.attachments.is_empty() {
        let att_list: Vec<Element<'_, Message>> = message
            .attachments
            .iter()
            .map(|att| {
                let icon = get_file_icon(&att.filename);
                container(
                    row![
                        text(icon).size(typography::SIZE_SM),
                        Space::with_width(spacing::XS),
                        text(&att.filename)
                            .size(typography::SIZE_XS)
                            .style(components::text_secondary),
                        Space::with_width(spacing::XS),
                        text(format!("({})", format_bytes(att.size_bytes)))
                            .size(typography::SIZE_XS)
                            .style(components::text_muted),
                    ]
                    .align_y(iced::Alignment::Center)
                )
                .padding([spacing::XS, spacing::SM])
                .style(|_| container::Style {
                    background: Some(Background::Color(colors::BG_ELEVATED)),
                    border: Border {
                        radius: 4.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .into()
            })
            .collect();

        column![
            text("Attachments")
                .size(typography::SIZE_XS)
                .style(components::text_muted),
            Space::with_height(spacing::XS),
            row(att_list).spacing(spacing::XS),
        ]
        .into()
    } else {
        Space::with_height(0).into()
    };

    // Labels (if any)
    let labels_section: Element<'_, Message> = if !message.labels.is_empty() {
        let label_widgets: Vec<Element<'_, Message>> = message
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
                    background: Some(Background::Color(colors::with_alpha(
                        colors::ACCENT_PRIMARY,
                        0.15,
                    ))),
                    border: Border {
                        radius: 4.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .into()
            })
            .collect();

        row(label_widgets).spacing(spacing::XS).into()
    } else {
        Space::with_height(0).into()
    };

    container(
        column![
            header,
            Space::with_height(spacing::XS),
            cc_section,
            labels_section,
            Space::with_height(spacing::SM),
            container(body_text)
                .width(Length::Fill)
                .padding([spacing::SM, 0]),
            Space::with_height(spacing::SM),
            attachments_section,
        ]
        .spacing(spacing::XS)
    )
    .width(Length::Fill)
    .padding(spacing::MD)
    .style(move |_| container::Style {
        background: Some(Background::Color(bg_color)),
        border: Border {
            radius: 6.0.into(),
            width: if is_focused { 2.0 } else { 1.0 },
            color: border_color,
        },
        ..Default::default()
    })
    .into()
}

/// Action buttons at the bottom of the thread view
fn action_buttons(thread_state: &ThreadState) -> Element<'_, Message> {
    // Get the last message for reply actions
    let last_message_id = thread_state
        .messages
        .last()
        .map(|m| m.id)
        .unwrap_or(0);

    let reply_btn = button(
        text("Reply")
            .size(typography::SIZE_SM)
    )
    .padding([spacing::SM, spacing::LG])
    .style(components::button_primary)
    .on_press(Message::OpenReply(last_message_id));

    let reply_all_btn = button(
        text("Reply All")
            .size(typography::SIZE_SM)
    )
    .padding([spacing::SM, spacing::LG])
    .style(components::button_secondary)
    .on_press(Message::OpenReplyAll(last_message_id));

    let forward_btn = button(
        text("Forward")
            .size(typography::SIZE_SM)
    )
    .padding([spacing::SM, spacing::LG])
    .style(components::button_secondary)
    .on_press(Message::OpenForward(last_message_id));

    row![
        reply_btn,
        Space::with_width(spacing::SM),
        reply_all_btn,
        Space::with_width(spacing::SM),
        forward_btn,
    ]
    .align_y(iced::Alignment::Center)
    .into()
}

/// Loading view while fetching thread
fn loading_view() -> Element<'static, Message> {
    container(
        column![
            text("Loading conversation...")
                .size(typography::SIZE_MD)
                .style(components::text_secondary),
        ]
        .align_x(iced::Alignment::Center)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .into()
}

/// Empty view when no messages
fn empty_view() -> Element<'static, Message> {
    container(
        column![
            text("No messages in this conversation")
                .size(typography::SIZE_MD)
                .style(components::text_muted),
        ]
        .align_x(iced::Alignment::Center)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .into()
}

/// Extract name from email address
fn extract_name(email: &str) -> String {
    if let Some(idx) = email.find('<') {
        let name = email[..idx].trim().trim_matches('"');
        if !name.is_empty() {
            return name.to_string();
        }
    }
    email.split('@').next().unwrap_or(email).to_string()
}

/// Format a DateTime for display
fn format_date(dt: &DateTime<Utc>) -> String {
    let local: DateTime<Local> = dt.with_timezone(&Local);
    local.format("%b %d, %Y at %I:%M %p").to_string()
}

/// Get file icon based on extension
fn get_file_icon(filename: &str) -> &'static str {
    let extension = filename.rsplit('.').next().unwrap_or("").to_lowercase();
    match extension.as_str() {
        "pdf" => "PDF",
        "doc" | "docx" => "DOC",
        "xls" | "xlsx" => "XLS",
        "ppt" | "pptx" => "PPT",
        "png" | "jpg" | "jpeg" | "gif" | "webp" => "IMG",
        "zip" | "tar" | "gz" | "rar" => "ZIP",
        "mp3" | "wav" | "m4a" => "AUD",
        "mp4" | "mov" | "avi" => "VID",
        _ => "FILE",
    }
}
