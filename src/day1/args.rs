use std::path::PathBuf;

#[derive(clap::Args, Debug)]
pub struct Args {
    /// Input data from the puzzle (messed up "calibration document" data)
    /// Empty lines are ignored.
    pub input: PathBuf,
}
