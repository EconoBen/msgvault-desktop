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
                    let filter_desc = format!("{}: {}", view_type.display_name(), &agg.key);
                    let filter_type = view_type.as_str().to_string();
                    let filter_value = agg.key.clone();

                    state.messages_offset = 0;
                    state.navigation.push(ViewLevel::Messages {
                        filter_description: filter_desc,
                    });

                    return Task::done(Message::FetchMessages {
                        filter_type,
                        filter_value,
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

        // === Messages ===
        Message::FetchMessages {
            filter_type,
            filter_value,
        } => {
            state.loading = LoadingState::Loading;
            state.message_selected_index = 0;
            state.filter_type = filter_type.clone();
            state.filter_value = filter_value.clone();

            let url = state.server_url.clone();
            let api_key = if state.api_key.is_empty() {
                None
            } else {
                Some(state.api_key.clone())
            };
            let offset = state.messages_offset;
            let limit = state.messages_limit;

            Task::perform(
                async move {
                    let client = ApiClient::new(url, api_key);
                    client
                        .messages_filter(&filter_type, &filter_value, offset, limit)
                        .await
                },
                Message::MessagesLoaded,
            )
        }

        Message::MessagesLoaded(result) => {
            match result {
                Ok(response) => {
                    state.messages = response.messages;
                    state.messages_total = response.total;
                    state.loading = LoadingState::Idle;
                }
                Err(e) => {
                    state.loading = LoadingState::Error(e.to_string());
                }
            }
            Task::none()
        }

        Message::SelectMessage(index) => {
            if index < state.messages.len() {
                state.message_selected_index = index;
            }
            Task::none()
        }

        Message::OpenMessage => {
            if let Some(msg) = state.messages.get(state.message_selected_index) {
                let message_id = msg.id;
                state.loading = LoadingState::Loading;

                // Navigate to detail view
                state.navigation.push(ViewLevel::MessageDetail { message_id });

                let url = state.server_url.clone();
                let api_key = if state.api_key.is_empty() {
                    None
                } else {
                    Some(state.api_key.clone())
                };

                return Task::perform(
                    async move {
                        let client = ApiClient::new(url, api_key);
                        client.message_detail(message_id).await
                    },
                    Message::MessageDetailLoaded,
                );
            }
            Task::none()
        }

        Message::MessageDetailLoaded(result) => {
            match result {
                Ok(detail) => {
                    state.current_message = Some(detail);
                    state.loading = LoadingState::Idle;
                }
                Err(e) => {
                    state.loading = LoadingState::Error(e.to_string());
                }
            }
            Task::none()
        }

        Message::NextPage => {
            let new_offset = state.messages_offset + state.messages_limit;
            if new_offset < state.messages_total {
                state.messages_offset = new_offset;
                return Task::done(Message::FetchMessages {
                    filter_type: state.filter_type.clone(),
                    filter_value: state.filter_value.clone(),
                });
            }
            Task::none()
        }

        Message::PreviousPage => {
            if state.messages_offset > 0 {
                state.messages_offset =
                    (state.messages_offset - state.messages_limit).max(0);
                return Task::done(Message::FetchMessages {
                    filter_type: state.filter_type.clone(),
                    filter_value: state.filter_value.clone(),
                });
            }
            Task::none()
        }

        Message::PreviousMessage => {
            if state.message_selected_index > 0 {
                state.message_selected_index -= 1;
                return Task::done(Message::OpenMessage);
            }
            Task::none()
        }

        Message::NextMessage => {
            if state.message_selected_index + 1 < state.messages.len() {
                state.message_selected_index += 1;
                return Task::done(Message::OpenMessage);
            }
            Task::none()
        }

        // === Search ===
        Message::OpenSearch => {
            state.navigation.push(ViewLevel::Search);
            state.search_query.clear();
            state.search_results.clear();
            state.search_selected_index = 0;
            state.search_total = 0;
            Task::none()
        }

        Message::SearchQueryChanged(query) => {
            state.search_query = query;
            // Execute search if query is not empty
            if !state.search_query.is_empty() {
                return Task::done(Message::ExecuteSearch);
            } else {
                state.search_results.clear();
                state.search_total = 0;
            }
            Task::none()
        }

        Message::ExecuteSearch => {
            if state.search_query.is_empty() {
                return Task::none();
            }

            state.is_searching = true;
            let query = state.search_query.clone();
            let is_deep = state.search_deep_mode;

            let url = state.server_url.clone();
            let api_key = if state.api_key.is_empty() {
                None
            } else {
                Some(state.api_key.clone())
            };

            Task::perform(
                async move {
                    let client = ApiClient::new(url, api_key);
                    if is_deep {
                        client.search_deep(&query, 0, 50).await
                    } else {
                        client.search_fast(&query, 50).await
                    }
                },
                Message::SearchLoaded,
            )
        }

        Message::SearchLoaded(result) => {
            state.is_searching = false;
            match result {
                Ok(response) => {
                    state.search_results = response.messages;
                    state.search_total = response.total;
                    state.search_selected_index = 0;
                }
                Err(e) => {
                    state.loading = LoadingState::Error(e.to_string());
                }
            }
            Task::none()
        }

        Message::ToggleSearchMode => {
            state.search_deep_mode = !state.search_deep_mode;
            // Re-execute search with new mode if query exists
            if !state.search_query.is_empty() {
                return Task::done(Message::ExecuteSearch);
            }
            Task::none()
        }

        Message::SelectSearchResult(index) => {
            if index < state.search_results.len() {
                state.search_selected_index = index;
            }
            Task::none()
        }

        Message::OpenSearchResult => {
            if let Some(msg) = state.search_results.get(state.search_selected_index) {
                let message_id = msg.id;
                state.loading = LoadingState::Loading;
                state.navigation.push(ViewLevel::MessageDetail { message_id });

                let url = state.server_url.clone();
                let api_key = if state.api_key.is_empty() {
                    None
                } else {
                    Some(state.api_key.clone())
                };

                return Task::perform(
                    async move {
                        let client = ApiClient::new(url, api_key);
                        client.message_detail(message_id).await
                    },
                    Message::MessageDetailLoaded,
                );
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

    // Determine current view type
    let in_aggregates = matches!(state.navigation.current(), ViewLevel::Aggregates { .. });
    let in_messages = matches!(state.navigation.current(), ViewLevel::Messages { .. });
    let in_detail = matches!(state.navigation.current(), ViewLevel::MessageDetail { .. });
    let in_search = matches!(state.navigation.current(), ViewLevel::Search);

    match key {
        // Escape - go back
        Key::Named(iced::keyboard::key::Named::Escape) => {
            if state.navigation.can_go_back() {
                Task::done(Message::GoBack)
            } else {
                Task::none()
            }
        }

        // Tab - cycle view types (aggregates) or toggle mode (search)
        Key::Named(iced::keyboard::key::Named::Tab) => {
            if in_aggregates {
                if modifiers.shift() {
                    Task::done(Message::PreviousViewType)
                } else {
                    Task::done(Message::NextViewType)
                }
            } else if in_search {
                Task::done(Message::ToggleSearchMode)
            } else {
                Task::none()
            }
        }

        // Enter - drill down, open message, or open search result
        Key::Named(iced::keyboard::key::Named::Enter) => {
            if in_aggregates {
                Task::done(Message::DrillDown)
            } else if in_messages {
                Task::done(Message::OpenMessage)
            } else if in_search {
                Task::done(Message::OpenSearchResult)
            } else {
                Task::none()
            }
        }

        // Arrow keys for navigation
        Key::Named(iced::keyboard::key::Named::ArrowUp) => {
            if in_aggregates {
                Task::done(Message::SelectPrevious)
            } else if in_messages {
                Task::done(Message::SelectMessage(
                    state.message_selected_index.saturating_sub(1),
                ))
            } else if in_search {
                Task::done(Message::SelectSearchResult(
                    state.search_selected_index.saturating_sub(1),
                ))
            } else {
                Task::none()
            }
        }

        Key::Named(iced::keyboard::key::Named::ArrowDown) => {
            if in_aggregates {
                Task::done(Message::SelectNext)
            } else if in_messages {
                let next = (state.message_selected_index + 1).min(state.messages.len().saturating_sub(1));
                Task::done(Message::SelectMessage(next))
            } else if in_search {
                let next = (state.search_selected_index + 1).min(state.search_results.len().saturating_sub(1));
                Task::done(Message::SelectSearchResult(next))
            } else {
                Task::none()
            }
        }

        // Left/Right - prev/next message in detail view
        Key::Named(iced::keyboard::key::Named::ArrowLeft) => {
            if in_detail {
                Task::done(Message::PreviousMessage)
            } else {
                Task::none()
            }
        }

        Key::Named(iced::keyboard::key::Named::ArrowRight) => {
            if in_detail {
                Task::done(Message::NextMessage)
            } else {
                Task::none()
            }
        }

        // j/k - vim-style navigation
        Key::Character(ref c) if c == "j" && !modifiers.shift() => {
            if in_aggregates {
                Task::done(Message::SelectNext)
            } else if in_messages {
                let next = (state.message_selected_index + 1).min(state.messages.len().saturating_sub(1));
                Task::done(Message::SelectMessage(next))
            } else if in_search {
                let next = (state.search_selected_index + 1).min(state.search_results.len().saturating_sub(1));
                Task::done(Message::SelectSearchResult(next))
            } else {
                Task::none()
            }
        }

        Key::Character(ref c) if c == "k" && !modifiers.shift() => {
            if in_aggregates {
                Task::done(Message::SelectPrevious)
            } else if in_messages {
                Task::done(Message::SelectMessage(
                    state.message_selected_index.saturating_sub(1),
                ))
            } else if in_search {
                Task::done(Message::SelectSearchResult(
                    state.search_selected_index.saturating_sub(1),
                ))
            } else {
                Task::none()
            }
        }

        // / - open search (not in search view)
        Key::Character(ref c) if c == "/" && !in_search => {
            Task::done(Message::OpenSearch)
        }

        // n/p - next/prev page in messages
        Key::Character(ref c) if c == "n" && !modifiers.shift() => {
            if in_messages {
                Task::done(Message::NextPage)
            } else {
                Task::none()
            }
        }

        Key::Character(ref c) if c == "p" && !modifiers.shift() => {
            if in_messages {
                Task::done(Message::PreviousPage)
            } else {
                Task::none()
            }
        }

        // s - toggle sort field (aggregates only)
        Key::Character(ref c) if c == "s" && !modifiers.shift() => {
            if in_aggregates {
                Task::done(Message::ToggleSortField)
            } else {
                Task::none()
            }
        }

        // r - toggle sort direction (aggregates only)
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
