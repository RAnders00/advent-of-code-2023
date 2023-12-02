mod args;
mod first_and_last_digit;

pub use args::Args;
pub use first_and_last_digit::*;

use anyhow::{anyhow, Context, Result};
use std::fs;

pub fn run(args: Args) -> Result<()> {
    let input = fs::read_to_string(&args.input).context(format!(
        "While trying to read file {}",
        args.input.display()
    ))?;

    let sum_decimal = sum_first_and_last_digits(&input, first_and_last_digit_decimal)?;
    let sum_decimal_or_spelled =
        sum_first_and_last_digits(&input, first_and_last_digit_decimal_or_spelled)?;
    tracing::info!(
        "Sum of all lines (Part 1 - Counting ASCII digits only): {}",
        sum_decimal
    );
    tracing::info!(
        "Sum of all lines (Part 2 - Counting ASCII digits and spelled-out digits): {}",
        sum_decimal_or_spelled
    );

    Ok(())
}

/// Split the given `input` string into lines. For each line,
/// run the given `digit_algorithm` to find the first and last digit inside.
/// The found first and last digit are combined using [`concatenate_digits`].
///
/// All such combined digits are then summed up and returned.
///
/// Returns an error if a non-empty line is encountered that does not
/// contain any digits.
///
/// For digit algorithms, see [`first_and_last_digit_decimal`] and
/// [`first_and_last_digit_decimal_or_spelled`].
pub fn sum_first_and_last_digits<F>(input: &str, digit_algorithm: F) -> Result<u64>
where
    F: Fn(&str) -> Option<(u8, u8)>,
{
    input
        .lines()
        .enumerate()
        .filter(|(_, line)| !line.is_empty())
        .map(|(line_idx, line)| {
            let (first, last) = digit_algorithm(line).ok_or_else(|| {
                anyhow!(
                    "Line {} (contents: `{}`) does not contain any digits",
                    line_idx + 1,
                    line
                )
            })?;
            // The first and last digits concatenate, e.g. 4 + 7 = 47
            let concatenated = concatenate_digits(first, last) as u64;
            tracing::debug!(
                "Line {} (contents: `{}`) -> {}",
                line_idx + 1,
                line,
                concatenated,
            );
            anyhow::Ok(concatenated)
        })
        .sum::<Result<u64>>()
}

/// Concatenates two decimal digits into a single `u8`.
/// Panics if either digit is larger than 9.
///
/// # Example
///
/// ```
/// # use advent_of_code_2023::day1::concatenate_digits;
/// assert_eq!(concatenate_digits(4, 7), 47);
/// assert_eq!(concatenate_digits(0, 0), 0);
/// assert_eq!(concatenate_digits(9, 9), 99);
/// ````
pub fn concatenate_digits(most_sigificant: u8, least_significant: u8) -> u8 {
    if (most_sigificant > 9) || (least_significant > 9) {
        panic!(
            "concatenate_digits expected digits <= 9, got: {} and {}",
            most_sigificant, least_significant
        );
    }
    (most_sigificant * 10) + least_significant
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_real_data_decimal() {
        let input = r#"9kfpfgzdjdgxjkltdkbkeightmxteightthree
9bdsbeightjvkrmhdkghfive73four3
xeightwoninehcrsdbnvtwovtbkhtxktjslsix3
15fourlgrsk
5xjqd9
four8ttpzxpnrqnkz1"#;

        let expected = [99, 93, 33, 15, 59, 81].into_iter().sum();

        assert_eq!(
            sum_first_and_last_digits(input, first_and_last_digit_decimal).unwrap(),
            expected
        );
    }

    #[test]
    fn test_real_data_decimal_or_spelled() {
        let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

        let expected = [29, 83, 13, 24, 42, 14, 76].into_iter().sum();
        assert_eq!(expected, 281);

        assert_eq!(
            sum_first_and_last_digits(input, first_and_last_digit_decimal_or_spelled).unwrap(),
            expected
        );
    }

    #[test]
    fn test_empty_lines() {
        let input = r#"99
2

1

7
5abc9
1abc3"#;

        let expected = [99, 22, 11, 77, 59, 13].into_iter().sum();

        assert_eq!(
            sum_first_and_last_digits(input, first_and_last_digit_decimal).unwrap(),
            expected
        );
    }
}
