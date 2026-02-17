//! Account management view
//!
//! Displays account list with add/remove functionality and OAuth flow UI.

use crate::api::types::{AccountSyncStatus, OAuthInitResponse};
use crate::message::Message;
use iced::widget::{button, column, container, row, scrollable, text, text_input, Space};
use iced::{Element, Length, Theme};

/// Render the accounts view
pub fn accounts_view<'a>(
    accounts: &'a [AccountSyncStatus],
    is_loading: bool,
    add_email: &'a str,
    adding_account: bool,
    oauth_response: Option<&'a OAuthInitResponse>,
    show_remove_modal: bool,
    removing_account: Option<&'a str>,
) -> Element<'a, Message> {
    // Header
    let title = text("Accounts").size(24);

    // Add account section
    let add_section = add_account_section(add_email, adding_account, oauth_response);

    // Account list
    let account_list: Element<'a, Message> = if is_loading && accounts.is_empty() {
        container(text("Loading accounts...").size(14))
            .padding(20)
            .into()
    } else if accounts.is_empty() {
        container(text("No accounts configured. Add one below.").size(14))
            .padding(20)
            .into()
    } else {
        let account_rows: Vec<Element<'a, Message>> = accounts
            .iter()
            .map(account_row)
            .collect();

        scrollable(column(account_rows).spacing(10))
            .height(Length::Fill)
            .into()
    };

    // Keyboard hints
    let hints = text("a: add account | Esc: back").size(12);

    let main_content: Element<'a, Message> = column![
        title,
        Space::with_height(20),
        account_list,
        Space::with_height(20),
        add_section,
        Space::with_height(10),
        hints,
    ]
    .spacing(5)
    .padding(20)
    .width(Length::Fill)
    .height(Length::Fill)
    .into();

    // Overlay remove confirmation modal if showing
    if show_remove_modal {
        if let Some(email) = removing_account {
            iced::widget::stack![
                main_content,
                remove_confirmation_modal(email)
            ]
            .into()
        } else {
            main_content
        }
    } else {
        main_content
    }
}

/// Add account section with email input and OAuth status
fn add_account_section<'a>(
    email: &'a str,
    adding: bool,
    oauth_response: Option<&'a OAuthInitResponse>,
) -> Element<'a, Message> {
    let section_title = text("Add Account").size(18);

    // Show device flow UI if we have that response
    if let Some(oauth) = oauth_response {
        if oauth.device_flow {
            return device_flow_section(oauth);
        }
    }

    // Show loading state while adding
    if adding {
        return container(
            column![
                section_title,
                Space::with_height(10),
                text("Initiating OAuth...").size(14),
                Space::with_height(10),
                button(text("Cancel").size(14))
                    .padding([8, 16])
                    .on_press(Message::CancelAddAccount),
            ]
            .spacing(5),
        )
        .style(section_style)
        .padding(15)
        .width(Length::Fill)
        .into();
    }

    // Normal add account form
    let email_input = text_input("Email address (e.g., user@gmail.com)", email)
        .on_input(Message::AddAccountEmailChanged)
        .padding(10)
        .width(Length::Fill);

    let add_button = if email.is_empty() {
        button(text("Add Account").size(14)).padding([8, 16])
    } else {
        button(text("Add Account").size(14))
            .padding([8, 16])
            .on_press(Message::StartAddAccount)
    };

    container(
        column![
            section_title,
            Space::with_height(10),
            row![email_input, Space::with_width(10), add_button]
                .align_y(iced::Alignment::Center),
        ]
        .spacing(5),
    )
    .style(section_style)
    .padding(15)
    .width(Length::Fill)
    .into()
}

/// Device flow section showing code and verification URL
fn device_flow_section<'a>(oauth: &'a OAuthInitResponse) -> Element<'a, Message> {
    let title = text("Device Authorization").size(18);

    let code = oauth
        .user_code
        .as_ref()
        .map(|c| c.as_str())
        .unwrap_or("------");
    let url = oauth
        .verification_url
        .as_ref()
        .map(|u| u.as_str())
        .unwrap_or("");

    let code_display = text(code)
        .size(32)
        .style(|theme: &Theme| {
            let palette = theme.palette();
            iced::widget::text::Style {
                color: Some(palette.primary),
            }
        });

    let instructions = column![
        text("Enter this code at:").size(14),
        text(url).size(14),
        Space::with_height(10),
        text("Waiting for authorization...").size(12),
    ]
    .spacing(5);

    let poll_button = button(text("Check Status").size(14))
        .padding([8, 16])
        .on_press(Message::PollDeviceFlow);

    let cancel_button = button(text("Cancel").size(14))
        .padding([8, 16])
        .on_press(Message::CancelAddAccount);

    container(
        column![
            title,
            Space::with_height(15),
            code_display,
            Space::with_height(15),
            instructions,
            Space::with_height(15),
            row![poll_button, Space::with_width(10), cancel_button],
        ]
        .spacing(5)
        .align_x(iced::Alignment::Center),
    )
    .style(section_style)
    .padding(20)
    .width(Length::Fill)
    .into()
}

/// Single account row
fn account_row(account: &AccountSyncStatus) -> Element<'_, Message> {
    let name = account
        .display_name
        .as_ref()
        .filter(|n| !n.is_empty())
        .unwrap_or(&account.email);

    let account_name = text(name).size(16);
    let account_email = text(&account.email).size(12).style(|theme: &Theme| {
        let palette = theme.palette();
        iced::widget::text::Style {
            color: Some(iced::Color {
                a: 0.6,
                ..palette.text
            }),
        }
    });

    let status_text = text(account.status.display_name()).size(12);

    let remove_button = button(text("Remove").size(12))
        .padding([6, 12])
        .style(|_theme: &Theme, _status| {
            iced::widget::button::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgb(0.8, 0.2, 0.2))),
                text_color: iced::Color::WHITE,
                border: iced::Border {
                    radius: 4.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }
        })
        .on_press(Message::ShowRemoveAccountModal(account.email.clone()));

    let left_col = column![account_name, account_email, status_text]
        .spacing(2)
        .width(Length::FillPortion(3));

    let right_col = column![remove_button]
        .width(Length::FillPortion(1))
        .align_x(iced::Alignment::End);

    let row_content = row![left_col, right_col]
        .spacing(20)
        .padding(15);

    container(row_content)
        .style(|theme: &Theme| {
            let palette = theme.palette();
            container::Style {
                background: Some(iced::Background::Color(iced::Color {
                    a: 0.05,
                    ..palette.text
                })),
                border: iced::Border {
                    radius: 8.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }
        })
        .width(Length::Fill)
        .into()
}

/// Remove account confirmation modal
fn remove_confirmation_modal(email: &str) -> Element<'static, Message> {
    // Semi-transparent backdrop
    let backdrop = container(Space::new(Length::Fill, Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_theme: &Theme| container::Style {
            background: Some(iced::Background::Color(iced::Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.5,
            })),
            ..Default::default()
        });

    // Modal dialog
    let title = text("Remove Account").size(20);
    let message = text(format!(
        "Are you sure you want to remove {}?",
        email
    ))
    .size(14);
    let warning = text("This will stop syncing this account. Existing messages will not be deleted.")
        .size(12)
        .style(|theme: &Theme| {
            let palette = theme.palette();
            iced::widget::text::Style {
                color: Some(iced::Color {
                    a: 0.7,
                    ..palette.text
                }),
            }
        });

    let cancel_button = button(text("Cancel").size(14))
        .padding([8, 16])
        .on_press(Message::HideRemoveAccountModal);

    let confirm_button = button(text("Remove").size(14))
        .padding([8, 16])
        .style(|_theme: &Theme, _status| {
            iced::widget::button::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgb(0.8, 0.2, 0.2))),
                text_color: iced::Color::WHITE,
                border: iced::Border {
                    radius: 4.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }
        })
        .on_press(Message::ConfirmRemoveAccount);

    let buttons = row![cancel_button, Space::with_width(10), confirm_button]
        .align_y(iced::Alignment::Center);

    let dialog_content = column![
        title,
        Space::with_height(15),
        message,
        Space::with_height(10),
        warning,
        Space::with_height(20),
        buttons,
    ]
    .spacing(5)
    .padding(20)
    .align_x(iced::Alignment::Center);

    let dialog = container(dialog_content)
        .style(|theme: &Theme| {
            let palette = theme.palette();
            container::Style {
                background: Some(iced::Background::Color(palette.background)),
                border: iced::Border {
                    radius: 8.0.into(),
                    width: 1.0,
                    color: iced::Color {
                        a: 0.3,
                        ..palette.text
                    },
                },
                ..Default::default()
            }
        })
        .padding(10);

    iced::widget::stack![
        backdrop,
        iced::widget::center(dialog)
    ]
    .into()
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
