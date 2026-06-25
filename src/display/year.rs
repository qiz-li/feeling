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
    let start = find_year_start(today, week_start);

    let map: HashMap<NaiveDate, u8> = entries.iter().map(|e| (e.date, e.feeling)).collect();

    let mut weeks: Vec<[Option<NaiveDate>; 7]> = Vec::new();
    let mut date = start;
    let mut current_week: [Option<NaiveDate>; 7] = [None; 7];

    let week_end = week_end_day(week_start);

    while date <= today {
        let row = weekday_row(date.weekday(), week_start);
        current_week[row] = Some(date);
        if date.weekday() == week_end || date == today {
            weeks.push(current_week);
            current_week = [None; 7];
        }
        date += Duration::days(1);
    }

    writeln!(out)?;

    for row in 0..7 {
        for week in &weeks {
            match week[row] {
                Some(d) => {
                    if let Some(&feeling) = map.get(&d) {
                        if colored {
                            write!(out, "{}", style::style(&chars.filled).with(feeling_color(feeling)))?;
                        } else {
                            write!(out, "{}", chars.filled)?;
                        }
                    } else {
                        write!(out, "{}", chars.empty)?;
                    }
                }
                None => write!(out, " ")?,
            }
        }
        writeln!(out)?;
    }

    writeln!(out)?;
    Ok(())
}

fn find_year_start(today: NaiveDate, week_start: Weekday) -> NaiveDate {
    let mut start = today - Duration::days(364);
    while start.weekday() != week_start {
        start -= Duration::days(1);
    }
    start
}

fn weekday_row(wd: Weekday, week_start: Weekday) -> usize {
    let start_num = week_start.num_days_from_monday();
    let wd_num = wd.num_days_from_monday();
    ((wd_num + 7 - start_num) % 7) as usize
}

fn week_end_day(week_start: Weekday) -> Weekday {
    match week_start {
        Weekday::Mon => Weekday::Sun,
        Weekday::Sun => Weekday::Sat,
        other => other.pred(),
    }
}
