//! Reusable widget components
//!
//! Custom widgets used across the application.

pub mod aggregate_row;
pub mod avatar;
pub mod badge;
pub mod breadcrumb;
pub mod loading;
pub mod stats_card;

pub use aggregate_row::aggregate_row;
pub use avatar::avatar;
pub use badge::{badge, count_badge, unread_dot, attachment_indicator, BadgeStyle};
pub use breadcrumb::breadcrumb;
pub use loading::{empty_state, error, loading};
pub use stats_card::{format_bytes, format_number, stats_card};
