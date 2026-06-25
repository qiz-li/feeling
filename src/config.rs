use chrono::Weekday;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    pub view: Option<String>,
    pub sunday_start: Option<bool>,
    pub data_path: Option<String>,
    pub chars: Option<CharsConfig>,
}

#[derive(Debug, Default, Deserialize)]
pub struct CharsConfig {
    pub filled: Option<String>,
    pub empty: Option<String>,
    pub year: Option<YearCharsConfig>,
}

#[derive(Debug, Default, Deserialize)]
pub struct YearCharsConfig {
    pub filled: Option<String>,
    pub empty: Option<String>,
}

impl Config {
    pub fn load() -> Self {
        let path = config_path();
        match std::fs::read_to_string(&path) {
            Ok(content) => match toml::from_str(&content) {
                Ok(config) => config,
                Err(e) => {
                    eprintln!("warning: failed to parse {}: {e}", path.display());
                    Self::default()
                }
            },
            Err(_) => Self::default(),
        }
    }

    pub fn resolved_view(&self) -> Option<String> {
        env_non_empty("FEELING_VIEW")
            .or_else(|| self.view.clone())
    }

    pub fn resolved_data_path(&self) -> Option<String> {
        env_non_empty("FEELING_DATA_PATH")
            .or_else(|| self.data_path.clone())
    }

    pub fn week_start(&self) -> Weekday {
        let sunday = std::env::var("FEELING_SUNDAY_START")
            .ok()
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .or(self.sunday_start)
            .unwrap_or(false);

        if sunday { Weekday::Sun } else { Weekday::Mon }
    }

    pub fn filled_char(&self) -> String {
        env_non_empty("FEELING_FILLED_CHAR")
            .or_else(|| self.chars.as_ref()?.filled.clone())
            .unwrap_or_else(|| "●".into())
    }

    pub fn empty_char(&self) -> String {
        env_non_empty("FEELING_EMPTY_CHAR")
            .or_else(|| self.chars.as_ref()?.empty.clone())
            .unwrap_or_else(|| "◯".into())
    }

    pub fn year_filled_char(&self) -> String {
        env_non_empty("FEELING_YEAR_FILLED_CHAR")
            .or_else(|| self.chars.as_ref()?.year.as_ref()?.filled.clone())
            .or_else(|| env_non_empty("FEELING_FILLED_CHAR"))
            .or_else(|| self.chars.as_ref()?.filled.clone())
            .unwrap_or_else(|| "●".into())
    }

    pub fn year_empty_char(&self) -> String {
        env_non_empty("FEELING_YEAR_EMPTY_CHAR")
            .or_else(|| self.chars.as_ref()?.year.as_ref()?.empty.clone())
            .or_else(|| env_non_empty("FEELING_EMPTY_CHAR"))
            .or_else(|| self.chars.as_ref()?.empty.clone())
            .unwrap_or_else(|| "·".into())
    }
}

fn config_path() -> PathBuf {
    if let Ok(p) = std::env::var("FEELING_CONFIG_PATH") {
        return PathBuf::from(p);
    }

    // Prefer XDG_CONFIG_HOME, fall back to ~/.config (cross-platform)
    let config_dir = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
            PathBuf::from(home).join(".config")
        });

    config_dir.join("feeling").join("config.toml")
}

fn env_non_empty(key: &str) -> Option<String> {
    std::env::var(key).ok().filter(|s| !s.is_empty())
}
