#![feature(
    extend_one,
    is_sorted,
    iter_advance_by,
    trusted_len,
    try_find,
    try_trait
)]

mod cycle_bounded_impl;
mod replicate_impl;

pub use cycle_bounded_impl::{cycle_bounded, CycleBounded};
pub use replicate_impl::{replicate, Replicate};
