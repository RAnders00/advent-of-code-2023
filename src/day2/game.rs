use crate::day2::Draw;
use anyhow::{Context, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

/// A single game of draw-the-cubes.
#[derive(Debug, PartialEq, Eq)]
pub struct Game {
    pub id: u64,
    /// List of subsets of cubes that were revealed from the bag
    pub draws: Vec<Draw>,
}

lazy_static! {
    // https://regex101.com/r/bccoKD/1
    // Capture group 1 = Game ID
    // Capture group 2 = Unparsed List of Draws (ensures somewhat proper format though)
    static ref GAME_STR_FORMAT: Regex = Regex::new(r"^Game (\d+): ((?:\d+ (?:red|green|blue)(?:[,;] )?)+)$").unwrap();
}

impl FromStr for Game {
    type Err = anyhow::Error;

    /// Parses a string like `Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green`
    fn from_str(input: &str) -> Result<Game> {
        let captures = GAME_STR_FORMAT
            .captures(input)
            .with_context(|| format!("Game `{}` is of invalid format", input))?;

        let game_id = captures
            .get(1)
            .unwrap()
            .as_str()
            .parse::<u64>()
            .with_context(|| format!("Game ID in `{}` is not valid", input))?;
        let all_draws_str = captures.get(2).unwrap().as_str();

        let draws = all_draws_str
            .split("; ")
            .map(Draw::from_str)
            .collect::<Result<Vec<Draw>>>()
            .with_context(|| format!("A draw in game `{}` has an invalid format", input))?;

        Ok(Game { id: game_id, draws })
    }
}

impl Game {
    /// Returns whether this game's draws had been theoretically possible if the given number of
    /// red, green and blue cubes were in a bag.
    pub fn was_possible(&self, max_red: u8, max_green: u8, max_blue: u8) -> bool {
        self.draws.iter().all(|draw| {
            draw.num_red <= max_red && draw.num_green <= max_green && draw.num_blue <= max_blue
        })
    }

    /// Given the draws in this game, finds what amount of cubes would have had
    /// to be in the bag for all draws in this game to be possible.
    ///
    /// Panics if this game has no draws.
    pub fn minimum_bag_contents(&self) -> Draw {
        self.draws
            .iter()
            .fold(Draw::default(), |previous_max, curr| Draw {
                num_red: u8::max(previous_max.num_red, curr.num_red),
                num_green: u8::max(previous_max.num_green, curr.num_green),
                num_blue: u8::max(previous_max.num_blue, curr.num_blue),
            })
    }

    /// First finds the [`minimum_bag_contents`], then calculates the product
    /// `num_red * num_green * num_blue`.
    pub fn calculate_power(&self) -> u32 {
        let minimum_bag_contents = self.minimum_bag_contents();
        (minimum_bag_contents.num_red as u32)
            * (minimum_bag_contents.num_green as u32)
            * (minimum_bag_contents.num_blue as u32)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_game_example_data_game1() {
        let game_str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        assert_eq!(
            game_str.parse::<Game>().unwrap(),
            Game {
                id: 1,
                draws: vec! {
                    Draw {
                        num_red: 4,
                        num_green: 0,
                        num_blue: 3,
                    },
                    Draw {
                        num_red: 1,
                        num_green: 2,
                        num_blue: 6,
                    },
                    Draw {
                        num_red: 0,
                        num_green: 2,
                        num_blue: 0,
                    }
                }
            }
        );
    }

    #[test]
    fn test_parse_game_example_data_game2() {
        let game_str = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        assert_eq!(
            game_str.parse::<Game>().unwrap(),
            Game {
                id: 2,
                draws: vec! {
                    Draw {
                        num_red: 0,
                        num_green: 2,
                        num_blue: 1,
                    },
                    Draw {
                        num_red: 1,
                        num_green: 3,
                        num_blue: 4,
                    },
                    Draw {
                        num_red: 0,
                        num_green: 1,
                        num_blue: 1,
                    }
                }
            }
        );
    }

    #[test]
    fn test_parse_game_example_data_game3() {
        let game_str = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        assert_eq!(
            game_str.parse::<Game>().unwrap(),
            Game {
                id: 3,
                draws: vec! {
                    Draw {
                        num_red: 20,
                        num_green: 8,
                        num_blue: 6,
                    },
                    Draw {
                        num_red: 4,
                        num_green: 13,
                        num_blue: 5,
                    },
                    Draw {
                        num_red: 1,
                        num_green: 5,
                        num_blue: 0,
                    }
                }
            }
        );
    }

    #[test]
    fn test_parse_game_example_data_game4() {
        let game_str = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        assert_eq!(
            game_str.parse::<Game>().unwrap(),
            Game {
                id: 4,
                draws: vec! {
                    Draw {
                        num_red: 3,
                        num_green: 1,
                        num_blue: 6,
                    },
                    Draw {
                        num_red: 6,
                        num_green: 3,
                        num_blue: 0,
                    },
                    Draw {
                        num_red: 14,
                        num_green: 3,
                        num_blue: 15,
                    }
                }
            }
        );
    }

    #[test]
    fn test_parse_game_example_data_game5() {
        let game_str = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(
            game_str.parse::<Game>().unwrap(),
            Game {
                id: 5,
                draws: vec! {
                    Draw {
                        num_red: 6,
                        num_green: 3,
                        num_blue: 1,
                    },
                    Draw {
                        num_red: 1,
                        num_green: 2,
                        num_blue: 2,
                    },
                }
            }
        );
    }

    #[test]
    fn test_parse_game_single_draw() {
        let game_str = "Game 6: 6 red, 1 blue, 3 green";
        assert_eq!(
            game_str.parse::<Game>().unwrap(),
            Game {
                id: 6,
                draws: vec! {
                    Draw {
                        num_red: 6,
                        num_green: 3,
                        num_blue: 1,
                    },
                }
            }
        );
    }

    #[test]
    fn test_parse_game_single_draw_single_color() {
        let game_str = "Game 7: 4 green";
        assert_eq!(
            game_str.parse::<Game>().unwrap(),
            Game {
                id: 7,
                draws: vec! {
                    Draw {
                        num_red: 0,
                        num_green: 4,
                        num_blue: 0,
                    },
                }
            }
        );
    }

    #[test]
    fn test_parse_game_bad_input_no_draws() {
        assert!("Game 5: ".parse::<Game>().is_err());
    }

    #[test]
    fn test_parse_game_single_draw_max_values() {
        let game_str = "Game 6: 255 red, 255 blue, 255 green";
        assert_eq!(
            game_str.parse::<Game>().unwrap(),
            Game {
                id: 6,
                draws: vec! {
                    Draw {
                        num_red: 255,
                        num_green: 255,
                        num_blue: 255,
                    },
                }
            }
        );
    }

    #[test]
    fn test_parse_game_bad_input_too_large_draws() {
        assert!("Game 5: 256 red".parse::<Game>().is_err());
    }

    #[test]
    fn test_parse_game_bad_input_zero() {
        assert!("Game 5: 0 red".parse::<Game>().is_err());
        assert!("Game 5: 0 red, 0 green".parse::<Game>().is_err());
        assert!("Game 5: 0 red, 0 green, 0 blue".parse::<Game>().is_err());
        assert!("Game 5: 1 red, 0 green".parse::<Game>().is_err());
        assert!("Game 5: 1 red, 1 green, 0 green".parse::<Game>().is_err());
        assert!("Game 5: 1 red, 0 green, 1 green".parse::<Game>().is_err());
    }

    #[test]
    fn test_parse_game_bad_color_specified_twice() {
        assert!("Game 5: 2 red, 3 red".parse::<Game>().is_err());
        assert!("Game 5: 1 green, 2 red, 3 red".parse::<Game>().is_err());
        assert!("Game 5: 1 green, 2 red, 15 blue, 3 red"
            .parse::<Game>()
            .is_err());
    }

    #[test]
    fn test_parse_game_bad_negative_numbers() {
        assert!("Game 5: -1 red".parse::<Game>().is_err());
        assert!("Game -1: 5 red".parse::<Game>().is_err());
    }

    #[test]
    fn test_possible_single_draw() {
        let game = Game {
            id: 17,
            draws: vec![Draw {
                num_red: 4,
                num_green: 0,
                num_blue: 3,
            }],
        };

        assert!(game.was_possible(4, 0, 3));
        assert!(game.was_possible(5, 1, 4));
        assert!(!game.was_possible(3, 0, 3));
        assert!(!game.was_possible(3, 0, 2));
        assert!(!game.was_possible(0, 0, 0));
    }

    #[test]
    fn test_possible_multiple_draws() {
        let game = Game {
            id: 100,
            draws: vec![
                Draw {
                    num_red: 3,
                    num_green: 6,
                    num_blue: 3,
                },
                Draw {
                    num_red: 7,
                    num_green: 2,
                    num_blue: 16,
                },
                Draw {
                    num_red: 9,
                    num_green: 14,
                    num_blue: 9,
                },
                Draw {
                    num_red: 8,
                    num_green: 10,
                    num_blue: 9,
                },
                Draw {
                    num_red: 11,
                    num_green: 0,
                    num_blue: 6,
                },
            ],
        };

        assert!(game.was_possible(11, 14, 16));
        assert!(game.was_possible(20, 30, 50));
        assert!(!game.was_possible(10, 14, 16));
        assert!(!game.was_possible(11, 13, 16));
        assert!(!game.was_possible(11, 14, 15));
        assert!(!game.was_possible(12, 13, 14));
        assert!(!game.was_possible(1, 1, 1));
        assert!(!game.was_possible(0, 0, 0));
    }

    #[test]
    fn test_game_minimum_bag_contents_and_power_1() {
        let game = Game {
            id: 17,
            draws: vec![Draw {
                num_red: 4,
                num_green: 0,
                num_blue: 3,
            }],
        };
        assert_eq!(
            game.minimum_bag_contents(),
            Draw {
                num_red: 4,
                num_green: 0,
                num_blue: 3
            }
        );
        assert_eq!(game.calculate_power(), 0);
    }

    #[test]
    fn test_game_minimum_bag_contents_and_power_2() {
        let game = Game {
            id: 100,
            draws: vec![
                Draw {
                    num_red: 3,
                    num_green: 6,
                    num_blue: 3,
                },
                Draw {
                    num_red: 7,
                    num_green: 2,
                    num_blue: 16,
                },
                Draw {
                    num_red: 9,
                    num_green: 14,
                    num_blue: 9,
                },
                Draw {
                    num_red: 8,
                    num_green: 10,
                    num_blue: 9,
                },
                Draw {
                    num_red: 11,
                    num_green: 0,
                    num_blue: 6,
                },
            ],
        };

        assert_eq!(
            game.minimum_bag_contents(),
            Draw {
                num_red: 11,
                num_green: 14,
                num_blue: 16
            }
        );
        assert_eq!(game.calculate_power(), 11 * 14 * 16);
    }

    #[test]
    fn test_game_minimum_bag_contents_and_power_empty() {
        let game = Game {
            id: 100,
            draws: vec![],
        };

        assert_eq!(
            game.minimum_bag_contents(),
            Draw {
                num_red: 0,
                num_green: 0,
                num_blue: 0
            }
        );
        assert_eq!(game.calculate_power(), 0);
    }
}
