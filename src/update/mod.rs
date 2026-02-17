//! Update logic (message handlers)
//!
//! The Update in the MVU pattern.
//! Processes Messages and returns Commands for async operations.

use crate::api::types::{SortDirection, SortField};
use crate::api::ApiClient;
use crate::message::Message;
use crate::model::{AppState, ConnectionStatus, LoadingState, ViewLevel};
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

        // === Aggregates ===
        Message::FetchAggregates(view_type) => {
            state.loading = LoadingState::Loading;
            state.selected_index = 0;

            let url = state.server_url.clone();
            let api_key = if state.api_key.is_empty() {
                None
            } else {
                Some(state.api_key.clone())
            };
            let sort_field = state.sort_field;
            let sort_dir = state.sort_dir;

            Task::perform(
                async move {
                    let client = ApiClient::new(url, api_key);
                    client.aggregates(view_type, sort_field, sort_dir).await
                },
                Message::AggregatesLoaded,
            )
        }

        Message::AggregatesLoaded(result) => {
            match result {
                Ok(response) => {
                    state.aggregates = response.rows;
                    state.loading = LoadingState::Idle;
                }
                Err(e) => {
                    state.loading = LoadingState::Error(e.to_string());
                }
            }
            Task::none()
        }

        Message::SelectAggregate(index) => {
            if index < state.aggregates.len() {
                state.selected_index = index;
            }
            Task::none()
        }

        Message::SelectPrevious => {
            if state.selected_index > 0 {
                state.selected_index -= 1;
            }
            Task::none()
        }

        Message::SelectNext => {
            if state.selected_index + 1 < state.aggregates.len() {
                state.selected_index += 1;
            }
            Task::none()
        }

        Message::DrillDown => {
            if let Some(agg) = state.aggregates.get(state.selected_index) {
                if let ViewLevel::Aggregates { view_type } = state.navigation.current().clone() {
                    // Navigate to messages filtered by this aggregate
                    let filter = format!("{}: {}", view_type.display_name(), &agg.key);
                    state.navigation.push(ViewLevel::Messages {
                        filter_description: filter,
                    });
                }
            }
            Task::none()
        }

        Message::ToggleSortField => {
            state.sort_field = match state.sort_field {
                SortField::Name => SortField::Count,
                SortField::Count => SortField::Size,
                SortField::Size => SortField::AttachmentSize,
                SortField::AttachmentSize => SortField::Name,
            };
            // Refetch with new sort
            if let ViewLevel::Aggregates { view_type } = state.navigation.current().clone() {
                return Task::done(Message::FetchAggregates(view_type));
            }
            Task::none()
        }

        Message::ToggleSortDirection => {
            state.sort_dir = match state.sort_dir {
                SortDirection::Asc => SortDirection::Desc,
                SortDirection::Desc => SortDirection::Asc,
            };
            // Refetch with new sort
            if let ViewLevel::Aggregates { view_type } = state.navigation.current().clone() {
                return Task::done(Message::FetchAggregates(view_type));
            }
            Task::none()
        }

        // === Navigation ===
        Message::NavigateTo(view) => {
            let fetch_task = if let ViewLevel::Aggregates { view_type } = &view {
                Some(Task::done(Message::FetchAggregates(*view_type)))
            } else {
                None
            };

            state.navigation.push(view);

            fetch_task.unwrap_or(Task::none())
        }

        Message::GoBack => {
            state.navigation.pop();
            // If we're back at an aggregate view, refetch
            if let ViewLevel::Aggregates { view_type } = state.navigation.current().clone() {
                return Task::done(Message::FetchAggregates(view_type));
            }
            Task::none()
        }

        Message::JumpToBreadcrumb(index) => {
            state.navigation.jump_to(index);
            // If we're at an aggregate view, refetch
            if let ViewLevel::Aggregates { view_type } = state.navigation.current().clone() {
                return Task::done(Message::FetchAggregates(view_type));
            }
            Task::none()
        }

        Message::NextViewType => {
            if let ViewLevel::Aggregates { view_type } = state.navigation.current().clone() {
                let next_type = view_type.next();
                // Replace current view with new view type
                state.navigation.pop();
                state.navigation.push(ViewLevel::Aggregates {
                    view_type: next_type,
                });
                return Task::done(Message::FetchAggregates(next_type));
            }
            Task::none()
        }

        Message::PreviousViewType => {
            if let ViewLevel::Aggregates { view_type } = state.navigation.current().clone() {
                let prev_type = view_type.previous();
                // Replace current view with new view type
                state.navigation.pop();
                state.navigation.push(ViewLevel::Aggregates {
                    view_type: prev_type,
                });
                return Task::done(Message::FetchAggregates(prev_type));
            }
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

    // Check if we're in an aggregate view for list navigation
    let in_aggregates = matches!(state.navigation.current(), ViewLevel::Aggregates { .. });

    match key {
        // Escape - go back
        Key::Named(iced::keyboard::key::Named::Escape) => {
            if state.navigation.can_go_back() {
                Task::done(Message::GoBack)
            } else {
                Task::none()
            }
        }

        // Tab - cycle view types
        Key::Named(iced::keyboard::key::Named::Tab) => {
            if modifiers.shift() {
                Task::done(Message::PreviousViewType)
            } else {
                Task::done(Message::NextViewType)
            }
        }

        // Enter - drill down into selected aggregate
        Key::Named(iced::keyboard::key::Named::Enter) => {
            if in_aggregates {
                Task::done(Message::DrillDown)
            } else {
                Task::none()
            }
        }

        // Arrow keys for navigation
        Key::Named(iced::keyboard::key::Named::ArrowUp) => {
            if in_aggregates {
                Task::done(Message::SelectPrevious)
            } else {
                Task::none()
            }
        }

        Key::Named(iced::keyboard::key::Named::ArrowDown) => {
            if in_aggregates {
                Task::done(Message::SelectNext)
            } else {
                Task::none()
            }
        }

        // j/k - vim-style navigation
        Key::Character(ref c) if c == "j" && !modifiers.shift() => {
            if in_aggregates {
                Task::done(Message::SelectNext)
            } else {
                Task::none()
            }
        }

        Key::Character(ref c) if c == "k" && !modifiers.shift() => {
            if in_aggregates {
                Task::done(Message::SelectPrevious)
            } else {
                Task::none()
            }
        }

        // s - toggle sort field
        Key::Character(ref c) if c == "s" && !modifiers.shift() => {
            if in_aggregates {
                Task::done(Message::ToggleSortField)
            } else {
                Task::none()
            }
        }

        // r - toggle sort direction (reverse)
        Key::Character(ref c) if c == "r" && !modifiers.shift() => {
            if in_aggregates {
                Task::done(Message::ToggleSortDirection)
            } else {
                Task::none()
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
