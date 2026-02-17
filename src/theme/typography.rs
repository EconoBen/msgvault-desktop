//! Typography scale for the design system
//!
//! Defines font sizes, weights, and line heights for consistent text styling.

use iced::font::Weight;

// === Font Sizes ===
// Based on a 14px base with a modular scale

/// Extra small - timestamps, badges, fine print
pub const SIZE_XS: f32 = 11.0;

/// Small - secondary text, captions
pub const SIZE_SM: f32 = 12.0;

/// Body - default text size for content
pub const SIZE_BODY: f32 = 14.0;

/// Medium - slightly emphasized text
pub const SIZE_MD: f32 = 16.0;

/// Large - subheadings, section titles
pub const SIZE_LG: f32 = 18.0;

/// Extra large - headings
pub const SIZE_XL: f32 = 24.0;

/// 2X large - page titles, hero text
pub const SIZE_2XL: f32 = 32.0;

// === Font Weights ===

/// Normal weight - body text
pub const WEIGHT_NORMAL: Weight = Weight::Normal;

/// Medium weight - slightly emphasized
pub const WEIGHT_MEDIUM: Weight = Weight::Medium;

/// Semibold - headings, labels
pub const WEIGHT_SEMIBOLD: Weight = Weight::Semibold;

/// Bold - strong emphasis
pub const WEIGHT_BOLD: Weight = Weight::Bold;

// === Line Heights ===
// As multipliers of font size

/// Tight line height - headings, single-line elements
pub const LINE_HEIGHT_TIGHT: f32 = 1.25;

/// Normal line height - body text
pub const LINE_HEIGHT_NORMAL: f32 = 1.5;

/// Relaxed line height - longer form content
pub const LINE_HEIGHT_RELAXED: f32 = 1.75;
