//! Compose email state
//!
//! State management for email composition, replies, and forwards.

use std::path::PathBuf;

/// Mode of email composition
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ComposeMode {
    /// New email from scratch
    #[default]
    New,
    /// Reply to sender only
    Reply,
    /// Reply to all recipients
    ReplyAll,
    /// Forward an email
    Forward,
}

impl ComposeMode {
    /// Get display name for the mode
    pub fn display_name(&self) -> &'static str {
        match self {
            ComposeMode::New => "New Message",
            ComposeMode::Reply => "Reply",
            ComposeMode::ReplyAll => "Reply All",
            ComposeMode::Forward => "Forward",
        }
    }
}

/// Draft attachment (not yet sent)
#[derive(Debug, Clone)]
pub struct AttachmentDraft {
    /// File path on disk
    pub path: PathBuf,
    /// Original filename
    pub filename: String,
    /// File size in bytes
    pub size_bytes: i64,
    /// MIME type (if detected)
    pub mime_type: Option<String>,
}

/// State for the compose modal
#[derive(Debug, Clone, Default)]
pub struct ComposeState {
    /// Whether the compose modal is open
    pub is_open: bool,
    /// Composition mode
    pub mode: ComposeMode,
    /// ID of the message being replied to (for Reply/ReplyAll/Forward)
    pub reply_to_id: Option<i64>,
    /// From account (email address)
    pub from_account: String,
    /// To recipients
    pub to: Vec<String>,
    /// CC recipients
    pub cc: Vec<String>,
    /// BCC recipients
    pub bcc: Vec<String>,
    /// Email subject
    pub subject: String,
    /// Email body
    pub body: String,
    /// Draft attachments
    pub attachments: Vec<AttachmentDraft>,
    /// Whether currently sending
    pub is_sending: bool,
    /// Send error (if any)
    pub send_error: Option<String>,
    /// Whether the draft has unsaved changes
    pub is_dirty: bool,
    /// Show CC/BCC fields
    pub show_cc_bcc: bool,
    /// Current input field (for CC/BCC toggle)
    pub to_input: String,
    pub cc_input: String,
    pub bcc_input: String,
}

impl ComposeState {
    /// Create a new compose state
    pub fn new() -> Self {
        Self::default()
    }

    /// Open compose for a new email
    pub fn open_new(from_account: String) -> Self {
        Self {
            is_open: true,
            mode: ComposeMode::New,
            from_account,
            ..Default::default()
        }
    }

    /// Open compose for a reply
    pub fn open_reply(
        from_account: String,
        reply_to_id: i64,
        to: String,
        subject: String,
        quoted_body: String,
    ) -> Self {
        let subject = if subject.to_lowercase().starts_with("re:") {
            subject
        } else {
            format!("Re: {}", subject)
        };

        Self {
            is_open: true,
            mode: ComposeMode::Reply,
            reply_to_id: Some(reply_to_id),
            from_account,
            to: vec![to],
            subject,
            body: format!("\n\n{}", quoted_body),
            ..Default::default()
        }
    }

    /// Open compose for reply-all
    pub fn open_reply_all(
        from_account: String,
        reply_to_id: i64,
        to: Vec<String>,
        cc: Vec<String>,
        subject: String,
        quoted_body: String,
    ) -> Self {
        let subject = if subject.to_lowercase().starts_with("re:") {
            subject
        } else {
            format!("Re: {}", subject)
        };

        let show_cc = !cc.is_empty();

        Self {
            is_open: true,
            mode: ComposeMode::ReplyAll,
            reply_to_id: Some(reply_to_id),
            from_account,
            to,
            cc,
            subject,
            body: format!("\n\n{}", quoted_body),
            show_cc_bcc: show_cc,
            ..Default::default()
        }
    }

    /// Open compose for forward
    pub fn open_forward(
        from_account: String,
        original_id: i64,
        subject: String,
        forward_body: String,
    ) -> Self {
        let subject = if subject.to_lowercase().starts_with("fwd:") {
            subject
        } else {
            format!("Fwd: {}", subject)
        };

        Self {
            is_open: true,
            mode: ComposeMode::Forward,
            reply_to_id: Some(original_id),
            from_account,
            subject,
            body: format!("\n\n---------- Forwarded message ----------\n{}", forward_body),
            ..Default::default()
        }
    }

    /// Close the compose modal
    pub fn close(&mut self) {
        self.is_open = false;
        self.mode = ComposeMode::New;
        self.reply_to_id = None;
        self.to.clear();
        self.cc.clear();
        self.bcc.clear();
        self.subject.clear();
        self.body.clear();
        self.attachments.clear();
        self.is_sending = false;
        self.send_error = None;
        self.is_dirty = false;
        self.show_cc_bcc = false;
        self.to_input.clear();
        self.cc_input.clear();
        self.bcc_input.clear();
    }

    /// Check if there's content to potentially save as draft
    pub fn has_content(&self) -> bool {
        !self.to.is_empty()
            || !self.cc.is_empty()
            || !self.bcc.is_empty()
            || !self.subject.is_empty()
            || !self.body.is_empty()
            || !self.attachments.is_empty()
    }

    /// Add a recipient to the To field
    pub fn add_to(&mut self, email: String) {
        if !email.is_empty() && !self.to.contains(&email) {
            self.to.push(email);
            self.is_dirty = true;
        }
    }

    /// Add a recipient to the CC field
    pub fn add_cc(&mut self, email: String) {
        if !email.is_empty() && !self.cc.contains(&email) {
            self.cc.push(email);
            self.is_dirty = true;
        }
    }

    /// Add a recipient to the BCC field
    pub fn add_bcc(&mut self, email: String) {
        if !email.is_empty() && !self.bcc.contains(&email) {
            self.bcc.push(email);
            self.is_dirty = true;
        }
    }

    /// Remove a recipient from To
    pub fn remove_to(&mut self, index: usize) {
        if index < self.to.len() {
            self.to.remove(index);
            self.is_dirty = true;
        }
    }

    /// Remove a recipient from CC
    pub fn remove_cc(&mut self, index: usize) {
        if index < self.cc.len() {
            self.cc.remove(index);
            self.is_dirty = true;
        }
    }

    /// Remove a recipient from BCC
    pub fn remove_bcc(&mut self, index: usize) {
        if index < self.bcc.len() {
            self.bcc.remove(index);
            self.is_dirty = true;
        }
    }

    /// Check if the email is valid to send
    pub fn can_send(&self) -> bool {
        !self.from_account.is_empty()
            && (!self.to.is_empty() || !self.cc.is_empty() || !self.bcc.is_empty())
            && !self.is_sending
    }
}

/// Format a quoted body for replies
pub fn format_quoted_body(from: &str, date: &str, body: &str) -> String {
    let mut quoted = format!("On {}, {} wrote:\n", date, from);
    for line in body.lines() {
        quoted.push_str("> ");
        quoted.push_str(line);
        quoted.push('\n');
    }
    quoted
}
