//! Error types for msgvault-desktop

use thiserror::Error;

/// Application-level errors
#[derive(Error, Debug, Clone)]
pub enum AppError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    #[error("API error ({status}): {message}")]
    ApiError { status: u16, message: String },

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Request failed: {0}")]
    RequestFailed(String),
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_connect() {
            AppError::ConnectionFailed(err.to_string())
        } else {
            AppError::RequestFailed(err.to_string())
        }
    }
}
