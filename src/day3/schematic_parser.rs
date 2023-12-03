use anyhow::{Context, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::ops::Range;
use std::str::FromStr;

lazy_static! {
    static ref NUMBER_REGEX: Regex = Regex::new(r"[0-9]+").unwrap();
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Schematic {
    pub part_numbers: Vec<PartNumber>,
    pub gears: Vec<Gear>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartNumber {
    pub part_number: u64,
    pub line_idx: usize,
    // Range in terms of bytes.
    pub range_bytes: Range<usize>,
    // Range in terms of the `chars()` iterator.
    pub range_chars: CharsRange,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gear {
    pub line_idx: usize,
    // Index in terms of bytes.
    pub index_bytes: usize,
    // Index in terms of the `chars()` iterator.
    pub index_chars: usize,
    // The two neighboring part numbers.
    pub neighbors: (PartNumber, PartNumber),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharsRange(pub Range<usize>);

impl FromStr for Schematic {
    type Err = anyhow::Error;

    fn from_str(schematic: &str) -> Result<Schematic> {
        let lines = schematic.lines().collect::<Vec<_>>();
        let mut part_numbers = vec![];

        for (line_idx, &line) in lines.iter().enumerate() {
            // Find all numbers in the line.
            for number_match in NUMBER_REGEX.find_iter(line) {
                let part_number = number_match.as_str().parse::<u64>().with_context(|| {
                    format!(
                        "While parsing line `{}`: `{}` is not a valid unsigned 64 bit integer",
                        line,
                        number_match.as_str()
                    )
                })?;

                // If this number has at least one symbol around it, it is considered
                // to be a "part number", and is therefore returned.
                // A symbol is any character that is not a digit or a dot (".").

                // Find out the chars() offset in the line.
                // (Regex gives us the byte offset, which we need to convert)
                // This implementation respects and correctly handles multi-byte UTF8 characters.
                let match_char_range = CharsRange::from_bytes_range(line, number_match.range());

                let has_adjacent_symbol = is_symbol_left(line, number_match.range())
                    || is_symbol_right(line, number_match.range())
                    || is_symbol_above(&lines, line_idx, match_char_range.clone())
                    || is_symbol_below(&lines, line_idx, match_char_range.clone());

                if has_adjacent_symbol {
                    part_numbers.push(PartNumber {
                        part_number,
                        line_idx,
                        range_bytes: number_match.range(),
                        range_chars: match_char_range,
                    });
                }
            }
        }

        let mut gears = vec![];

        for (line_idx, &line) in lines.iter().enumerate() {
            for (gear_match_index_bytes, _) in line.match_indices('*') {
                let chars_index =
                    CharsRange::bytes_index_to_chars_index(line, gear_match_index_bytes);
                // This is a *potential* gear. We need to check if a number is neighbouring it.

                // If exactly two part numbers neighbour this '*' char, it is considered a gear.
                let mut neighbors: Vec<PartNumber> = part_numbers
                    .iter()
                    .filter(|part| part.is_neighboring_char(line_idx, chars_index))
                    .cloned()
                    .collect();

                if neighbors.len() == 2 {
                    gears.push(Gear {
                        line_idx,
                        index_bytes: gear_match_index_bytes,
                        index_chars: CharsRange::bytes_index_to_chars_index(
                            line,
                            gear_match_index_bytes,
                        ),
                        neighbors: (neighbors.remove(0), neighbors.remove(0)),
                    });
                }
            }
        }

        Ok(Schematic {
            part_numbers,
            gears,
        })
    }
}

impl PartNumber {
    /// Determines whether the a character on the given line at the given position neighbours this part number.
    /// Diagnonal neighbours are included.
    /// `index_chars` is an index in terms of the `chars()` iterator.
    fn is_neighboring_char(&self, line_idx: usize, index_chars: usize) -> bool {
        let is_gear_on_same_line = self.line_idx == line_idx;
        let is_gear_on_line_above = line_idx.checked_add(1) == Some(self.line_idx);
        let is_gear_on_line_below = line_idx.checked_sub(1) == Some(self.line_idx);

        let is_gear_to_left =
            is_gear_on_same_line && self.range_chars.0.start.checked_sub(1) == Some(index_chars);
        let is_gear_to_right = is_gear_on_same_line && self.range_chars.0.end == index_chars;

        let is_gear_above =
            is_gear_on_line_above && self.range_chars.grown_by_one().0.contains(&index_chars);
        let is_gear_below =
            is_gear_on_line_below && self.range_chars.grown_by_one().0.contains(&index_chars);

        is_gear_to_left || is_gear_to_right || is_gear_above || is_gear_below
    }
}

impl Gear {
    pub fn gear_ratio(&self) -> u64 {
        self.neighbors.0.part_number * self.neighbors.1.part_number
    }
}

impl CharsRange {
    /// Given that `bytes_range` refers to a substring in the `input`, determines
    /// what index is necessary to find the same substring in terms of the `chars()`
    /// iterator on `str`.
    fn from_bytes_range(input: &str, bytes_range: Range<usize>) -> CharsRange {
        let start_char_idx = Self::bytes_index_to_chars_index(input, bytes_range.start);
        let end_char_idx = Self::bytes_index_to_chars_index(input, bytes_range.end);

        CharsRange(start_char_idx..end_char_idx)
    }

    fn bytes_index_to_chars_index(input: &str, bytes_index: usize) -> usize {
        input[..bytes_index].chars().count()
    }

    fn grown_by_one(&self) -> Self {
        CharsRange(self.0.start.saturating_sub(1)..self.0.end + 1)
    }
}

/// Returns whether there is a symbol to the left of the given (bytes) range in the string.
/// Returns `false` in case there is no character to the left.
fn is_symbol_left(input: &str, number_bytes_range: Range<usize>) -> bool {
    input[..number_bytes_range.start]
        .chars()
        .last()
        .map(is_symbol)
        .unwrap_or(false)
}

/// Returns whether there is a symbol to the left of the given (bytes) range in the string.
/// Returns `false` in case there is no character to the right.
fn is_symbol_right(input: &str, number_bytes_range: Range<usize>) -> bool {
    input[number_bytes_range.end..]
        .chars()
        .next()
        .map(is_symbol)
        .unwrap_or(false)
}

/// Returns whether a symbol can be found in the line above the line where the number was found.
/// Includes diagonal neighbours.
/// `number_chars_range` is a range in terms of the `chars()` iterator.
fn is_symbol_above(lines: &[&str], number_line_idx: usize, number_chars_range: CharsRange) -> bool {
    is_symbol_in_line_idx(lines, number_line_idx.checked_sub(1), number_chars_range)
}

/// Returns whether a symbol can be found in the line below the line where the number was found.
/// Includes diagonal neighbours.
/// `number_chars_range` is a range in terms of the `chars()` iterator.
fn is_symbol_below(lines: &[&str], number_line_idx: usize, number_chars_range: CharsRange) -> bool {
    is_symbol_in_line_idx(lines, number_line_idx.checked_add(1), number_chars_range)
}

fn is_symbol_in_line_idx(
    lines: &[&str],
    line_idx: Option<usize>,
    number_chars_range: CharsRange,
) -> bool {
    line_idx
        .and_then(|line_idx| lines.get(line_idx))
        .map(|line_above| is_symbol_in_or_next_to_range(line_above, number_chars_range))
        .unwrap_or(false)
}

/// Returns whether a symbol can be found in the given range, expanded by 1 in each direction, in the string.
/// `number_chars_range` is a range in terms of the `chars()` iterator.
fn is_symbol_in_or_next_to_range(input: &str, number_chars_range: CharsRange) -> bool {
    let range_grown_by_one = number_chars_range.grown_by_one();

    input
        .chars()
        .skip(range_grown_by_one.0.start)
        .take(range_grown_by_one.0.len())
        .any(is_symbol)
}

/// Returns whether this character is considered to be a "symbol" for the purposes of this puzzle.
/// This means: Any character that is not a digit (0-9) or a dot (".").
fn is_symbol(input: char) -> bool {
    !input.is_ascii_digit() && input != '.'
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_symbol() {
        assert!(is_symbol('a'));
        assert!(is_symbol('*'));
        assert!(is_symbol('#'));
        assert!(is_symbol('+'));
        assert!(!is_symbol('.'));
        assert!(!is_symbol('0'));
        assert!(!is_symbol('1'));
        assert!(!is_symbol('8'));
        assert!(!is_symbol('9'));
    }

    #[test]
    fn test_is_symbol_above() {
        assert!(!is_symbol_above(
            &vec!["+......", "..123.."],
            1,
            CharsRange(2..5)
        ));
        assert!(is_symbol_above(
            &vec![".+.....", "..123.."],
            1,
            CharsRange(2..5)
        ));
        assert!(is_symbol_above(
            &vec!["..+....", "..123.."],
            1,
            CharsRange(2..5)
        ));
        assert!(is_symbol_above(
            &vec!["...+...", "..123.."],
            1,
            CharsRange(2..5)
        ));
        assert!(is_symbol_above(
            &vec!["....+..", "..123.."],
            1,
            CharsRange(2..5)
        ));
        assert!(is_symbol_above(
            &vec![".....+.", "..123.."],
            1,
            CharsRange(2..5)
        ));
        assert!(is_symbol_above(
            &vec![".+++++.", "..123.."],
            1,
            CharsRange(2..5)
        ));
        assert!(!is_symbol_above(
            &vec!["......+", "..123.."],
            1,
            CharsRange(2..5)
        ));
        assert!(!is_symbol_above(&vec!["", "..123.."], 1, CharsRange(2..5)));
        assert!(!is_symbol_above(&vec![".", "..123.."], 1, CharsRange(2..5)));
        assert!(is_symbol_above(&vec![".+", "..123.."], 1, CharsRange(2..5)));
        assert!(!is_symbol_above(
            &vec!["+.....+", "..123.."],
            1,
            CharsRange(2..5)
        ));
        assert!(!is_symbol_above(&vec!["..123.."], 0, CharsRange(2..5)));
    }

    #[test]
    fn test_is_symbol_above_utf8() {
        assert_eq!("߷".len(), 2);
        assert_eq!("߷".chars().count(), 1);

        // Two-byte char in line 0 in position 0, Single-byte char in line 1
        // The algorithm is expected to locate the same char offset in the string above
        assert_eq!(
            CharsRange::from_bytes_range("...123...", 3..6),
            CharsRange(3..6)
        );
        assert!(is_symbol_above(
            &vec!["߷.+.....", "...123..."],
            1,
            CharsRange(3..6)
        ));

        assert_eq!(
            CharsRange::from_bytes_range("߷..123...", 4..7),
            CharsRange(3..6)
        );
        assert!(is_symbol_above(
            &vec!["..+.....", "߷..123..."],
            1,
            CharsRange(3..6)
        ));

        assert!(is_symbol_above(
            &vec!["......+..", "߷..123..."],
            1,
            CharsRange(3..6)
        ));
        assert!(is_symbol_above(
            &vec!["߷.+.....", "߷..123..."],
            1,
            CharsRange(3..6)
        ));
    }

    #[test]
    fn test_is_symbol_below() {
        assert!(!is_symbol_below(
            &vec!["..123..", "+......"],
            0,
            CharsRange(2..5)
        ));
        assert!(is_symbol_below(
            &vec!["..123..", ".+....."],
            0,
            CharsRange(2..5)
        ));
        assert!(is_symbol_below(
            &vec!["..123..", "..+...."],
            0,
            CharsRange(2..5)
        ));
        assert!(is_symbol_below(
            &vec!["..123..", "...+..."],
            0,
            CharsRange(2..5)
        ));
        assert!(is_symbol_below(
            &vec!["..123..", "....+.."],
            0,
            CharsRange(2..5)
        ));
        assert!(is_symbol_below(
            &vec!["..123..", ".....+."],
            0,
            CharsRange(2..5)
        ));
        assert!(!is_symbol_below(&vec!["..123..", ""], 0, CharsRange(2..5)));
        assert!(!is_symbol_below(&vec!["..123..", "."], 0, CharsRange(2..5)));
        assert!(is_symbol_below(&vec!["..123..", ".+"], 0, CharsRange(2..5)));
        assert!(!is_symbol_below(
            &vec!["..123..", "......+"],
            0,
            CharsRange(2..5)
        ));
        assert!(!is_symbol_below(&vec!["..123.."], 0, CharsRange(2..5)));
    }

    // Not bothering with utf8 test for below method since it's implemented
    // almost the same was as above.

    #[test]
    fn test_is_symbol_left() {
        assert!(!is_symbol_left("..123..", 2..5));
        assert!(!is_symbol_left("+.123.+", 2..5));
        assert!(is_symbol_left(".+123..", 2..5));
        assert!(is_symbol_left(".+123+.", 2..5));
        assert!(!is_symbol_left("..123+.", 2..5));
    }

    #[test]
    fn test_is_symbol_left_utf8() {
        assert_eq!("߷".len(), 2);
        assert_eq!("߷".chars().count(), 1);
        assert!(!is_symbol_left("߷..123..߷", 4..7));
        assert!(!is_symbol_left("߷+.123.+߷", 4..7));
        assert!(is_symbol_left("߷.+123..߷", 4..7));
        assert!(is_symbol_left("߷.+123+.߷", 4..7));
        assert!(!is_symbol_left("߷..123+.߷", 4..7));
    }

    #[test]
    fn test_is_symbol_right() {
        assert!(!is_symbol_right("..123..", 2..5));
        assert!(!is_symbol_right("+.123.+", 2..5));
        assert!(is_symbol_right("..123+.", 2..5));
        assert!(is_symbol_right(".+123+.", 2..5));
        assert!(!is_symbol_right(".+123..", 2..5));
    }

    #[test]
    fn test_is_symbol_right_utf8() {
        assert_eq!("߷".len(), 2);
        assert_eq!("߷".chars().count(), 1);
        assert!(!is_symbol_right("߷..123..߷", 4..7));
        assert!(!is_symbol_right("߷+.123.+߷", 4..7));
        assert!(is_symbol_right("߷..123+.߷", 4..7));
        assert!(is_symbol_right("߷.+123+.߷", 4..7));
        assert!(!is_symbol_right("߷.+123..߷", 4..7));
    }

    #[test]
    fn test_parse_schematic_example_data() {
        // 114 and 58 are not considered schematic symbols. The rest are.
        let example_input = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(
            example_input.parse::<Schematic>().unwrap(),
            Schematic {
                part_numbers: vec![
                    PartNumber {
                        part_number: 467,
                        line_idx: 0,
                        range_bytes: 0..3,
                        range_chars: CharsRange(0..3),
                    },
                    PartNumber {
                        part_number: 35,
                        line_idx: 2,
                        range_bytes: 2..4,
                        range_chars: CharsRange(2..4),
                    },
                    PartNumber {
                        part_number: 633,
                        line_idx: 2,
                        range_bytes: 6..9,
                        range_chars: CharsRange(6..9),
                    },
                    PartNumber {
                        part_number: 617,
                        line_idx: 4,
                        range_bytes: 0..3,
                        range_chars: CharsRange(0..3),
                    },
                    PartNumber {
                        part_number: 592,
                        line_idx: 6,
                        range_bytes: 2..5,
                        range_chars: CharsRange(2..5),
                    },
                    PartNumber {
                        part_number: 755,
                        line_idx: 7,
                        range_bytes: 6..9,
                        range_chars: CharsRange(6..9),
                    },
                    PartNumber {
                        part_number: 664,
                        line_idx: 9,
                        range_bytes: 1..4,
                        range_chars: CharsRange(1..4),
                    },
                    PartNumber {
                        part_number: 598,
                        line_idx: 9,
                        range_bytes: 5..8,
                        range_chars: CharsRange(5..8),
                    },
                ],
                gears: vec![
                    Gear {
                        line_idx: 1,
                        index_bytes: 3,
                        index_chars: 3,
                        neighbors: (
                            PartNumber {
                                part_number: 467,
                                line_idx: 0,
                                range_bytes: 0..3,
                                range_chars: CharsRange(0..3),
                            },
                            PartNumber {
                                part_number: 35,
                                line_idx: 2,
                                range_bytes: 2..4,
                                range_chars: CharsRange(2..4),
                            },
                        )
                    },
                    Gear {
                        line_idx: 8,
                        index_bytes: 5,
                        index_chars: 5,
                        neighbors: (
                            PartNumber {
                                part_number: 755,
                                line_idx: 7,
                                range_bytes: 6..9,
                                range_chars: CharsRange(6..9),
                            },
                            PartNumber {
                                part_number: 598,
                                line_idx: 9,
                                range_bytes: 5..8,
                                range_chars: CharsRange(5..8),
                            },
                        )
                    },
                ]
            }
        );
    }
}
