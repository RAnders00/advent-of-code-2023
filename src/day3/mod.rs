mod schematic_parser;

pub use schematic_parser::*;

use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use tracing::info;

#[derive(clap::Args, Debug)]
pub struct Args {
    /// Input data from the puzzle (schematic text file).
    /// Empty lines are ignored.
    pub input: PathBuf,
}

pub fn run(args: Args) -> Result<()> {
    let input: String = fs::read_to_string(&args.input)
        .with_context(|| format!("While trying to read file {}", args.input.display()))?;

    let schematic = input.parse::<Schematic>()?;

    let part_numbers_sum: u64 = schematic
        .part_numbers
        .iter()
        .map(|part| part.part_number)
        .sum();
    info!("(Part 1) Sum of all part numbers: {part_numbers_sum}");

    let gear_ratio_sum: u64 = schematic.gears.iter().map(|gear| gear.gear_ratio()).sum();
    info!("(Part 2) Sum of all gear ratios: {gear_ratio_sum}");

    Ok(())
}
