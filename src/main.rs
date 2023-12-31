use std::process::ExitCode;

use advent_of_code_2023::{Args, Day};
use clap::Parser;

fn main() -> ExitCode {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    let res = match args.day {
        Day::Day1(day1_args) => advent_of_code_2023::day1::run(day1_args),
        Day::Day2(day2_args) => advent_of_code_2023::day2::run(day2_args),
        Day::Day3(day3_args) => advent_of_code_2023::day3::run(day3_args),
        Day::Day4(day4_args) => advent_of_code_2023::day4::run(day4_args),
    };

    if let Err(err) = res {
        // {:#} shows the full error context, not just the outermost layer
        tracing::error!("{:#}", err);
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
