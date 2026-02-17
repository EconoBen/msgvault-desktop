//! Component style functions for the design system
//!
//! Provides reusable style functions for common UI patterns.

use iced::widget::{button, container, text, text_input};
use iced::{Background, Border, Color, Shadow, Theme, Vector};

use super::colors;

// === Container Styles ===

/// Card style - elevated surface with subtle border and shadow
pub fn card_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(colors::BG_SURFACE)),
        border: Border {
            radius: 6.0.into(),
            width: 1.0,
            color: colors::BORDER_SUBTLE,
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.15),
            offset: Vector::new(0.0, 2.0),
            blur_radius: 8.0,
        },
        ..Default::default()
    }
}

/// Panel style - surface container without shadow
pub fn panel_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(colors::BG_SURFACE)),
        border: Border {
            radius: 4.0.into(),
            width: 1.0,
            color: colors::BORDER_SUBTLE,
        },
        ..Default::default()
    }
}

/// Sidebar style - base background for navigation
pub fn sidebar_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(colors::BG_BASE)),
        border: Border {
            radius: 0.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        ..Default::default()
    }
}

/// Modal backdrop - semi-transparent overlay
pub fn modal_backdrop_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.5))),
        ..Default::default()
    }
}

/// Modal dialog - elevated overlay container
pub fn modal_dialog_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(colors::BG_OVERLAY)),
        border: Border {
            radius: 8.0.into(),
            width: 1.0,
            color: colors::BORDER_VISIBLE,
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
            offset: Vector::new(0.0, 8.0),
            blur_radius: 24.0,
        },
        ..Default::default()
    }
}

/// Selected row background
pub fn selected_row_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(colors::SELECTION_BG)),
        border: Border {
            radius: 4.0.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

/// Hover row background
pub fn hover_row_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(colors::BG_ELEVATED)),
        border: Border {
            radius: 4.0.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

// === Button Styles ===

/// Primary button - main action button
pub fn button_primary(_theme: &Theme, status: button::Status) -> button::Style {
    let bg_color = match status {
        button::Status::Hovered => colors::lighten(colors::ACCENT_PRIMARY, 0.1),
        button::Status::Pressed => colors::darken(colors::ACCENT_PRIMARY, 0.1),
        button::Status::Disabled => colors::with_alpha(colors::ACCENT_PRIMARY, 0.5),
        _ => colors::ACCENT_PRIMARY,
    };

    button::Style {
        background: Some(Background::Color(bg_color)),
        text_color: Color::WHITE,
        border: Border {
            radius: 4.0.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

/// Secondary button - less prominent action
pub fn button_secondary(_theme: &Theme, status: button::Status) -> button::Style {
    let bg_color = match status {
        button::Status::Hovered => colors::BG_ELEVATED,
        button::Status::Pressed => colors::BG_OVERLAY,
        button::Status::Disabled => colors::with_alpha(colors::BG_SURFACE, 0.5),
        _ => colors::BG_SURFACE,
    };

    button::Style {
        background: Some(Background::Color(bg_color)),
        text_color: colors::TEXT_PRIMARY,
        border: Border {
            radius: 4.0.into(),
            width: 1.0,
            color: colors::BORDER_VISIBLE,
        },
        ..Default::default()
    }
}

/// Ghost button - minimal visual weight
pub fn button_ghost(_theme: &Theme, status: button::Status) -> button::Style {
    let bg_color = match status {
        button::Status::Hovered => colors::BG_ELEVATED,
        button::Status::Pressed => colors::BG_OVERLAY,
        _ => Color::TRANSPARENT,
    };

    button::Style {
        background: Some(Background::Color(bg_color)),
        text_color: colors::TEXT_SECONDARY,
        border: Border {
            radius: 4.0.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

/// Danger button - destructive action
pub fn button_danger(_theme: &Theme, status: button::Status) -> button::Style {
    let bg_color = match status {
        button::Status::Hovered => colors::lighten(colors::ACCENT_ERROR, 0.1),
        button::Status::Pressed => colors::darken(colors::ACCENT_ERROR, 0.1),
        button::Status::Disabled => colors::with_alpha(colors::ACCENT_ERROR, 0.5),
        _ => colors::ACCENT_ERROR,
    };

    button::Style {
        background: Some(Background::Color(bg_color)),
        text_color: Color::WHITE,
        border: Border {
            radius: 4.0.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

/// Icon button - circular, minimal
pub fn button_icon(_theme: &Theme, status: button::Status) -> button::Style {
    let bg_color = match status {
        button::Status::Hovered => colors::BG_ELEVATED,
        button::Status::Pressed => colors::BG_OVERLAY,
        _ => Color::TRANSPARENT,
    };

    button::Style {
        background: Some(Background::Color(bg_color)),
        text_color: colors::TEXT_SECONDARY,
        border: Border {
            radius: 16.0.into(), // More rounded for icon buttons
            ..Default::default()
        },
        ..Default::default()
    }
}

// === Text Input Styles ===

/// Default text input style
pub fn text_input_style(_theme: &Theme, status: text_input::Status) -> text_input::Style {
    let (border_color, bg_color) = match status {
        text_input::Status::Focused => (colors::ACCENT_PRIMARY, colors::BG_ELEVATED),
        text_input::Status::Hovered => (colors::BORDER_VISIBLE, colors::BG_ELEVATED),
        text_input::Status::Disabled => (colors::BORDER_SUBTLE, colors::BG_BASE),
        _ => (colors::BORDER_VISIBLE, colors::BG_SURFACE),
    };

    text_input::Style {
        background: Background::Color(bg_color),
        border: Border {
            radius: 4.0.into(),
            width: 1.0,
            color: border_color,
        },
        icon: colors::TEXT_MUTED,
        placeholder: colors::TEXT_DISABLED,
        value: colors::TEXT_PRIMARY,
        selection: colors::SELECTION_BG,
    }
}

// === Text Styles ===

/// Primary text style
pub fn text_primary(_theme: &Theme) -> text::Style {
    text::Style {
        color: Some(colors::TEXT_PRIMARY),
    }
}

/// Secondary text style
pub fn text_secondary(_theme: &Theme) -> text::Style {
    text::Style {
        color: Some(colors::TEXT_SECONDARY),
    }
}

/// Muted text style
pub fn text_muted(_theme: &Theme) -> text::Style {
    text::Style {
        color: Some(colors::TEXT_MUTED),
    }
}

/// Accent text style
pub fn text_accent(_theme: &Theme) -> text::Style {
    text::Style {
        color: Some(colors::ACCENT_PRIMARY),
    }
}

/// Success text style
pub fn text_success(_theme: &Theme) -> text::Style {
    text::Style {
        color: Some(colors::ACCENT_SUCCESS),
    }
}

/// Error text style
pub fn text_error(_theme: &Theme) -> text::Style {
    text::Style {
        color: Some(colors::ACCENT_ERROR),
    }
}
