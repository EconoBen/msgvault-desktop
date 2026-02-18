//! Toolbar component
//!
//! Thin action bar at the top of the content area.
//! Shows contextual actions based on current view.

use crate::message::Message;
use crate::model::ViewLevel;
use crate::theme::{colors, components, icons, spacing, typography};
use iced::widget::{button, container, horizontal_rule, row, text, Space};
use iced::{Background, Border, Element, Length};

/// Height of the toolbar
const TOOLBAR_HEIGHT: f32 = 36.0;

/// Render the toolbar based on current view context
pub fn toolbar<'a>(
    current_view: &ViewLevel,
    has_selection: bool,
    selection_count: usize,
) -> Element<'a, Message> {
    let left_actions = left_actions(current_view);
    let right_actions = right_actions(current_view, has_selection, selection_count);

    let bar = row![
        left_actions,
        Space::with_width(Length::Fill),
        right_actions,
    ]
    .align_y(iced::Alignment::Center)
    .padding([0, spacing::LG])
    .height(Length::Fixed(TOOLBAR_HEIGHT));

    container(bar)
        .width(Length::Fill)
        .style(|_| container::Style {
            background: Some(Background::Color(colors::BG_BASE)),
            border: Border {
                width: 0.0,
                color: colors::BORDER_SUBTLE,
                radius: 0.0.into(),
            },
            ..Default::default()
        })
        .into()
}

/// Left-side actions (view-specific)
fn left_actions<'a>(current_view: &ViewLevel) -> Element<'a, Message> {
    match current_view {
        ViewLevel::Dashboard => {
            row![
                view_label("Dashboard"),
            ]
            .spacing(spacing::SM)
            .align_y(iced::Alignment::Center)
            .into()
        }
        ViewLevel::Aggregates { view_type } => {
            row![
                view_label(&view_type.display_name()),
                toolbar_separator(),
                toolbar_button(icons::SYNC, "Refresh", Message::FetchAggregates(*view_type)),
            ]
            .spacing(spacing::SM)
            .align_y(iced::Alignment::Center)
            .into()
        }
        ViewLevel::Messages { filter_description } => {
            row![
                view_label(filter_description),
            ]
            .spacing(spacing::SM)
            .align_y(iced::Alignment::Center)
            .into()
        }
        ViewLevel::MessageDetail { .. } => {
            row![
                toolbar_button(icons::ARROW_LEFT, "Back", Message::GoBack),
                toolbar_separator(),
                toolbar_button(icons::REPLY, "Reply", Message::GoBack), // placeholder
                toolbar_button(icons::FORWARD, "Forward", Message::GoBack), // placeholder
            ]
            .spacing(spacing::SM)
            .align_y(iced::Alignment::Center)
            .into()
        }
        ViewLevel::Search => {
            row![
                view_label("Search"),
            ]
            .spacing(spacing::SM)
            .align_y(iced::Alignment::Center)
            .into()
        }
        ViewLevel::Sync => {
            row![
                view_label("Sync Status"),
                toolbar_separator(),
                toolbar_button(icons::SYNC, "Refresh", Message::FetchSyncStatus),
            ]
            .spacing(spacing::SM)
            .align_y(iced::Alignment::Center)
            .into()
        }
        ViewLevel::Accounts => {
            row![
                view_label("Accounts"),
            ]
            .spacing(spacing::SM)
            .align_y(iced::Alignment::Center)
            .into()
        }
        ViewLevel::Settings => {
            row![
                view_label("Settings"),
            ]
            .spacing(spacing::SM)
            .align_y(iced::Alignment::Center)
            .into()
        }
        ViewLevel::Thread { .. } => {
            row![
                toolbar_button(icons::ARROW_LEFT, "Back", Message::GoBack),
                toolbar_separator(),
                view_label("Thread"),
            ]
            .spacing(spacing::SM)
            .align_y(iced::Alignment::Center)
            .into()
        }
        _ => Space::with_width(0).into(),
    }
}

/// Right-side actions (selection-aware + global)
fn right_actions<'a>(
    current_view: &ViewLevel,
    has_selection: bool,
    selection_count: usize,
) -> Element<'a, Message> {
    let mut items = row![].spacing(spacing::SM).align_y(iced::Alignment::Center);

    // Show selection actions when messages are selected
    if has_selection {
        items = items.push(
            text(format!("{} selected", selection_count))
                .size(typography::SIZE_2XS)
                .font(typography::FONT_MONO)
                .style(components::text_accent),
        );
        items = items.push(toolbar_button(icons::CROSS, "Clear", Message::ClearSelection));
        items = items.push(toolbar_button(icons::DELETE, "Delete", Message::ShowDeleteModal));
        items = items.push(toolbar_separator());
    }

    // Compose button (always available in list/detail views)
    match current_view {
        ViewLevel::Messages { .. }
        | ViewLevel::MessageDetail { .. }
        | ViewLevel::Dashboard
        | ViewLevel::Aggregates { .. } => {
            items = items.push(toolbar_button(icons::COMPOSE, "Compose", Message::OpenCompose));
        }
        _ => {}
    }

    items.into()
}

/// A small ghost button for toolbar actions
fn toolbar_button(icon: &str, label: &str, message: Message) -> Element<'static, Message> {
    let icon_owned = icon.to_string();
    let label_owned = label.to_string();

    button(
        row![
            text(icon_owned)
                .size(typography::SIZE_XS)
                .font(typography::FONT_PRIMARY),
            Space::with_width(spacing::SPACE_1),
            text(label_owned)
                .size(typography::SIZE_2XS)
                .font(typography::FONT_MEDIUM),
        ]
        .align_y(iced::Alignment::Center),
    )
    .padding([spacing::SPACE_1, spacing::SM])
    .style(|_theme: &iced::Theme, status| {
        let bg = match status {
            button::Status::Hovered => colors::BG_ELEVATED,
            button::Status::Pressed => colors::BG_OVERLAY,
            _ => colors::TRANSPARENT,
        };
        button::Style {
            background: Some(Background::Color(bg)),
            text_color: colors::TEXT_SECONDARY,
            border: Border {
                radius: spacing::RADIUS_SM.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    })
    .on_press(message)
    .into()
}

/// View label (non-interactive title in the toolbar)
fn view_label(label: &str) -> Element<'static, Message> {
    text(label.to_string())
        .size(typography::SIZE_SM)
        .font(typography::FONT_SEMIBOLD)
        .style(components::text_primary)
        .into()
}

/// Thin vertical separator
fn toolbar_separator() -> Element<'static, Message> {
    container(Space::new(1, 16))
        .style(|_| container::Style {
            background: Some(Background::Color(colors::BORDER_SUBTLE)),
            ..Default::default()
        })
        .into()
}
