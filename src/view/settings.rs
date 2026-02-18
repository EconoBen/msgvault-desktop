//! Settings view
//!
//! Configuration UI with tabs for server settings and display preferences.

use crate::message::Message;
use crate::model::SettingsTab;
use crate::theme::{colors, components, spacing, typography};
use iced::widget::{button, column, container, row, text, text_input, Space};
use iced::{Background, Border, Element, Length, Theme};

/// Render the settings view
pub fn settings_view<'a>(
    current_tab: SettingsTab,
    server_url: &'a str,
    api_key: &'a str,
    testing_connection: bool,
    connection_result: Option<&'a Result<(), String>>,
) -> Element<'a, Message> {
    // Header
    let title = text("Settings")
        .size(typography::SIZE_XL)
        .font(typography::FONT_MEDIUM)
        .style(components::text_primary);

    // Tab bar
    let tab_bar = tab_bar_widget(current_tab);

    // Tab content
    let content = match current_tab {
        SettingsTab::Server => server_tab(server_url, api_key, testing_connection, connection_result),
        SettingsTab::Display => display_tab(),
    };

    // Save button
    let save_button = button(text("Save Settings").size(typography::SIZE_SM))
        .padding([spacing::SM, spacing::XL])
        .style(components::button_primary)
        .on_press(Message::SaveSettings);

    // Keyboard hints in FONT_MONO
    let hints = text(",: settings | Esc: back (without saving)")
        .size(typography::SIZE_2XS)
        .font(typography::FONT_MONO)
        .style(components::text_muted);

    column![
        title,
        Space::with_height(spacing::XL),
        tab_bar,
        Space::with_height(spacing::XL),
        content,
        Space::with_height(Length::Fill),
        row![Space::with_width(Length::Fill), save_button],
        Space::with_height(spacing::SM),
        hints,
    ]
    .spacing(spacing::XS)
    .padding(spacing::XL)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

/// Tab bar for switching between settings sections
fn tab_bar_widget(current: SettingsTab) -> Element<'static, Message> {
    let server_tab = tab_button("Server", SettingsTab::Server, current == SettingsTab::Server);
    let display_tab = tab_button("Display", SettingsTab::Display, current == SettingsTab::Display);

    container(
        row![server_tab, Space::with_width(spacing::XS), display_tab],
    )
    .style(|_| container::Style {
        border: Border {
            width: 0.0,
            color: colors::BORDER_SUBTLE,
            radius: 0.0.into(),
        },
        ..Default::default()
    })
    .into()
}

/// Single tab button with copper left border when active
fn tab_button(label: &'static str, tab: SettingsTab, is_active: bool) -> Element<'static, Message> {
    let label_text = text(label)
        .size(typography::SIZE_SM)
        .font(typography::FONT_MEDIUM);

    let btn = button(label_text)
        .padding([spacing::SM, spacing::XL]);

    if is_active {
        // Wrap in container for copper left-border indicator
        let styled_btn = btn
            .style(|_theme: &Theme, status| {
                let bg = match status {
                    button::Status::Hovered => colors::BG_ELEVATED,
                    _ => colors::BG_SURFACE,
                };
                button::Style {
                    background: Some(Background::Color(bg)),
                    text_color: colors::ACCENT_PRIMARY,
                    border: Border {
                        radius: spacing::RADIUS_MD.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            })
            .on_press(Message::SwitchSettingsTab(tab));

        // Copper left border indicator
        container(styled_btn)
            .style(|_| container::Style {
                border: Border {
                    width: 2.0,
                    color: colors::ACCENT_PRIMARY,
                    radius: spacing::RADIUS_MD.into(),
                },
                ..Default::default()
            })
            .into()
    } else {
        btn.style(components::button_ghost)
           .on_press(Message::SwitchSettingsTab(tab))
           .into()
    }
}

/// Server settings tab content
fn server_tab<'a>(
    server_url: &'a str,
    api_key: &'a str,
    testing: bool,
    result: Option<&'a Result<(), String>>,
) -> Element<'a, Message> {
    // Section header
    let section_header = text("Connection")
        .size(typography::SIZE_LG)
        .font(typography::FONT_MEDIUM)
        .style(components::text_primary);

    let url_label = text("Server URL")
        .size(typography::SIZE_SM)
        .font(typography::FONT_MEDIUM)
        .style(components::text_secondary);

    let url_input = text_input("http://localhost:8080", server_url)
        .on_input(Message::SettingsServerUrlChanged)
        .padding(spacing::MD)
        .width(Length::Fill)
        .style(components::text_input_style);

    let api_key_label = text("API Key")
        .size(typography::SIZE_SM)
        .font(typography::FONT_MEDIUM)
        .style(components::text_secondary);

    let api_key_input = text_input("(optional)", api_key)
        .on_input(Message::SettingsApiKeyChanged)
        .padding(spacing::MD)
        .width(Length::Fill)
        .style(components::text_input_style)
        .secure(true);

    // Test connection button and result
    let test_button = if testing {
        button(text("Testing...").size(typography::SIZE_SM))
            .padding([spacing::SM, spacing::LG])
            .style(components::button_secondary)
    } else {
        button(text("Test Connection").size(typography::SIZE_SM))
            .padding([spacing::SM, spacing::LG])
            .style(components::button_secondary)
            .on_press(Message::TestConnection)
    };

    let test_result: Element<'a, Message> = match result {
        Some(Ok(())) => text("Connected successfully!")
            .size(typography::SIZE_SM)
            .style(components::text_success)
            .into(),
        Some(Err(e)) => text(format!("Failed: {}", truncate_error(e, 50)))
            .size(typography::SIZE_SM)
            .style(components::text_error)
            .into(),
        None => Space::new(0, 0).into(),
    };

    container(
        column![
            section_header,
            Space::with_height(spacing::LG),
            url_label,
            url_input,
            Space::with_height(spacing::LG),
            api_key_label,
            api_key_input,
            Space::with_height(spacing::XL),
            row![test_button, Space::with_width(spacing::LG), test_result]
                .align_y(iced::Alignment::Center),
        ]
        .spacing(spacing::XS),
    )
    .style(section_style)
    .padding(spacing::XL)
    .width(Length::Fill)
    .into()
}

/// Display settings tab content
fn display_tab<'a>() -> Element<'a, Message> {
    // Section header
    let section_header = text("Display Settings")
        .size(typography::SIZE_LG)
        .font(typography::FONT_MEDIUM)
        .style(components::text_primary);

    container(
        column![
            section_header,
            Space::with_height(spacing::LG),
            text("Theme: System Default")
                .size(typography::SIZE_SM)
                .style(components::text_secondary),
            Space::with_height(spacing::SM),
            text("Date Format: Auto")
                .size(typography::SIZE_SM)
                .style(components::text_secondary),
            Space::with_height(spacing::SM),
            text("(More display options coming soon)")
                .size(typography::SIZE_XS)
                .style(components::text_muted),
        ]
        .spacing(spacing::XS),
    )
    .style(section_style)
    .padding(spacing::XL)
    .width(Length::Fill)
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

/// Section container style with RADIUS_MD
fn section_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(colors::BG_SURFACE)),
        border: Border {
            radius: spacing::RADIUS_MD.into(),
            width: 1.0,
            color: colors::BORDER_SUBTLE,
        },
        ..Default::default()
    }
}
