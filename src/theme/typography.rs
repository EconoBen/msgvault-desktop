//! Typography scale for the design system
//!
//! IBM Plex font family with a refined modular scale.

use iced::font::Weight;
use iced::Font;

// === Fonts ===

/// Primary UI font — IBM Plex Sans
pub const FONT_PRIMARY: Font = Font::with_name("IBM Plex Sans");

/// Medium weight variant
pub const FONT_MEDIUM: Font = Font {
    family: iced::font::Family::Name("IBM Plex Sans"),
    weight: Weight::Medium,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal,
};

/// SemiBold weight variant — headings, emphasis
pub const FONT_SEMIBOLD: Font = Font {
    family: iced::font::Family::Name("IBM Plex Sans"),
    weight: Weight::Semibold,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal,
};

/// Monospace font — keyboard shortcuts, data, code
pub const FONT_MONO: Font = Font::with_name("IBM Plex Mono");

// === Font Sizes ===

/// 2XS — keyboard hints, fine metadata
pub const SIZE_2XS: f32 = 10.0;

/// Extra small — timestamps, badges
pub const SIZE_XS: f32 = 11.0;

/// Small — secondary text, captions
pub const SIZE_SM: f32 = 12.0;

/// Body — default text size for content
pub const SIZE_BODY: f32 = 13.0;

/// Medium — slightly emphasized text
pub const SIZE_MD: f32 = 15.0;

/// Large — section titles, subheadings
pub const SIZE_LG: f32 = 18.0;

/// Extra large — page headings
pub const SIZE_XL: f32 = 22.0;

/// 2X large — hero text, wizard titles
pub const SIZE_2XL: f32 = 28.0;

/// 3X large — impact numbers (stats dashboard)
pub const SIZE_3XL: f32 = 36.0;

// === Font Weights ===

/// Normal weight — body text
pub const WEIGHT_NORMAL: Weight = Weight::Normal;

/// Medium weight — slightly emphasized
pub const WEIGHT_MEDIUM: Weight = Weight::Medium;

/// Semibold — headings, labels
pub const WEIGHT_SEMIBOLD: Weight = Weight::Semibold;

/// Bold — strong emphasis
pub const WEIGHT_BOLD: Weight = Weight::Bold;

// === Line Heights ===

/// Tight line height — headings, single-line elements
pub const LINE_HEIGHT_TIGHT: f32 = 1.25;

/// Normal line height — body text
pub const LINE_HEIGHT_NORMAL: f32 = 1.5;

/// Relaxed line height — longer form content
pub const LINE_HEIGHT_RELAXED: f32 = 1.75;
