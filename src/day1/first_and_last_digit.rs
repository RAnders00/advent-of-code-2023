/// Return the first and last decimal digit, ignoring zero, found in the given string. Ignores
/// any character not between '1' and '9'.
/// Returns `None` in case not a single digit is found.
/// If only a single digit is found in the string, it is returned as both first and last.
pub fn first_and_last_digit_decimal(input: &str) -> Option<(u8, u8)> {
    let mut digits = input
        .chars()
        .filter_map(|c| c.to_string().parse::<u8>().ok())
        .filter(|&digit| digit != 0);

    let first = digits.next()?;
    // If there is no distinct second digit, use the first digit again
    let last = digits.next_back().unwrap_or(first);

    Some((first, last))
}

const DIGITS: [(&str, u8); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

/// Same as [`first_and_last_digit_decimal`], but also accepts spelled-out digits between "one" and "nine".
pub fn first_and_last_digit_decimal_or_spelled(input: &str) -> Option<(u8, u8)> {
    let first_digit = DIGITS
        .into_iter()
        .filter_map(|(spelled_digit, digit)| {
            // For every digit, find the index where this digit can be found in the string
            let ascii_digit = char::from_digit(digit as u32, /* radix = */ 10).unwrap();

            // Try to find either the spelled digit or ascii digit in the input
            let spelled_digit_first_idx = input.find(spelled_digit);
            let ascii_digit_first_idx = input.find(ascii_digit);

            let first_idx = [spelled_digit_first_idx, ascii_digit_first_idx]
                .into_iter()
                // flatten() removes any None elements
                .flatten()
                // ? returns None in case both searches were unsuccessful,
                // filter_map will remove this iteration
                .min()?;

            Some((digit, first_idx))
        })
        // In case multiple digits were found: Take the best digit
        .min_by_key(|(_, idx)| *idx)
        // Unwrap by removing the accompanying index.
        // ? returns None in case not a single digit was found.
        .map(|(digit, _)| digit)?;

    let last_digit = DIGITS
        .into_iter()
        .filter_map(|(spelled_digit, digit)| {
            let ascii_digit = char::from_digit(digit as u32, 10).unwrap();

            // rfind instead of find
            let spelled_digit_last_idx = input.rfind(spelled_digit);
            let ascii_digit_last_idx = input.rfind(ascii_digit);

            let last_idx = [spelled_digit_last_idx, ascii_digit_last_idx]
                .into_iter()
                .flatten()
                // max instead of min
                .max()?;

            Some((digit, last_idx))
        })
        // max_by_key instead of min_by_key
        .max_by_key(|(_, idx)| *idx)
        .map(|(digit, _)| digit)?;

    Some((first_digit, last_digit))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decimal_two_digits() {
        assert_eq!(first_and_last_digit_decimal("abc12d34ef"), Some((1, 4)));
        assert_eq!(first_and_last_digit_decimal("1234"), Some((1, 4)));
        assert_eq!(first_and_last_digit_decimal("99"), Some((9, 9)));
    }

    #[test]
    fn test_decimal_single_digit() {
        assert_eq!(first_and_last_digit_decimal("1"), Some((1, 1)));
        assert_eq!(first_and_last_digit_decimal("a1"), Some((1, 1)));
        assert_eq!(first_and_last_digit_decimal("a1b"), Some((1, 1)));
        assert_eq!(first_and_last_digit_decimal("1b"), Some((1, 1)));
        assert_eq!(first_and_last_digit_decimal("aAA1bBBB"), Some((1, 1)));
    }

    #[test]
    fn test_decimal_does_not_count_zero() {
        assert_eq!(first_and_last_digit_decimal("zero"), None);
        assert_eq!(first_and_last_digit_decimal("0"), None);
    }

    #[test]
    fn test_decimal_no_digits() {
        assert_eq!(first_and_last_digit_decimal(""), None);
        assert_eq!(first_and_last_digit_decimal("x"), None);
        assert_eq!(first_and_last_digit_decimal("foobarasdf hello world"), None);
    }

    #[test]
    fn test_decimal_or_spelled_two_digits() {
        assert_eq!(
            first_and_last_digit_decimal_or_spelled("two1nine"),
            Some((2, 9))
        );
        assert_eq!(
            first_and_last_digit_decimal_or_spelled("eightwothree"),
            Some((8, 3))
        );
        assert_eq!(
            first_and_last_digit_decimal_or_spelled("abcone2threexyz"),
            Some((1, 3))
        );
        assert_eq!(
            first_and_last_digit_decimal_or_spelled("xtwone3four"),
            Some((2, 4))
        );
        assert_eq!(
            first_and_last_digit_decimal_or_spelled("4nineeightseven2"),
            Some((4, 2))
        );
        assert_eq!(
            first_and_last_digit_decimal_or_spelled("4nineeight2seven"),
            Some((4, 7))
        );
        assert_eq!(
            first_and_last_digit_decimal_or_spelled("zoneight234"),
            Some((1, 4))
        );
        assert_eq!(
            first_and_last_digit_decimal_or_spelled("7pqrstsixteen"),
            Some((7, 6))
        );
        assert_eq!(
            first_and_last_digit_decimal_or_spelled("6tvxlgrsevenjvbxbfqrsk4seven"),
            Some((6, 7))
        );
    }

    #[test]
    fn test_decimal_or_spelled_single_digit() {
        assert_eq!(first_and_last_digit_decimal_or_spelled("one"), Some((1, 1)));
        assert_eq!(first_and_last_digit_decimal_or_spelled("two"), Some((2, 2)));
        assert_eq!(
            first_and_last_digit_decimal_or_spelled("three"),
            Some((3, 3))
        );
        assert_eq!(
            first_and_last_digit_decimal_or_spelled("four"),
            Some((4, 4))
        );
        assert_eq!(
            first_and_last_digit_decimal_or_spelled("five"),
            Some((5, 5))
        );
        assert_eq!(first_and_last_digit_decimal_or_spelled("six"), Some((6, 6)));
        assert_eq!(
            first_and_last_digit_decimal_or_spelled("seven"),
            Some((7, 7))
        );
        assert_eq!(
            first_and_last_digit_decimal_or_spelled("eight"),
            Some((8, 8))
        );
        assert_eq!(
            first_and_last_digit_decimal_or_spelled("nine"),
            Some((9, 9))
        );
        assert_eq!(first_and_last_digit_decimal_or_spelled("2"), Some((2, 2)));
    }

    #[test]
    fn test_decimal_or_spelled_does_not_count_zero() {
        assert_eq!(first_and_last_digit_decimal_or_spelled("zero"), None);
        assert_eq!(first_and_last_digit_decimal_or_spelled("0"), None);
    }

    #[test]
    fn test_decimal_or_spelled_no_digits() {
        assert_eq!(first_and_last_digit_decimal_or_spelled(""), None);
        assert_eq!(first_and_last_digit_decimal_or_spelled("x"), None);
        assert_eq!(
            first_and_last_digit_decimal_or_spelled("foobarasdf hello world"),
            None
        );
        assert_eq!(first_and_last_digit_decimal_or_spelled("thirteen"), None);
    }
}
