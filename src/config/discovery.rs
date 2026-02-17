//! Server discovery module
//!
//! Implements zero-configuration server discovery chain:
//! 1. MSGVAULT_HOME environment variable
//! 2. Default config file locations
//! 3. Localhost port probing
//! 4. Fall back to wizard

use std::path::PathBuf;
use std::time::Duration;

/// Result of server discovery
#[derive(Debug, Clone)]
pub struct DiscoveryResult {
    /// Discovered server URL (if found)
    pub server_url: Option<String>,
    /// API key (if found in config)
    pub api_key: Option<String>,
    /// How the server was discovered
    pub source: DiscoverySource,
    /// Discovery steps that were tried
    pub steps: Vec<DiscoveryStep>,
}

/// Source of the discovered configuration
#[derive(Debug, Clone, PartialEq)]
pub enum DiscoverySource {
    /// Found via MSGVAULT_HOME environment variable
    EnvVar,
    /// Found in default config location
    ConfigFile(PathBuf),
    /// Found by probing localhost
    LocalhostProbe(u16),
    /// No server found, need wizard
    NeedsWizard,
}

/// A single step in the discovery process
#[derive(Debug, Clone)]
pub struct DiscoveryStep {
    pub name: String,
    pub status: DiscoveryStepStatus,
}

#[derive(Debug, Clone)]
pub enum DiscoveryStepStatus {
    Checking,
    Found(String),
    NotFound,
    Failed(String),
}

impl DiscoveryResult {
    /// Check if discovery found a server
    pub fn found_server(&self) -> bool {
        self.server_url.is_some()
    }

    /// Check if wizard is needed
    pub fn needs_wizard(&self) -> bool {
        matches!(self.source, DiscoverySource::NeedsWizard)
    }
}

/// Run the full discovery chain
pub async fn discover_server() -> DiscoveryResult {
    let mut steps = Vec::new();

    // Step 1: Check MSGVAULT_HOME environment variable
    if let Some(result) = check_env_var(&mut steps).await {
        return result;
    }

    // Step 2: Check default config locations
    if let Some(result) = check_config_files(&mut steps).await {
        return result;
    }

    // Step 3: Probe localhost ports
    if let Some(result) = probe_localhost(&mut steps).await {
        return result;
    }

    // Step 4: No server found
    steps.push(DiscoveryStep {
        name: "No server found".to_string(),
        status: DiscoveryStepStatus::NotFound,
    });

    DiscoveryResult {
        server_url: None,
        api_key: None,
        source: DiscoverySource::NeedsWizard,
        steps,
    }
}

/// Check MSGVAULT_HOME environment variable
async fn check_env_var(steps: &mut Vec<DiscoveryStep>) -> Option<DiscoveryResult> {
    let step_name = "MSGVAULT_HOME".to_string();

    if let Ok(home) = std::env::var("MSGVAULT_HOME") {
        let home_path = PathBuf::from(&home);
        let config_path = home_path.join("config.toml");

        if config_path.exists() {
            if let Ok(contents) = std::fs::read_to_string(&config_path) {
                if let Ok(config) = toml::from_str::<MsgvaultConfig>(&contents) {
                    if !config.server_url.is_empty() {
                        // Verify the server is reachable
                        if ping_server(&config.server_url).await {
                            steps.push(DiscoveryStep {
                                name: step_name,
                                status: DiscoveryStepStatus::Found(config.server_url.clone()),
                            });

                            return Some(DiscoveryResult {
                                server_url: Some(config.server_url),
                                api_key: config.api_key,
                                source: DiscoverySource::EnvVar,
                                steps: steps.clone(),
                            });
                        }
                    }
                }
            }
        }

        steps.push(DiscoveryStep {
            name: step_name,
            status: DiscoveryStepStatus::NotFound,
        });
    } else {
        steps.push(DiscoveryStep {
            name: step_name,
            status: DiscoveryStepStatus::NotFound,
        });
    }

    None
}

/// Check default config file locations
async fn check_config_files(steps: &mut Vec<DiscoveryStep>) -> Option<DiscoveryResult> {
    let config_paths = get_config_paths();

    for path in config_paths {
        let step_name = format!("Config: {}", path.display());

        if path.exists() {
            if let Ok(contents) = std::fs::read_to_string(&path) {
                if let Ok(config) = toml::from_str::<MsgvaultConfig>(&contents) {
                    if !config.server_url.is_empty() {
                        // Verify the server is reachable
                        if ping_server(&config.server_url).await {
                            steps.push(DiscoveryStep {
                                name: step_name,
                                status: DiscoveryStepStatus::Found(config.server_url.clone()),
                            });

                            return Some(DiscoveryResult {
                                server_url: Some(config.server_url),
                                api_key: config.api_key,
                                source: DiscoverySource::ConfigFile(path),
                                steps: steps.clone(),
                            });
                        }
                    }
                }
            }
        }

        steps.push(DiscoveryStep {
            name: step_name,
            status: DiscoveryStepStatus::NotFound,
        });
    }

    None
}

/// Probe localhost ports for running server
async fn probe_localhost(steps: &mut Vec<DiscoveryStep>) -> Option<DiscoveryResult> {
    let ports = [8080, 8081, 3000, 9000];

    for port in ports {
        let url = format!("http://localhost:{}", port);
        let step_name = format!("Probe: localhost:{}", port);

        if ping_server(&url).await {
            steps.push(DiscoveryStep {
                name: step_name,
                status: DiscoveryStepStatus::Found(url.clone()),
            });

            return Some(DiscoveryResult {
                server_url: Some(url),
                api_key: None,
                source: DiscoverySource::LocalhostProbe(port),
                steps: steps.clone(),
            });
        }

        steps.push(DiscoveryStep {
            name: step_name,
            status: DiscoveryStepStatus::NotFound,
        });
    }

    None
}

/// Get list of default config paths to check
fn get_config_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    // XDG config directory
    if let Some(config_dir) = directories::BaseDirs::new().map(|d| d.config_dir().to_path_buf()) {
        paths.push(config_dir.join("msgvault").join("config.toml"));
    }

    // Home directory
    if let Some(home_dir) = directories::BaseDirs::new().map(|d| d.home_dir().to_path_buf()) {
        paths.push(home_dir.join(".msgvault").join("config.toml"));
        paths.push(home_dir.join(".config").join("msgvault").join("config.toml"));
    }

    // Application-specific directory (ProjectDirs)
    if let Some(proj_dirs) =
        directories::ProjectDirs::from("com", "msgvault", "msgvault-desktop")
    {
        paths.push(proj_dirs.config_dir().join("config.toml"));
    }

    paths
}

/// Ping a server to check if it's reachable
async fn ping_server(url: &str) -> bool {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(2))
        .build()
        .ok();

    let client = match client {
        Some(c) => c,
        None => return false,
    };

    let health_url = format!("{}/api/v1/health", url.trim_end_matches('/'));

    match client.get(&health_url).send().await {
        Ok(response) => response.status().is_success(),
        Err(_) => false,
    }
}

/// Msgvault config structure (for reading existing configs)
#[derive(Debug, serde::Deserialize)]
struct MsgvaultConfig {
    #[serde(default)]
    server_url: String,
    #[serde(default)]
    api_key: Option<String>,
}
