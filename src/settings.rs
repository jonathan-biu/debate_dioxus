use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Settings {
    pub language: String,
    pub theme: String,
    pub speech_timer_default: u32,
    pub enable_sound: bool,
    pub font_size: String,
    pub include_rebuttal: bool,
    pub include_poi: bool,
    pub always_on_top: bool,
    pub turso_url: String,
    pub turso_token: String,
}

#[allow(dead_code)]
pub const DEFAULT_SETTINGS: Settings = Settings {
    language: String::new(),
    theme: String::new(),
    speech_timer_default: 7,
    enable_sound: true,
    font_size: String::new(),
    include_rebuttal: true,
    include_poi: true,
    always_on_top: false,
    turso_url: String::new(),
    turso_token: String::new(),
};

impl Default for Settings {
    fn default() -> Self {
        Settings {
            language: "en".into(),
            theme: "light".into(),
            speech_timer_default: 7,
            enable_sound: true,
            font_size: "medium".into(),
            include_rebuttal: true,
            include_poi: true,
            always_on_top: false,
            turso_url: String::new(),
            turso_token: String::new(),
        }
    }
}

fn settings_path() -> std::path::PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("debate_dioxus")
        .join("settings.toml")
}

pub fn load() -> Settings {
    fs::read_to_string(settings_path())
        .ok()
        .and_then(|s| toml::from_str(&s).ok())
        .unwrap_or_default()
}

pub fn save(s: &Settings) {
    let path = settings_path();
    if let Some(dir) = path.parent() {
        let _ = fs::create_dir_all(dir);
    }
    if let Ok(toml) = toml::to_string_pretty(s) {
        let _ = fs::write(path, toml);
    }
}
