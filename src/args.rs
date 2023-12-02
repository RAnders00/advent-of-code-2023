use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Args {
    #[command(subcommand)]
    pub day: Day,
}

#[derive(Subcommand, Debug)]
pub enum Day {
    /// Run the two algorithms for day 1's challenge
    Day1(crate::day1::Args),
    /// Run the algorithm for day 2's challenge
    Day2(crate::day2::Args),
}
