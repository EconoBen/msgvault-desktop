//! Avatar widget
//!
//! Displays a circular avatar with initials from a name.

use crate::message::Message;
// Theme imports not currently used but will be needed for future styling
use iced::widget::{center, container, text};
use iced::{Background, Border, Color, Element, Length};

/// Create an avatar circle with initials
pub fn avatar(name: &str, size: u16) -> Element<'static, Message> {
    let initials = get_initials(name);
    let bg_color = color_from_name(name);

    let avatar_text = text(initials)
        .size(size as f32 * 0.4)
        .style(move |_| iced::widget::text::Style {
            color: Some(Color::WHITE),
        });

    container(center(avatar_text))
        .width(Length::Fixed(size as f32))
        .height(Length::Fixed(size as f32))
        .style(move |_| container::Style {
            background: Some(Background::Color(bg_color)),
            border: Border {
                radius: (size as f32 / 2.0).into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .into()
}

/// Get initials from a name (up to 2 characters)
fn get_initials(name: &str) -> String {
    let trimmed = name.trim();

    if trimmed.is_empty() {
        return "?".to_string();
    }

    // Handle email addresses
    let display_name = if trimmed.contains('@') {
        trimmed.split('@').next().unwrap_or(trimmed)
    } else {
        trimmed
    };

    let parts: Vec<&str> = display_name.split_whitespace().collect();

    match parts.len() {
        0 => "?".to_string(),
        1 => {
            // Single word - take first 1-2 characters
            let chars: Vec<char> = parts[0].chars().collect();
            if chars.len() >= 2 {
                chars[..2].iter().collect::<String>().to_uppercase()
            } else if !chars.is_empty() {
                chars[0].to_uppercase().to_string()
            } else {
                "?".to_string()
            }
        }
        _ => {
            // Multiple words - take first letter of first two words
            let first = parts[0].chars().next().unwrap_or('?');
            let second = parts[1].chars().next().unwrap_or('?');
            format!("{}{}", first, second).to_uppercase()
        }
    }
}

/// Generate a consistent color from a name
fn color_from_name(name: &str) -> Color {
    // Color palette for avatars
    let colors = [
        Color::from_rgb(0.424, 0.549, 0.824), // Blue
        Color::from_rgb(0.482, 0.706, 0.482), // Green
        Color::from_rgb(0.706, 0.482, 0.706), // Purple
        Color::from_rgb(0.824, 0.549, 0.424), // Orange
        Color::from_rgb(0.549, 0.706, 0.706), // Teal
        Color::from_rgb(0.706, 0.549, 0.482), // Brown
        Color::from_rgb(0.549, 0.482, 0.706), // Indigo
        Color::from_rgb(0.706, 0.482, 0.549), // Pink
    ];

    // Hash the name to get a consistent index
    let hash: usize = name
        .bytes()
        .fold(0usize, |acc, b| acc.wrapping_add(b as usize));

    colors[hash % colors.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initials() {
        assert_eq!(get_initials("John Smith"), "JS");
        assert_eq!(get_initials("Alice"), "AL");
        assert_eq!(get_initials("john@example.com"), "JO");
        assert_eq!(get_initials(""), "?");
        assert_eq!(get_initials("A B C"), "AB");
    }
}
