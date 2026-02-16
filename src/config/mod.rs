//! Configuration management
//!
//! Handles loading and saving application settings.

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Application settings persisted to disk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// msgvault server URL (e.g., "http://localhost:8080")
    #[serde(default)]
    pub server_url: String,

    /// API key for authentication
    #[serde(default)]
    pub api_key: String,

    /// Allow insecure (HTTP) connections
    #[serde(default)]
    pub allow_insecure: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            server_url: String::new(),
            api_key: String::new(),
            allow_insecure: false,
        }
    }
}

impl Settings {
    /// Get the configuration directory path
    fn config_dir() -> Option<PathBuf> {
        ProjectDirs::from("com", "msgvault", "msgvault-desktop")
            .map(|dirs| dirs.config_dir().to_path_buf())
    }

    /// Get the configuration file path
    fn config_path() -> Option<PathBuf> {
        Self::config_dir().map(|dir| dir.join("config.toml"))
    }

    /// Load settings from disk, or return defaults if not found
    pub fn load() -> Result<Self, String> {
        let path = match Self::config_path() {
            Some(p) => p,
            None => return Ok(Self::default()),
        };

        if !path.exists() {
            return Ok(Self::default());
        }

        let contents =
            fs::read_to_string(&path).map_err(|e| format!("Failed to read config: {}", e))?;

        toml::from_str(&contents).map_err(|e| format!("Failed to parse config: {}", e))
    }

    /// Save settings to disk
    pub fn save(&self) -> Result<(), String> {
        let dir = match Self::config_dir() {
            Some(d) => d,
            None => return Err("Could not determine config directory".to_string()),
        };

        fs::create_dir_all(&dir)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;

        let path = dir.join("config.toml");
        let contents = toml::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;

        fs::write(&path, contents).map_err(|e| format!("Failed to write config: {}", e))
    }
}
