//! Thread/conversation state
//!
//! Manages the state for viewing email threads/conversations.

use crate::api::types::MessageDetail;

/// State for viewing a thread/conversation
#[derive(Debug, Clone, Default)]
pub struct ThreadState {
    /// Thread ID (from Gmail)
    pub thread_id: Option<String>,
    /// All messages in the thread, ordered chronologically
    pub messages: Vec<MessageDetail>,
    /// Which messages are expanded (by index)
    pub expanded: Vec<bool>,
    /// Currently focused message index
    pub focused_index: usize,
    /// Loading state
    pub is_loading: bool,
}

impl ThreadState {
    /// Create a new empty thread state
    pub fn new() -> Self {
        Self::default()
    }

    /// Load messages into the thread state
    pub fn load_messages(&mut self, thread_id: String, messages: Vec<MessageDetail>) {
        self.thread_id = Some(thread_id);
        // Initialize all messages as collapsed except the last one
        let len = messages.len();
        self.expanded = vec![false; len];
        if len > 0 {
            self.expanded[len - 1] = true; // Expand the most recent message
            self.focused_index = len - 1;
        }
        self.messages = messages;
        self.is_loading = false;
    }

    /// Toggle the expanded state of a message at the given index
    pub fn toggle_expanded(&mut self, index: usize) {
        if index < self.expanded.len() {
            self.expanded[index] = !self.expanded[index];
        }
    }

    /// Expand all messages in the thread
    pub fn expand_all(&mut self) {
        for exp in &mut self.expanded {
            *exp = true;
        }
    }

    /// Collapse all messages in the thread
    pub fn collapse_all(&mut self) {
        for exp in &mut self.expanded {
            *exp = false;
        }
    }

    /// Move focus to the previous message
    pub fn focus_previous(&mut self) {
        if self.focused_index > 0 {
            self.focused_index -= 1;
        }
    }

    /// Move focus to the next message
    pub fn focus_next(&mut self) {
        if self.focused_index + 1 < self.messages.len() {
            self.focused_index += 1;
        }
    }

    /// Check if a message at the given index is expanded
    pub fn is_expanded(&self, index: usize) -> bool {
        self.expanded.get(index).copied().unwrap_or(false)
    }

    /// Get the number of messages in the thread
    pub fn message_count(&self) -> usize {
        self.messages.len()
    }

    /// Clear the thread state
    pub fn clear(&mut self) {
        self.thread_id = None;
        self.messages.clear();
        self.expanded.clear();
        self.focused_index = 0;
        self.is_loading = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn mock_message(id: i64) -> MessageDetail {
        MessageDetail {
            id,
            subject: format!("Message {}", id),
            from_addr: "test@example.com".to_string(),
            to: vec!["recipient@example.com".to_string()],
            cc: vec![],
            bcc: vec![],
            sent_at: Utc::now(),
            body: format!("Body of message {}", id),
            labels: vec![],
            attachments: vec![],
            thread_id: Some("thread123".to_string()),
        }
    }

    #[test]
    fn test_new_thread_state() {
        let state = ThreadState::new();
        assert!(state.thread_id.is_none());
        assert!(state.messages.is_empty());
        assert_eq!(state.focused_index, 0);
        assert!(!state.is_loading);
    }

    #[test]
    fn test_load_messages() {
        let mut state = ThreadState::new();
        let messages = vec![mock_message(1), mock_message(2), mock_message(3)];

        state.load_messages("thread123".to_string(), messages);

        assert_eq!(state.thread_id, Some("thread123".to_string()));
        assert_eq!(state.messages.len(), 3);
        assert_eq!(state.expanded.len(), 3);
        // Last message should be expanded
        assert!(!state.is_expanded(0));
        assert!(!state.is_expanded(1));
        assert!(state.is_expanded(2));
        assert_eq!(state.focused_index, 2);
    }

    #[test]
    fn test_toggle_expanded() {
        let mut state = ThreadState::new();
        let messages = vec![mock_message(1), mock_message(2)];
        state.load_messages("thread123".to_string(), messages);

        assert!(!state.is_expanded(0));
        state.toggle_expanded(0);
        assert!(state.is_expanded(0));
        state.toggle_expanded(0);
        assert!(!state.is_expanded(0));
    }

    #[test]
    fn test_expand_collapse_all() {
        let mut state = ThreadState::new();
        let messages = vec![mock_message(1), mock_message(2), mock_message(3)];
        state.load_messages("thread123".to_string(), messages);

        state.expand_all();
        assert!(state.is_expanded(0));
        assert!(state.is_expanded(1));
        assert!(state.is_expanded(2));

        state.collapse_all();
        assert!(!state.is_expanded(0));
        assert!(!state.is_expanded(1));
        assert!(!state.is_expanded(2));
    }

    #[test]
    fn test_focus_navigation() {
        let mut state = ThreadState::new();
        let messages = vec![mock_message(1), mock_message(2), mock_message(3)];
        state.load_messages("thread123".to_string(), messages);

        assert_eq!(state.focused_index, 2);

        state.focus_previous();
        assert_eq!(state.focused_index, 1);

        state.focus_previous();
        assert_eq!(state.focused_index, 0);

        // Should not go below 0
        state.focus_previous();
        assert_eq!(state.focused_index, 0);

        state.focus_next();
        assert_eq!(state.focused_index, 1);

        state.focus_next();
        assert_eq!(state.focused_index, 2);

        // Should not go beyond last message
        state.focus_next();
        assert_eq!(state.focused_index, 2);
    }
}
