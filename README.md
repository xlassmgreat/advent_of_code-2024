These are my solutions for Advent of Code 2024 written in Rust. See below for steps to run.

## To run

- Ensure cargo and rustc are installed.
- Go to the project directory.
- Create a folder called `inputs`.
- One file for each day's input should be in this folder.
- Name them like this: day*N*, where *N* is the day number. For eg. day1, day2, day3, etc.

A sample folder may look like this:

advent_of_code-2024
- Cargo.lock
- Cargo.toml
- inputs
  - day1
  - day2
  - day3
  - ...
- src
  - main.rs
  - day1.rs
  - day2.rs
  - ...
- target

```
cargo run --release <day_number>
```
