use anyhow::{bail, ensure, Context, Result};
use std::str::FromStr;

/// Subset of cubes that were revealed from the bag
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Draw {
    /// Number of red cubes in this draw
    pub num_red: u8,
    /// Number of green cubes in this draw
    pub num_green: u8,
    /// Number of blue cubes in this draw
    pub num_blue: u8,
}

impl FromStr for Draw {
    type Err = anyhow::Error;

    /// Parses a string like `3 blue, 4 red`, `2 green` or `1 red, 2 green, 6 blue`
    fn from_str(draw_str: &str) -> Result<Draw> {
        let mut draw = Draw::default(); // Initializes a new `Draw` with everything set to 0

        // `single_draw_str` is e.g `3 blue`, `1 red` or `14 green`
        for single_draw_str in draw_str.split(", ") {
            let (num_str, color_str) = single_draw_str.split_once(' ').with_context(|| {
                format!(
                    "While parsing draw `{}`: No space between number and color in `{}`",
                    draw_str, single_draw_str
                )
            })?;
            let num = num_str.parse::<u8>().with_context(|| {
                format!(
                    "While parsing draw `{}`: In single draw `{}`: Number `{}` is not valid",
                    draw_str, single_draw_str, num_str
                )
            })?;
            ensure!(
                num > 0,
                "While parsing draw `{}`: In single draw `{}`: Cannot specify that zero were drawn",
                draw_str,
                single_draw_str
            );
            let struct_field = match color_str {
                "red" => &mut draw.num_red,
                "green" => &mut draw.num_green,
                "blue" => &mut draw.num_blue,
                _ => bail!(
                    "While parsing draw `{}`: In single draw `{}`: Color `{}` is not valid",
                    draw_str,
                    single_draw_str,
                    color_str
                ),
            };
            ensure!(
                *struct_field == 0,
                "While parsing draw `{}`: Multiple instances of {} draw",
                draw_str,
                color_str
            );
            *struct_field += num;
        }

        ensure!(
            draw.num_red > 0 || draw.num_green > 0 || draw.num_blue > 0,
            "While parsing draw `{}`: No cubes were drawn (empty string)",
            draw_str
        );

        Ok(draw)
    }
}
