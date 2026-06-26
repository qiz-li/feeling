mod cli;
mod config;
mod display;
mod model;
mod storage;

use chrono::{Local, NaiveDate};
use clap::Parser;
use cli::{Cli, Command, View};
use config::Config;
use display::DisplayChars;
use model::Entry;
use std::io::{self, BufRead, Write};
use std::process;

fn main() {
    let cli = Cli::parse();
    let config = Config::load();

    let path = cli
        .data_path
        .or_else(|| config.resolved_data_path())
        .map(Into::into)
        .unwrap_or_else(storage::default_data_path);

    storage::migrate_legacy(&path);

    let mut entries = match storage::read_entries(&path) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("error: {e}");
            process::exit(1);
        }
    };

    let chars = DisplayChars::from_config(&config);
    let year_chars = DisplayChars::year_from_config(&config);
    let week_start = config.week_start();

    match cli.command {
        Some(Command::Week) => render_or_exit(display::week::render(&entries, &chars, week_start)),
        Some(Command::Month) => render_or_exit(display::month::render(&entries, &chars, week_start)),
        Some(Command::Year) => render_or_exit(display::year::render(&entries, &year_chars, week_start)),
        Some(Command::Prompt) => {
            let today = Local::now().date_naive();
            let force_color = display::use_color_force();
            if let Some(entry) = entries.iter().find(|e| e.date == today) {
                if force_color {
                    use crossterm::style::{self, Stylize};
                    print!("{}", style::style(&chars.filled).with(display::feeling_color(entry.feeling)));
                } else {
                    print!("{}", chars.filled);
                }
            } else {
                print!("{}", chars.empty);
            }
        }
        Some(Command::Remove { date }) => {
            let target = parse_date_or_today(date.as_deref());
            let existing = entries.iter().find(|e| e.date == target);
            if existing.is_none() {
                eprintln!("no entry found for {target}");
                process::exit(1);
            }
            if !cli.yes {
                let feeling = existing.unwrap().feeling;
                if !confirm(&format!("remove entry for {target} (feeling: {feeling})?")) {
                    process::exit(0);
                }
            }
            entries.retain(|e| e.date != target);
            if let Err(e) = storage::write_entries(&path, &entries) {
                eprintln!("error: {e}");
                process::exit(1);
            }
        }
        Some(Command::Export) => {
            println!("date,feeling");
            for entry in &entries {
                println!("{}", entry.to_csv_row());
            }
        }
        None => {
            if let Some(feeling) = cli.feeling {
                let date = parse_date_or_today(cli.date.as_deref());
                let entry = match Entry::new(date, feeling) {
                    Ok(e) => e,
                    Err(e) => {
                        eprintln!("error: {e}");
                        process::exit(1);
                    }
                };

                if let Some(pos) = entries.iter().position(|e| e.date == date) {
                    if !cli.yes {
                        let old = entries[pos].feeling;
                        if !confirm(&format!("overwrite {date} (currently {old}) with {feeling}?")) {
                            process::exit(0);
                        }
                    }
                    entries[pos] = entry;
                } else {
                    let pos = entries.partition_point(|e| e.date < date);
                    entries.insert(pos, entry);
                }

                if let Err(e) = storage::write_entries(&path, &entries) {
                    eprintln!("error: {e}");
                    process::exit(1);
                }
            } else {
                let default_view = cli
                    .view
                    .or_else(|| view_from_str(config.resolved_view()?.as_str()))
                    .unwrap_or(View::Month);

                match default_view {
                    View::Week => render_or_exit(display::week::render(&entries, &chars, week_start)),
                    View::Month => render_or_exit(display::month::render(&entries, &chars, week_start)),
                    View::Year => render_or_exit(display::year::render(&entries, &year_chars, week_start)),
                }
            }
        }
    }
}

fn render_or_exit(result: std::io::Result<()>) {
    if let Err(e) = result {
        eprintln!("error: {e}");
        process::exit(1);
    }
}

fn view_from_str(s: &str) -> Option<View> {
    match s.to_lowercase().as_str() {
        "week" => Some(View::Week),
        "month" => Some(View::Month),
        "year" => Some(View::Year),
        _ => None,
    }
}

fn confirm(msg: &str) -> bool {
    eprint!("{msg} [y/N] ");
    io::stderr().flush().unwrap();
    let mut line = String::new();
    if io::stdin().lock().read_line(&mut line).is_err() {
        return false;
    }
    matches!(line.trim(), "y" | "Y" | "yes" | "YES")
}

fn parse_date_or_today(date_str: Option<&str>) -> NaiveDate {
    match date_str {
        Some(s) => match NaiveDate::parse_from_str(s, "%Y-%m-%d") {
            Ok(d) => d,
            Err(_) => {
                eprintln!("invalid date format: {s} (expected YYYY-MM-DD)");
                process::exit(1);
            }
        },
        None => Local::now().date_naive(),
    }
}
