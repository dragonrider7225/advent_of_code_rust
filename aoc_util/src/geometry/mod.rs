/// Directions in 2- and 3-dimensional space.
pub mod direction;
pub use direction::{Axis3D, Direction2D as Direction, Direction3D};

/// Locations in n-dimensional space.
pub mod point;
pub use point::{Point2D, Point3D};
