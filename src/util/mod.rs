use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[deprecated = "Use Point2D directly"]
pub use aoc_util::geometry::Point2D as Point;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    Down,
    Left,
    Right,
    Up,
}

impl Direction {
    pub const fn values() -> &'static [Self; 4] {
        &[Self::Down, Self::Left, Self::Right, Self::Up]
    }
}

impl<T> Add<Direction> for Point<T>
where
    T: Add<usize, Output = T>,
    T: Sub<usize, Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::Down => self - Point::at(0, 1),
            Direction::Left => self - Point::at(1, 0),
            Direction::Right => self + Point::at(1, 0),
            Direction::Up => self + Point::at(0, 1),
        }
    }
}

impl<T> AddAssign<Direction> for Point<T>
where
    T: AddAssign<usize>,
    T: SubAssign<usize>,
{
    fn add_assign(&mut self, rhs: Direction) {
        match rhs {
            Direction::Down => *self -= Point::at(0, 1),
            Direction::Left => *self -= Point::at(1, 0),
            Direction::Right => *self += Point::at(1, 0),
            Direction::Up => *self += Point::at(0, 1),
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
