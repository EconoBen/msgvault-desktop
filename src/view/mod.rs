//! View logic (UI rendering)
//!
//! The View in the MVU pattern.
//! Renders the UI based on current application state.

pub mod accounts;
pub mod aggregates;
pub mod dashboard;
pub mod layout;
pub mod message_detail;
pub mod messages;
pub mod search;
pub mod settings;
pub mod sidebar;
pub mod sync;
pub mod widgets;
pub mod wizard;

pub use accounts::accounts_view;
pub use aggregates::aggregates_view;
pub use layout::{three_panel_layout, two_panel_layout};
pub use message_detail::message_detail_view;
pub use messages::messages_view;
pub use search::search_view;
pub use settings::settings_view;
pub use sidebar::sidebar;
pub use sync::sync_view;
pub use wizard::wizard_view;

use crate::message::Message;
use crate::model::{AppState, ConnectionStatus, LoadingState, ViewLevel, WizardStep};
use crate::theme::{colors, components, spacing, typography};
use dashboard::dashboard;
use iced::widget::{button, center, column, container, row, stack, text, text_input, Space};
use iced::{Element, Length, Theme};
use widgets::{breadcrumb, error, loading};

/// Render the application view based on current state
pub fn render(state: &AppState) -> Element<'_, Message> {
    let content = if state.first_run && state.wizard_step != WizardStep::Complete {
        // Show wizard for first-run setup
        wizard_view(
            state.wizard_step,
            state.discovering,
            &state.discovery_steps,
            state.discovery_result.as_ref(),
            &state.server_url,
            &state.api_key,
        )
    } else if !state.is_connected() {
        // Show connection view (for reconnection after setup)
        connection_view(state)
    } else {
        // Show main application view
        connected_view(state)
    };

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(spacing::XL)
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(colors::BG_BASE)),
            ..Default::default()
        })
        .into()
}

/// Connection setup view - shown on first run or when disconnected
fn connection_view(state: &AppState) -> Element<'_, Message> {
    let title = text("msgvault")
        .size(typography::SIZE_2XL)
        .style(components::text_primary);

    let subtitle = text("Connect to your msgvault server")
        .size(typography::SIZE_MD)
        .style(components::text_secondary);

    let url_input = text_input(
        "Server URL (e.g., http://localhost:8080)",
        &state.server_url,
    )
    .on_input(Message::ServerUrlChanged)
    .padding(spacing::MD)
    .width(Length::Fixed(400.0))
    .style(components::text_input_style);

    let api_key_input = text_input("API Key (optional)", &state.api_key)
        .on_input(Message::ApiKeyChanged)
        .padding(spacing::MD)
        .width(Length::Fixed(400.0))
        .style(components::text_input_style)
        .secure(true);

    let connect_button = button(
        text("Connect")
            .size(typography::SIZE_BODY)
    )
    .padding([spacing::SM, spacing::XL])
    .style(components::button_primary)
    .on_press(Message::CheckHealth);

    let status_text: Element<'_, Message> = match &state.connection_status {
        ConnectionStatus::Unknown => Space::with_height(typography::SIZE_SM).into(),
        ConnectionStatus::Connecting => text("Connecting...")
            .size(typography::SIZE_SM)
            .style(components::text_muted)
            .into(),
        ConnectionStatus::Connected => text("Connected!")
            .size(typography::SIZE_SM)
            .style(components::text_success)
            .into(),
        ConnectionStatus::Failed(err) => text(format!("Failed: {}", truncate_error(err, 50)))
            .size(typography::SIZE_SM)
            .style(components::text_error)
            .into(),
    };

    // Card container for the form
    let form_card = container(
        column![
            title,
            Space::with_height(spacing::SM),
            subtitle,
            Space::with_height(spacing::XXL),
            url_input,
            Space::with_height(spacing::MD),
            api_key_input,
            Space::with_height(spacing::XL),
            connect_button,
            Space::with_height(spacing::MD),
            status_text,
        ]
        .spacing(spacing::XS)
        .align_x(iced::Alignment::Center)
    )
    .style(components::card_style)
    .padding(spacing::XXL);

    center(form_card).into()
}

/// Main connected view with navigation and content
fn connected_view(state: &AppState) -> Element<'_, Message> {
    // Get account emails for sidebar
    let account_emails: Vec<String> = state
        .sync_accounts
        .iter()
        .map(|a| a.email.clone())
        .collect();

    // Get labels (empty for now - would come from API)
    let labels: Vec<String> = vec![];

    // Create sidebar
    let sidebar_element = sidebar(state.navigation.current(), &account_emails, &labels);

    // Main content based on loading state and current view
    let content = match &state.loading {
        LoadingState::Loading => loading("Loading..."),
        LoadingState::Error(msg) => error(msg),
        LoadingState::Idle => view_content(state),
    };

    // Use three-panel layout for message detail view
    let main_view: Element<'_, Message> = match state.navigation.current() {
        ViewLevel::MessageDetail { .. } => {
            // Three-panel: sidebar + message list + detail
            let filter_desc = state
                .navigation
                .current_filter_description()
                .unwrap_or_else(|| "Messages".to_string());

            let list_content = messages_view(
                filter_desc,
                &state.messages,
                state.message_selected_index,
                state.messages_offset,
                state.messages_total,
                &state.selected_messages,
            );

            let detail_content = if let Some(detail) = &state.current_message {
                Some(message_detail_view(detail))
            } else {
                Some(loading("Loading message..."))
            };

            three_panel_layout(sidebar_element, list_content, detail_content)
        }
        _ => {
            // Two-panel: sidebar + content
            two_panel_layout(sidebar_element, content)
        }
    };

    // Overlay modals if showing
    if state.show_help_modal {
        stack![main_view, help_modal()].into()
    } else if state.show_delete_modal {
        stack![
            main_view,
            delete_confirmation_modal(state.selected_messages.len())
        ]
        .into()
    } else {
        main_view
    }
}

/// Keyboard shortcuts help modal
fn help_modal() -> Element<'static, Message> {
    // Semi-transparent backdrop
    let backdrop = container(Space::new(Length::Fill, Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(components::modal_backdrop_style);

    // Help content
    let title = text("Keyboard Shortcuts")
        .size(typography::SIZE_LG)
        .style(components::text_primary);

    let shortcuts = column![
        shortcut_row("Navigation", ""),
        shortcut_row("  j / ↓", "Move down"),
        shortcut_row("  k / ↑", "Move up"),
        shortcut_row("  Enter", "Open / Drill down"),
        shortcut_row("  Esc", "Go back"),
        shortcut_row("  Tab", "Cycle view types"),
        Space::with_height(spacing::SM),
        shortcut_row("Views", ""),
        shortcut_row("  /", "Search"),
        shortcut_row("  y", "Sync status"),
        shortcut_row("  a", "Accounts"),
        shortcut_row("  ,", "Settings"),
        Space::with_height(spacing::SM),
        shortcut_row("Actions", ""),
        shortcut_row("  Space", "Toggle selection"),
        shortcut_row("  Shift+A", "Select all"),
        shortcut_row("  x", "Clear selection"),
        shortcut_row("  d", "Delete selected"),
        shortcut_row("  s", "Toggle sort field"),
        shortcut_row("  r", "Reverse sort"),
        Space::with_height(spacing::SM),
        shortcut_row("Messages", ""),
        shortcut_row("  n", "Next page"),
        shortcut_row("  p", "Previous page"),
        shortcut_row("  ← / →", "Prev/next message"),
        Space::with_height(spacing::SM),
        shortcut_row("General", ""),
        shortcut_row("  ?", "Toggle this help"),
    ]
    .spacing(spacing::XS);

    let close_button = button(text("Close").size(typography::SIZE_SM))
        .padding([spacing::SM, spacing::LG])
        .style(components::button_secondary)
        .on_press(Message::HideHelp);

    let dialog_content = column![
        title,
        Space::with_height(spacing::LG),
        shortcuts,
        Space::with_height(spacing::XL),
        close_button,
    ]
    .spacing(spacing::XS)
    .padding(spacing::XL)
    .align_x(iced::Alignment::Start);

    let dialog = container(dialog_content)
        .style(components::modal_dialog_style)
        .padding(spacing::SM);

    stack![backdrop, center(dialog)].into()
}

/// Single shortcut row
fn shortcut_row<'a>(key: &'a str, description: &'a str) -> Element<'a, Message> {
    if description.is_empty() {
        // Section header
        text(key)
            .size(typography::SIZE_SM)
            .style(components::text_accent)
            .into()
    } else {
        row![
            text(key)
                .size(typography::SIZE_XS)
                .width(Length::Fixed(100.0))
                .style(components::text_secondary),
            text(description)
                .size(typography::SIZE_XS)
                .style(components::text_muted),
        ]
        .spacing(spacing::SM)
        .into()
    }
}

/// Delete confirmation modal overlay
fn delete_confirmation_modal(count: usize) -> Element<'static, Message> {
    // Semi-transparent backdrop
    let backdrop = container(Space::new(Length::Fill, Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(components::modal_backdrop_style);

    // Modal dialog
    let title = text("Confirm Delete")
        .size(typography::SIZE_LG)
        .style(components::text_primary);

    let message = text(format!(
        "Are you sure you want to stage {} message{} for deletion?",
        count,
        if count == 1 { "" } else { "s" }
    ))
    .size(typography::SIZE_SM)
    .style(components::text_secondary);

    let cancel_button = button(text("Cancel").size(typography::SIZE_SM))
        .padding([spacing::SM, spacing::LG])
        .style(components::button_secondary)
        .on_press(Message::HideDeleteModal);

    let confirm_button = button(text("Delete").size(typography::SIZE_SM))
        .padding([spacing::SM, spacing::LG])
        .style(components::button_danger)
        .on_press(Message::ConfirmDelete);

    let buttons = row![cancel_button, Space::with_width(spacing::SM), confirm_button]
        .align_y(iced::Alignment::Center);

    let dialog_content = column![
        title,
        Space::with_height(spacing::LG),
        message,
        Space::with_height(spacing::XL),
        buttons,
    ]
    .spacing(spacing::XS)
    .padding(spacing::XL)
    .align_x(iced::Alignment::Center);

    let dialog = container(dialog_content)
        .style(components::modal_dialog_style)
        .padding(spacing::SM);

    // Center the dialog on the backdrop
    stack![
        backdrop,
        center(dialog)
    ]
    .into()
}

/// Render the header with breadcrumb navigation
fn header_view(state: &AppState) -> Element<'_, Message> {
    let breadcrumbs = state.navigation.breadcrumbs();

    let title = text("msgvault")
        .size(typography::SIZE_XL)
        .style(components::text_primary);

    let breadcrumb_bar = if !breadcrumbs.is_empty() {
        breadcrumb(breadcrumbs)
    } else {
        row![].into()
    };

    let server_info = text(format!("Connected: {}", &state.server_url))
        .size(typography::SIZE_XS)
        .style(components::text_muted);

    column![
        row![title, Space::with_width(Length::Fill), server_info]
            .align_y(iced::Alignment::Center)
            .padding([spacing::SM, spacing::XL]),
        container(breadcrumb_bar).padding([0, spacing::XL]),
    ]
    .spacing(spacing::XS)
    .into()
}

/// Render content based on current view level
fn view_content(state: &AppState) -> Element<'_, Message> {
    match state.navigation.current() {
        ViewLevel::Dashboard => {
            // Show dashboard with stats if loaded
            if let Some(stats) = &state.stats {
                dashboard(stats)
            } else {
                loading("Loading statistics...")
            }
        }
        ViewLevel::Aggregates { view_type } => {
            // Show aggregate list view
            aggregates_view(
                view_type,
                &state.aggregates,
                state.selected_index,
                state.sort_field,
                state.sort_dir,
            )
        }
        ViewLevel::SubAggregates {
            parent_key,
            view_type,
            ..
        } => {
            // Placeholder for sub-aggregates (will use same aggregates_view with different data)
            center(
                column![
                    text(format!("{} → {}", parent_key, view_type.display_name())).size(20),
                    Space::with_height(10),
                    text("Sub-aggregates coming soon...").size(14),
                ]
                .align_x(iced::Alignment::Center),
            )
            .into()
        }
        ViewLevel::Messages { filter_description } => {
            // Show message list view
            messages_view(
                filter_description.clone(),
                &state.messages,
                state.message_selected_index,
                state.messages_offset,
                state.messages_total,
                &state.selected_messages,
            )
        }
        ViewLevel::MessageDetail { .. } => {
            // Show message detail view
            if let Some(detail) = &state.current_message {
                message_detail_view(detail)
            } else {
                loading("Loading message...")
            }
        }
        ViewLevel::Search => {
            // Show search view
            search_view(
                &state.search_query,
                state.search_deep_mode,
                &state.search_results,
                state.search_selected_index,
                state.search_total,
                state.is_searching,
                &state.selected_messages,
            )
        }
        ViewLevel::Sync => {
            // Show sync status view
            sync_view(
                &state.sync_accounts,
                state.sync_loading,
                state.syncing_account.as_deref(),
            )
        }
        ViewLevel::Accounts => {
            // Show accounts management view
            accounts_view(
                &state.sync_accounts,
                state.sync_loading,
                &state.add_account_email,
                state.adding_account,
                state.oauth_response.as_ref(),
                state.show_remove_modal,
                state.removing_account.as_deref(),
            )
        }
        ViewLevel::Settings => {
            // Show settings view
            settings_view(
                state.settings_tab,
                &state.settings_server_url,
                &state.settings_api_key,
                state.testing_connection,
                state.connection_test_result.as_ref(),
            )
        }
    }
}

/// Truncate error messages for display
fn truncate_error(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len])
    }
}
