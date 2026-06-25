use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "feeling", version, about = "A beautiful mood tracker for your terminal")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Log a feeling (1-10) for today
    #[arg(value_parser = clap::value_parser!(u8).range(1..=10))]
    pub feeling: Option<u8>,

    /// Specify a date (YYYY-MM-DD), defaults to today
    #[arg(short, long)]
    pub date: Option<String>,

    /// Override data file path
    #[arg(long)]
    pub data_path: Option<String>,

    /// Default view when no subcommand is given
    #[arg(long, value_enum)]
    pub view: Option<View>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Show the current week
    Week,
    /// Show the last 4 weeks
    Month,
    /// Show the full-year heatmap
    Year,
    /// Output a single glyph for prompt integration (starship, p10k)
    Prompt,
    /// Remove an entry
    Remove {
        /// Date to remove (YYYY-MM-DD), defaults to today
        #[arg(short, long)]
        date: Option<String>,
    },
    /// Export raw CSV to stdout
    Export,
}

#[derive(Clone, ValueEnum)]
pub enum View {
    Week,
    Month,
    Year,
}
