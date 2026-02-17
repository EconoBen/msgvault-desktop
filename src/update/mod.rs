//! Update logic (message handlers)
//!
//! The Update in the MVU pattern.
//! Processes Messages and returns Commands for async operations.

use crate::api::ApiClient;
use crate::message::Message;
use crate::model::{AppState, ConnectionStatus, LoadingState};
use iced::keyboard::{Key, Modifiers};
use iced::Task;

/// Process a message and update state
///
/// Returns a Task that may spawn async work (like API calls).
pub fn handle(state: &mut AppState, message: Message) -> Task<Message> {
    match message {
        // === Connection ===
        Message::CheckHealth => {
            state.connection_status = ConnectionStatus::Connecting;

            let url = state.server_url.clone();
            let api_key = if state.api_key.is_empty() {
                None
            } else {
                Some(state.api_key.clone())
            };

            Task::perform(
                async move {
                    let client = ApiClient::new(url, api_key);
                    client.health().await
                },
                Message::HealthChecked,
            )
        }

        Message::HealthChecked(result) => match result {
            Ok(_health) => {
                state.connection_status = ConnectionStatus::Connected;
                // After connecting, fetch stats
                Task::done(Message::FetchStats)
            }
            Err(e) => {
                state.connection_status = ConnectionStatus::Failed(e.to_string());
                Task::none()
            }
        },

        // === Stats ===
        Message::FetchStats => {
            state.loading = LoadingState::Loading;

            let url = state.server_url.clone();
            let api_key = if state.api_key.is_empty() {
                None
            } else {
                Some(state.api_key.clone())
            };

            Task::perform(
                async move {
                    let client = ApiClient::new(url, api_key);
                    client.stats().await
                },
                Message::StatsLoaded,
            )
        }

        Message::StatsLoaded(result) => {
            match result {
                Ok(stats) => {
                    state.stats = Some(stats);
                    state.loading = LoadingState::Idle;
                }
                Err(e) => {
                    state.loading = LoadingState::Error(e.to_string());
                }
            }
            Task::none()
        }

        // === Navigation ===
        Message::NavigateTo(view) => {
            state.navigation.push(view);
            Task::none()
        }

        Message::GoBack => {
            state.navigation.pop();
            Task::none()
        }

        Message::JumpToBreadcrumb(index) => {
            state.navigation.jump_to(index);
            Task::none()
        }

        Message::NextViewType => {
            // This will be handled in Phase 3 when we have aggregate views
            Task::none()
        }

        Message::PreviousViewType => {
            // This will be handled in Phase 3 when we have aggregate views
            Task::none()
        }

        // === User Input ===
        Message::ServerUrlChanged(url) => {
            state.server_url = url;
            Task::none()
        }

        Message::ApiKeyChanged(key) => {
            state.api_key = key;
            Task::none()
        }

        Message::RetryConnection => Task::done(Message::CheckHealth),

        // === Keyboard ===
        Message::KeyPressed(key, modifiers) => handle_key_press(state, key, modifiers),

        Message::None => Task::none(),
    }
}

/// Handle keyboard shortcuts
fn handle_key_press(state: &mut AppState, key: Key, modifiers: Modifiers) -> Task<Message> {
    // Only handle keys when connected
    if !state.is_connected() {
        return Task::none();
    }

    match key {
        // Escape - go back
        Key::Named(iced::keyboard::key::Named::Escape) => {
            if state.navigation.can_go_back() {
                Task::done(Message::GoBack)
            } else {
                Task::none()
            }
        }

        // Tab - cycle view types (will be used in Phase 3)
        Key::Named(iced::keyboard::key::Named::Tab) => {
            if modifiers.shift() {
                Task::done(Message::PreviousViewType)
            } else {
                Task::done(Message::NextViewType)
            }
        }

        // q - quit (handled by window, but we could show confirmation)
        Key::Character(ref c) if c == "q" && !modifiers.shift() => {
            // For now, do nothing - quit is handled by window close
            Task::none()
        }

        // ? - help (will be implemented later)
        Key::Character(ref c) if c == "?" => {
            // TODO: Show help modal
            Task::none()
        }

        _ => Task::none(),
    }
}
