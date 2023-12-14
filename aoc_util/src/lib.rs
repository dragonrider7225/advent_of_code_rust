//! Utilities that are general enough to be useful in many situations.

#![warn(clippy::all)]
#![warn(missing_copy_implementations, missing_docs, rust_2018_idioms)]
#![deny(unsafe_op_in_unsafe_fn, missing_debug_implementations)]

use nom::{branch, bytes::complete as bytes, character::complete as character, IResult};

/// Utilities for axis-aligned bounding boxes.
pub mod aabb;

/// A generic implementation of the A* search algorithm. Currently does not work correctly.
#[doc(hidden)]
pub mod a_star;

/// Recognizes both `\n` and `\r\n`.
pub fn newline(s: &str) -> IResult<&str, &str> {
    branch::alt((bytes::tag("\n"), bytes::tag("\r\n")))(s)
}

/// Parses a decimal number from `s`.
#[deprecated = "Use character::{complete, streaming}::u8"]
pub fn recognize_u8(s: &str) -> IResult<&str, u8> {
    character::u8(s)
}

/// Parses a decimal number from `s`.
#[deprecated = "Use character::{complete, streaming}::u16"]
pub fn recognize_u16(s: &str) -> IResult<&str, u16> {
    character::u16(s)
}

/// Parses a decimal number from `s`.
#[deprecated = "Use character::{complete, streaming}::u32"]
pub fn recognize_u32(s: &str) -> IResult<&str, u32> {
    character::u32(s)
}

/// Parses a decimal number from `s`.
#[deprecated = "Use character::{complete, streaming}::u64"]
pub fn recognize_u64(s: &str) -> IResult<&str, u64> {
    character::u64(s)
}

/// Parses a decimal number from `s`.
#[deprecated = "Use character::{complete, streaming}::u128"]
pub fn recognize_u128(s: &str) -> IResult<&str, u128> {
    character::u128(s)
}

/// Parses a decimal number from `s`.
#[deprecated = "Use character::{complete, streaming}::i8"]
pub fn recognize_i8(s: &str) -> IResult<&str, i8> {
    character::i8(s)
}

/// Parses a decimal number from `s`.
#[deprecated = "Use character::{complete, streaming}::i16"]
pub fn recognize_i16(s: &str) -> IResult<&str, i16> {
    character::i16(s)
}

/// Parses a decimal number from `s`.
#[deprecated = "Use character::{complete, streaming}::i32"]
pub fn recognize_i32(s: &str) -> IResult<&str, i32> {
    character::i32(s)
}

/// Parses a decimal number from `s`.
#[deprecated = "Use character::{complete, streaming}::i64"]
pub fn recognize_i64(s: &str) -> IResult<&str, i64> {
    character::i64(s)
}

/// Parses a decimal number from `s`.
#[deprecated = "Use character::{complete, streaming}::i128"]
pub fn recognize_i128(s: &str) -> IResult<&str, i128> {
    character::i128(s)
}
