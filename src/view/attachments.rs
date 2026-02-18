//! Attachment list view component
//!
//! Displays attachments with download buttons and progress indicators.

use crate::api::types::Attachment;
use crate::message::Message;
use crate::model::downloads::{DownloadState, DownloadTracker};
use crate::theme::{colors, components, icons, spacing, typography};
use crate::view::widgets::format_bytes;
use iced::widget::{button, column, container, progress_bar, row, text, Space};
use iced::{Background, Border, Element, Length};

/// Render the attachments section for a message
pub fn attachments_section<'a>(
    message_id: i64,
    attachments: &'a [Attachment],
    downloads: &'a DownloadTracker,
) -> Element<'a, Message> {
    if attachments.is_empty() {
        return column![].into();
    }

    let title = text("Attachments")
        .size(typography::SIZE_SM)
        .font(typography::FONT_MEDIUM)
        .style(components::text_primary);

    let attachment_rows: Vec<Element<'a, Message>> = attachments
        .iter()
        .enumerate()
        .map(|(idx, att)| attachment_row(message_id, idx, att, downloads.get(message_id, idx)))
        .collect();

    container(
        column![title, Space::with_height(spacing::SM),]
            .push(column(attachment_rows).spacing(spacing::XS)),
    )
    .width(Length::Fill)
    .padding(spacing::MD)
    .style(|_| container::Style {
        background: Some(Background::Color(colors::BG_ELEVATED)),
        border: Border {
            radius: spacing::RADIUS_MD.into(),
            width: 1.0,
            color: colors::BORDER_SUBTLE,
        },
        ..Default::default()
    })
    .into()
}

/// Render a single attachment row with download functionality
fn attachment_row<'a>(
    message_id: i64,
    idx: usize,
    attachment: &'a Attachment,
    download_state: &'a DownloadState,
) -> Element<'a, Message> {
    // File type icon from theme icons module
    let icon_label = icons::file_icon(&attachment.filename);

    // File type label in a small colored container with copper accent
    let icon_badge = container(
        text(icon_label)
            .size(typography::SIZE_2XS)
            .font(typography::FONT_MONO)
            .style(components::text_accent),
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
    });

    let filename = text(&attachment.filename)
        .size(typography::SIZE_SM)
        .style(components::text_secondary);
    let size = text(format!("({})", format_bytes(attachment.size_bytes)))
        .size(typography::SIZE_XS)
        .style(components::text_muted);

    // Build the action element based on download state
    let action_element: Element<'a, Message> = match download_state {
        DownloadState::NotStarted => {
            let download_btn = button(
                row![
                    text(icons::DOWNLOAD).size(typography::SIZE_XS),
                    Space::with_width(spacing::XS),
                    text("Download").size(typography::SIZE_XS),
                ]
                .align_y(iced::Alignment::Center),
            )
            .padding([spacing::XS, spacing::SM])
            .style(components::button_secondary)
            .on_press(Message::DownloadAttachment {
                message_id,
                attachment_idx: idx,
                filename: attachment.filename.clone(),
            });
            download_btn.into()
        }
        DownloadState::Downloading { progress } => {
            // Show progress bar
            let progress_text = text(format!("{}%", (*progress * 100.0) as i32))
                .size(typography::SIZE_XS)
                .font(typography::FONT_MONO)
                .style(components::text_muted);

            let bar = progress_bar(0.0..=1.0, *progress)
                .height(Length::Fixed(6.0))
                .width(Length::Fixed(80.0))
                .style(|_| progress_bar_style());

            row![bar, Space::with_width(spacing::XS), progress_text]
                .align_y(iced::Alignment::Center)
                .into()
        }
        DownloadState::Complete { path } => {
            // Show "Open" button with icon
            let open_btn = button(
                row![
                    text(icons::OPEN).size(typography::SIZE_XS),
                    Space::with_width(spacing::XS),
                    text("Open").size(typography::SIZE_XS),
                ]
                .align_y(iced::Alignment::Center),
            )
            .padding([spacing::XS, spacing::SM])
            .style(components::button_primary)
            .on_press(Message::OpenFile(path.clone()));

            let status = text(icons::CHECK)
                .size(typography::SIZE_XS)
                .style(components::text_success);

            row![status, Space::with_width(spacing::SM), open_btn]
                .align_y(iced::Alignment::Center)
                .into()
        }
        DownloadState::Failed { error } => {
            // Show error with retry button
            let error_text = text(truncate_error(error, 20))
                .size(typography::SIZE_XS)
                .style(components::text_error);

            let retry_btn = button(text("Retry").size(typography::SIZE_XS))
                .padding([spacing::XS, spacing::SM])
                .style(components::button_secondary)
                .on_press(Message::DownloadAttachment {
                    message_id,
                    attachment_idx: idx,
                    filename: attachment.filename.clone(),
                });

            row![error_text, Space::with_width(spacing::SM), retry_btn]
                .align_y(iced::Alignment::Center)
                .into()
        }
    };

    container(
        row![
            icon_badge,
            Space::with_width(spacing::SM),
            filename,
            Space::with_width(spacing::SM),
            size,
            Space::with_width(Length::Fill),
            action_element,
        ]
        .align_y(iced::Alignment::Center),
    )
    .padding([spacing::XS, spacing::SM])
    .style(|_| container::Style {
        background: Some(Background::Color(colors::BG_SURFACE)),
        border: Border {
            radius: spacing::RADIUS_SM.into(),
            ..Default::default()
        },
        ..Default::default()
    })
    .into()
}

/// Truncate error message for display
fn truncate_error(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len])
    }
}

/// Progress bar style
fn progress_bar_style() -> progress_bar::Style {
    progress_bar::Style {
        background: Background::Color(colors::BG_SURFACE),
        bar: Background::Color(colors::ACCENT_PRIMARY),
        border: Border {
            radius: spacing::RADIUS_SM.into(),
            ..Default::default()
        },
    }
}
