mod scratchcard;

use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use tracing::info;

use crate::day4::scratchcard::Scratchcard;

#[derive(clap::Args, Debug)]
pub struct Args {
    /// Input data from the puzzle (schematic text file).
    /// Empty lines are ignored.
    pub input: PathBuf,
}

pub fn run(args: Args) -> Result<()> {
    let input: String = fs::read_to_string(&args.input)
        .with_context(|| format!("While trying to read file {}", args.input.display()))?;

    let mut scratchcards = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.parse::<Scratchcard>()
                .with_context(|| format!("Failed to parse scratchcard `{}`", line))
        })
        .collect::<Result<Vec<_>>>()?;

    let sum_of_points = scratchcards
        .iter()
        .map(|scratchcard| scratchcard.points())
        .sum::<Result<u64>>()?;

    info!("(Part 1) Sum of points: {sum_of_points}");

    for scratchcard_idx in 0..scratchcards.len() {
        let scratchcard = &scratchcards[scratchcard_idx];
        let num_matches = scratchcard.num_matches();
        let scratchcard_copies = scratchcard.copies;

        for following_scratchcard in scratchcards
            .iter_mut()
            .skip(scratchcard_idx + 1)
            .take(num_matches)
        {
            // For each copy we have of this scratchcard, we win a copy of the next N scratchcards
            // where N is the number of matching numbers on the scratchcard.
            following_scratchcard.copies += scratchcard_copies;
        }
    }

    let num_scratchcards = scratchcards
        .iter()
        .map(|scratchcard| scratchcard.copies)
        .sum::<u64>();
    info!("(Part 2) Number of scratchcards after following proper rules: {num_scratchcards}");

    Ok(())
}
