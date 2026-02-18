//! Color tokens for the design system
//!
//! "Foundry Dark" palette — warm neutrals with copper accent.
//! Backgrounds have a slight amber cast; text has a cool cast for contrast.

use iced::Color;

// === Background Hierarchy (warm undertone) ===

/// Deep background — app chrome, window frame
pub const BG_DEEP: Color = Color {
    r: 0.102,
    g: 0.094,
    b: 0.086,
    a: 1.0,
}; // #1a1816

/// Base background — primary application background
pub const BG_BASE: Color = Color {
    r: 0.129,
    g: 0.122,
    b: 0.110,
    a: 1.0,
}; // #211f1c

/// Surface background — cards, panels, containers
pub const BG_SURFACE: Color = Color {
    r: 0.165,
    g: 0.157,
    b: 0.145,
    a: 1.0,
}; // #2a2825

/// Elevated background — hover states, raised elements
pub const BG_ELEVATED: Color = Color {
    r: 0.200,
    g: 0.188,
    b: 0.173,
    a: 1.0,
}; // #33302c

/// Overlay background — modals, dropdowns, tooltips
pub const BG_OVERLAY: Color = Color {
    r: 0.239,
    g: 0.224,
    b: 0.208,
    a: 1.0,
}; // #3d3935

// === Text Hierarchy (cool neutral for contrast) ===

/// Primary text — headings, important content
pub const TEXT_PRIMARY: Color = Color {
    r: 0.910,
    g: 0.894,
    b: 0.875,
    a: 1.0,
}; // #e8e4df

/// Secondary text — body text, descriptions
pub const TEXT_SECONDARY: Color = Color {
    r: 0.659,
    g: 0.635,
    b: 0.620,
    a: 1.0,
}; // #a8a29e

/// Muted text — timestamps, metadata, hints
pub const TEXT_MUTED: Color = Color {
    r: 0.420,
    g: 0.396,
    b: 0.376,
    a: 1.0,
}; // #6b6560

/// Disabled text — inactive elements, placeholders
pub const TEXT_DISABLED: Color = Color {
    r: 0.290,
    g: 0.271,
    b: 0.255,
    a: 1.0,
}; // #4a4541

// === Accent: Copper ===

/// Primary accent — links, primary actions, brand color
pub const ACCENT_PRIMARY: Color = Color {
    r: 0.831,
    g: 0.584,
    b: 0.416,
    a: 1.0,
}; // #d4956a

/// Accent hover state
pub const ACCENT_HOVER: Color = Color {
    r: 0.878,
    g: 0.659,
    b: 0.494,
    a: 1.0,
}; // #e0a87e

/// Accent pressed state
pub const ACCENT_PRESSED: Color = Color {
    r: 0.722,
    g: 0.490,
    b: 0.333,
    a: 1.0,
}; // #b87d55

// === Semantic Colors ===

/// Success — confirmations, positive
pub const ACCENT_SUCCESS: Color = Color {
    r: 0.478,
    g: 0.722,
    b: 0.478,
    a: 1.0,
}; // #7ab87a

/// Warning — caution, attention
pub const ACCENT_WARNING: Color = Color {
    r: 0.831,
    g: 0.722,
    b: 0.416,
    a: 1.0,
}; // #d4b86a

/// Error — errors, destructive actions
pub const ACCENT_ERROR: Color = Color {
    r: 0.780,
    g: 0.361,
    b: 0.361,
    a: 1.0,
}; // #c75c5c

/// Info — informational, threads
pub const ACCENT_INFO: Color = Color {
    r: 0.416,
    g: 0.624,
    b: 0.831,
    a: 1.0,
}; // #6a9fd4

// === Borders ===

/// Subtle border — dividers, separators
pub const BORDER_SUBTLE: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
    a: 0.06,
};

/// Visible border — input fields, card outlines
pub const BORDER_VISIBLE: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
    a: 0.12,
};

/// Focus border — keyboard focus indicator
pub const BORDER_FOCUS: Color = Color {
    r: 0.831,
    g: 0.584,
    b: 0.416,
    a: 0.6,
};

// === Selection ===

/// Selection background — highlighted items
pub const SELECTION_BG: Color = Color {
    r: 0.831,
    g: 0.584,
    b: 0.416,
    a: 0.12,
};

/// Strong selection — active selection
pub const SELECTION_STRONG: Color = Color {
    r: 0.831,
    g: 0.584,
    b: 0.416,
    a: 0.20,
};

/// Focus ring color
pub const FOCUS_RING: Color = Color {
    r: 0.831,
    g: 0.584,
    b: 0.416,
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
