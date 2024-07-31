use super::{Point2D, Point3D};
use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

/// A cardinal direction in 2-dimensional space.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Direction2D {
    #[allow(missing_docs)]
    Down,
    #[allow(missing_docs)]
    Left,
    #[allow(missing_docs)]
    Right,
    #[allow(missing_docs)]
    Up,
}

impl Direction2D {
    /// All directions.
    pub const fn values() -> &'static [Self] {
        &[Self::Down, Self::Left, Self::Right, Self::Up]
    }
}

impl<T> Add<Direction2D> for Point2D<T>
where
    T: Add<usize, Output = T>,
    T: Sub<usize, Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Direction2D) -> Self::Output {
        match rhs {
            Direction2D::Down => self - Point2D::at(0, 1),
            Direction2D::Left => self - Point2D::at(1, 0),
            Direction2D::Right => self + Point2D::at(1, 0),
            Direction2D::Up => self + Point2D::at(0, 1),
        }
    }
}

impl<T> AddAssign<Direction2D> for Point2D<T>
where
    T: AddAssign<usize>,
    T: SubAssign<usize>,
{
    fn add_assign(&mut self, rhs: Direction2D) {
        match rhs {
            Direction2D::Down => *self -= Point2D::at(0, 1),
            Direction2D::Left => *self -= Point2D::at(1, 0),
            Direction2D::Right => *self += Point2D::at(1, 0),
            Direction2D::Up => *self += Point2D::at(0, 1),
        }
    }
}

impl Neg for Direction2D {
    type Output = Direction2D;

    fn neg(self) -> Self::Output {
        match self {
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
        }
    }
}

/// An unsigned axis in 3-space.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Axis3D {
    #[allow(missing_docs)]
    NorthSouth,
    #[allow(missing_docs)]
    EastWest,
    #[allow(missing_docs)]
    UpDown,
}

impl From<Direction3D> for Axis3D {
    fn from(value: Direction3D) -> Self {
        match value {
            Direction3D::North | Direction3D::South => Self::NorthSouth,
            Direction3D::East | Direction3D::West => Self::EastWest,
            Direction3D::Up | Direction3D::Down => Self::UpDown,
        }
    }
}

/// A cardinal direction in 3-dimensional space.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Direction3D {
    /// The positive x-axis.
    East,
    /// The negative x-axis.
    West,
    /// The positive y-axis.
    North,
    /// The negative y-axis.
    South,
    /// The positive z-axis.
    Up,
    /// The negative z-axis.
    Down,
}

impl<T> Add<Direction3D> for Point3D<T>
where
    T: Add<usize, Output = T>,
    T: Sub<usize, Output = T>,
{
    type Output = Self;

    /// Take one step in the direction of the axis.
    fn add(self, rhs: Direction3D) -> Self::Output {
        match rhs {
            Direction3D::East => self + Point3D::at(1, 0, 0),
            Direction3D::West => self - Point3D::at(1, 0, 0),
            Direction3D::North => self + Point3D::at(0, 1, 0),
            Direction3D::South => self - Point3D::at(0, 1, 0),
            Direction3D::Up => self + Point3D::at(0, 0, 1),
            Direction3D::Down => self - Point3D::at(0, 0, 1),
        }
    }
}

impl<T> AddAssign<Direction3D> for Point3D<T>
where
    T: AddAssign<usize>,
    T: SubAssign<usize>,
{
    fn add_assign(&mut self, rhs: Direction3D) {
        match rhs {
            Direction3D::East => *self += Point3D::at(1, 0, 0),
            Direction3D::West => *self -= Point3D::at(1, 0, 0),
            Direction3D::North => *self += Point3D::at(0, 1, 0),
            Direction3D::South => *self -= Point3D::at(0, 1, 0),
            Direction3D::Up => *self += Point3D::at(0, 0, 1),
            Direction3D::Down => *self -= Point3D::at(0, 0, 1),
        }
    }
}

impl Neg for Direction3D {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::East => Self::West,
            Self::West => Self::East,
            Self::North => Self::South,
            Self::South => Self::North,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}
