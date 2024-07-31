//! An executable wrapper around (my) advent of code solutions.
use advent_of_code as aoc;

use clap::{App, Arg};

use std::io;

fn app() -> App<'static> {
    App::new("Advent of Code")
        .version("0.1.0")
        .author("Kevin M. <dragonrider7225@gmail.com>")
        .about("Runs one day of one year of the Advent of Code <adventofcode.com>")
        .max_term_width(100)
        .arg(
            Arg::new("year")
                .short('y')
                .long("year")
                .takes_value(true)
                .value_name("YEAR")
                .possible_values(["2018", "2019", "2020", "2021", "2022", "2023"])
                .help("Selects the year to run"),
        )
        .arg(
            Arg::new("day")
                .short('d')
                .long("day")
                .takes_value(true)
                .value_name("DAY")
                .possible_values([
                    "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                    "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25",
                ])
                .help("Selects the day to run"),
        )
}

fn main() -> io::Result<()> {
    let matches = app().get_matches();
    let year = matches.value_of("year").and_then(|s| s.parse::<u32>().ok());
    let day = matches.value_of("day").and_then(|s| s.parse::<u32>().ok());
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
