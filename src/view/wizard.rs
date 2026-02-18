//! Wizard view for first-run setup
//!
//! Shows auto-discovery progress and server configuration.
//! Uses "Foundry Dark" design system with copper accents.

use crate::config::{DiscoveryResult, DiscoverySource, DiscoveryStep, DiscoveryStepStatus};
use crate::message::Message;
use crate::model::WizardStep;
use crate::theme::{colors, components, icons, spacing, typography};
use iced::widget::{button, center, column, container, row, text, text_input, Space};
use iced::{Element, Length};

/// Render the wizard view based on current step
pub fn wizard_view<'a>(
    step: WizardStep,
    _discovering: bool,
    discovery_steps: &'a [DiscoveryStep],
    discovery_result: Option<&'a DiscoveryResult>,
    server_url: &'a str,
    api_key: &'a str,
) -> Element<'a, Message> {
    let content = match step {
        WizardStep::Discovering => discovering_view(discovery_steps),
        WizardStep::FoundServer => {
            if let Some(result) = discovery_result {
                found_server_view(result)
            } else {
                discovering_view(discovery_steps)
            }
        }
        WizardStep::ManualEntry => manual_entry_view(server_url, api_key),
        WizardStep::Complete => {
            // Should not show wizard when complete
            column![text("Ready to connect...")].into()
        }
    };

    // Full-screen BG_DEEP background behind the centered card
    container(center(content))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(colors::BG_DEEP)),
            ..Default::default()
        })
        .into()
}

/// Logo mark: large copper diamond above the card
fn logo_mark<'a>() -> Element<'a, Message> {
    column![
        // Diamond icon
        text(icons::DIAMOND)
            .size(typography::SIZE_3XL)
            .style(components::text_accent),
        Space::with_height(spacing::SM),
        // Wordmark
        text("msgvault")
            .size(typography::SIZE_2XL)
            .font(typography::FONT_SEMIBOLD)
            .style(components::text_primary),
        Space::with_height(spacing::XS),
        // Tagline
        text("Your email, your archive")
            .size(typography::SIZE_SM)
            .style(components::text_muted),
    ]
    .align_x(iced::Alignment::Center)
    .into()
}

/// Discovering view - shows progress of auto-discovery
fn discovering_view<'a>(steps: &'a [DiscoveryStep]) -> Element<'a, Message> {
    let subtitle = text("Looking for your msgvault server...")
        .size(typography::SIZE_MD)
        .style(components::text_secondary);

    // Progress indicator
    let progress = text(icons::DOTS)
        .size(typography::SIZE_SM)
        .style(components::text_muted);

    // Show discovery steps
    let steps_list: Element<'a, Message> = if steps.is_empty() {
        column![
            step_row("MSGVAULT_HOME", DiscoveryStepStatus::Checking),
            step_row("Config files", DiscoveryStepStatus::Checking),
            step_row("Localhost", DiscoveryStepStatus::Checking),
        ]
        .spacing(spacing::SM)
        .into()
    } else {
        let step_elements: Vec<Element<'a, Message>> = steps
            .iter()
            .map(|s| step_row(&s.name, s.status.clone()))
            .collect();

        column(step_elements).spacing(spacing::SM).into()
    };

    let card = container(
        column![
            logo_mark(),
            Space::with_height(spacing::LG),
            subtitle,
            Space::with_height(spacing::XXL),
            steps_list,
            Space::with_height(spacing::XL),
            progress,
        ]
        .align_x(iced::Alignment::Center),
    )
    .style(components::card_style)
    .padding(spacing::XXXL)
    .width(Length::Fixed(400.0));

    card.into()
}

/// Found server view - shows discovered server and confirmation
fn found_server_view(result: &DiscoveryResult) -> Element<'static, Message> {
    let server_url_str = result.server_url.clone().unwrap_or_else(|| "Unknown".to_string());

    let source_text = match &result.source {
        DiscoverySource::EnvVar => "Found via MSGVAULT_HOME environment variable".to_string(),
        DiscoverySource::ConfigFile(path) => format!("Found in config: {}", path.display()),
        DiscoverySource::LocalhostProbe(port) => format!("Found server running on port {}", port),
        DiscoverySource::NeedsWizard => "Manual configuration needed".to_string(),
    };

    let server_label = text("Server URL")
        .size(typography::SIZE_SM)
        .style(components::text_muted);

    // Copper highlight on the server URL
    let server_value = text(server_url_str)
        .size(typography::SIZE_LG)
        .font(typography::FONT_SEMIBOLD)
        .style(components::text_accent);

    let source_label = text(source_text)
        .size(typography::SIZE_XS)
        .style(components::text_muted);

    // Buttons: Connect = primary, Manual = ghost
    let connect_button = button(
        text("Connect")
            .size(typography::SIZE_SM)
            .font(typography::FONT_MEDIUM),
    )
    .padding([spacing::SM, spacing::XL])
    .style(components::button_primary)
    .on_press(Message::ConfirmDiscoveredServer);

    let manual_button = button(
        text("Enter Different Server")
            .size(typography::SIZE_SM),
    )
    .padding([spacing::SM, spacing::LG])
    .style(components::button_ghost)
    .on_press(Message::ChooseManualEntry);

    let card = container(
        column![
            logo_mark(),
            Space::with_height(spacing::XL),
            text("Server Found!")
                .size(typography::SIZE_LG)
                .font(typography::FONT_SEMIBOLD)
                .style(components::text_success),
            Space::with_height(spacing::LG),
            server_label,
            Space::with_height(spacing::XS),
            server_value,
            Space::with_height(spacing::XS),
            source_label,
            Space::with_height(spacing::XXL),
            row![connect_button, Space::with_width(spacing::SM), manual_button]
                .align_y(iced::Alignment::Center),
        ]
        .align_x(iced::Alignment::Center),
    )
    .style(components::card_style)
    .padding(spacing::XXXL)
    .width(Length::Fixed(400.0));

    card.into()
}

/// Manual entry view - form for entering server details
fn manual_entry_view<'a>(server_url: &'a str, api_key: &'a str) -> Element<'a, Message> {
    let subtitle = text("Enter your msgvault server details")
        .size(typography::SIZE_MD)
        .style(components::text_secondary);

    let url_label = text("Server URL")
        .size(typography::SIZE_SM)
        .style(components::text_secondary);

    let url_input = text_input("http://localhost:8080", server_url)
        .on_input(Message::WizardServerUrlChanged)
        .padding(spacing::MD)
        .width(Length::Fill)
        .style(components::text_input_style);

    let api_key_label = text("API Key (optional)")
        .size(typography::SIZE_SM)
        .style(components::text_secondary);

    let api_key_input = text_input("", api_key)
        .on_input(Message::WizardApiKeyChanged)
        .padding(spacing::MD)
        .width(Length::Fill)
        .style(components::text_input_style)
        .secure(true);

    let connect_button = button(
        text("Connect")
            .size(typography::SIZE_SM)
            .font(typography::FONT_MEDIUM),
    )
    .padding([spacing::SM, spacing::XL])
    .style(components::button_primary)
    .on_press(Message::FinishWizard);

    let hint = text("Make sure your msgvault server is running")
        .size(typography::SIZE_XS)
        .style(components::text_muted);

    let card = container(
        column![
            logo_mark(),
            Space::with_height(spacing::LG),
            subtitle,
            Space::with_height(spacing::XXL),
            url_label,
            Space::with_height(spacing::XS),
            url_input,
            Space::with_height(spacing::LG),
            api_key_label,
            Space::with_height(spacing::XS),
            api_key_input,
            Space::with_height(spacing::XXL),
            connect_button,
            Space::with_height(spacing::SM),
            hint,
        ]
        .align_x(iced::Alignment::Center),
    )
    .style(components::card_style)
    .padding(spacing::XXXL)
    .width(Length::Fixed(400.0));

    card.into()
}

/// Single discovery step row
fn step_row(name: &str, status: DiscoveryStepStatus) -> Element<'static, Message> {
    let (icon, color) = match &status {
        DiscoveryStepStatus::Checking => (icons::DOTS, colors::TEXT_MUTED),
        DiscoveryStepStatus::Found(_) => (icons::CHECK, colors::ACCENT_SUCCESS),
        DiscoveryStepStatus::NotFound => (icons::CROSS, colors::TEXT_MUTED),
        DiscoveryStepStatus::Failed(_) => (icons::CROSS, colors::ACCENT_ERROR),
    };

    let icon_text = text(icon)
        .size(typography::SIZE_SM)
        .style(move |_| iced::widget::text::Style { color: Some(color) });

    let name_text = text(name.to_string())
        .size(typography::SIZE_SM)
        .style(components::text_secondary);

    let status_text: Element<'static, Message> = match status {
        DiscoveryStepStatus::Found(url) => text(url)
            .size(typography::SIZE_XS)
            .style(components::text_accent)
            .into(),
        DiscoveryStepStatus::Failed(err) => text(err)
            .size(typography::SIZE_XS)
            .style(components::text_error)
            .into(),
        _ => Space::with_width(0).into(),
    };

    row![
        icon_text,
        Space::with_width(spacing::SM),
        name_text,
        Space::with_width(spacing::SM),
        status_text,
    ]
    .align_y(iced::Alignment::Center)
    .into()
}
