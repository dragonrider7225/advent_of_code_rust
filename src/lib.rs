//! This crate aggregates my solutions to all
//! [advent of code](https://adventofcode.com/) problems.
#![feature(box_syntax)]
#![feature(generators, generator_trait)]
#![feature(optin_builtin_traits)]
#![feature(step_trait)]
use std::{
    convert::AsRef,
    fmt::Debug,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
    str::FromStr,
};

use extended_io as eio;

#[macro_use]
mod parse;
mod util;
mod year_2018;
mod year_2019;
mod year_2020;

fn get_lines<P>(path: P) -> io::Result<impl Iterator<Item = String>>
where
    P: AsRef<Path>,
{
    let ret = BufReader::new(File::open(path)?)
        .lines()
        .map(|res| res.expect("Failed to read line"));
    Ok(ret)
}

fn parse_lines<I, P>(path: P) -> io::Result<impl Iterator<Item = I>>
where
    I: FromStr,
    <I as FromStr>::Err: Debug,
    P: AsRef<Path>,
{
    let lines = get_lines(path)?;
    Ok(lines.map(|s| s.parse().expect(&format!(r#"Invalid line: "{}""#, s))))
}

fn run_year(year: u32) -> io::Result<()> {
    let day_prompt = || eio::prompt("Enter day to run: ");
    match year {
        2018 => year_2018::run_day(day_prompt()?),
        2019 => year_2019::run_day(day_prompt()?),
        2020 => year_2020::run_day(day_prompt()?),
        _ => unimplemented!("Year {}", year),
    }
}

/// The entry point for my solutions to advent of code.
pub fn run() -> io::Result<()> {
    run_year(eio::prompt("Enter the year to run: ")?)
}
