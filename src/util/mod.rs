use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    pub const fn at(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub const fn x(&self) -> &T {
        &self.x
    }

    pub const fn y(&self) -> &T {
        &self.y
    }
}

macro_rules! impl_manhattan_distance_const {
    ($($t:ty)+) => ($(
        impl Point<$t> {
            pub const fn manhattan_distance(&self, rhs: &Self) -> $t {
                (if self.x < rhs.x { rhs.x - self.x } else { self.x - rhs.x })
                    + (if self.y < rhs.y { rhs.y - self.y } else { self.y - rhs.y })
            }
        }
    )+)
}

impl_manhattan_distance_const!(
    u8 u16 u32 u64 u128 usize
    i8 i16 i32 i64 i128 isize
);

macro_rules! impl_manhattan_distance {
    ($($t:ty)+) => ($(
        impl Point<$t> {
            pub fn manhattan_distance(&self, rhs: &Self) -> $t {
                (if self.x < rhs.x { rhs.x - self.x } else { self.x - rhs.x })
                    + (if self.y < rhs.y { rhs.y - self.y } else { self.y - rhs.y })
            }
        }
    )+)
}

impl_manhattan_distance!(f32 f64);

impl<T, U, V> Add<Point<U>> for Point<T>
where
    T: Add<U, Output = V>,
{
    type Output = Point<V>;

    fn add(self, other: Point<U>) -> Self::Output {
        Point::at(self.x + other.x, self.y + other.y)
    }
}

impl<'a, T, U, V> Add<&'a Point<U>> for Point<T>
where
    T: Add<&'a U, Output = V>,
{
    type Output = Point<V>;

    fn add(self, other: &'a Point<U>) -> Self::Output {
        Point::at(self.x + &other.x, self.y + &other.y)
    }
}

impl<'a, T, U, V> Add<&'a mut Point<U>> for Point<T>
where
    T: Add<&'a mut U, Output = V>,
{
    type Output = Point<V>;

    fn add(self, other: &'a mut Point<U>) -> Self::Output {
        Point::at(self.x + &mut other.x, self.y + &mut other.y)
    }
}

impl<'a, T, U, V> Add<Point<U>> for &'a Point<T>
where
    &'a T: Add<U, Output = V>,
{
    type Output = Point<V>;

    fn add(self, other: Point<U>) -> Self::Output {
        Point::at(&self.x + other.x, &self.y + other.y)
    }
}

impl<'a, 'b, T, U, V> Add<&'b Point<U>> for &'a Point<T>
where
    &'a T: Add<&'b U, Output = V>,
{
    type Output = Point<V>;

    fn add(self, other: &'b Point<U>) -> Self::Output {
        Point::at(&self.x + &other.x, &self.y + &other.y)
    }
}

impl<'a, 'b, T, U, V> Add<&'b mut Point<U>> for &'a Point<T>
where
    &'a T: Add<&'b mut U, Output = V>,
{
    type Output = Point<V>;

    fn add(self, other: &'b mut Point<U>) -> Self::Output {
        Point::at(&self.x + &mut other.x, &self.y + &mut other.y)
    }
}

impl<'a, T, U, V> Add<Point<U>> for &'a mut Point<T>
where
    &'a mut T: Add<U, Output = V>,
{
    type Output = Point<V>;

    fn add(self, other: Point<U>) -> Self::Output {
        Point::at(&mut self.x + other.x, &mut self.y + other.y)
    }
}

impl<'a, 'b, T, U, V> Add<&'b Point<U>> for &'a mut Point<T>
where
    &'a mut T: Add<&'b U, Output = V>,
{
    type Output = Point<V>;

    fn add(self, other: &'b Point<U>) -> Self::Output {
        Point::at(&mut self.x + &other.x, &mut self.y + &other.y)
    }
}

impl<'a, 'b, T, U, V> Add<&'b mut Point<U>> for &'a mut Point<T>
where
    &'a mut T: Add<&'b mut U, Output = V>,
{
    type Output = Point<V>;

    fn add(self, other: &'b mut Point<U>) -> Self::Output {
        Point::at(&mut self.x + &mut other.x, &mut self.y + &mut other.y)
    }
}

impl<T, U> AddAssign<Point<U>> for Point<T>
where
    T: AddAssign<U>,
{
    fn add_assign(&mut self, other: Point<U>) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<'a, T, U> AddAssign<&'a Point<U>> for Point<T>
where
    T: AddAssign<&'a U>,
{
    fn add_assign(&mut self, other: &'a Point<U>) {
        self.x += &other.x;
        self.y += &other.y;
    }
}

impl<'a, T, U> AddAssign<&'a mut Point<U>> for Point<T>
where
    T: AddAssign<&'a mut U>,
{
    fn add_assign(&mut self, other: &'a mut Point<U>) {
        self.x += &mut other.x;
        self.y += &mut other.y;
    }
}

impl<'a, T, U> AddAssign<Point<U>> for &'a mut Point<T>
where
    T: AddAssign<U>,
{
    fn add_assign(&mut self, other: Point<U>) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<'a, 'b, T, U> AddAssign<&'b Point<U>> for &'a mut Point<T>
where
    T: AddAssign<&'b U>,
{
    fn add_assign(&mut self, other: &'b Point<U>) {
        self.x += &other.x;
        self.y += &other.y;
    }
}

impl<'a, 'b, T, U> AddAssign<&'b mut Point<U>> for &'a mut Point<T>
where
    T: AddAssign<&'b mut U>,
{
    fn add_assign(&mut self, other: &'b mut Point<U>) {
        self.x += &mut other.x;
        self.y += &mut other.y;
    }
}

impl<T, U, V> Div<U> for Point<T>
where
    T: Div<U, Output = V>,
    U: Clone,
{
    type Output = Point<V>;

    fn div(self, other: U) -> Self::Output {
        Point::at(self.x / other.clone(), self.y / other)
    }
}

impl<'a, T, U, V> Div<U> for &'a Point<T>
where
    &'a T: Div<U, Output = V>,
    U: Clone,
{
    type Output = Point<V>;

    fn div(self, other: U) -> Self::Output {
        Point::at(&self.x / other.clone(), &self.y / other)
    }
}

impl<'a, T, U, V> Div<U> for &'a mut Point<T>
where
    &'a mut T: Div<U, Output = V>,
    U: Clone,
{
    type Output = Point<V>;

    fn div(self, other: U) -> Self::Output {
        Point::at(&mut self.x / other.clone(), &mut self.y / other)
    }
}

impl<T, U> DivAssign<U> for Point<T>
where
    T: DivAssign<U>,
    U: Clone,
{
    fn div_assign(&mut self, other: U) {
        self.x /= other.clone();
        self.y /= other;
    }
}

impl<'a, T, U> DivAssign<U> for &'a mut Point<T>
where
    T: DivAssign<U>,
    U: Clone,
{
    fn div_assign(&mut self, other: U) {
        self.x /= other.clone();
        self.y /= other;
    }
}

impl<T, U, V> Mul<U> for Point<T>
where
    T: Mul<U, Output = V>,
    U: Clone,
{
    type Output = Point<V>;

    fn mul(self, other: U) -> Self::Output {
        Point::at(self.x * other.clone(), self.y * other)
    }
}

impl<'a, T, U, V> Mul<U> for &'a Point<T>
where
    &'a T: Mul<U, Output = V>,
    U: Clone,
{
    type Output = Point<V>;

    fn mul(self, other: U) -> Self::Output {
        Point::at(&self.x * other.clone(), &self.y * other)
    }
}

impl<'a, T, U, V> Mul<U> for &'a mut Point<T>
where
    &'a mut T: Mul<U, Output = V>,
    U: Clone,
{
    type Output = Point<V>;

    fn mul(self, other: U) -> Self::Output {
        Point::at(&mut self.x * other.clone(), &mut self.y * other)
    }
}

impl<T, U> MulAssign<U> for Point<T>
where
    T: DivAssign<U>,
    U: Clone,
{
    fn mul_assign(&mut self, other: U) {
        self.x /= other.clone();
        self.y /= other;
    }
}

impl<'a, T, U> MulAssign<U> for &'a mut Point<T>
where
    T: DivAssign<U>,
    U: Clone,
{
    fn mul_assign(&mut self, other: U) {
        self.x /= other.clone();
        self.y /= other;
    }
}

impl<T, U> Neg for Point<T>
where
    T: Neg<Output = U>,
{
    type Output = Point<U>;

    fn neg(self) -> Self::Output {
        Point::at(-self.x, -self.y)
    }
}

impl<'a, T, U> Neg for &'a Point<T>
where
    &'a T: Neg<Output = U>,
{
    type Output = Point<U>;

    fn neg(self) -> Self::Output {
        Point::at(-&self.x, -&self.y)
    }
}

impl<'a, T, U> Neg for &'a mut Point<T>
where
    &'a mut T: Neg<Output = U>,
{
    type Output = Point<U>;

    fn neg(self) -> Self::Output {
        Point::at(-&mut self.x, -&mut self.y)
    }
}

impl<T, U, V> Sub<Point<U>> for Point<T>
where
    T: Sub<U, Output = V>,
{
    type Output = Point<V>;

    fn sub(self, other: Point<U>) -> Self::Output {
        Point::at(self.x - other.x, self.y - other.y)
    }
}

impl<'a, T, U, V> Sub<&'a Point<U>> for Point<T>
where
    T: Sub<&'a U, Output = V>,
{
    type Output = Point<V>;

    fn sub(self, other: &'a Point<U>) -> Self::Output {
        Point::at(self.x - &other.x, self.y - &other.y)
    }
}

impl<'a, T, U, V> Sub<&'a mut Point<U>> for Point<T>
where
    T: Sub<&'a mut U, Output = V>,
{
    type Output = Point<V>;

    fn sub(self, other: &'a mut Point<U>) -> Self::Output {
        Point::at(self.x - &mut other.x, self.y - &mut other.y)
    }
}

impl<'a, T, U, V> Sub<Point<U>> for &'a Point<T>
where
    &'a T: Sub<U, Output = V>,
{
    type Output = Point<V>;

    fn sub(self, other: Point<U>) -> Self::Output {
        Point::at(&self.x - other.x, &self.y - other.y)
    }
}

impl<'a, 'b, T, U, V> Sub<&'b Point<U>> for &'a Point<T>
where
    &'a T: Sub<&'b U, Output = V>,
{
    type Output = Point<V>;

    fn sub(self, other: &'b Point<U>) -> Self::Output {
        Point::at(&self.x - &other.x, &self.y - &other.y)
    }
}

impl<'a, 'b, T, U, V> Sub<&'b mut Point<U>> for &'a Point<T>
where
    &'a T: Sub<&'b mut U, Output = V>,
{
    type Output = Point<V>;

    fn sub(self, other: &'b mut Point<U>) -> Self::Output {
        Point::at(&self.x - &mut other.x, &self.y - &mut other.y)
    }
}

impl<'a, T, U, V> Sub<Point<U>> for &'a mut Point<T>
where
    &'a mut T: Sub<U, Output = V>,
{
    type Output = Point<V>;

    fn sub(self, other: Point<U>) -> Self::Output {
        Point::at(&mut self.x - other.x, &mut self.y - other.y)
    }
}

impl<'a, 'b, T, U, V> Sub<&'b Point<U>> for &'a mut Point<T>
where
    &'a mut T: Sub<&'b U, Output = V>,
{
    type Output = Point<V>;

    fn sub(self, other: &'b Point<U>) -> Self::Output {
        Point::at(&mut self.x - &other.x, &mut self.y - &other.y)
    }
}

impl<'a, 'b, T, U, V> Sub<&'b mut Point<U>> for &'a mut Point<T>
where
    &'a mut T: Sub<&'b mut U, Output = V>,
{
    type Output = Point<V>;

    fn sub(self, other: &'b mut Point<U>) -> Self::Output {
        Point::at(&mut self.x - &mut other.x, &mut self.y - &mut other.y)
    }
}

impl<T, U> SubAssign<Point<U>> for Point<T>
where
    T: SubAssign<U>,
{
    fn sub_assign(&mut self, other: Point<U>) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl<'a, T, U> SubAssign<&'a Point<U>> for Point<T>
where
    T: SubAssign<&'a U>,
{
    fn sub_assign(&mut self, other: &'a Point<U>) {
        self.x -= &other.x;
        self.y -= &other.y;
    }
}

impl<'a, T, U> SubAssign<&'a mut Point<U>> for Point<T>
where
    T: SubAssign<&'a mut U>,
{
    fn sub_assign(&mut self, other: &'a mut Point<U>) {
        self.x -= &mut other.x;
        self.y -= &mut other.y;
    }
}

impl<'a, T, U> SubAssign<Point<U>> for &'a mut Point<T>
where
    T: SubAssign<U>,
{
    fn sub_assign(&mut self, other: Point<U>) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl<'a, 'b, T, U> SubAssign<&'b Point<U>> for &'a mut Point<T>
where
    T: SubAssign<&'b U>,
{
    fn sub_assign(&mut self, other: &'b Point<U>) {
        self.x -= &other.x;
        self.y -= &other.y;
    }
}

impl<'a, 'b, T, U> SubAssign<&'b mut Point<U>> for &'a mut Point<T>
where
    T: SubAssign<&'b mut U>,
{
    fn sub_assign(&mut self, other: &'b mut Point<U>) {
        self.x -= &mut other.x;
        self.y -= &mut other.y;
    }
}

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
