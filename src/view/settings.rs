//! Settings view
//!
//! Configuration UI with tabs for server settings and display preferences.

use crate::message::Message;
use crate::model::SettingsTab;
use iced::widget::{button, column, container, row, text, text_input, Space};
use iced::{Element, Length, Theme};

/// Render the settings view
pub fn settings_view<'a>(
    current_tab: SettingsTab,
    server_url: &'a str,
    api_key: &'a str,
    testing_connection: bool,
    connection_result: Option<&'a Result<(), String>>,
) -> Element<'a, Message> {
    // Header
    let title = text("Settings").size(24);

    // Tab bar
    let tab_bar = tab_bar_widget(current_tab);

    // Tab content
    let content = match current_tab {
        SettingsTab::Server => server_tab(server_url, api_key, testing_connection, connection_result),
        SettingsTab::Display => display_tab(),
    };

    // Save button
    let save_button = button(text("Save Settings").size(14))
        .padding([10, 20])
        .on_press(Message::SaveSettings);

    // Keyboard hints
    let hints = text(",: settings | Esc: back (without saving)").size(12);

    column![
        title,
        Space::with_height(20),
        tab_bar,
        Space::with_height(20),
        content,
        Space::with_height(Length::Fill),
        row![Space::with_width(Length::Fill), save_button],
        Space::with_height(10),
        hints,
    ]
    .spacing(5)
    .padding(20)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

/// Tab bar for switching between settings sections
fn tab_bar_widget(current: SettingsTab) -> Element<'static, Message> {
    let server_tab = tab_button("Server", SettingsTab::Server, current == SettingsTab::Server);
    let display_tab = tab_button("Display", SettingsTab::Display, current == SettingsTab::Display);

    row![server_tab, Space::with_width(5), display_tab]
        .into()
}

/// Single tab button
fn tab_button(label: &'static str, tab: SettingsTab, is_active: bool) -> Element<'static, Message> {
    let btn = button(text(label).size(14))
        .padding([8, 20]);

    if is_active {
        btn.style(|theme: &Theme, _status| {
            let palette = theme.palette();
            iced::widget::button::Style {
                background: Some(iced::Background::Color(palette.primary)),
                text_color: iced::Color::WHITE,
                border: iced::Border {
                    radius: 4.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }
        })
        .on_press(Message::SwitchSettingsTab(tab))
        .into()
    } else {
        btn.on_press(Message::SwitchSettingsTab(tab)).into()
    }
}

/// Server settings tab content
fn server_tab<'a>(
    server_url: &'a str,
    api_key: &'a str,
    testing: bool,
    result: Option<&'a Result<(), String>>,
) -> Element<'a, Message> {
    let url_label = text("Server URL").size(14);
    let url_input = text_input("http://localhost:8080", server_url)
        .on_input(Message::SettingsServerUrlChanged)
        .padding(10)
        .width(Length::Fill);

    let api_key_label = text("API Key").size(14);
    let api_key_input = text_input("(optional)", api_key)
        .on_input(Message::SettingsApiKeyChanged)
        .padding(10)
        .width(Length::Fill)
        .secure(true);

    // Test connection button and result
    let test_button = if testing {
        button(text("Testing...").size(14)).padding([8, 16])
    } else {
        button(text("Test Connection").size(14))
            .padding([8, 16])
            .on_press(Message::TestConnection)
    };

    let test_result: Element<'a, Message> = match result {
        Some(Ok(())) => text("Connected successfully!")
            .size(14)
            .style(|_theme: &Theme| iced::widget::text::Style {
                color: Some(iced::Color::from_rgb(0.0, 0.6, 0.0)),
            })
            .into(),
        Some(Err(e)) => text(format!("Failed: {}", truncate_error(e, 50)))
            .size(14)
            .style(|_theme: &Theme| iced::widget::text::Style {
                color: Some(iced::Color::from_rgb(0.8, 0.0, 0.0)),
            })
            .into(),
        None => Space::new(0, 0).into(),
    };

    container(
        column![
            url_label,
            url_input,
            Space::with_height(15),
            api_key_label,
            api_key_input,
            Space::with_height(20),
            row![test_button, Space::with_width(15), test_result]
                .align_y(iced::Alignment::Center),
        ]
        .spacing(5),
    )
    .style(section_style)
    .padding(20)
    .width(Length::Fill)
    .into()
}

/// Display settings tab content
fn display_tab<'a>() -> Element<'a, Message> {
    // Placeholder for display settings
    // Could include: theme selection, date format, font size, etc.

    container(
        column![
            text("Display Settings").size(18),
            Space::with_height(15),
            text("Theme: System Default").size(14),
            Space::with_height(10),
            text("Date Format: Auto").size(14),
            Space::with_height(10),
            text("(More display options coming soon)").size(12).style(|theme: &Theme| {
                let palette = theme.palette();
                iced::widget::text::Style {
                    color: Some(iced::Color {
                        a: 0.6,
                        ..palette.text
                    }),
                }
            }),
        ]
        .spacing(5),
    )
    .style(section_style)
    .padding(20)
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

/// Section container style
fn section_style(theme: &Theme) -> container::Style {
    let palette = theme.palette();
    container::Style {
        background: Some(iced::Background::Color(iced::Color {
            a: 0.03,
            ..palette.text
        })),
        border: iced::Border {
            radius: 8.0.into(),
            width: 1.0,
            color: iced::Color {
                a: 0.1,
                ..palette.text
            },
        },
        ..Default::default()
    }
}
