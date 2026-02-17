//! Navigation state
//!
//! Tracks where the user is in the app and maintains breadcrumb history.

use crate::api::types::ViewType;

/// Represents the current view/screen in the application
#[derive(Debug, Clone, PartialEq)]
pub enum ViewLevel {
    /// Stats dashboard - the home screen
    Dashboard,

    /// Aggregate list view (senders, domains, labels, etc.)
    Aggregates {
        view_type: ViewType,
    },

    /// Sub-aggregate after drilling down (e.g., sender's labels)
    SubAggregates {
        parent_view_type: ViewType,
        parent_key: String,
        view_type: ViewType,
    },

    /// Message list (filtered by aggregate)
    Messages {
        filter_description: String,
    },

    /// Single message detail
    MessageDetail {
        message_id: i64,
    },

    /// Search view
    Search,
}

impl ViewLevel {
    /// Get a display title for the current view
    pub fn title(&self) -> String {
        match self {
            ViewLevel::Dashboard => "Dashboard".to_string(),
            ViewLevel::Aggregates { view_type } => view_type.display_name().to_string(),
            ViewLevel::SubAggregates {
                parent_key,
                view_type,
                ..
            } => format!("{} → {}", parent_key, view_type.display_name()),
            ViewLevel::Messages { filter_description } => filter_description.clone(),
            ViewLevel::MessageDetail { message_id } => format!("Message #{}", message_id),
            ViewLevel::Search => "Search".to_string(),
        }
    }

    /// Check if this is the dashboard (home)
    pub fn is_dashboard(&self) -> bool {
        matches!(self, ViewLevel::Dashboard)
    }
}

/// Breadcrumb entry for navigation history
#[derive(Debug, Clone)]
pub struct BreadcrumbEntry {
    pub label: String,
    pub view: ViewLevel,
}

/// Navigation history stack
#[derive(Debug, Clone, Default)]
pub struct NavigationStack {
    /// Stack of previous views (for back navigation)
    history: Vec<ViewLevel>,
    /// Current view
    current: Option<ViewLevel>,
}

impl NavigationStack {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            current: Some(ViewLevel::Dashboard),
        }
    }

    /// Get the current view
    pub fn current(&self) -> &ViewLevel {
        self.current.as_ref().unwrap_or(&ViewLevel::Dashboard)
    }

    /// Navigate to a new view, pushing current to history
    pub fn push(&mut self, view: ViewLevel) {
        if let Some(current) = self.current.take() {
            self.history.push(current);
        }
        self.current = Some(view);
    }

    /// Go back to previous view
    pub fn pop(&mut self) -> bool {
        if let Some(previous) = self.history.pop() {
            self.current = Some(previous);
            true
        } else {
            false
        }
    }

    /// Check if we can go back
    pub fn can_go_back(&self) -> bool {
        !self.history.is_empty()
    }

    /// Get breadcrumb trail
    pub fn breadcrumbs(&self) -> Vec<BreadcrumbEntry> {
        let mut crumbs: Vec<BreadcrumbEntry> = self
            .history
            .iter()
            .map(|v| BreadcrumbEntry {
                label: v.title(),
                view: v.clone(),
            })
            .collect();

        if let Some(current) = &self.current {
            crumbs.push(BreadcrumbEntry {
                label: current.title(),
                view: current.clone(),
            });
        }

        crumbs
    }

    /// Navigate directly to a breadcrumb (truncates history)
    pub fn jump_to(&mut self, index: usize) {
        if index < self.history.len() {
            let view = self.history[index].clone();
            self.history.truncate(index);
            self.current = Some(view);
        }
    }

    /// Reset to dashboard
    pub fn reset(&mut self) {
        self.history.clear();
        self.current = Some(ViewLevel::Dashboard);
    }
}
