use clap::Parser;

/// Simple Advent of Code CLI
#[derive(Parser, Debug)]
pub struct Cli {
    /// Day number (1–12)
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=12))]
    pub day: u8,

    /// Part number (1 or 2)
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=2))]
    pub part: u8,
}
