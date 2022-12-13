//! Utilities that are general enough to be useful in many situations.

#![warn(clippy::all)]
#![warn(missing_copy_implementations, missing_docs, rust_2018_idioms)]
#![deny(unsafe_op_in_unsafe_fn, missing_debug_implementations)]

use std::{
    fmt::Debug,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
    str::FromStr,
};

/// Utilities for axis-aligned bounding boxes.
pub mod aabb;

/// A generic implementation of the A* search algorithm. Currently does not work correctly.
#[doc(hidden)]
pub mod a_star;

/// Extensions to the `nom` crate.
pub mod nom_extended;

/// Utilities dealing with geometry.
pub mod geometry;

/// Read the lines of the file at `path` one at a time and panic if the next line can't be read for
/// any reason other than EOF.
#[deprecated = "Use a dyn BufRead and handle the lines inline"]
pub fn get_lines<P>(path: P) -> io::Result<impl Iterator<Item = String>>
where
    P: AsRef<Path>,
{
    let ret = BufReader::new(File::open(path)?)
        .lines()
        .map(|res| res.expect("Failed to read line"));
    Ok(ret)
}

/// Like [`get_lines()`] but parse each line as an `I` using [`FromStr`] (and panic if the parse
/// fails) instead of returning the raw string.
#[deprecated = "Parse the lines inline"]
pub fn parse_lines<I, P>(path: P) -> io::Result<impl Iterator<Item = I>>
where
    I: FromStr,
    <I as FromStr>::Err: Debug,
    P: AsRef<Path>,
{
    let lines = get_lines(path)?;
    Ok(lines.map(|s| {
        s.parse()
            .map_err(|e| format!("Invalid line {s:?}: {e:?}"))
            .unwrap()
    }))
}
