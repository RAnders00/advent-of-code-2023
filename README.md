# advent-of-code-2023

My personal completion of the [Advent of Code](https://adventofcode.com/2023) challenges for 2023.

1. Install the Rust programming language
2. `cargo run`, e.g. `cargo run day1`

```none
Usage: advent-of-code-2023 <COMMAND>

Commands:
  day1  Run the two algorithms for day 1's challenge
  day2  Run the two algorithms for day 2's challenge
  day3  Run the two algorithms for day 3's challenge
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Verbose logging

Set the `RUST_LOG` environment variable to `advent_of_code_2023=debug`.

```bash
RUST_LOG=advent_of_code_2023=debug cargo run day1 data/day1/input.txt
```
