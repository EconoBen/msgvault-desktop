//! Download state management
//!
//! Tracks download progress and status for message attachments.

use std::collections::HashMap;
use std::path::PathBuf;

/// Download state for a single attachment
#[derive(Debug, Clone)]
pub enum DownloadState {
    /// Not yet started
    NotStarted,
    /// Currently downloading
    Downloading { progress: f32 }, // 0.0 to 1.0
    /// Download complete
    Complete { path: PathBuf },
    /// Download failed
    Failed { error: String },
}

impl Default for DownloadState {
    fn default() -> Self {
        Self::NotStarted
    }
}

impl DownloadState {
    /// Check if download is in progress
    pub fn is_downloading(&self) -> bool {
        matches!(self, Self::Downloading { .. })
    }

    /// Check if download is complete
    pub fn is_complete(&self) -> bool {
        matches!(self, Self::Complete { .. })
    }

    /// Check if download failed
    pub fn is_failed(&self) -> bool {
        matches!(self, Self::Failed { .. })
    }

    /// Get the download path if complete
    pub fn path(&self) -> Option<&PathBuf> {
        match self {
            Self::Complete { path } => Some(path),
            _ => None,
        }
    }

    /// Get the progress if downloading
    pub fn progress(&self) -> Option<f32> {
        match self {
            Self::Downloading { progress } => Some(*progress),
            _ => None,
        }
    }

    /// Get the error message if failed
    pub fn error(&self) -> Option<&str> {
        match self {
            Self::Failed { error } => Some(error),
            _ => None,
        }
    }
}

/// Tracks download state for all attachments
#[derive(Debug, Clone, Default)]
pub struct DownloadTracker {
    /// Map of (message_id, attachment_index) -> download state
    pub downloads: HashMap<(i64, usize), DownloadState>,
}

impl DownloadTracker {
    /// Create a new download tracker
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the download state for an attachment
    pub fn get(&self, message_id: i64, attachment_idx: usize) -> &DownloadState {
        static NOT_STARTED: DownloadState = DownloadState::NotStarted;
        self.downloads
            .get(&(message_id, attachment_idx))
            .unwrap_or(&NOT_STARTED)
    }

    /// Set an attachment as downloading with progress
    pub fn set_downloading(&mut self, message_id: i64, attachment_idx: usize, progress: f32) {
        self.downloads.insert(
            (message_id, attachment_idx),
            DownloadState::Downloading { progress: progress.clamp(0.0, 1.0) },
        );
    }

    /// Set an attachment download as complete
    pub fn set_complete(&mut self, message_id: i64, attachment_idx: usize, path: PathBuf) {
        self.downloads.insert(
            (message_id, attachment_idx),
            DownloadState::Complete { path },
        );
    }

    /// Set an attachment download as failed
    pub fn set_failed(&mut self, message_id: i64, attachment_idx: usize, error: String) {
        self.downloads.insert(
            (message_id, attachment_idx),
            DownloadState::Failed { error },
        );
    }

    /// Clear the download state for an attachment
    pub fn clear(&mut self, message_id: i64, attachment_idx: usize) {
        self.downloads.remove(&(message_id, attachment_idx));
    }

    /// Clear all downloads for a message
    pub fn clear_message(&mut self, message_id: i64) {
        self.downloads
            .retain(|(msg_id, _), _| *msg_id != message_id);
    }

    /// Clear all downloads
    pub fn clear_all(&mut self) {
        self.downloads.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_download_state_default() {
        let state = DownloadState::default();
        assert!(matches!(state, DownloadState::NotStarted));
    }

    #[test]
    fn test_download_tracker_basic() {
        let mut tracker = DownloadTracker::new();

        // Should return NotStarted for unknown downloads
        let state = tracker.get(1, 0);
        assert!(matches!(state, DownloadState::NotStarted));

        // Set downloading
        tracker.set_downloading(1, 0, 0.5);
        let state = tracker.get(1, 0);
        assert!(state.is_downloading());
        assert_eq!(state.progress(), Some(0.5));

        // Set complete
        tracker.set_complete(1, 0, PathBuf::from("/tmp/test.pdf"));
        let state = tracker.get(1, 0);
        assert!(state.is_complete());
        assert_eq!(state.path(), Some(&PathBuf::from("/tmp/test.pdf")));
    }

    #[test]
    fn test_download_tracker_failed() {
        let mut tracker = DownloadTracker::new();

        tracker.set_failed(1, 0, "Network error".to_string());
        let state = tracker.get(1, 0);
        assert!(state.is_failed());
        assert_eq!(state.error(), Some("Network error"));
    }

    #[test]
    fn test_download_tracker_clear() {
        let mut tracker = DownloadTracker::new();

        tracker.set_downloading(1, 0, 0.5);
        tracker.set_downloading(1, 1, 0.3);
        tracker.set_downloading(2, 0, 0.7);

        // Clear single download
        tracker.clear(1, 0);
        assert!(matches!(tracker.get(1, 0), DownloadState::NotStarted));
        assert!(tracker.get(1, 1).is_downloading());

        // Clear all downloads for message
        tracker.clear_message(1);
        assert!(matches!(tracker.get(1, 1), DownloadState::NotStarted));
        assert!(tracker.get(2, 0).is_downloading());
    }
}
