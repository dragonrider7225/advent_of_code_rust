use advent_of_code as aoc;

use extended_io as eio;

use std::io;

fn main() -> io::Result<()> {
    aoc::run_day(eio::prompt("Enter the number of the day to run: ")?)
}
