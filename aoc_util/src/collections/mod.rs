/// A priority queue has a constant-time lookup for the element with the greatest priority.
pub mod priority_queue;
pub use priority_queue::PriorityQueue;

/// A FILO queue produces its elements in the reverse order that they were added. Two
/// [`SharedFilo`]s created by pushing two different elements onto the same precursor will share
/// memory for the parts that are identical.
pub mod shared_filo;
pub use shared_filo::SharedFilo;
