//! Account management view
//!
//! Displays account list with add/remove functionality and OAuth flow UI.

use crate::api::types::{AccountSyncStatus, OAuthInitResponse};
use crate::message::Message;
use crate::theme::{colors, components, icons, spacing, typography};
use crate::view::widgets::avatar;
use iced::widget::{button, column, container, row, scrollable, text, text_input, Space};
use iced::{Background, Border, Element, Length, Theme};

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
    let title = text("Accounts")
        .size(typography::SIZE_XL)
        .font(typography::FONT_MEDIUM)
        .style(components::text_primary);

    // Add account section
    let add_section = add_account_section(add_email, adding_account, oauth_response);

    // Account list
    let account_list: Element<'a, Message> = if is_loading && accounts.is_empty() {
        container(
            text("Loading accounts...")
                .size(typography::SIZE_SM)
                .style(components::text_muted),
        )
        .padding(spacing::XL)
        .into()
    } else if accounts.is_empty() {
        container(
            text("No accounts configured. Add one below.")
                .size(typography::SIZE_SM)
                .style(components::text_muted),
        )
        .padding(spacing::XL)
        .into()
    } else {
        let account_rows: Vec<Element<'a, Message>> = accounts
            .iter()
            .map(account_row)
            .collect();

        scrollable(column(account_rows).spacing(spacing::SM))
            .height(Length::Fill)
            .into()
    };

    // Keyboard hints in FONT_MONO
    let hints = text("a: add account | Esc: back")
        .size(typography::SIZE_2XS)
        .font(typography::FONT_MONO)
        .style(components::text_muted);

    let main_content: Element<'a, Message> = column![
        title,
        Space::with_height(spacing::XL),
        account_list,
        Space::with_height(spacing::XL),
        add_section,
        Space::with_height(spacing::SM),
        hints,
    ]
    .spacing(spacing::XS)
    .padding(spacing::XL)
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
    let section_title = text("Add Account")
        .size(typography::SIZE_LG)
        .font(typography::FONT_MEDIUM)
        .style(components::text_primary);

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
                Space::with_height(spacing::SM),
                text("Initiating OAuth...")
                    .size(typography::SIZE_SM)
                    .style(components::text_muted),
                Space::with_height(spacing::SM),
                button(text("Cancel").size(typography::SIZE_SM))
                    .padding([spacing::SM, spacing::LG])
                    .style(components::button_secondary)
                    .on_press(Message::CancelAddAccount),
            ]
            .spacing(spacing::XS),
        )
        .style(section_style)
        .padding(spacing::LG)
        .width(Length::Fill)
        .into();
    }

    // Normal add account form
    let email_input = text_input("Email address (e.g., user@gmail.com)", email)
        .on_input(Message::AddAccountEmailChanged)
        .padding(spacing::MD)
        .width(Length::Fill)
        .style(components::text_input_style);

    let add_button = if email.is_empty() {
        button(text("Add Account").size(typography::SIZE_SM))
            .padding([spacing::SM, spacing::LG])
            .style(components::button_primary)
    } else {
        button(text("Add Account").size(typography::SIZE_SM))
            .padding([spacing::SM, spacing::LG])
            .style(components::button_primary)
            .on_press(Message::StartAddAccount)
    };

    container(
        column![
            section_title,
            Space::with_height(spacing::SM),
            row![email_input, Space::with_width(spacing::SM), add_button]
                .align_y(iced::Alignment::Center),
        ]
        .spacing(spacing::XS),
    )
    .style(section_style)
    .padding(spacing::LG)
    .width(Length::Fill)
    .into()
}

/// Device flow section showing code and verification URL
fn device_flow_section<'a>(oauth: &'a OAuthInitResponse) -> Element<'a, Message> {
    let title = text("Device Authorization")
        .size(typography::SIZE_LG)
        .font(typography::FONT_MEDIUM)
        .style(components::text_primary);

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
        .size(typography::SIZE_2XL)
        .font(typography::FONT_MONO)
        .style(components::text_accent);

    let instructions = column![
        text("Enter this code at:")
            .size(typography::SIZE_SM)
            .style(components::text_secondary),
        text(url)
            .size(typography::SIZE_SM)
            .font(typography::FONT_MONO)
            .style(components::text_accent),
        Space::with_height(spacing::SM),
        text("Waiting for authorization...")
            .size(typography::SIZE_XS)
            .style(components::text_muted),
    ]
    .spacing(spacing::XS);

    let poll_button = button(text("Check Status").size(typography::SIZE_SM))
        .padding([spacing::SM, spacing::LG])
        .style(components::button_primary)
        .on_press(Message::PollDeviceFlow);

    let cancel_button = button(text("Cancel").size(typography::SIZE_SM))
        .padding([spacing::SM, spacing::LG])
        .style(components::button_secondary)
        .on_press(Message::CancelAddAccount);

    container(
        column![
            title,
            Space::with_height(spacing::LG),
            code_display,
            Space::with_height(spacing::LG),
            instructions,
            Space::with_height(spacing::LG),
            row![poll_button, Space::with_width(spacing::SM), cancel_button],
        ]
        .spacing(spacing::XS)
        .align_x(iced::Alignment::Center),
    )
    .style(section_style)
    .padding(spacing::XL)
    .width(Length::Fill)
    .into()
}

/// Single account row with avatar and status badge
fn account_row(account: &AccountSyncStatus) -> Element<'_, Message> {
    let name = account
        .display_name
        .as_ref()
        .filter(|n| !n.is_empty())
        .unwrap_or(&account.email);

    // Avatar
    let avatar_widget = avatar(name, 40);

    let account_name = text(name)
        .size(typography::SIZE_MD)
        .font(typography::FONT_MEDIUM)
        .style(components::text_primary);

    let account_email = text(&account.email)
        .size(typography::SIZE_XS)
        .style(components::text_secondary);

    // Status badge with RADIUS_SM
    let status_color = match account.status {
        crate::api::types::SyncState::Idle => colors::ACCENT_SUCCESS,
        crate::api::types::SyncState::Running => colors::ACCENT_INFO,
        crate::api::types::SyncState::Paused => colors::ACCENT_WARNING,
        crate::api::types::SyncState::Error => colors::ACCENT_ERROR,
    };

    let status_badge = container(
        text(account.status.display_name())
            .size(typography::SIZE_2XS)
            .style(move |_: &Theme| iced::widget::text::Style {
                color: Some(status_color),
            }),
    )
    .padding([spacing::SPACE_1, spacing::SM])
    .style(move |_| container::Style {
        background: Some(Background::Color(iced::Color {
            a: 0.12,
            ..status_color
        })),
        border: Border {
            radius: spacing::RADIUS_SM.into(),
            ..Default::default()
        },
        ..Default::default()
    });

    let remove_button = button(
        text(icons::DELETE)
            .size(typography::SIZE_SM)
    )
    .padding([spacing::XS, spacing::SM])
    .style(components::button_danger)
    .on_press(Message::ShowRemoveAccountModal(account.email.clone()));

    let left_col = row![
        avatar_widget,
        Space::with_width(spacing::MD),
        column![account_name, account_email, Space::with_height(spacing::XS), status_badge]
            .spacing(spacing::SPACE_1),
    ]
    .align_y(iced::Alignment::Center)
    .width(Length::FillPortion(3));

    let right_col = column![remove_button]
        .width(Length::FillPortion(1))
        .align_x(iced::Alignment::End);

    let row_content = row![left_col, right_col]
        .spacing(spacing::XL)
        .padding(spacing::LG);

    container(row_content)
        .style(components::card_style)
        .width(Length::Fill)
        .into()
}

/// Remove account confirmation modal
fn remove_confirmation_modal(email: &str) -> Element<'static, Message> {
    // Semi-transparent backdrop
    let backdrop = container(Space::new(Length::Fill, Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(components::modal_backdrop_style);

    // Modal dialog
    let title = text("Remove Account")
        .size(typography::SIZE_LG)
        .font(typography::FONT_MEDIUM)
        .style(components::text_primary);

    let message = text(format!(
        "Are you sure you want to remove {}?",
        email
    ))
    .size(typography::SIZE_SM)
    .style(components::text_secondary);

    let warning = text("This will stop syncing this account. Existing messages will not be deleted.")
        .size(typography::SIZE_XS)
        .style(components::text_muted);

    let cancel_button = button(text("Cancel").size(typography::SIZE_SM))
        .padding([spacing::SM, spacing::LG])
        .style(components::button_secondary)
        .on_press(Message::HideRemoveAccountModal);

    let confirm_button = button(text("Remove").size(typography::SIZE_SM))
        .padding([spacing::SM, spacing::LG])
        .style(components::button_danger)
        .on_press(Message::ConfirmRemoveAccount);

    let buttons = row![cancel_button, Space::with_width(spacing::SM), confirm_button]
        .align_y(iced::Alignment::Center);

    let dialog_content = column![
        title,
        Space::with_height(spacing::LG),
        message,
        Space::with_height(spacing::SM),
        warning,
        Space::with_height(spacing::XL),
        buttons,
    ]
    .spacing(spacing::XS)
    .padding(spacing::XL)
    .align_x(iced::Alignment::Center);

    let dialog = container(dialog_content)
        .style(components::modal_dialog_style)
        .padding(spacing::SM);

    iced::widget::stack![
        backdrop,
        iced::widget::center(dialog)
    ]
    .into()
}

/// Section container style
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
