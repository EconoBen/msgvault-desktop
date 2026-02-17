//! Sidebar navigation component
//!
//! Shows folders, labels, accounts, and quick navigation.

use crate::api::types::ViewType;
use crate::message::Message;
use crate::model::ViewLevel;
use crate::theme::{colors, components, spacing, typography};
use iced::widget::{button, column, row, scrollable, text, Space};
use iced::{Background, Border, Element, Length};

/// Render the sidebar
pub fn sidebar<'a>(
    current_view: &ViewLevel,
    accounts: &[String],
    labels: &[String],
) -> Element<'a, Message> {
    let header = sidebar_header();

    let nav_section = nav_section(current_view);

    let browse_section = browse_section(current_view);

    let labels_section = if !labels.is_empty() {
        labels_section_view(labels)
    } else {
        Space::with_height(0).into()
    };

    let accounts_section = if !accounts.is_empty() {
        accounts_section_view(accounts)
    } else {
        Space::with_height(0).into()
    };

    let bottom_nav = bottom_navigation();

    let content = column![
        header,
        Space::with_height(spacing::LG),
        nav_section,
        Space::with_height(spacing::XL),
        browse_section,
        Space::with_height(spacing::XL),
        labels_section,
        Space::with_height(spacing::XL),
        accounts_section,
        Space::with_height(Length::Fill),
        bottom_nav,
    ]
    .padding([spacing::LG, spacing::MD])
    .width(Length::Fill);

    scrollable(content)
        .height(Length::Fill)
        .into()
}

/// Sidebar header with app name
fn sidebar_header() -> Element<'static, Message> {
    text("msgvault")
        .size(typography::SIZE_LG)
        .style(components::text_primary)
        .into()
}

/// Main navigation section (Dashboard, Search, etc.)
fn nav_section(current_view: &ViewLevel) -> Element<'static, Message> {
    let is_dashboard = matches!(current_view, ViewLevel::Dashboard);
    let is_search = matches!(current_view, ViewLevel::Search);
    let is_sync = matches!(current_view, ViewLevel::Sync);

    column![
        section_label("Navigate"),
        nav_item("Dashboard", Message::NavigateTo(ViewLevel::Dashboard), is_dashboard),
        nav_item("Search", Message::OpenSearch, is_search),
        nav_item("Sync Status", Message::OpenSync, is_sync),
    ]
    .spacing(spacing::XS)
    .into()
}

/// Browse by section (Senders, Domains, Labels, Time)
fn browse_section(current_view: &ViewLevel) -> Element<'static, Message> {
    let active_view_type = match current_view {
        ViewLevel::Aggregates { view_type } => Some(*view_type),
        _ => None,
    };

    column![
        section_label("Browse"),
        browse_item("Senders", ViewType::Senders, active_view_type),
        browse_item("Domains", ViewType::Domains, active_view_type),
        browse_item("Labels", ViewType::Labels, active_view_type),
        browse_item("Time", ViewType::Time, active_view_type),
    ]
    .spacing(spacing::XS)
    .into()
}

/// Labels section
fn labels_section_view(labels: &[String]) -> Element<'static, Message> {
    let label_items: Vec<Element<'static, Message>> = labels
        .iter()
        .take(8) // Show max 8 labels in sidebar
        .map(|label| label_item(label))
        .collect();

    let mut content = column![section_label("Labels")].spacing(spacing::XS);

    for item in label_items {
        content = content.push(item);
    }

    content.into()
}

/// Accounts section
fn accounts_section_view(accounts: &[String]) -> Element<'static, Message> {
    let account_items: Vec<Element<'static, Message>> = accounts
        .iter()
        .map(|account| account_item(account))
        .collect();

    let mut content = column![section_label("Accounts")].spacing(spacing::XS);

    for item in account_items {
        content = content.push(item);
    }

    content.into()
}

/// Bottom navigation (Settings, Help, Accounts)
fn bottom_navigation() -> Element<'static, Message> {
    column![
        nav_item_icon("Settings", ",", Message::OpenSettings),
        nav_item_icon("Accounts", "a", Message::OpenAccounts),
        nav_item_icon("Help", "?", Message::ShowHelp),
    ]
    .spacing(spacing::XS)
    .into()
}

/// Section label
fn section_label(label: &'static str) -> Element<'static, Message> {
    text(label)
        .size(typography::SIZE_XS)
        .style(components::text_muted)
        .into()
}

/// Navigation item button
fn nav_item(label: &'static str, message: Message, is_active: bool) -> Element<'static, Message> {
    let style = if is_active {
        |_theme: &iced::Theme, _status| button::Style {
            background: Some(Background::Color(colors::SELECTION_BG)),
            text_color: colors::TEXT_PRIMARY,
            border: Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    } else {
        |_theme: &iced::Theme, _status| button::Style {
            background: None,
            text_color: colors::TEXT_SECONDARY,
            border: Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    };

    button(
        text(label)
            .size(typography::SIZE_SM)
    )
    .width(Length::Fill)
    .padding([spacing::XS, spacing::SM])
    .style(style)
    .on_press(message)
    .into()
}

/// Browse item button
fn browse_item(label: &'static str, view_type: ViewType, active: Option<ViewType>) -> Element<'static, Message> {
    let is_active = active == Some(view_type);
    nav_item(label, Message::NavigateTo(ViewLevel::Aggregates { view_type }), is_active)
}

/// Navigation item with keyboard shortcut hint
fn nav_item_icon(label: &'static str, shortcut: &'static str, message: Message) -> Element<'static, Message> {
    button(
        row![
            text(label).size(typography::SIZE_SM),
            Space::with_width(Length::Fill),
            text(shortcut)
                .size(typography::SIZE_XS)
                .style(components::text_muted),
        ]
        .align_y(iced::Alignment::Center)
    )
    .width(Length::Fill)
    .padding([spacing::XS, spacing::SM])
    .style(|_theme: &iced::Theme, _status| button::Style {
        background: None,
        text_color: colors::TEXT_SECONDARY,
        border: Border {
            radius: 4.0.into(),
            ..Default::default()
        },
        ..Default::default()
    })
    .on_press(message)
    .into()
}

/// Label item
fn label_item(label: &str) -> Element<'static, Message> {
    let label_owned = label.to_string();

    button(
        row![
            text("●").size(typography::SIZE_XS).style(components::text_accent),
            Space::with_width(spacing::XS),
            text(label_owned.clone()).size(typography::SIZE_SM),
        ]
        .align_y(iced::Alignment::Center)
    )
    .width(Length::Fill)
    .padding([spacing::XS, spacing::SM])
    .style(|_theme: &iced::Theme, _status| button::Style {
        background: None,
        text_color: colors::TEXT_SECONDARY,
        border: Border {
            radius: 4.0.into(),
            ..Default::default()
        },
        ..Default::default()
    })
    .on_press(Message::NavigateTo(ViewLevel::Messages {
        filter_description: format!("Label: {}", label_owned),
    }))
    .into()
}

/// Account item
fn account_item(account: &str) -> Element<'static, Message> {
    let account_owned = account.to_string();

    button(
        text(truncate_email(&account_owned))
            .size(typography::SIZE_XS)
    )
    .width(Length::Fill)
    .padding([spacing::XS, spacing::SM])
    .style(|_theme: &iced::Theme, _status| button::Style {
        background: None,
        text_color: colors::TEXT_MUTED,
        border: Border {
            radius: 4.0.into(),
            ..Default::default()
        },
        ..Default::default()
    })
    .on_press(Message::NavigateTo(ViewLevel::Messages {
        filter_description: format!("Account: {}", account_owned),
    }))
    .into()
}

/// Truncate email for display
fn truncate_email(email: &str) -> String {
    if email.len() > 25 {
        format!("{}...", &email[..22])
    } else {
        email.to_string()
    }
}
