use crate::transform::Transform;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotkeyConfig {
    pub modifiers: Vec<String>,
    pub key: String,
    pub transform: Transform,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub hotkeys: Vec<HotkeyConfig>,
    pub settings_hotkey: HotkeyConfig,
    pub restore_clipboard: bool,
    pub clipboard_delay_ms: u64,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            hotkeys: vec![
                HotkeyConfig {
                    modifiers: vec!["CONTROL".to_string(), "SHIFT".to_string()],
                    key: "KeyR".to_string(),
                    transform: Transform::ToRussian,
                },
                HotkeyConfig {
                    modifiers: vec!["CONTROL".to_string(), "SHIFT".to_string()],
                    key: "KeyE".to_string(),
                    transform: Transform::ToEnglish,
                },
                HotkeyConfig {
                    modifiers: vec!["CONTROL".to_string(), "SHIFT".to_string()],
                    key: "KeyT".to_string(),
                    transform: Transform::ToggleLayout,
                },
                HotkeyConfig {
                    modifiers: vec!["CONTROL".to_string(), "SHIFT".to_string()],
                    key: "KeyU".to_string(),
                    transform: Transform::ToggleCase,
                },
                HotkeyConfig {
                    modifiers: vec!["CONTROL".to_string(), "SHIFT".to_string()],
                    key: "KeyC".to_string(),
                    transform: Transform::CleanWhitespace,
                },
            ],
            settings_hotkey: HotkeyConfig {
                modifiers: vec!["CONTROL".to_string(), "SHIFT".to_string()],
                key: "KeyS".to_string(),
                transform: Transform::ToggleLayout,
            },
            restore_clipboard: false,
            clipboard_delay_ms: 150,
        }
    }
}

impl Settings {
    pub fn config_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("txtfrmt");
        path.push("config.toml");
        path
    }

    fn legacy_config_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("textfmt");
        path.push("config.toml");
        path
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        if path.exists() {
            return match fs::read_to_string(&path) {
                Ok(content) => match toml::from_str::<Settings>(&content) {
                    Ok(settings) => settings,
                    Err(error) => {
                        eprintln!(
                            "Invalid config at {}: {error}; using defaults without overwriting it",
                            path.display()
                        );
                        Settings::default()
                    }
                },
                Err(error) => {
                    eprintln!(
                        "Cannot read config at {}: {error}; using defaults",
                        path.display()
                    );
                    Settings::default()
                }
            };
        }

        let legacy_path = Self::legacy_config_path();
        if legacy_path.exists() {
            return match fs::read_to_string(&legacy_path) {
                Ok(content) => match toml::from_str::<Settings>(&content) {
                    Ok(settings) => {
                        eprintln!(
                            "Migrating settings from {} to {}",
                            legacy_path.display(),
                            path.display()
                        );
                        settings.save();
                        settings
                    }
                    Err(error) => {
                        eprintln!(
                            "Invalid legacy config at {}: {error}; using defaults without overwriting it",
                            legacy_path.display()
                        );
                        Settings::default()
                    }
                },
                Err(error) => {
                    eprintln!(
                        "Cannot read legacy config at {}: {error}; using defaults",
                        legacy_path.display()
                    );
                    Settings::default()
                }
            };
        }

        let default = Settings::default();
        default.save();
        default
    }

    pub fn save(&self) {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            if let Err(error) = fs::create_dir_all(parent) {
                eprintln!(
                    "Cannot create config directory {}: {error}",
                    parent.display()
                );
                return;
            }
        }
        if let Ok(content) = toml::to_string_pretty(self) {
            if let Err(error) = fs::write(&path, content) {
                eprintln!("Cannot write config at {}: {error}", path.display());
            }
        }
    }
}
