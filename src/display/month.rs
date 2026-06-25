use crate::display::{feeling_color, use_color, DisplayChars};
use crate::model::Entry;
use chrono::{Datelike, Duration, Local, NaiveDate, Weekday};
use crossterm::style::{self, Stylize};
use std::collections::HashMap;
use std::io::{self, Write};

pub fn render(entries: &[Entry], chars: &DisplayChars, week_start: Weekday) -> io::Result<()> {
    let stdout = io::stdout();
    let mut out = stdout.lock();
    let colored = use_color();

    let today = Local::now().date_naive();
    let week_end = week_end_day(week_start);
    let start = find_start_date(today, week_start);

    let map: HashMap<NaiveDate, u8> = entries.iter().map(|e| (e.date, e.feeling)).collect();

    writeln!(out)?;

    let mut date = start;
    let mut week = String::new();

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

        week.push(' ');
        week.push_str(&ch);
        week.push(' ');

        if date.weekday() == week_end {
            writeln!(out, "{week}")?;
            writeln!(out)?;
            week.clear();
        }

        date += Duration::days(1);
    }

    if !week.is_empty() {
        writeln!(out, "{week}")?;
    }

    writeln!(out)?;
    Ok(())
}

fn find_start_date(today: NaiveDate, week_start: Weekday) -> NaiveDate {
    let mut start = today - Duration::days(27);
    while start.weekday() != week_start {
        start += Duration::days(1);
    }
    start
}

fn week_end_day(week_start: Weekday) -> Weekday {
    match week_start {
        Weekday::Mon => Weekday::Sun,
        Weekday::Sun => Weekday::Sat,
        other => other.pred(),
    }
}
