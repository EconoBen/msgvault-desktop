//! Main application struct
//!
//! Implements the Iced Application pattern with MVU architecture.

use crate::config::Settings;
use crate::message::Message;
use crate::model::AppState;
use crate::update;
use crate::view;
use iced::event::Event;
use iced::keyboard;
use iced::{Element, Subscription, Task};

/// Main application state container
pub struct MsgVaultApp {
    state: AppState,
    settings: Settings,
}

impl MsgVaultApp {
    /// Create new application with settings
    pub fn new(settings: Settings) -> (Self, Task<Message>) {
        let state = AppState::new(&settings);

        let app = Self {
            state,
            settings: settings.clone(),
        };

        // Determine startup behavior
        let initial_task = if settings.server_url.is_empty() {
            // First run - start discovery
            Task::done(Message::StartDiscovery)
        } else {
            // Have a server URL - check health
            Task::done(Message::CheckHealth)
        };

        (app, initial_task)
    }

    /// Window title
    pub fn title(&self) -> String {
        match self.state.is_connected() {
            true => format!("msgvault - {}", &self.state.server_url),
            false => "msgvault".to_string(),
        }
    }

    /// Handle messages and update state
    pub fn update(&mut self, message: Message) -> Task<Message> {
        update::handle(&mut self.state, message)
    }

    /// Render the view
    pub fn view(&self) -> Element<'_, Message> {
        view::render(&self.state)
    }

    /// Subscribe to events (keyboard, etc.)
    pub fn subscription(&self) -> Subscription<Message> {
        iced::event::listen().map(|event| match event {
            Event::Keyboard(keyboard::Event::KeyPressed {
                key, modifiers, ..
            }) => Message::KeyPressed(key, modifiers),
            _ => Message::None,
        })
    }
}
