use super::Point2D;
use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

/// A direction in 2-dimensional space.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    #[allow(missing_docs)]
    Down,
    #[allow(missing_docs)]
    Left,
    #[allow(missing_docs)]
    Right,
    #[allow(missing_docs)]
    Up,
}

impl Direction {
    /// All directions.
    pub const fn values() -> &'static [Self] {
        &[Self::Down, Self::Left, Self::Right, Self::Up]
    }
}

impl<T> Add<Direction> for Point2D<T>
where
    T: Add<usize, Output = T>,
    T: Sub<usize, Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::Down => self - Point2D::at(0, 1),
            Direction::Left => self - Point2D::at(1, 0),
            Direction::Right => self + Point2D::at(1, 0),
            Direction::Up => self + Point2D::at(0, 1),
        }
    }
}

impl<T> AddAssign<Direction> for Point2D<T>
where
    T: AddAssign<usize>,
    T: SubAssign<usize>,
{
    fn add_assign(&mut self, rhs: Direction) {
        match rhs {
            Direction::Down => *self -= Point2D::at(0, 1),
            Direction::Left => *self -= Point2D::at(1, 0),
            Direction::Right => *self += Point2D::at(1, 0),
            Direction::Up => *self += Point2D::at(0, 1),
        }
    }
}

impl Neg for Direction {
    type Output = Direction;

    fn neg(self) -> Self::Output {
        match self {
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
        }
    }
}
