mod cli;
mod days;
mod utils;

use clap::Parser;
use cli::Cli;

fn main() {
    let args = Cli::parse();
    println!("Running day {}, part {}", args.day, args.part);

    days::run(args.day, args.part);
}
