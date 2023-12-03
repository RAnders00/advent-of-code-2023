mod draw;
mod game;

pub use draw::Draw;
pub use game::Game;

use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use tracing::{debug, info, trace};

const PART1_MAX_RED_CUBES: u8 = 12;
const PART1_MAX_GREEN_CUBES: u8 = 13;
const PART1_MAX_BLUE_CUBES: u8 = 14;

#[derive(clap::Args, Debug)]
pub struct Args {
    /// Input data from the puzzle (list of games).
    /// Empty lines are ignored.
    pub input: PathBuf,
}

pub fn run(args: Args) -> Result<()> {
    let input: String = fs::read_to_string(&args.input)
        .with_context(|| format!("While trying to read file {}", args.input.display()))?;

    let mut sum_of_possible_game_ids: u64 = 0;
    let mut sum_of_powers: u64 = 0;

    for (line_idx, line) in input.lines().enumerate() {
        if line.is_empty() {
            continue;
        }

        let game = line
            .parse::<Game>()
            .with_context(|| format!("While trying to parse line {} (`{}`)", line_idx + 1, line))?;

        let game_was_possible = game.was_possible(
            PART1_MAX_RED_CUBES,
            PART1_MAX_GREEN_CUBES,
            PART1_MAX_BLUE_CUBES,
        );
        let power = game.calculate_power();

        debug!(
            "{}: {}, power = {}",
            line,
            if game_was_possible {
                "possible"
            } else {
                "impossible"
            },
            power
        );
        trace!("(was parsed as {:?})", game);

        if game_was_possible {
            sum_of_possible_game_ids += game.id;
        }
        sum_of_powers += power as u64;
    }

    info!(
        "(Part 1) Sum of all possible games IDs: {}",
        sum_of_possible_game_ids
    );
    info!("(Part 2) Sum of all powers: {}", sum_of_powers);

    Ok(())
}
