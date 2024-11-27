//! An executable wrapper around (my) advent of code solutions.
use advent_of_code as aoc;

use clap::{Arg, Command};

use std::io;

fn app() -> Command {
    Command::new("Advent of Code")
        .version("0.1.0")
        .author("Kevin M. <dragonrider7225@gmail.com>")
        .about("Runs one day of one year of the Advent of Code <adventofcode.com>")
        .max_term_width(100)
        .arg(
            Arg::new("year")
                .short('y')
                .long("year")
                .value_name("YEAR")
                .value_parser(2018..=2023)
                .help("Selects the year to run"),
        )
        .arg(
            Arg::new("day")
                .short('d')
                .long("day")
                .value_name("DAY")
                .value_parser(1..=25)
                .help("Selects the day to run (1..=25)"),
        )
}

fn main() -> io::Result<()> {
    let matches = app().get_matches();
    let year = matches.get_one::<i64>("year").map(|&n| n as _);
    let day = matches.get_one::<i64>("day").map(|&n| n as _);
    aoc::run(year, day)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_app() {
        app().debug_assert();
    }
}
