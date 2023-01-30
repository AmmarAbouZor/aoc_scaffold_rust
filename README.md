
# aoc_scaffold_rust

Cli tool written in rust to scaffold the source code for [advnent of code](https://adventofcode.com/) solutions in the rust programming language with a template including unit tests + a unit tests runner.


## Features
This tool can:
- Creates a rust source code file for each day with the options for another input text file in a separate folder (So you hopefully will add it to your gitignore file).
- Manages the project structures by itself so you can hit "cargo run" and the current day of the selected year will run. 
- Runs unit tests for the current day only without having to specify the day. 
- Manages different years in one project
- Generates shell completion file for a given shell

## Installation

Install directly from github:

```bash
cargo install --git https://github.com/ammarabouzor/aoc_scaffold_rust

```
    
## Usage/Examples

This tool should be used inside a rust project. Ideally an empty one with a main.rs file only with an empty main() function.

```bash
Usage: aoc_scaffold <COMMAND>

Commands:
  config    Manage the configs [aliases: c]
  next-day  scaffold the the repo incrementing the current day and adding a new year if the current year doesn't exist [aliases: n]
  run-test  Run unit tests for the last day only [aliases: t]
  complete  Generate completion file for a given shell. Save the output of this command to your shell to get auto completion [aliases: comp]
  help      Print this message or the help of the given subcommand(s)

```

Configs management:
```bash
Usage: aoc_scaffold config <COMMAND>

Commands:
  set-year  set the current year in the settings [aliases: sy]
  get-year  return the current year in the settings [aliases: gy]
  open      open the config file in the default text editor [aliases: oc]
  get-path  return the path of the config file [aliases: gp]
```

Scaffolding the next day:
```bash
Usage: aoc_scaffold next-day [OPTIONS]

Options:
  -i, --input       create an input file for the next day inside the input folder
  -o, --open-input  open the created input file for the next if it created
  -h, --help        Print help
```

Run unit tests for the current day:
```bash
Usage: aoc_scaffold run-test [OPTIONS]

Options:
  -r, --release  run tests in release mode
  -h, --help     Print help
```
## Documentation

This tool creates a new module for each year with a sub-module for each day. The year module manage with day to call through a match statement:

```rust
mod day_01;
mod day_02;
mod day_03;

pub fn run() {
    run_day(3);
}

fn run_day(day: u8) {
    match day {
        1 => day_01::run(),
        2 => day_02::run(),
        3 => day_03::run(),
        _ => unreachable!("day not implemented"),
    }
}
```

main.rs file manage the years in his turn:
At the beginning the main.rs file should have an empty main function only. Having some code inside it will cause problems by manipulating it.
```rust
mod year_15;
mod year_22;

fn main() {
    run_year("22");
}

fn run_year(year: &str) {
    match year {
        "15" => year_15::run(),
        "22" => year_22::run(),
        _ => unreachable!("year not implemented"),
    }
}
```
This tool manages these files for you so you shouldn't change anything in them

## Contributing

This is my first open source project and I would appreciate all kinds of contributions!

## License

[MIT](https://choosealicense.com/licenses/mit/)

