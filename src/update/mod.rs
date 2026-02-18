//! Update logic (message handlers)
//!
//! The Update in the MVU pattern.
//! Processes Messages and returns Commands for async operations.

use crate::api::types::{DeviceFlowState, SortDirection, SortField};
use crate::api::ApiClient;
use crate::config::{discover_server, Settings};
use crate::message::Message;
use crate::model::{AppState, ConnectionStatus, LoadingState, SettingsTab, ViewLevel, WizardStep};
use iced::keyboard::{Key, Modifiers};
use iced::Task;

/// Process a message and update state
///
/// Returns a Task that may spawn async work (like API calls).
pub fn handle(state: &mut AppState, message: Message) -> Task<Message> {
    match message {
        // === Discovery ===
        Message::StartDiscovery => {
            state.discovering = true;
            state.wizard_step = WizardStep::Discovering;

            Task::perform(
                async { discover_server().await },
                Message::DiscoveryComplete,
            )
        }

        Message::DiscoveryComplete(result) => {
            state.discovering = false;
            state.discovery_steps = result.steps.clone();

            if result.found_server() {
                // Found a server - show confirmation
                state.wizard_step = WizardStep::FoundServer;
                if let Some(url) = &result.server_url {
                    state.server_url = url.clone();
                }
                if let Some(key) = &result.api_key {
                    state.api_key = key.clone();
                }
            } else {
                // No server found - show manual entry
                state.wizard_step = WizardStep::ManualEntry;
            }

            state.discovery_result = Some(result);
            Task::none()
        }

        Message::ConfirmDiscoveredServer => {
            state.wizard_step = WizardStep::Complete;
            state.first_run = false;

            // Save settings and connect
            let settings = Settings {
                server_url: state.server_url.clone(),
                api_key: state.api_key.clone(),
                allow_insecure: true,
            };
            let _ = settings.save();

            // Now connect to the server
            Task::done(Message::CheckHealth)
        }

        Message::ChooseManualEntry => {
            state.wizard_step = WizardStep::ManualEntry;
            Task::none()
        }

        Message::WizardServerUrlChanged(url) => {
            state.server_url = url;
            Task::none()
        }

        Message::WizardApiKeyChanged(key) => {
            state.api_key = key;
            Task::none()
        }

        Message::FinishWizard => {
            if state.server_url.is_empty() {
                return Task::none();
            }

            state.wizard_step = WizardStep::Complete;
            state.first_run = false;

            // Save settings
            let settings = Settings {
                server_url: state.server_url.clone(),
                api_key: state.api_key.clone(),
                allow_insecure: true,
            };
            let _ = settings.save();

            // Connect to the server
            Task::done(Message::CheckHealth)
        }

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
                // Fetch both stats AND sync status for sidebar accounts
                Task::batch([
                    Task::done(Message::FetchStats),
                    Task::done(Message::FetchSyncStatus),
                ])
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

        // === Threading ===
        Message::ViewThread(thread_id) => {
            state.thread.is_loading = true;
            state.thread.clear();
            state.navigation.push(ViewLevel::Thread {
                thread_id: thread_id.clone(),
            });

            let url = state.server_url.clone();
            let api_key = if state.api_key.is_empty() {
                None
            } else {
                Some(state.api_key.clone())
            };

            Task::perform(
                async move {
                    let client = ApiClient::new(url, api_key);
                    client.thread_messages(&thread_id).await
                },
                Message::ThreadLoaded,
            )
        }

        Message::ThreadLoaded(result) => {
            state.thread.is_loading = false;
            match result {
                Ok(messages) => {
                    if let ViewLevel::Thread { thread_id } = state.navigation.current().clone() {
                        state.thread.load_messages(thread_id, messages);
                    }
                }
                Err(e) => {
                    state.loading = LoadingState::Error(e.to_string());
                }
            }
            Task::none()
        }

        Message::ToggleThreadMessage(index) => {
            state.thread.toggle_expanded(index);
            Task::none()
        }

        Message::ExpandAllThread => {
            state.thread.expand_all();
            Task::none()
        }

        Message::CollapseAllThread => {
            state.thread.collapse_all();
            Task::none()
        }

        Message::ThreadFocusPrevious => {
            state.thread.focus_previous();
            Task::none()
        }

        Message::ThreadFocusNext => {
            state.thread.focus_next();
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

        // === Sync ===
        Message::OpenSync => {
            state.navigation.push(ViewLevel::Sync);
            // Immediately fetch sync status
            Task::done(Message::FetchSyncStatus)
        }

        Message::FetchSyncStatus => {
            state.sync_loading = true;

            let url = state.server_url.clone();
            let api_key = if state.api_key.is_empty() {
                None
            } else {
                Some(state.api_key.clone())
            };

            Task::perform(
                async move {
                    let client = ApiClient::new(url, api_key);
                    client.scheduler_status().await
                },
                Message::SyncStatusLoaded,
            )
        }

        Message::SyncStatusLoaded(result) => {
            state.sync_loading = false;
            match result {
                Ok(status) => {
                    state.sync_accounts = status.accounts;
                }
                Err(e) => {
                    state.loading = LoadingState::Error(e.to_string());
                }
            }
            Task::none()
        }

        Message::TriggerSync(email) => {
            state.syncing_account = Some(email.clone());

            let url = state.server_url.clone();
            let api_key = if state.api_key.is_empty() {
                None
            } else {
                Some(state.api_key.clone())
            };

            Task::perform(
                async move {
                    let client = ApiClient::new(url, api_key);
                    client.trigger_sync(&email).await
                },
                Message::SyncTriggered,
            )
        }

        Message::SyncTriggered(result) => {
            state.syncing_account = None;
            match result {
                Ok(_) => {
                    // Refresh status after triggering
                    return Task::done(Message::FetchSyncStatus);
                }
                Err(e) => {
                    state.loading = LoadingState::Error(e.to_string());
                }
            }
            Task::none()
        }

        Message::RefreshSyncStatus => {
            // Only refresh if we're on the sync view
            if matches!(state.navigation.current(), ViewLevel::Sync) {
                return Task::done(Message::FetchSyncStatus);
            }
            Task::none()
        }

        Message::SyncTick => {
            // Periodic sync poll: fetch latest sync status to keep sidebar accounts updated
            Task::done(Message::FetchSyncStatus)
        }

        // === Account Management ===
        Message::OpenAccounts => {
            state.navigation.push(ViewLevel::Accounts);
            // Reset add account state
            state.add_account_email.clear();
            state.adding_account = false;
            state.oauth_response = None;
            // Fetch current account list (using scheduler status)
            Task::done(Message::FetchSyncStatus)
        }

        Message::AddAccountEmailChanged(email) => {
            state.add_account_email = email;
            Task::none()
        }

        Message::StartAddAccount => {
            if state.add_account_email.is_empty() {
                return Task::none();
            }

            state.adding_account = true;
            let email = state.add_account_email.clone();

            let url = state.server_url.clone();
            let api_key = if state.api_key.is_empty() {
                None
            } else {
                Some(state.api_key.clone())
            };

            Task::perform(
                async move {
                    let client = ApiClient::new(url, api_key);
                    client.initiate_oauth(&email).await
                },
                Message::OAuthInitiated,
            )
        }

        Message::OAuthInitiated(result) => {
            match result {
                Ok(response) => {
                    state.oauth_response = Some(response.clone());
                    if response.device_flow {
                        // Start polling for device flow completion
                        state.polling_device_flow = true;
                        // Note: In a real app, we'd set up a timer subscription
                        // For now, manual polling via PollDeviceFlow message
                    } else {
                        // Open browser for OAuth
                        return Task::done(Message::OpenOAuthBrowser(response.auth_url));
                    }
                }
                Err(e) => {
                    state.adding_account = false;
                    state.loading = LoadingState::Error(e.to_string());
                }
            }
            Task::none()
        }

        Message::OpenOAuthBrowser(url) => {
            // Open URL in default browser
            #[cfg(target_os = "macos")]
            {
                let _ = std::process::Command::new("open").arg(&url).spawn();
            }
            #[cfg(target_os = "windows")]
            {
                let _ = std::process::Command::new("cmd")
                    .args(["/C", "start", "", &url])
                    .spawn();
            }
            #[cfg(target_os = "linux")]
            {
                let _ = std::process::Command::new("xdg-open").arg(&url).spawn();
            }
            Task::none()
        }

        Message::PollDeviceFlow => {
            if !state.polling_device_flow {
                return Task::none();
            }

            let email = state.add_account_email.clone();
            let url = state.server_url.clone();
            let api_key = if state.api_key.is_empty() {
                None
            } else {
                Some(state.api_key.clone())
            };

            Task::perform(
                async move {
                    let client = ApiClient::new(url, api_key);
                    client.check_device_flow(&email).await
                },
                Message::DeviceFlowStatusReceived,
            )
        }

        Message::DeviceFlowStatusReceived(result) => {
            match result {
                Ok(status) => {
                    match status.status {
                        DeviceFlowState::Complete => {
                            // Account added successfully
                            state.adding_account = false;
                            state.polling_device_flow = false;
                            state.oauth_response = None;
                            state.add_account_email.clear();
                            // Refresh account list
                            return Task::done(Message::FetchSyncStatus);
                        }
                        DeviceFlowState::Pending => {
                            // Keep polling - in a real app this would be on a timer
                        }
                        DeviceFlowState::Expired | DeviceFlowState::Error => {
                            state.adding_account = false;
                            state.polling_device_flow = false;
                            state.loading = LoadingState::Error(
                                status.error.unwrap_or_else(|| "Device flow failed".to_string()),
                            );
                        }
                    }
                }
                Err(e) => {
                    state.polling_device_flow = false;
                    state.loading = LoadingState::Error(e.to_string());
                }
            }
            Task::none()
        }

        Message::CancelAddAccount => {
            state.adding_account = false;
            state.polling_device_flow = false;
            state.oauth_response = None;
            state.add_account_email.clear();
            Task::none()
        }

        Message::ShowRemoveAccountModal(email) => {
            state.removing_account = Some(email);
            state.show_remove_modal = true;
            Task::none()
        }

        Message::HideRemoveAccountModal => {
            state.removing_account = None;
            state.show_remove_modal = false;
            Task::none()
        }

        Message::ConfirmRemoveAccount => {
            state.show_remove_modal = false;
            if let Some(email) = state.removing_account.take() {
                let url = state.server_url.clone();
                let api_key = if state.api_key.is_empty() {
                    None
                } else {
                    Some(state.api_key.clone())
                };

                return Task::perform(
                    async move {
                        let client = ApiClient::new(url, api_key);
                        client.remove_account(&email).await
                    },
                    Message::AccountRemoved,
                );
            }
            Task::none()
        }

        Message::AccountRemoved(result) => {
            match result {
                Ok(_) => {
                    // Refresh account list
                    return Task::done(Message::FetchSyncStatus);
                }
                Err(e) => {
                    state.loading = LoadingState::Error(e.to_string());
                }
            }
            Task::none()
        }

        // === Settings ===
        Message::OpenSettings => {
            // Copy current values to editable fields
            state.settings_server_url = state.server_url.clone();
            state.settings_api_key = state.api_key.clone();
            state.settings_tab = SettingsTab::Server;
            state.connection_test_result = None;
            state.navigation.push(ViewLevel::Settings);
            Task::none()
        }

        Message::SwitchSettingsTab(tab) => {
            state.settings_tab = tab;
            Task::none()
        }

        Message::SettingsServerUrlChanged(url) => {
            state.settings_server_url = url;
            state.connection_test_result = None; // Clear previous test result
            Task::none()
        }

        Message::SettingsApiKeyChanged(key) => {
            state.settings_api_key = key;
            state.connection_test_result = None;
            Task::none()
        }

        Message::TestConnection => {
            state.testing_connection = true;
            state.connection_test_result = None;

            let url = state.settings_server_url.clone();
            let api_key = if state.settings_api_key.is_empty() {
                None
            } else {
                Some(state.settings_api_key.clone())
            };

            Task::perform(
                async move {
                    let client = ApiClient::new(url, api_key);
                    client.health().await
                },
                Message::ConnectionTested,
            )
        }

        Message::ConnectionTested(result) => {
            state.testing_connection = false;
            state.connection_test_result = Some(result.map(|_| ()).map_err(|e| e.to_string()));
            Task::none()
        }

        Message::SaveSettings => {
            // Update the app state with new values
            state.server_url = state.settings_server_url.clone();
            state.api_key = state.settings_api_key.clone();

            // Save to config file
            let settings = Settings {
                server_url: state.server_url.clone(),
                api_key: state.api_key.clone(),
                allow_insecure: true, // Allow HTTP for local development
            };

            Task::perform(
                async move { settings.save() },
                Message::SettingsSaved,
            )
        }

        Message::SettingsSaved(result) => {
            match result {
                Ok(_) => {
                    // Go back to previous view
                    state.navigation.pop();
                }
                Err(e) => {
                    state.loading = LoadingState::Error(format!("Failed to save settings: {}", e));
                }
            }
            Task::none()
        }

        // === Help ===
        Message::ShowHelp => {
            state.show_help_modal = true;
            Task::none()
        }

        Message::HideHelp => {
            state.show_help_modal = false;
            Task::none()
        }

        // === Selection ===
        Message::ToggleSelection => {
            // Toggle selection based on current view
            let message_id = match state.navigation.current() {
                ViewLevel::Messages { .. } => {
                    state.messages.get(state.message_selected_index).map(|m| m.id)
                }
                ViewLevel::Search => {
                    state.search_results.get(state.search_selected_index).map(|m| m.id)
                }
                _ => None,
            };

            if let Some(id) = message_id {
                if state.selected_messages.contains(&id) {
                    state.selected_messages.remove(&id);
                } else {
                    state.selected_messages.insert(id);
                }
            }
            Task::none()
        }

        Message::SelectAll => {
            // Select all visible messages based on current view
            match state.navigation.current() {
                ViewLevel::Messages { .. } => {
                    for msg in &state.messages {
                        state.selected_messages.insert(msg.id);
                    }
                }
                ViewLevel::Search => {
                    for msg in &state.search_results {
                        state.selected_messages.insert(msg.id);
                    }
                }
                _ => {}
            }
            Task::none()
        }

        Message::ClearSelection => {
            state.selected_messages.clear();
            Task::none()
        }

        Message::ShowDeleteModal => {
            if !state.selected_messages.is_empty() {
                state.show_delete_modal = true;
            }
            Task::none()
        }

        Message::HideDeleteModal => {
            state.show_delete_modal = false;
            Task::none()
        }

        Message::ConfirmDelete => {
            state.show_delete_modal = false;
            // Trigger staging for deletion
            Task::done(Message::StageForDeletion)
        }

        Message::StageForDeletion => {
            // TODO: Phase 6 server endpoint - POST /api/v1/deletion/stage
            // For now, just clear selection as a placeholder
            let count = state.selected_messages.len();
            state.selected_messages.clear();
            // Log to console for now (will be replaced with API call)
            #[cfg(debug_assertions)]
            println!("Staged {} messages for deletion", count);
            let _ = count; // suppress unused warning in release
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

        // === Attachments ===
        Message::DownloadAttachment {
            message_id,
            attachment_idx,
            filename,
        } => {
            // Mark as downloading
            state.downloads.set_downloading(message_id, attachment_idx, 0.0);

            let url = state.server_url.clone();
            let api_key = if state.api_key.is_empty() {
                None
            } else {
                Some(state.api_key.clone())
            };

            Task::perform(
                async move {
                    let client = reqwest::Client::builder()
                        .timeout(std::time::Duration::from_secs(300))
                        .build()
                        .expect("Failed to create HTTP client");

                    crate::api::download_attachment(
                        &client,
                        &url,
                        api_key.as_deref(),
                        message_id,
                        attachment_idx,
                        &filename,
                    )
                    .await
                },
                move |result| match result {
                    Ok(path) => Message::DownloadComplete {
                        message_id,
                        attachment_idx,
                        path,
                    },
                    Err(e) => Message::DownloadFailed {
                        message_id,
                        attachment_idx,
                        error: e.to_string(),
                    },
                },
            )
        }

        Message::DownloadProgress {
            message_id,
            attachment_idx,
            progress,
        } => {
            state.downloads.set_downloading(message_id, attachment_idx, progress);
            Task::none()
        }

        Message::DownloadComplete {
            message_id,
            attachment_idx,
            path,
        } => {
            state.downloads.set_complete(message_id, attachment_idx, path);
            Task::none()
        }

        Message::DownloadFailed {
            message_id,
            attachment_idx,
            error,
        } => {
            state.downloads.set_failed(message_id, attachment_idx, error);
            Task::none()
        }

        Message::OpenFile(path) => {
            // Open file with default application
            let _ = open::that(&path);
            Task::none()
        }

        // === Compose ===
        Message::OpenCompose => {
            // Get first account email for the from field
            let from_account = state
                .sync_accounts
                .first()
                .map(|a| a.email.clone())
                .unwrap_or_default();
            state.compose = crate::model::ComposeState::open_new(from_account);
            Task::none()
        }

        Message::OpenReply(message_id) => {
            // TODO: Fetch message detail and populate reply
            // For now, use current message if available
            if let Some(msg) = &state.current_message {
                let from_account = state
                    .sync_accounts
                    .first()
                    .map(|a| a.email.clone())
                    .unwrap_or_default();
                let quoted = crate::model::compose::format_quoted_body(
                    &msg.from_addr,
                    &msg.sent_at.format("%b %d, %Y at %H:%M").to_string(),
                    &msg.body,
                );
                state.compose = crate::model::ComposeState::open_reply(
                    from_account,
                    message_id,
                    msg.from_addr.clone(),
                    msg.subject.clone(),
                    quoted,
                );
            }
            Task::none()
        }

        Message::OpenReplyAll(message_id) => {
            if let Some(msg) = &state.current_message {
                let from_account = state
                    .sync_accounts
                    .first()
                    .map(|a| a.email.clone())
                    .unwrap_or_default();
                let quoted = crate::model::compose::format_quoted_body(
                    &msg.from_addr,
                    &msg.sent_at.format("%b %d, %Y at %H:%M").to_string(),
                    &msg.body,
                );
                // Combine to and cc, removing our own email
                let mut all_recipients: Vec<String> = msg.to.clone();
                all_recipients.push(msg.from_addr.clone());
                all_recipients.retain(|e| e != &from_account);
                let cc = msg.cc.iter().filter(|e| *e != &from_account).cloned().collect();

                state.compose = crate::model::ComposeState::open_reply_all(
                    from_account,
                    message_id,
                    all_recipients,
                    cc,
                    msg.subject.clone(),
                    quoted,
                );
            }
            Task::none()
        }

        Message::OpenForward(message_id) => {
            if let Some(msg) = &state.current_message {
                let from_account = state
                    .sync_accounts
                    .first()
                    .map(|a| a.email.clone())
                    .unwrap_or_default();
                let forward_body = format!(
                    "From: {}\nDate: {}\nSubject: {}\nTo: {}\n\n{}",
                    msg.from_addr,
                    msg.sent_at.format("%b %d, %Y at %H:%M"),
                    msg.subject,
                    msg.to.join(", "),
                    msg.body
                );
                state.compose = crate::model::ComposeState::open_forward(
                    from_account,
                    message_id,
                    msg.subject.clone(),
                    forward_body,
                );
            }
            Task::none()
        }

        Message::ComposeToChanged(input) => {
            state.compose.to_input = input;
            Task::none()
        }

        Message::ComposeAddTo => {
            let email = state.compose.to_input.trim().to_string();
            state.compose.add_to(email);
            state.compose.to_input.clear();
            Task::none()
        }

        Message::ComposeRemoveTo(index) => {
            state.compose.remove_to(index);
            Task::none()
        }

        Message::ComposeCcChanged(input) => {
            state.compose.cc_input = input;
            Task::none()
        }

        Message::ComposeAddCc => {
            let email = state.compose.cc_input.trim().to_string();
            state.compose.add_cc(email);
            state.compose.cc_input.clear();
            Task::none()
        }

        Message::ComposeRemoveCc(index) => {
            state.compose.remove_cc(index);
            Task::none()
        }

        Message::ComposeBccChanged(input) => {
            state.compose.bcc_input = input;
            Task::none()
        }

        Message::ComposeAddBcc => {
            let email = state.compose.bcc_input.trim().to_string();
            state.compose.add_bcc(email);
            state.compose.bcc_input.clear();
            Task::none()
        }

        Message::ComposeRemoveBcc(index) => {
            state.compose.remove_bcc(index);
            Task::none()
        }

        Message::ComposeSubjectChanged(subject) => {
            state.compose.subject = subject;
            state.compose.is_dirty = true;
            Task::none()
        }

        Message::ComposeBodyChanged(body) => {
            state.compose.body = body;
            state.compose.is_dirty = true;
            Task::none()
        }

        Message::ComposeFromChanged(account) => {
            state.compose.from_account = account;
            Task::none()
        }

        Message::ComposeToggleCcBcc => {
            state.compose.show_cc_bcc = !state.compose.show_cc_bcc;
            Task::none()
        }

        Message::ComposeAddAttachment => {
            // TODO: Open file picker dialog
            // This would require native file dialog integration
            Task::none()
        }

        Message::ComposeAttachmentSelected(path) => {
            if let Ok(metadata) = std::fs::metadata(&path) {
                let filename = path
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| "attachment".to_string());
                state.compose.attachments.push(crate::model::AttachmentDraft {
                    path,
                    filename,
                    size_bytes: metadata.len() as i64,
                    mime_type: None,
                });
                state.compose.is_dirty = true;
            }
            Task::none()
        }

        Message::ComposeRemoveAttachment(index) => {
            if index < state.compose.attachments.len() {
                state.compose.attachments.remove(index);
                state.compose.is_dirty = true;
            }
            Task::none()
        }

        Message::ComposeSend => {
            if !state.compose.can_send() {
                return Task::none();
            }
            state.compose.is_sending = true;
            state.compose.send_error = None;

            // TODO: Implement actual send via API
            // POST /api/v1/messages/send
            // For now, just simulate success after a delay
            Task::none()
        }

        Message::ComposeSent(result) => {
            state.compose.is_sending = false;
            match result {
                Ok(_) => {
                    state.compose.close();
                }
                Err(e) => {
                    state.compose.send_error = Some(e.to_string());
                }
            }
            Task::none()
        }

        Message::ComposeSaveDraft => {
            // TODO: Implement draft saving via API
            // POST /api/v1/messages/draft
            Task::none()
        }

        Message::ComposeDraftSaved(result) => {
            match result {
                Ok(_draft_id) => {
                    state.compose.is_dirty = false;
                    // Optionally close or show confirmation
                }
                Err(e) => {
                    state.compose.send_error = Some(format!("Failed to save draft: {}", e));
                }
            }
            Task::none()
        }

        Message::ComposeDiscard => {
            state.compose.close();
            Task::none()
        }

        Message::ComposeClose => {
            if state.compose.is_dirty && state.compose.has_content() {
                // TODO: Show confirmation dialog
                // For now, just close
                state.compose.close();
            } else {
                state.compose.close();
            }
            Task::none()
        }

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
    let in_thread = matches!(state.navigation.current(), ViewLevel::Thread { .. });
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

        // Enter - drill down, open message, open search result, or toggle thread message
        Key::Named(iced::keyboard::key::Named::Enter) => {
            if in_aggregates {
                Task::done(Message::DrillDown)
            } else if in_messages {
                Task::done(Message::OpenMessage)
            } else if in_search {
                Task::done(Message::OpenSearchResult)
            } else if in_thread {
                // Toggle expand/collapse of focused message
                Task::done(Message::ToggleThreadMessage(state.thread.focused_index))
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
            } else if in_thread {
                Task::done(Message::ThreadFocusPrevious)
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
            } else if in_thread {
                Task::done(Message::ThreadFocusNext)
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
            } else if in_thread {
                Task::done(Message::ThreadFocusNext)
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
            } else if in_thread {
                Task::done(Message::ThreadFocusPrevious)
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

        // ? - help
        Key::Character(ref c) if c == "?" => {
            if state.show_help_modal {
                Task::done(Message::HideHelp)
            } else {
                Task::done(Message::ShowHelp)
            }
        }

        // Space - toggle selection (messages/search)
        Key::Named(iced::keyboard::key::Named::Space) => {
            if in_messages || in_search {
                Task::done(Message::ToggleSelection)
            } else {
                Task::none()
            }
        }

        // A (shift) - select all visible
        Key::Character(ref c) if c == "A" && modifiers.shift() => {
            if in_messages || in_search {
                Task::done(Message::SelectAll)
            } else {
                Task::none()
            }
        }

        // x - clear selection
        Key::Character(ref c) if c == "x" && !modifiers.shift() => {
            Task::done(Message::ClearSelection)
        }

        // d - show delete confirmation for selected
        Key::Character(ref c) if c == "d" && !modifiers.shift() => {
            if !state.selected_messages.is_empty() {
                Task::done(Message::ShowDeleteModal)
            } else {
                Task::none()
            }
        }

        // y - open sync status view (sYnc)
        Key::Character(ref c) if c == "y" && !modifiers.shift() => {
            Task::done(Message::OpenSync)
        }

        // a - open accounts view
        Key::Character(ref c) if c == "a" && !modifiers.shift() => {
            Task::done(Message::OpenAccounts)
        }

        // comma - open settings (standard macOS shortcut)
        Key::Character(ref c) if c == "," => {
            Task::done(Message::OpenSettings)
        }

        // c - compose new message
        Key::Character(ref c) if c == "c" && !modifiers.shift() && !state.compose.is_open => {
            Task::done(Message::OpenCompose)
        }

        // r - reply (when viewing message detail)
        Key::Character(ref c) if c == "r" && !modifiers.shift() && in_detail => {
            if let ViewLevel::MessageDetail { message_id } = state.navigation.current() {
                Task::done(Message::OpenReply(*message_id))
            } else {
                Task::none()
            }
        }

        // R (shift) - reply all (when viewing message detail)
        Key::Character(ref c) if c == "R" && modifiers.shift() && in_detail => {
            if let ViewLevel::MessageDetail { message_id } = state.navigation.current() {
                Task::done(Message::OpenReplyAll(*message_id))
            } else {
                Task::none()
            }
        }

        // f - forward (when viewing message detail)
        Key::Character(ref c) if c == "f" && !modifiers.shift() && in_detail => {
            if let ViewLevel::MessageDetail { message_id } = state.navigation.current() {
                Task::done(Message::OpenForward(*message_id))
            } else {
                Task::none()
            }
        }

        // t - view full thread (when viewing message detail)
        Key::Character(ref c) if c == "t" && !modifiers.shift() && in_detail => {
            if let Some(msg) = &state.current_message {
                if let Some(thread_id) = &msg.thread_id {
                    Task::done(Message::ViewThread(thread_id.clone()))
                } else {
                    Task::none()
                }
            } else {
                Task::none()
            }
        }

        // e - expand all (in thread view)
        Key::Character(ref c) if c == "e" && !modifiers.shift() && in_thread => {
            Task::done(Message::ExpandAllThread)
        }

        // E (shift+e) - collapse all (in thread view)
        Key::Character(ref c) if c == "E" && modifiers.shift() && in_thread => {
            Task::done(Message::CollapseAllThread)
        }

        _ => Task::none(),
    }
}
