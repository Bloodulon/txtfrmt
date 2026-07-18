use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use crate::transform::Transform;

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
        path.push("textfmt");
        path.push("config.toml");
        path
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(settings) = toml::from_str(&content) {
                    return settings;
                }
            }
        }
        let default = Settings::default();
        default.save();
        default
    }

    pub fn save(&self) {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(content) = toml::to_string_pretty(self) {
            let _ = fs::write(path, content);
        }
    }
}
