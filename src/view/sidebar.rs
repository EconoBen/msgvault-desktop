//! Sidebar navigation component
//!
//! Foundry Dark design: warm browns, copper accent, icon-driven nav.
//! Shows logo mark, navigation, browse sections, labels, accounts,
//! and a bottom status bar with connection indicator.

use crate::api::types::ViewType;
use crate::message::Message;
use crate::model::ViewLevel;
use crate::theme::{colors, components, icons, spacing, typography};
use iced::widget::{button, column, container, horizontal_rule, row, scrollable, text, Space};
use iced::{Background, Border, Color, Element, Length};

// === Avatar palette (deterministic dot color per account) ===

const AVATAR_PALETTE: [Color; 8] = [
    Color {
        r: 0.424,
        g: 0.549,
        b: 0.824,
        a: 1.0,
    }, // Blue
    Color {
        r: 0.482,
        g: 0.706,
        b: 0.482,
        a: 1.0,
    }, // Green
    Color {
        r: 0.706,
        g: 0.482,
        b: 0.706,
        a: 1.0,
    }, // Purple
    Color {
        r: 0.824,
        g: 0.549,
        b: 0.424,
        a: 1.0,
    }, // Orange
    Color {
        r: 0.549,
        g: 0.706,
        b: 0.706,
        a: 1.0,
    }, // Teal
    Color {
        r: 0.706,
        g: 0.549,
        b: 0.482,
        a: 1.0,
    }, // Brown
    Color {
        r: 0.549,
        g: 0.482,
        b: 0.706,
        a: 1.0,
    }, // Indigo
    Color {
        r: 0.706,
        g: 0.482,
        b: 0.549,
        a: 1.0,
    }, // Pink
];

/// Pick a deterministic color from the avatar palette for a string.
fn dot_color_for(name: &str) -> Color {
    let hash: usize = name
        .bytes()
        .fold(0usize, |acc, b| acc.wrapping_add(b as usize));
    AVATAR_PALETTE[hash % AVATAR_PALETTE.len()]
}

// ───────────────────────────────────────────────────────────────
// Public entry point
// ───────────────────────────────────────────────────────────────

/// Render the full sidebar.
pub fn sidebar<'a>(
    current_view: &ViewLevel,
    accounts: &[String],
    labels: &[String],
) -> Element<'a, Message> {
    let header = sidebar_header();
    let nav = nav_section(current_view);
    let browse = browse_section(current_view);

    let labels_el: Element<'a, Message> = if !labels.is_empty() {
        labels_section_view(labels)
    } else {
        Space::with_height(0).into()
    };

    let accounts_el: Element<'a, Message> = if !accounts.is_empty() {
        accounts_section_view(accounts)
    } else {
        Space::with_height(0).into()
    };

    let divider = divider_line();
    let bottom = bottom_navigation();
    let status = connection_status();

    let content = column![
        header,
        Space::with_height(spacing::LG),
        nav,
        Space::with_height(spacing::XL),
        browse,
        Space::with_height(spacing::XL),
        labels_el,
        Space::with_height(spacing::XL),
        accounts_el,
        Space::with_height(Length::Fill),
        divider,
        Space::with_height(spacing::SM),
        bottom,
        Space::with_height(spacing::SM),
        status,
    ]
    .padding([spacing::LG, spacing::MD])
    .width(Length::Fill);

    scrollable(content).height(Length::Fill).into()
}

// ───────────────────────────────────────────────────────────────
// Header — logo mark
// ───────────────────────────────────────────────────────────────

/// "◆ msgvault" logo mark. Diamond in copper, text in primary.
fn sidebar_header() -> Element<'static, Message> {
    row![
        text(icons::DIAMOND)
            .size(typography::SIZE_LG)
            .font(typography::FONT_PRIMARY)
            .style(components::text_accent),
        Space::with_width(spacing::SM),
        text("msgvault")
            .size(typography::SIZE_LG)
            .font(typography::FONT_SEMIBOLD)
            .style(components::text_primary),
    ]
    .align_y(iced::Alignment::Center)
    .into()
}

// ───────────────────────────────────────────────────────────────
// Section label  (uppercase, letter-spaced)
// ───────────────────────────────────────────────────────────────

/// Uppercase, letter-spaced section label (e.g. "N A V I G A T E").
fn section_label(label: &'static str) -> Element<'static, Message> {
    let upper = label.to_uppercase();
    let spaced: String = upper
        .chars()
        .enumerate()
        .fold(String::new(), |mut acc, (i, c)| {
            if i > 0 {
                acc.push(' ');
            }
            acc.push(c);
            acc
        });

    container(
        text(spaced)
            .size(typography::SIZE_2XS)
            .font(typography::FONT_MEDIUM)
            .style(components::text_muted),
    )
    .padding([0, spacing::SM])
    .into()
}

// ───────────────────────────────────────────────────────────────
// Navigation section
// ───────────────────────────────────────────────────────────────

fn nav_section(current_view: &ViewLevel) -> Element<'static, Message> {
    let is_dashboard = matches!(current_view, ViewLevel::Dashboard);
    let is_search = matches!(current_view, ViewLevel::Search);
    let is_sync = matches!(current_view, ViewLevel::Sync);

    column![
        section_label("Navigate"),
        Space::with_height(spacing::XS),
        nav_item(icons::DASHBOARD, "Dashboard", Message::NavigateTo(ViewLevel::Dashboard), is_dashboard, None),
        nav_item(icons::SEARCH, "Search", Message::OpenSearch, is_search, Some("/")),
        nav_item(icons::SYNC, "Sync Status", Message::OpenSync, is_sync, None),
    ]
    .spacing(spacing::SPACE_1)
    .into()
}

// ───────────────────────────────────────────────────────────────
// Browse section
// ───────────────────────────────────────────────────────────────

fn browse_section(current_view: &ViewLevel) -> Element<'static, Message> {
    let active_view_type = match current_view {
        ViewLevel::Aggregates { view_type } => Some(*view_type),
        _ => None,
    };

    column![
        section_label("Browse"),
        Space::with_height(spacing::XS),
        browse_item(icons::ACCOUNTS, "Senders", ViewType::Senders, active_view_type),
        browse_item(icons::DOT_FILLED, "Domains", ViewType::Domains, active_view_type),
        browse_item(icons::DIAMOND_SM, "Labels", ViewType::Labels, active_view_type),
        browse_item(icons::DOTS, "Time", ViewType::Time, active_view_type),
    ]
    .spacing(spacing::SPACE_1)
    .into()
}

fn browse_item(
    icon: &'static str,
    label: &'static str,
    view_type: ViewType,
    active: Option<ViewType>,
) -> Element<'static, Message> {
    let is_active = active == Some(view_type);
    nav_item(
        icon,
        label,
        Message::NavigateTo(ViewLevel::Aggregates { view_type }),
        is_active,
        None,
    )
}

// ───────────────────────────────────────────────────────────────
// Labels section
// ───────────────────────────────────────────────────────────────

fn labels_section_view(labels: &[String]) -> Element<'static, Message> {
    let mut content = column![
        section_label("Labels"),
        Space::with_height(spacing::XS),
    ]
    .spacing(spacing::SPACE_1);

    for label in labels.iter().take(8) {
        content = content.push(label_item(label));
    }

    content.into()
}

fn label_item(label: &str) -> Element<'static, Message> {
    let label_owned = label.to_string();

    button(
        row![
            text(icons::DOT_FILLED)
                .size(typography::SIZE_2XS)
                .style(components::text_accent),
            Space::with_width(spacing::SM),
            text(label_owned.clone())
                .size(typography::SIZE_SM)
                .font(typography::FONT_PRIMARY),
        ]
        .align_y(iced::Alignment::Center),
    )
    .width(Length::Fill)
    .padding([spacing::XS, spacing::SM])
    .style(|_theme: &iced::Theme, _status| button::Style {
        background: None,
        text_color: colors::TEXT_SECONDARY,
        border: Border {
            radius: spacing::RADIUS_SM.into(),
            ..Default::default()
        },
        ..Default::default()
    })
    .on_press(Message::NavigateTo(ViewLevel::Messages {
        filter_description: format!("Label: {}", label_owned),
    }))
    .into()
}

// ───────────────────────────────────────────────────────────────
// Accounts section (colored dots from avatar palette)
// ───────────────────────────────────────────────────────────────

fn accounts_section_view(accounts: &[String]) -> Element<'static, Message> {
    let mut content = column![
        section_label("Accounts"),
        Space::with_height(spacing::XS),
    ]
    .spacing(spacing::SPACE_1);

    for account in accounts.iter() {
        content = content.push(account_item(account));
    }

    content.into()
}

fn account_item(account: &str) -> Element<'static, Message> {
    let account_owned = account.to_string();
    let dot_col = dot_color_for(account);

    button(
        row![
            text(icons::DOT_FILLED)
                .size(typography::SIZE_2XS)
                .style(move |_| iced::widget::text::Style {
                    color: Some(dot_col),
                }),
            Space::with_width(spacing::SM),
            text(truncate_email(&account_owned))
                .size(typography::SIZE_XS)
                .font(typography::FONT_PRIMARY),
        ]
        .align_y(iced::Alignment::Center),
    )
    .width(Length::Fill)
    .padding([spacing::XS, spacing::SM])
    .style(|_theme: &iced::Theme, _status| button::Style {
        background: None,
        text_color: colors::TEXT_MUTED,
        border: Border {
            radius: spacing::RADIUS_SM.into(),
            ..Default::default()
        },
        ..Default::default()
    })
    .on_press(Message::NavigateTo(ViewLevel::Messages {
        filter_description: format!("Account: {}", account_owned),
    }))
    .into()
}

// ───────────────────────────────────────────────────────────────
// Divider
// ───────────────────────────────────────────────────────────────

/// Subtle horizontal divider before the bottom nav.
fn divider_line() -> Element<'static, Message> {
    container(horizontal_rule(1))
        .style(|_| container::Style {
            text_color: Some(colors::BORDER_SUBTLE),
            ..Default::default()
        })
        .width(Length::Fill)
        .into()
}

// ───────────────────────────────────────────────────────────────
// Bottom navigation
// ───────────────────────────────────────────────────────────────

fn bottom_navigation() -> Element<'static, Message> {
    column![
        nav_item(icons::SETTINGS, "Settings", Message::OpenSettings, false, Some(",")),
        nav_item(icons::ACCOUNTS, "Accounts", Message::OpenAccounts, false, Some("a")),
        nav_item(icons::HELP, "Help", Message::ShowHelp, false, Some("?")),
    ]
    .spacing(spacing::SPACE_1)
    .into()
}

// ───────────────────────────────────────────────────────────────
// Connection status
// ───────────────────────────────────────────────────────────────

/// "Connected ●" status line at the very bottom.
fn connection_status() -> Element<'static, Message> {
    container(
        row![
            text("Connected")
                .size(typography::SIZE_2XS)
                .font(typography::FONT_PRIMARY)
                .style(components::text_muted),
            Space::with_width(spacing::XS),
            text(icons::DOT_FILLED)
                .size(typography::SIZE_2XS)
                .style(components::text_success),
        ]
        .align_y(iced::Alignment::Center),
    )
    .padding([spacing::XS, spacing::SM])
    .width(Length::Fill)
    .into()
}

// ───────────────────────────────────────────────────────────────
// Nav item (icon + label + optional shortcut, active state)
// ───────────────────────────────────────────────────────────────

/// A single navigation row.
///
/// Active items get:
///   - SELECTION_BG background
///   - 2 px copper left border
///   - TEXT_PRIMARY text color
///
/// Inactive items get TEXT_SECONDARY text, transparent background.
/// An optional `shortcut` is rendered right-aligned in FONT_MONO.
fn nav_item(
    icon: &'static str,
    label: &'static str,
    message: Message,
    is_active: bool,
    shortcut: Option<&'static str>,
) -> Element<'static, Message> {
    // Build the inner row: icon + label + (optional shortcut)
    let mut content = row![
        text(icon)
            .size(typography::SIZE_SM)
            .style(if is_active {
                components::text_accent as fn(&iced::Theme) -> iced::widget::text::Style
            } else {
                components::text_muted as fn(&iced::Theme) -> iced::widget::text::Style
            }),
        Space::with_width(spacing::SM),
        text(label)
            .size(typography::SIZE_SM)
            .font(typography::FONT_PRIMARY),
    ]
    .align_y(iced::Alignment::Center);

    if let Some(key) = shortcut {
        content = content.push(Space::with_width(Length::Fill));
        content = content.push(
            text(key)
                .size(typography::SIZE_2XS)
                .font(typography::FONT_MONO)
                .style(components::text_muted),
        );
    }

    let style = if is_active {
        move |_theme: &iced::Theme, _status: button::Status| button::Style {
            background: Some(Background::Color(colors::SELECTION_BG)),
            text_color: colors::TEXT_PRIMARY,
            border: Border {
                radius: spacing::RADIUS_SM.into(),
                width: 0.0,
                ..Default::default()
            },
            ..Default::default()
        }
    } else {
        move |_theme: &iced::Theme, _status: button::Status| button::Style {
            background: None,
            text_color: colors::TEXT_SECONDARY,
            border: Border {
                radius: spacing::RADIUS_SM.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    };

    // Wrap in a container that provides the left accent border when active.
    let btn: Element<'static, Message> = button(content)
        .width(Length::Fill)
        .padding([spacing::XS, spacing::SM])
        .style(style)
        .on_press(message)
        .into();

    if is_active {
        // Wrap the button in a container that draws a 2px copper left border.
        container(btn)
            .width(Length::Fill)
            .style(move |_| container::Style {
                border: Border {
                    color: colors::ACCENT_PRIMARY,
                    width: 2.0,
                    radius: spacing::RADIUS_SM.into(),
                },
                ..Default::default()
            })
            .into()
    } else {
        btn
    }
}

// ───────────────────────────────────────────────────────────────
// Helpers
// ───────────────────────────────────────────────────────────────

/// Truncate email for display (max 25 chars).
fn truncate_email(email: &str) -> String {
    if email.len() > 25 {
        format!("{}...", &email[..22])
    } else {
        email.to_string()
    }
}
