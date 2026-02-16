//! Update logic (message handlers)
//!
//! The Update in the MVU pattern.
//! Processes Messages and returns Commands for async operations.

use crate::api::ApiClient;
use crate::message::Message;
use crate::model::{AppState, ConnectionStatus};
use iced::Task;

/// Process a message and update state
///
/// Returns a Task that may spawn async work (like API calls).
pub fn handle(state: &mut AppState, message: Message) -> Task<Message> {
    match message {
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

        Message::HealthChecked(result) => {
            match result {
                Ok(_health) => {
                    state.connection_status = ConnectionStatus::Connected;
                }
                Err(e) => {
                    state.connection_status = ConnectionStatus::Failed(e.to_string());
                }
            }
            Task::none()
        }

        Message::ServerUrlChanged(url) => {
            state.server_url = url;
            Task::none()
        }

        Message::ApiKeyChanged(key) => {
            state.api_key = key;
            Task::none()
        }

        Message::RetryConnection => {
            // Trigger a new health check
            Task::done(Message::CheckHealth)
        }

        Message::KeyPressed(_key) => {
            // Keyboard handling will be expanded in Phase 2
            Task::none()
        }

        Message::None => Task::none(),
    }
}
