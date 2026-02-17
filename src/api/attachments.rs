//! Attachment download API
//!
//! Handles downloading attachments from the msgvault server.

use crate::error::AppError;
use reqwest::Client;
use std::path::PathBuf;
use std::time::Duration;
use tokio::io::AsyncWriteExt;

/// Download an attachment from the server
///
/// # Arguments
/// * `client` - HTTP client for making requests
/// * `base_url` - Base URL of the msgvault server
/// * `api_key` - Optional API key for authentication
/// * `message_id` - ID of the message containing the attachment
/// * `attachment_idx` - Index of the attachment in the message
/// * `filename` - Filename to save the attachment as
///
/// # Returns
/// The path where the attachment was saved
pub async fn download_attachment(
    client: &Client,
    base_url: &str,
    api_key: Option<&str>,
    message_id: i64,
    attachment_idx: usize,
    filename: &str,
) -> Result<PathBuf, AppError> {
    // Build the download URL
    let url = format!(
        "{}/api/v1/messages/{}/attachments/{}",
        base_url.trim_end_matches('/'),
        message_id,
        attachment_idx
    );

    // Build the request with optional auth
    let mut request = client
        .get(&url)
        .timeout(Duration::from_secs(300)); // 5 minute timeout for large files

    if let Some(key) = api_key {
        request = request.header("X-API-Key", key);
    }

    // Send the request
    let response = request.send().await?;

    if !response.status().is_success() {
        return Err(AppError::ApiError {
            status: response.status().as_u16(),
            message: format!("Failed to download attachment: {}", response.status()),
        });
    }

    // Get the Downloads directory
    let downloads_dir = dirs::download_dir()
        .unwrap_or_else(|| dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")));

    // Sanitize the filename
    let safe_filename = sanitize_filename(filename);
    let mut download_path = downloads_dir.join(&safe_filename);

    // Handle duplicate filenames
    download_path = unique_path(download_path);

    // Download the file
    let bytes = response.bytes().await?;

    // Write to file
    let mut file = tokio::fs::File::create(&download_path).await.map_err(|e| {
        AppError::ConfigError(format!("Failed to create file: {}", e))
    })?;

    file.write_all(&bytes).await.map_err(|e| {
        AppError::ConfigError(format!("Failed to write file: {}", e))
    })?;

    file.flush().await.map_err(|e| {
        AppError::ConfigError(format!("Failed to flush file: {}", e))
    })?;

    Ok(download_path)
}

/// Sanitize a filename to remove potentially dangerous characters
fn sanitize_filename(filename: &str) -> String {
    // Remove path separators and other dangerous characters
    filename
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            c if c.is_control() => '_',
            c => c,
        })
        .collect::<String>()
        .trim()
        .to_string()
}

/// Generate a unique path if the file already exists
fn unique_path(path: PathBuf) -> PathBuf {
    if !path.exists() {
        return path;
    }

    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("attachment");
    let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");
    let parent = path.parent().unwrap_or_else(|| std::path::Path::new("."));

    for i in 1..1000 {
        let new_name = if extension.is_empty() {
            format!("{} ({})", stem, i)
        } else {
            format!("{} ({}).{}", stem, i, extension)
        };
        let new_path = parent.join(new_name);
        if !new_path.exists() {
            return new_path;
        }
    }

    // Fallback: use timestamp
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let new_name = if extension.is_empty() {
        format!("{}_{}", stem, timestamp)
    } else {
        format!("{}_{}.{}", stem, timestamp, extension)
    };
    parent.join(new_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("test.pdf"), "test.pdf");
        assert_eq!(sanitize_filename("test/file.pdf"), "test_file.pdf");
        assert_eq!(sanitize_filename("test\\file.pdf"), "test_file.pdf");
        assert_eq!(sanitize_filename("test:file.pdf"), "test_file.pdf");
        assert_eq!(sanitize_filename("test*file?.pdf"), "test_file_.pdf");
        assert_eq!(sanitize_filename("  test.pdf  "), "test.pdf");
    }

    #[test]
    fn test_unique_path_new_file() {
        let path = PathBuf::from("/tmp/nonexistent_test_file_12345.pdf");
        assert_eq!(unique_path(path.clone()), path);
    }
}
