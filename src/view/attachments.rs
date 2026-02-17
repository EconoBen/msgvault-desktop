//! Attachment list view component
//!
//! Displays attachments with download buttons and progress indicators.

use crate::api::types::Attachment;
use crate::message::Message;
use crate::model::downloads::{DownloadState, DownloadTracker};
use crate::theme::{colors, components, spacing, typography};
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
            radius: 6.0.into(),
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
    let icon = get_file_icon(&attachment.filename);
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
                text("Download")
                    .size(typography::SIZE_XS)
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
            // Show "Open" button
            let open_btn = button(
                text("Open")
                    .size(typography::SIZE_XS)
            )
            .padding([spacing::XS, spacing::SM])
            .style(components::button_primary)
            .on_press(Message::OpenFile(path.clone()));

            let status = text("Downloaded")
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

            let retry_btn = button(
                text("Retry")
                    .size(typography::SIZE_XS)
            )
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
            text(icon).size(typography::SIZE_MD),
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
            radius: 4.0.into(),
            ..Default::default()
        },
        ..Default::default()
    })
    .into()
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
        "txt" => "TXT",
        "csv" => "CSV",
        "json" => "JSON",
        "xml" => "XML",
        "html" | "htm" => "HTML",
        _ => "FILE",
    }
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
            radius: 3.0.into(),
            ..Default::default()
        },
    }
}
