use crate::display::{feeling_color, use_color, DisplayChars};
use crate::model::Entry;
use chrono::{Datelike, Duration, Local, NaiveDate, Weekday};
use crossterm::style::{self, Stylize};
use std::collections::HashMap;
use std::io::{self, Write};

pub fn render(entries: &[Entry], chars: &DisplayChars, week_start: Weekday, show_stats: bool) -> io::Result<()> {
    let stdout = io::stdout();
    let mut out = stdout.lock();
    let colored = use_color();

    let today = Local::now().date_naive();
    let start = find_week_start(today, week_start);

    let map: HashMap<NaiveDate, u8> = entries.iter().map(|e| (e.date, e.feeling)).collect();

    writeln!(out)?;

    let mut date = start;
    let mut line = String::new();

    while date <= today {
        let ch = if let Some(&feeling) = map.get(&date) {
            if colored {
                format!("{}", style::style(&chars.filled).with(feeling_color(feeling)))
            } else {
                chars.filled.clone()
            }
        } else {
            chars.empty.clone()
        };

        line.push(' ');
        line.push_str(&ch);
        line.push(' ');

        date += Duration::days(1);
    }

    writeln!(out, "{line}")?;

    if show_stats {
        let stats = crate::stats::Stats::from_entries(entries, start, today);
        writeln!(out, "{stats}")?;
    }

    writeln!(out)?;
    Ok(())
}

fn find_week_start(today: NaiveDate, week_start: Weekday) -> NaiveDate {
    let mut start = today;
    while start.weekday() != week_start {
        start -= Duration::days(1);
    }
    start
}
