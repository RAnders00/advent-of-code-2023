use anyhow::{anyhow, Context, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct Scratchcard {
    pub winning_numbers: HashSet<u8>,
    pub our_numbers: HashSet<u8>,

    // initially this is 1
    pub copies: u64,
}

lazy_static! {
    // https://regex101.com/r/4MNT2z/3
    // Group 1 = winning numbers
    // Group 2 = our numbers
    static ref SCRATCHCARD_FORMAT: Regex = Regex::new(r"^Card +[0-9]+: +([0-9 ]+?) +\| +([0-9 ]+)$").unwrap();

    static ref ANY_NUMBER_OF_SPACES: Regex = Regex::new(r" +").unwrap();
}

impl FromStr for Scratchcard {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Scratchcard> {
        let captures = SCRATCHCARD_FORMAT
            .captures(input)
            .ok_or_else(|| anyhow!(format!("Invalid scratchcard format: {}", input)))?;

        let winning_numbers_str = captures.get(1).unwrap().as_str();
        let our_numbers_str = captures.get(2).unwrap().as_str();

        let winning_numbers = parse_space_separated_values(winning_numbers_str)?;
        let our_numbers = parse_space_separated_values(our_numbers_str)?;

        Ok(Scratchcard {
            winning_numbers,
            our_numbers,
            copies: 1,
        })
    }
}

fn parse_space_separated_values<N>(input: &str) -> Result<HashSet<N>>
where
    N: FromStr + std::hash::Hash + Eq,
    <N as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    ANY_NUMBER_OF_SPACES
        .split(input)
        .map(parse_number)
        .collect()
}

fn parse_number<N>(input: &str) -> Result<N>
where
    N: FromStr,
    <N as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    input
        .parse::<N>()
        .with_context(|| format!("Invalid number `{}`", input))
}

impl Scratchcard {
    pub fn num_matches(&self) -> usize {
        self.winning_numbers.intersection(&self.our_numbers).count()
    }

    pub fn points(&self) -> Result<u64> {
        match self.num_matches() {
            0 => Ok(0),
            num_wins => u64::checked_pow(2, (num_wins - 1) as u32)
                .with_context(|| format!("overflow while trying to calculate points for {num_wins} wins, 2^{num_wins} > u64::max_value"))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_space_separated_values() {
        let input = "1 2 3 4 5";
        let expected = [1, 2, 3, 4, 5].into_iter().collect::<HashSet<u8>>();
        let actual = parse_space_separated_values(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_space_separated_values_multiple_spaces() {
        let input = "65  2 33    3 5";
        let expected = [65, 2, 33, 3, 5].into_iter().collect::<HashSet<u8>>();
        let actual = parse_space_separated_values(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_space_separated_values_bad_number() {
        let input = "1 2 3 abc 5";
        let err = parse_space_separated_values::<u8>(input).unwrap_err();
        assert_eq!(err.to_string(), "Invalid number `abc`");
        assert_eq!(
            format!("{:#}", err),
            "Invalid number `abc`: invalid digit found in string"
        );
    }

    #[test]
    fn test_parse_space_separated_values_too_high_number() {
        let input = "1 2 3 256 5";
        let err = parse_space_separated_values::<u8>(input).unwrap_err();
        assert_eq!(err.to_string(), "Invalid number `256`");
        assert_eq!(
            format!("{:#}", err),
            "Invalid number `256`: number too large to fit in target type"
        );
    }

    #[test]
    fn test_parse_scratchcard() {
        let input = "Card 1: 1 2 3 4 5 | 6 7 8 9 10";
        let expected = Scratchcard {
            winning_numbers: [1, 2, 3, 4, 5].into_iter().collect::<HashSet<u8>>(),
            our_numbers: [6, 7, 8, 9, 10].into_iter().collect::<HashSet<u8>>(),
            copies: 1,
        };
        let actual = input.parse::<Scratchcard>().unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_scratchcard_double_space_after_colon() {
        let input = "Card  11:  7 78 75 90 36 14 62 16 55 97 | 49 54 93  4 52 67 31 84 25  1 77 18 50 21 46 76 89 69 24 53  5 96 86 32 99";
        let expected = Scratchcard {
            winning_numbers: [7, 78, 75, 90, 36, 14, 62, 16, 55, 97]
                .into_iter()
                .collect::<HashSet<u8>>(),
            our_numbers: [
                49, 54, 93, 4, 52, 67, 31, 84, 25, 1, 77, 18, 50, 21, 46, 76, 89, 69, 24, 53, 5,
                96, 86, 32, 99,
            ]
            .into_iter()
            .collect::<HashSet<u8>>(),
            copies: 1,
        };
        let actual = input.parse::<Scratchcard>().unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_scratchcard_double_space_around_pipe() {
        let input = "Card  14: 63 34 29 59 23 98 65 66 12  1  |  8 80 93 74 68 22 26 76 82 11 39 95 58 19 94 97 35 49 44 37 86 51 79 75 60";
        let expected = Scratchcard {
            winning_numbers: [63, 34, 29, 59, 23, 98, 65, 66, 12, 1]
                .into_iter()
                .collect::<HashSet<u8>>(),
            our_numbers: [
                8, 80, 93, 74, 68, 22, 26, 76, 82, 11, 39, 95, 58, 19, 94, 97, 35, 49, 44, 37, 86,
                51, 79, 75, 60,
            ]
            .into_iter()
            .collect::<HashSet<u8>>(),
            copies: 1,
        };
        let actual = input.parse::<Scratchcard>().unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_points() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let scratchcard = input.parse::<Scratchcard>().unwrap();
        // 4 winning numbers = 2^3 = 8 points
        assert_eq!(scratchcard.points().unwrap(), 8);
    }

    #[test]
    fn test_zero_points() {
        let input = "Card 1: 41 48 83 86 17 | 1 2 3 4 5 6 7 8";
        let scratchcard = input.parse::<Scratchcard>().unwrap();
        // no winning numbers = 0 points
        assert_eq!(scratchcard.points().unwrap(), 0);
    }
}
