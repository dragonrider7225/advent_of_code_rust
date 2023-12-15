//! Utilities that are general enough to be useful in many situations.

#![warn(clippy::all)]
#![warn(missing_copy_implementations, missing_docs, rust_2018_idioms)]
#![deny(unsafe_op_in_unsafe_fn, missing_debug_implementations)]

/// Utilities for axis-aligned bounding boxes.
pub mod aabb;

/// A generic implementation of the A* search algorithm. Currently does not work correctly.
#[doc(hidden)]
pub mod a_star;

/// Extensions to the `nom` crate.
pub mod nom_extended;

/// Utilities dealing with geometry.
pub mod geometry;
