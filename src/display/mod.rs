pub mod month;
pub mod week;
pub mod year;

use crate::config::Config;
use crossterm::style::Color;
use std::io::IsTerminal;

pub fn feeling_color(feeling: u8) -> Color {
    match feeling {
        7..=10 => Color::DarkGreen,
        4..=6 => Color::DarkYellow,
        1..=3 => Color::DarkRed,
        _ => Color::DarkGrey,
    }
}

pub fn use_color() -> bool {
    !no_color_set() && std::io::stdout().is_terminal()
}

/// For prompt integration — color unless NO_COLOR is set, ignores TTY check
pub fn use_color_force() -> bool {
    !no_color_set()
}

fn no_color_set() -> bool {
    std::env::var_os("NO_COLOR").is_some_and(|v| !v.is_empty())
}

pub struct DisplayChars {
    pub filled: String,
    pub empty: String,
}

impl DisplayChars {
    pub fn from_config(config: &Config) -> Self {
        Self {
            filled: config.filled_char(),
            empty: config.empty_char(),
        }
    }

    pub fn year_from_config(config: &Config) -> Self {
        Self {
            filled: config.year_filled_char(),
            empty: config.year_empty_char(),
        }
    }
}
