//! HTTP API client
//!
//! Handles all HTTP communication with the msgvault server.

use crate::api::types::{
    AggregateResponse, DeviceFlowStatus, HealthResponse, MessageDetail, MessageListResponse,
    OAuthInitResponse, RemoveAccountResponse, SchedulerStatus, SearchResponse, SortDirection,
    SortField, StatsResponse, SyncTriggerResponse, ThreadResponse, ViewType,
};
use crate::error::AppError;
use reqwest::Client;
use std::time::Duration;

/// API client for msgvault server
#[derive(Debug, Clone)]
pub struct ApiClient {
    client: Client,
    base_url: String,
    api_key: Option<String>,
}

impl ApiClient {
    /// Create a new API client
    pub fn new(base_url: impl Into<String>, api_key: Option<String>) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url: base_url.into().trim_end_matches('/').to_string(),
            api_key,
        }
    }

    /// Build a request with authentication headers
    fn request(&self, method: reqwest::Method, path: &str) -> reqwest::RequestBuilder {
        let url = format!("{}{}", self.base_url, path);
        let mut req = self.client.request(method, &url);

        if let Some(ref key) = self.api_key {
            req = req.header("X-API-Key", key);
        }

        req.header("Accept", "application/json")
    }

    /// Check server health
    ///
    /// This is the first request made on startup to verify connectivity.
    pub async fn health(&self) -> Result<HealthResponse, AppError> {
        let response = self.request(reqwest::Method::GET, "/health").send().await?;

        if !response.status().is_success() {
            return Err(AppError::ApiError {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            });
        }

        let health: HealthResponse = response.json().await.map_err(|e| AppError::ApiError {
            status: 0,
            message: format!("Invalid response: {}", e),
        })?;

        Ok(health)
    }

    /// Fetch archive statistics
    ///
    /// Returns total messages, threads, accounts, labels, attachments, and database size.
    pub async fn stats(&self) -> Result<StatsResponse, AppError> {
        let response = self
            .request(reqwest::Method::GET, "/api/v1/stats")
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(AppError::ApiError {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            });
        }

        let stats: StatsResponse = response.json().await.map_err(|e| AppError::ApiError {
            status: 0,
            message: format!("Invalid stats response: {}", e),
        })?;

        Ok(stats)
    }

    /// Fetch aggregate data for a given view type
    ///
    /// Returns aggregate rows grouped by the specified view type (senders, domains, etc.)
    pub async fn aggregates(
        &self,
        view_type: ViewType,
        sort_field: SortField,
        sort_dir: SortDirection,
    ) -> Result<AggregateResponse, AppError> {
        let path = format!(
            "/api/v1/aggregates?view_type={}&sort={}&order={}",
            view_type.as_str(),
            sort_field.as_str(),
            sort_dir.as_str()
        );

        let response = self.request(reqwest::Method::GET, &path).send().await?;

        if !response.status().is_success() {
            return Err(AppError::ApiError {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            });
        }

        let aggregates: AggregateResponse =
            response.json().await.map_err(|e| AppError::ApiError {
                status: 0,
                message: format!("Invalid aggregates response: {}", e),
            })?;

        Ok(aggregates)
    }

    /// Fetch filtered messages
    ///
    /// Returns paginated messages matching the specified filter criteria.
    pub async fn messages_filter(
        &self,
        filter_type: &str,
        filter_value: &str,
        offset: i64,
        limit: i64,
    ) -> Result<MessageListResponse, AppError> {
        let response = self
            .request(reqwest::Method::GET, "/api/v1/messages/filter")
            .query(&[
                ("type", filter_type),
                ("value", filter_value),
                ("offset", &offset.to_string()),
                ("limit", &limit.to_string()),
            ])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(AppError::ApiError {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            });
        }

        let messages: MessageListResponse =
            response.json().await.map_err(|e| AppError::ApiError {
                status: 0,
                message: format!("Invalid messages response: {}", e),
            })?;

        Ok(messages)
    }

    /// Fetch message detail
    ///
    /// Returns full details for a single message by ID.
    pub async fn message_detail(&self, message_id: i64) -> Result<MessageDetail, AppError> {
        let path = format!("/api/v1/messages/{}", message_id);

        let response = self.request(reqwest::Method::GET, &path).send().await?;

        if !response.status().is_success() {
            return Err(AppError::ApiError {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            });
        }

        let detail: MessageDetail = response.json().await.map_err(|e| AppError::ApiError {
            status: 0,
            message: format!("Invalid message detail response: {}", e),
        })?;

        Ok(detail)
    }

    /// Fast search (subject/from only)
    ///
    /// Searches message subjects and sender fields for quick results.
    pub async fn search_fast(
        &self,
        query: &str,
        limit: i64,
    ) -> Result<SearchResponse, AppError> {
        let response = self
            .request(reqwest::Method::GET, "/api/v1/search/fast")
            .query(&[("q", query), ("limit", &limit.to_string())])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(AppError::ApiError {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            });
        }

        let search: SearchResponse = response.json().await.map_err(|e| AppError::ApiError {
            status: 0,
            message: format!("Invalid search response: {}", e),
        })?;

        Ok(search)
    }

    /// Deep search (full text)
    ///
    /// Performs full-text search across all message content.
    pub async fn search_deep(
        &self,
        query: &str,
        offset: i64,
        limit: i64,
    ) -> Result<SearchResponse, AppError> {
        let response = self
            .request(reqwest::Method::GET, "/api/v1/search/deep")
            .query(&[
                ("q", query),
                ("offset", &offset.to_string()),
                ("limit", &limit.to_string()),
            ])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(AppError::ApiError {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            });
        }

        let search: SearchResponse = response.json().await.map_err(|e| AppError::ApiError {
            status: 0,
            message: format!("Invalid search response: {}", e),
        })?;

        Ok(search)
    }

    /// Fetch scheduler status for all accounts
    ///
    /// Returns sync status for all configured email accounts.
    pub async fn scheduler_status(&self) -> Result<SchedulerStatus, AppError> {
        let response = self
            .request(reqwest::Method::GET, "/api/v1/scheduler/status")
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(AppError::ApiError {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            });
        }

        let status: SchedulerStatus = response.json().await.map_err(|e| AppError::ApiError {
            status: 0,
            message: format!("Invalid scheduler status response: {}", e),
        })?;

        Ok(status)
    }

    /// Trigger sync for a specific account
    ///
    /// Starts a manual sync for the specified email account.
    pub async fn trigger_sync(&self, email: &str) -> Result<SyncTriggerResponse, AppError> {
        let path = format!("/api/v1/sync/{}", urlencoding::encode(email));

        let response = self
            .request(reqwest::Method::POST, &path)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(AppError::ApiError {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            });
        }

        let result: SyncTriggerResponse =
            response.json().await.map_err(|e| AppError::ApiError {
                status: 0,
                message: format!("Invalid sync trigger response: {}", e),
            })?;

        Ok(result)
    }

    /// Initiate OAuth flow for adding an account
    ///
    /// Returns URL to open in browser or device flow info.
    pub async fn initiate_oauth(&self, email: &str) -> Result<OAuthInitResponse, AppError> {
        let path = format!("/api/v1/auth/initiate/{}", urlencoding::encode(email));

        let response = self
            .request(reqwest::Method::POST, &path)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(AppError::ApiError {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            });
        }

        let result: OAuthInitResponse =
            response.json().await.map_err(|e| AppError::ApiError {
                status: 0,
                message: format!("Invalid OAuth init response: {}", e),
            })?;

        Ok(result)
    }

    /// Check device flow status
    ///
    /// Polls to see if user has completed device flow authorization.
    pub async fn check_device_flow(&self, email: &str) -> Result<DeviceFlowStatus, AppError> {
        let path = format!(
            "/api/v1/auth/device/{}/status",
            urlencoding::encode(email)
        );

        let response = self.request(reqwest::Method::GET, &path).send().await?;

        if !response.status().is_success() {
            return Err(AppError::ApiError {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            });
        }

        let result: DeviceFlowStatus =
            response.json().await.map_err(|e| AppError::ApiError {
                status: 0,
                message: format!("Invalid device flow status response: {}", e),
            })?;

        Ok(result)
    }

    /// Remove an account
    ///
    /// Deletes the specified email account from the server.
    pub async fn remove_account(&self, email: &str) -> Result<RemoveAccountResponse, AppError> {
        let path = format!("/api/v1/accounts/{}", urlencoding::encode(email));

        let response = self
            .request(reqwest::Method::DELETE, &path)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(AppError::ApiError {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            });
        }

        let result: RemoveAccountResponse =
            response.json().await.map_err(|e| AppError::ApiError {
                status: 0,
                message: format!("Invalid remove account response: {}", e),
            })?;

        Ok(result)
    }

    /// Fetch all messages in a thread
    ///
    /// Returns all messages that belong to the specified thread, ordered chronologically.
    pub async fn thread_messages(&self, thread_id: &str) -> Result<Vec<MessageDetail>, AppError> {
        let path = format!("/api/v1/threads/{}", urlencoding::encode(thread_id));

        let response = self.request(reqwest::Method::GET, &path).send().await?;

        if !response.status().is_success() {
            return Err(AppError::ApiError {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            });
        }

        let thread_response: ThreadResponse =
            response.json().await.map_err(|e| AppError::ApiError {
                status: 0,
                message: format!("Invalid thread response: {}", e),
            })?;

        Ok(thread_response.messages)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = ApiClient::new("http://localhost:8080", None);
        assert_eq!(client.base_url, "http://localhost:8080");
    }

    #[test]
    fn test_client_trims_trailing_slash() {
        let client = ApiClient::new("http://localhost:8080/", None);
        assert_eq!(client.base_url, "http://localhost:8080");
    }
}
