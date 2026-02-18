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
    // Warm-toned palette matching Foundry Dark design system
    let colors = [
        Color::from_rgb(0.831, 0.584, 0.416), // Copper   #d4956a
        Color::from_rgb(0.416, 0.624, 0.627), // Teal     #6a9fa0
        Color::from_rgb(0.478, 0.722, 0.478), // Sage     #7ab87a
        Color::from_rgb(0.831, 0.722, 0.416), // Amber    #d4b86a
        Color::from_rgb(0.780, 0.361, 0.486), // Rose     #c75c7c
        Color::from_rgb(0.416, 0.498, 0.831), // Indigo   #6a7fd4
        Color::from_rgb(0.604, 0.478, 0.722), // Mauve    #9a7ab8
        Color::from_rgb(0.722, 0.490, 0.333), // Sienna   #b87d55
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
