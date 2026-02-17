//! Color tokens for the design system
//!
//! Inspired by the Zed One Dark color palette.
//! Colors are organized by semantic meaning rather than visual appearance.

use iced::Color;

// === Background Hierarchy ===
// From deepest to most elevated

/// Base background - used for the main application background
pub const BG_BASE: Color = Color {
    r: 0.157,
    g: 0.173,
    b: 0.2,
    a: 1.0,
}; // #282c33

/// Surface background - used for cards, panels, and containers
pub const BG_SURFACE: Color = Color {
    r: 0.188,
    g: 0.208,
    b: 0.239,
    a: 1.0,
}; // #303540

/// Elevated background - used for hover states and raised elements
pub const BG_ELEVATED: Color = Color {
    r: 0.22,
    g: 0.243,
    b: 0.278,
    a: 1.0,
}; // #383e47

/// Overlay background - used for modals, dropdowns, and tooltips
pub const BG_OVERLAY: Color = Color {
    r: 0.251,
    g: 0.278,
    b: 0.318,
    a: 1.0,
}; // #404751

// === Text Hierarchy ===

/// Primary text - main content, headings, important information
pub const TEXT_PRIMARY: Color = Color {
    r: 0.863,
    g: 0.878,
    b: 0.898,
    a: 1.0,
}; // #dce0e5

/// Secondary text - supporting content, descriptions
pub const TEXT_SECONDARY: Color = Color {
    r: 0.671,
    g: 0.698,
    b: 0.737,
    a: 1.0,
}; // #abb2bc

/// Muted text - timestamps, metadata, hints
pub const TEXT_MUTED: Color = Color {
    r: 0.459,
    g: 0.494,
    b: 0.545,
    a: 1.0,
}; // #757e8b

/// Disabled text - inactive elements, placeholders
pub const TEXT_DISABLED: Color = Color {
    r: 0.333,
    g: 0.361,
    b: 0.404,
    a: 1.0,
}; // #555c67

// === Accent Colors ===

/// Primary accent - links, focus states, primary buttons
pub const ACCENT_PRIMARY: Color = Color {
    r: 0.455,
    g: 0.678,
    b: 0.91,
    a: 1.0,
}; // #74ade8

/// Success accent - confirmation, positive actions
pub const ACCENT_SUCCESS: Color = Color {
    r: 0.596,
    g: 0.765,
    b: 0.478,
    a: 1.0,
}; // #98c379

/// Warning accent - caution, attention needed
pub const ACCENT_WARNING: Color = Color {
    r: 0.906,
    g: 0.773,
    b: 0.424,
    a: 1.0,
}; // #e7c56c

/// Error accent - errors, destructive actions
pub const ACCENT_ERROR: Color = Color {
    r: 0.878,
    g: 0.439,
    b: 0.439,
    a: 1.0,
}; // #e07070

// === Semantic Colors ===

/// Subtle border - dividers, separators
pub const BORDER_SUBTLE: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
    a: 0.08,
};

/// Visible border - input fields, card outlines
pub const BORDER_VISIBLE: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
    a: 0.15,
};

/// Selection background - highlighted items
pub const SELECTION_BG: Color = Color {
    r: 0.455,
    g: 0.678,
    b: 0.91,
    a: 0.2,
};

/// Focus ring color - keyboard focus indicator
pub const FOCUS_RING: Color = Color {
    r: 0.455,
    g: 0.678,
    b: 0.91,
    a: 0.5,
};

// === Utility Functions ===

/// Lighten a color by a factor (0.0 to 1.0)
pub fn lighten(color: Color, factor: f32) -> Color {
    Color {
        r: color.r + (1.0 - color.r) * factor,
        g: color.g + (1.0 - color.g) * factor,
        b: color.b + (1.0 - color.b) * factor,
        a: color.a,
    }
}

/// Darken a color by a factor (0.0 to 1.0)
pub fn darken(color: Color, factor: f32) -> Color {
    Color {
        r: color.r * (1.0 - factor),
        g: color.g * (1.0 - factor),
        b: color.b * (1.0 - factor),
        a: color.a,
    }
}

/// Set the alpha of a color
pub fn with_alpha(color: Color, alpha: f32) -> Color {
    Color { a: alpha, ..color }
}
