//! Spacing scale for the design system
//!
//! Based on a 4px base unit for consistent spacing throughout the UI.

// === Spacing Values ===
// Using u16 for compatibility with Iced's padding/spacing

/// Extra small - 4px - tight padding, icon margins
pub const XS: u16 = 4;

/// Small - 8px - compact spacing, inline elements
pub const SM: u16 = 8;

/// Medium - 12px - default gaps, form elements
pub const MD: u16 = 12;

/// Large - 16px - section padding, card content
pub const LG: u16 = 16;

/// Extra large - 24px - major sections, panels
pub const XL: u16 = 24;

/// 2X large - 32px - page margins, modal padding
pub const XXL: u16 = 32;

/// 3X large - 48px - large separations
pub const XXXL: u16 = 48;

// === Helper Functions ===

/// Convert spacing to f32 for use with Length::Fixed
pub const fn as_f32(spacing: u16) -> f32 {
    spacing as f32
}

/// Common padding configurations
pub mod padding {
    use super::*;

    /// Compact padding for buttons, chips
    pub const COMPACT: [u16; 2] = [XS, SM];

    /// Default padding for cards, panels
    pub const DEFAULT: [u16; 2] = [MD, LG];

    /// Comfortable padding for modals, larger containers
    pub const COMFORTABLE: [u16; 2] = [LG, XL];

    /// Spacious padding for page content
    pub const SPACIOUS: [u16; 2] = [XL, XXL];
}
