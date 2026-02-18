//! Compose email modal view
//!
//! Full-featured email composition with recipients, subject, body, and attachments.

use crate::message::Message;
use crate::model::ComposeState;
use crate::theme::{colors, components, icons, spacing, typography};
use crate::view::widgets::format_bytes;
use iced::widget::{button, column, container, horizontal_rule, row, text, text_input, Space};
use iced::{Background, Border, Element, Length};

/// Render the compose modal overlay
pub fn compose_modal(compose: &ComposeState) -> Element<'static, Message> {
    if !compose.is_open {
        return Space::with_height(0).into();
    }

    // Semi-transparent backdrop
    let backdrop = container(Space::new(Length::Fill, Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(components::modal_backdrop_style);

    // Modal dialog
    let dialog = compose_dialog(compose);

    iced::widget::stack![backdrop, iced::widget::center(dialog)].into()
}

/// The compose dialog content
fn compose_dialog(compose: &ComposeState) -> Element<'static, Message> {
    // Header with mode title and close button
    let header = compose_header(compose);

    // From selector
    let from_section = from_section(compose);

    // Section divider
    let divider1 = section_divider();

    // Recipients section
    let recipients = recipients_section(compose);

    // Section divider
    let divider2 = section_divider();

    // Subject
    let subject_section = subject_section(compose);

    // Section divider
    let divider3 = section_divider();

    // Body editor
    let body_section = body_section(compose);

    // Attachments
    let attachments: Element<'static, Message> = if !compose.attachments.is_empty() {
        attachments_section(compose)
    } else {
        Space::with_height(0).into()
    };

    // Footer with actions
    let footer = compose_footer(compose);

    // Error message if present
    let error_msg: Element<'static, Message> = if let Some(err) = &compose.send_error {
        container(
            text(err.clone())
                .size(typography::SIZE_SM)
                .style(components::text_error),
        )
        .padding([spacing::SM, spacing::MD])
        .style(|_| container::Style {
            background: Some(Background::Color(colors::with_alpha(
                colors::ACCENT_ERROR,
                0.15,
            ))),
            border: Border {
                radius: spacing::RADIUS_SM.into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .into()
    } else {
        Space::with_height(0).into()
    };

    let content = column![
        header,
        Space::with_height(spacing::MD),
        from_section,
        divider1,
        recipients,
        divider2,
        subject_section,
        divider3,
        body_section,
        attachments,
        error_msg,
        Space::with_height(spacing::MD),
        footer,
    ]
    .spacing(spacing::XS)
    .padding(spacing::LG)
    .width(Length::Fixed(700.0));

    container(content)
        .style(components::modal_dialog_style)
        .into()
}

/// Section divider line
fn section_divider() -> Element<'static, Message> {
    container(horizontal_rule(1))
        .padding([spacing::XS, 0])
        .style(|_| container::Style {
            ..Default::default()
        })
        .into()
}

/// Header with title and close button
fn compose_header(compose: &ComposeState) -> Element<'static, Message> {
    let title = text(compose.mode.display_name())
        .size(typography::SIZE_LG)
        .font(typography::FONT_MEDIUM)
        .style(components::text_primary);

    let close_btn = button(text(icons::DELETE).size(typography::SIZE_MD))
        .padding([spacing::XS, spacing::SM])
        .style(components::button_ghost)
        .on_press(Message::ComposeClose);

    row![title, Space::with_width(Length::Fill), close_btn]
        .align_y(iced::Alignment::Center)
        .into()
}

/// From account selector
fn from_section(compose: &ComposeState) -> Element<'static, Message> {
    let label = text("From")
        .size(typography::SIZE_XS)
        .style(components::text_muted);

    let account_display = text(compose.from_account.clone())
        .size(typography::SIZE_SM)
        .style(components::text_primary);

    column![label, account_display].spacing(spacing::SPACE_1).into()
}

/// Recipients section (To, CC, BCC)
fn recipients_section(compose: &ComposeState) -> Element<'static, Message> {
    // To field
    let to_label = text("To")
        .size(typography::SIZE_XS)
        .style(components::text_muted);

    let to_chips: Vec<Element<'static, Message>> = compose
        .to
        .iter()
        .enumerate()
        .map(|(i, email)| recipient_chip(email.clone(), Message::ComposeRemoveTo(i)))
        .collect();

    let to_input = text_input("Add recipient...", &compose.to_input)
        .on_input(Message::ComposeToChanged)
        .on_submit(Message::ComposeAddTo)
        .padding(spacing::SM)
        .size(typography::SIZE_SM)
        .style(components::text_input_style);

    let mut to_row = row(to_chips).spacing(spacing::XS);
    to_row = to_row.push(to_input);
    let to_row = to_row.align_y(iced::Alignment::Center);

    let mut sections = column![column![to_label, to_row].spacing(spacing::SPACE_1)].spacing(spacing::SM);

    // CC/BCC toggle
    if !compose.show_cc_bcc {
        let toggle_btn = button(
            text("CC/BCC")
                .size(typography::SIZE_XS)
                .style(components::text_accent),
        )
        .padding([spacing::SPACE_1, spacing::SM])
        .style(components::button_ghost)
        .on_press(Message::ComposeToggleCcBcc);

        sections = sections.push(toggle_btn);
    } else {
        // CC field
        let cc_label = text("CC")
            .size(typography::SIZE_XS)
            .style(components::text_muted);

        let cc_chips: Vec<Element<'static, Message>> = compose
            .cc
            .iter()
            .enumerate()
            .map(|(i, email)| recipient_chip(email.clone(), Message::ComposeRemoveCc(i)))
            .collect();

        let cc_input = text_input("Add CC...", &compose.cc_input)
            .on_input(Message::ComposeCcChanged)
            .on_submit(Message::ComposeAddCc)
            .padding(spacing::SM)
            .size(typography::SIZE_SM)
            .style(components::text_input_style);

        let mut cc_row = row(cc_chips).spacing(spacing::XS);
        cc_row = cc_row.push(cc_input);
        let cc_row = cc_row.align_y(iced::Alignment::Center);

        // BCC field
        let bcc_label = text("BCC")
            .size(typography::SIZE_XS)
            .style(components::text_muted);

        let bcc_chips: Vec<Element<'static, Message>> = compose
            .bcc
            .iter()
            .enumerate()
            .map(|(i, email)| recipient_chip(email.clone(), Message::ComposeRemoveBcc(i)))
            .collect();

        let bcc_input = text_input("Add BCC...", &compose.bcc_input)
            .on_input(Message::ComposeBccChanged)
            .on_submit(Message::ComposeAddBcc)
            .padding(spacing::SM)
            .size(typography::SIZE_SM)
            .style(components::text_input_style);

        let mut bcc_row = row(bcc_chips).spacing(spacing::XS);
        bcc_row = bcc_row.push(bcc_input);
        let bcc_row = bcc_row.align_y(iced::Alignment::Center);

        sections = sections
            .push(column![cc_label, cc_row].spacing(spacing::SPACE_1))
            .push(column![bcc_label, bcc_row].spacing(spacing::SPACE_1));
    }

    sections.into()
}

/// Single recipient chip with RADIUS_SM, BG_ELEVATED background, copper remove button
fn recipient_chip(email: String, on_remove: Message) -> Element<'static, Message> {
    let content = row![
        text(email)
            .size(typography::SIZE_XS)
            .style(components::text_primary),
        Space::with_width(spacing::XS),
        button(
            text(icons::DELETE)
                .size(typography::SIZE_2XS)
                .style(components::text_accent)
        )
        .padding([0, spacing::XS])
        .style(components::button_ghost)
        .on_press(on_remove),
    ]
    .align_y(iced::Alignment::Center);

    container(content)
        .padding([spacing::SPACE_1, spacing::SM])
        .style(|_| container::Style {
            background: Some(Background::Color(colors::BG_ELEVATED)),
            border: Border {
                radius: spacing::RADIUS_SM.into(),
                width: 1.0,
                color: colors::BORDER_SUBTLE,
            },
            ..Default::default()
        })
        .into()
}

/// Subject input with FONT_MEDIUM
fn subject_section(compose: &ComposeState) -> Element<'static, Message> {
    let label = text("Subject")
        .size(typography::SIZE_XS)
        .style(components::text_muted);

    let input = text_input("Subject", &compose.subject)
        .on_input(Message::ComposeSubjectChanged)
        .padding(spacing::SM)
        .size(typography::SIZE_SM)
        .font(typography::FONT_MEDIUM)
        .style(components::text_input_style);

    column![label, input].spacing(spacing::SPACE_1).into()
}

/// Body text editor
fn body_section(compose: &ComposeState) -> Element<'static, Message> {
    // Using text_input for now - a proper multiline editor would be better
    let body_input = text_input("Write your message...", &compose.body)
        .on_input(Message::ComposeBodyChanged)
        .padding(spacing::MD)
        .size(typography::SIZE_SM)
        .style(components::text_input_style);

    container(body_input)
        .width(Length::Fill)
        .height(Length::Fixed(200.0))
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

/// Attachments list
fn attachments_section(compose: &ComposeState) -> Element<'static, Message> {
    let title = row![
        text("Attachments")
            .size(typography::SIZE_SM)
            .font(typography::FONT_MEDIUM)
            .style(components::text_secondary),
        Space::with_width(spacing::SM),
        text(format!("({})", compose.attachments.len()))
            .size(typography::SIZE_XS)
            .style(components::text_muted),
    ];

    let attachment_rows: Vec<Element<'static, Message>> = compose
        .attachments
        .iter()
        .enumerate()
        .map(|(i, att)| {
            let file_icon = icons::file_icon(&att.filename);
            let filename = text(att.filename.clone())
                .size(typography::SIZE_SM)
                .style(components::text_primary);
            let size = text(format!("({})", format_bytes(att.size_bytes)))
                .size(typography::SIZE_XS)
                .style(components::text_muted);
            let remove_btn = button(
                text(icons::DELETE)
                    .size(typography::SIZE_SM)
                    .style(components::text_accent)
            )
            .padding([0, spacing::XS])
            .style(components::button_ghost)
            .on_press(Message::ComposeRemoveAttachment(i));

            row![
                container(
                    text(file_icon)
                        .size(typography::SIZE_2XS)
                        .font(typography::FONT_MONO)
                        .style(components::text_accent)
                )
                .padding([spacing::SPACE_1, spacing::XS])
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
                }),
                Space::with_width(spacing::SM),
                filename,
                Space::with_width(spacing::XS),
                size,
                Space::with_width(Length::Fill),
                remove_btn,
            ]
            .align_y(iced::Alignment::Center)
            .into()
        })
        .collect();

    column![title, Space::with_height(spacing::XS)]
        .push(column(attachment_rows).spacing(spacing::XS))
        .spacing(spacing::XS)
        .into()
}

/// Footer with action buttons -- Send as button_primary (copper)
fn compose_footer(compose: &ComposeState) -> Element<'static, Message> {
    // Left side: attach button
    let attach_btn = button(
        row![
            text(icons::ATTACH).size(typography::SIZE_SM),
            Space::with_width(spacing::XS),
            text("Attach").size(typography::SIZE_SM),
        ]
        .align_y(iced::Alignment::Center),
    )
    .padding([spacing::SM, spacing::MD])
    .style(components::button_secondary)
    .on_press(Message::ComposeAddAttachment);

    // Right side: discard, save draft, send
    let discard_btn = button(text("Discard").size(typography::SIZE_SM))
        .padding([spacing::SM, spacing::MD])
        .style(components::button_ghost)
        .on_press(Message::ComposeDiscard);

    let draft_btn = button(text("Save Draft").size(typography::SIZE_SM))
        .padding([spacing::SM, spacing::MD])
        .style(components::button_secondary)
        .on_press(Message::ComposeSaveDraft);

    let send_text = if compose.is_sending {
        "Sending..."
    } else {
        "Send"
    };

    // Send button uses button_primary (copper)
    let send_btn = if compose.can_send() {
        button(text(send_text).size(typography::SIZE_SM))
            .padding([spacing::SM, spacing::LG])
            .style(components::button_primary)
            .on_press(Message::ComposeSend)
    } else {
        button(text("Send").size(typography::SIZE_SM))
            .padding([spacing::SM, spacing::LG])
            .style(components::button_primary)
        // No on_press - disabled
    };

    row![
        attach_btn,
        Space::with_width(Length::Fill),
        discard_btn,
        Space::with_width(spacing::SM),
        draft_btn,
        Space::with_width(spacing::SM),
        send_btn,
    ]
    .align_y(iced::Alignment::Center)
    .into()
}
