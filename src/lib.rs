//! This crate aggregates my solutions to all [advent of code](https://adventofcode.com/) problems.

#![warn(rust_2018_idioms)]
#![feature(box_patterns)]
#![feature(coroutines, coroutine_trait)]
#![feature(hash_extract_if)]
#![feature(step_trait)]

use std::io;

use extended_io as eio;

mod year_2018;
mod year_2019;

fn run_year(year: u32, day: Option<u32>) -> io::Result<()> {
    let day_prompt = move || day.ok_or(()).or_else(|_| eio::prompt("Enter day to run: "));
    match year {
        2018 => year_2018::run_day(day_prompt()?),
        2019 => year_2019::run_day(day_prompt()?),
        2020 => aoc_2020::run_day(day_prompt()?),
        2021 => aoc_2021::run_day(day_prompt()?),
        2022 => aoc_2022::run_day(day_prompt()?),
        2023 => aoc_2023::run_day(day_prompt()?),
        _ => unimplemented!("Year {}", year),
    }
}

/// The entry point for my solutions to advent of code.
pub fn run(year: Option<u32>, day: Option<u32>) -> io::Result<()> {
    let year = match year {
        Some(year) => year,
        None => eio::prompt("Enter the year to run: ")?,
    };
    run_year(year, day)
}
