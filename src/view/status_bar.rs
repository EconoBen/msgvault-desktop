//! Status bar component
//!
//! Thin bar at the bottom of the connected view showing connection status,
//! sync activity, and message count.

use crate::api::types::AccountSyncStatus;
use crate::message::Message;
use crate::model::ConnectionStatus;
use crate::theme::{colors, components, icons, spacing, typography};
use iced::widget::{container, row, text, Space};
use iced::{Background, Border, Element, Length};

/// Height of the status bar in pixels
const STATUS_BAR_HEIGHT: f32 = 28.0;

/// Render the status bar
pub fn status_bar<'a>(
    connection_status: &ConnectionStatus,
    server_url: &str,
    sync_accounts: &[AccountSyncStatus],
    syncing_account: Option<&str>,
    total_messages: Option<i64>,
) -> Element<'a, Message> {
    // --- Left: connection indicator ---
    let connection_element = connection_indicator(connection_status, server_url);

    // --- Center: sync status ---
    let sync_element = sync_status(sync_accounts, syncing_account);

    // --- Right: message count ---
    let count_element = message_count(total_messages);

    let bar_content = row![
        connection_element,
        Space::with_width(Length::Fill),
        sync_element,
        Space::with_width(Length::Fill),
        count_element,
    ]
    .align_y(iced::Alignment::Center)
    .padding([0, spacing::MD]);

    container(bar_content)
        .width(Length::Fill)
        .height(Length::Fixed(STATUS_BAR_HEIGHT))
        .style(|_theme| container::Style {
            background: Some(Background::Color(colors::BG_DEEP)),
            border: Border {
                width: 1.0,
                color: colors::BORDER_SUBTLE,
                radius: 0.0.into(),
            },
            ..Default::default()
        })
        .into()
}

/// Connection status indicator (left side)
fn connection_indicator<'a>(
    status: &ConnectionStatus,
    server_url: &str,
) -> Element<'a, Message> {
    let (dot, dot_style, label): (&str, fn(&iced::Theme) -> text::Style, String) = match status {
        ConnectionStatus::Connected => (
            icons::DOT_FILLED,
            components::text_success,
            format!("Connected to {}", display_url(server_url)),
        ),
        ConnectionStatus::Connecting => (
            icons::DOT_FILLED,
            components::text_muted,
            "Connecting...".to_string(),
        ),
        ConnectionStatus::Failed(_) => (
            icons::DOT_EMPTY,
            components::text_error as fn(&iced::Theme) -> text::Style,
            "Disconnected".to_string(),
        ),
        ConnectionStatus::Unknown => (
            icons::DOT_EMPTY,
            components::text_muted,
            "Not connected".to_string(),
        ),
    };

    row![
        text(dot)
            .size(typography::SIZE_2XS)
            .style(dot_style),
        Space::with_width(spacing::XS),
        text(label)
            .size(typography::SIZE_2XS)
            .font(typography::FONT_MONO)
            .style(components::text_muted),
    ]
    .align_y(iced::Alignment::Center)
    .into()
}

/// Sync status display (center)
fn sync_status<'a>(
    accounts: &[AccountSyncStatus],
    syncing_account: Option<&str>,
) -> Element<'a, Message> {
    let label = if let Some(email) = syncing_account {
        // Currently syncing an account
        let short = truncate_email(email);
        format!("Syncing {}...", short)
    } else {
        // Show last sync time from most recently synced account
        most_recent_sync_label(accounts)
    };

    text(label)
        .size(typography::SIZE_2XS)
        .font(typography::FONT_MONO)
        .style(components::text_muted)
        .into()
}

/// Message count display (right side)
fn message_count<'a>(total: Option<i64>) -> Element<'a, Message> {
    let label = match total {
        Some(n) => format_count(n),
        None => "\u{2014}".to_string(), // em-dash
    };

    text(label)
        .size(typography::SIZE_2XS)
        .font(typography::FONT_MONO)
        .style(components::text_muted)
        .into()
}

// === Helpers ===

/// Strip scheme from URL for compact display
fn display_url(url: &str) -> &str {
    url.strip_prefix("http://")
        .or_else(|| url.strip_prefix("https://"))
        .unwrap_or(url)
}

/// Truncate an email for compact display (keep first 20 chars)
fn truncate_email(email: &str) -> &str {
    if email.len() <= 24 {
        email
    } else {
        &email[..24]
    }
}

/// Derive a human-readable "last sync" label from the most recent account
fn most_recent_sync_label(accounts: &[AccountSyncStatus]) -> String {
    // Find the most recent last_sync_at across all accounts
    let latest = accounts
        .iter()
        .filter_map(|a| a.last_sync_at.as_deref())
        .max();

    match latest {
        Some(ts) => format!("Last sync: {}", friendly_timestamp(ts)),
        None => "No sync history".to_string(),
    }
}

/// Convert an ISO timestamp string to a friendly relative description.
/// Falls back to the raw value if parsing fails.
fn friendly_timestamp(ts: &str) -> String {
    // Try to parse RFC 3339 / ISO 8601
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(ts) {
        let now = chrono::Utc::now();
        let elapsed = now.signed_duration_since(dt);

        if elapsed.num_seconds() < 60 {
            "just now".to_string()
        } else if elapsed.num_minutes() < 60 {
            let m = elapsed.num_minutes();
            format!("{}m ago", m)
        } else if elapsed.num_hours() < 24 {
            let h = elapsed.num_hours();
            format!("{}h ago", h)
        } else {
            let d = elapsed.num_days();
            format!("{}d ago", d)
        }
    } else {
        // Fallback: show the raw string (truncated)
        ts.chars().take(16).collect()
    }
}

/// Format a count with thousands separators
fn format_count(n: i64) -> String {
    if n < 1_000 {
        return n.to_string();
    }
    // Simple thousands formatting
    let s = n.to_string();
    let mut result = String::with_capacity(s.len() + s.len() / 3);
    for (i, ch) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(ch);
    }
    result.chars().rev().collect()
}
