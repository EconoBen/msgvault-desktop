//! Spacing & layout constants
//!
//! 4px base unit with more steps for precision.

// === Spacing Values ===

/// Hairline — 2px
pub const SPACE_1: u16 = 2;

/// Tight inline — 4px
pub const XS: u16 = 4;

/// Compact list items — 6px
pub const SPACE_3: u16 = 6;

/// Default inline gap — 8px
pub const SM: u16 = 8;

/// Form element gaps — 12px
pub const MD: u16 = 12;

/// Section gaps — 16px
pub const LG: u16 = 16;

/// Card padding — 20px
pub const SPACE_10: u16 = 20;

/// Panel padding — 24px
pub const XL: u16 = 24;

/// Major section breaks — 32px
pub const XXL: u16 = 32;

/// Page margins — 40px
pub const SPACE_20: u16 = 40;

/// Hero spacing — 48px
pub const XXXL: u16 = 48;

// === Border Radius ===

/// Small — badges, chips, inline elements
pub const RADIUS_SM: f32 = 3.0;

/// Medium — buttons, inputs, list items
pub const RADIUS_MD: f32 = 6.0;

/// Large — cards, panels
pub const RADIUS_LG: f32 = 10.0;

/// Extra large — modals, wizards
pub const RADIUS_XL: f32 = 16.0;

/// Full — avatars, circular buttons
pub const RADIUS_FULL: f32 = 999.0;

// === Layout Dimensions ===

/// Sidebar width
pub const SIDEBAR_WIDTH: f32 = 240.0;

/// Message list panel width (as fill portion)
pub const MESSAGE_LIST_PORTION: u16 = 2;

/// Detail panel width (as fill portion)
pub const DETAIL_PORTION: u16 = 3;

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
