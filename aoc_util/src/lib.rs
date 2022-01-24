//! Utilities that are general enough to be useful in many situations.

#![warn(clippy::all)]
#![warn(missing_copy_implementations, missing_docs, rust_2018_idioms)]
#![deny(unsafe_op_in_unsafe_fn, missing_debug_implementations)]

use nom::{character::complete as character, combinator as comb, multi, sequence, IResult};

/// Utilities for axis-aligned bounding boxes.
pub mod aabb;

macro_rules! digits {
    () => {
        multi::many1(character::one_of("0123456789"))
    };
}

macro_rules! recognize_num_unsigned {
    () => {
        comb::map(comb::recognize(digits!()), |s: &str| s.parse().unwrap())
    };
}

macro_rules! recognize_num_signed {
    () => {
        comb::map(
            comb::recognize(sequence::pair(comb::opt(character::char('-')), digits!())),
            |s: &str| s.parse().unwrap(),
        )
    };
}

/// Parses a decimal number from `s`.
pub fn recognize_u8(s: &str) -> IResult<&str, u8> {
    recognize_num_unsigned!()(s)
}

/// Parses a decimal number from `s`.
pub fn recognize_u16(s: &str) -> IResult<&str, u16> {
    recognize_num_unsigned!()(s)
}

/// Parses a decimal number from `s`.
pub fn recognize_u32(s: &str) -> IResult<&str, u32> {
    recognize_num_unsigned!()(s)
}

/// Parses a decimal number from `s`.
pub fn recognize_u64(s: &str) -> IResult<&str, u64> {
    recognize_num_unsigned!()(s)
}

/// Parses a decimal number from `s`.
pub fn recognize_u128(s: &str) -> IResult<&str, u128> {
    recognize_num_unsigned!()(s)
}

/// Parses a decimal number from `s`.
pub fn recognize_i8(s: &str) -> IResult<&str, i8> {
    recognize_num_signed!()(s)
}

/// Parses a decimal number from `s`.
pub fn recognize_i16(s: &str) -> IResult<&str, i16> {
    recognize_num_signed!()(s)
}

/// Parses a decimal number from `s`.
pub fn recognize_i32(s: &str) -> IResult<&str, i32> {
    recognize_num_signed!()(s)
}

/// Parses a decimal number from `s`.
pub fn recognize_i64(s: &str) -> IResult<&str, i64> {
    recognize_num_signed!()(s)
}

/// Parses a decimal number from `s`.
pub fn recognize_i128(s: &str) -> IResult<&str, i128> {
    recognize_num_signed!()(s)
}
