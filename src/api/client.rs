//! HTTP API client
//!
//! Handles all HTTP communication with the msgvault server.

use crate::api::types::{
    AggregateResponse, HealthResponse, SortDirection, SortField, StatsResponse, ViewType,
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
