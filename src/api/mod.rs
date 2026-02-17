//! API client module
//!
//! HTTP client for communicating with the msgvault server.

pub mod attachments;
pub mod client;
pub mod types;

pub use attachments::download_attachment;
pub use client::ApiClient;
