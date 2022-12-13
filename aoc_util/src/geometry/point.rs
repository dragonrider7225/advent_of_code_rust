use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// A 2-dimensional point.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Point2D<T> {
    x: T,
    y: T,
}

impl<T> Point2D<T> {
    /// Creates a new point with the given coordinates.
    pub const fn at(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// The x-coordinate of the point.
    pub const fn x(&self) -> &T {
        &self.x
    }

    /// The y-coordinate of the point.
    pub const fn y(&self) -> &T {
        &self.y
    }
}

macro_rules! impl_manhattan_distance_const {
    ($($t:ty)+) => ($(
        impl Point2D<$t> {
            /// Calculates the sum of the distance between the x-coordinates and the distance
            /// between the y-coordinates.
            pub const fn manhattan_distance(&self, rhs: &Self) -> $t {
                (self.x.abs_diff(rhs.x) + self.y.abs_diff(rhs.y)) as $t
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
        impl Point2D<$t> {
            /// Calculates the sum of the distance between the x-coordinates and the distance
            /// between the y-coordinates.
            pub fn manhattan_distance(&self, rhs: &Self) -> $t {
                (self.x - rhs.x).abs() + (self.y - rhs.y).abs()
            }
        }
    )+)
}

impl_manhattan_distance!(f32 f64);

impl<T, U, V> Add<Point2D<U>> for Point2D<T>
where
    T: Add<U, Output = V>,
{
    type Output = Point2D<V>;

    fn add(self, other: Point2D<U>) -> Self::Output {
        Point2D::at(self.x + other.x, self.y + other.y)
    }
}

impl<'a, T, U, V> Add<&'a Point2D<U>> for Point2D<T>
where
    T: Add<&'a U, Output = V>,
{
    type Output = Point2D<V>;

    fn add(self, other: &'a Point2D<U>) -> Self::Output {
        Point2D::at(self.x + &other.x, self.y + &other.y)
    }
}

impl<'a, T, U, V> Add<&'a mut Point2D<U>> for Point2D<T>
where
    T: Add<&'a mut U, Output = V>,
{
    type Output = Point2D<V>;

    fn add(self, other: &'a mut Point2D<U>) -> Self::Output {
        Point2D::at(self.x + &mut other.x, self.y + &mut other.y)
    }
}

impl<'a, T, U, V> Add<Point2D<U>> for &'a Point2D<T>
where
    &'a T: Add<U, Output = V>,
{
    type Output = Point2D<V>;

    fn add(self, other: Point2D<U>) -> Self::Output {
        Point2D::at(&self.x + other.x, &self.y + other.y)
    }
}

impl<'a, 'b, T, U, V> Add<&'b Point2D<U>> for &'a Point2D<T>
where
    &'a T: Add<&'b U, Output = V>,
{
    type Output = Point2D<V>;

    fn add(self, other: &'b Point2D<U>) -> Self::Output {
        Point2D::at(&self.x + &other.x, &self.y + &other.y)
    }
}

impl<'a, 'b, T, U, V> Add<&'b mut Point2D<U>> for &'a Point2D<T>
where
    &'a T: Add<&'b mut U, Output = V>,
{
    type Output = Point2D<V>;

    fn add(self, other: &'b mut Point2D<U>) -> Self::Output {
        Point2D::at(&self.x + &mut other.x, &self.y + &mut other.y)
    }
}

impl<'a, T, U, V> Add<Point2D<U>> for &'a mut Point2D<T>
where
    &'a mut T: Add<U, Output = V>,
{
    type Output = Point2D<V>;

    fn add(self, other: Point2D<U>) -> Self::Output {
        Point2D::at(&mut self.x + other.x, &mut self.y + other.y)
    }
}

impl<'a, 'b, T, U, V> Add<&'b Point2D<U>> for &'a mut Point2D<T>
where
    &'a mut T: Add<&'b U, Output = V>,
{
    type Output = Point2D<V>;

    fn add(self, other: &'b Point2D<U>) -> Self::Output {
        Point2D::at(&mut self.x + &other.x, &mut self.y + &other.y)
    }
}

impl<'a, 'b, T, U, V> Add<&'b mut Point2D<U>> for &'a mut Point2D<T>
where
    &'a mut T: Add<&'b mut U, Output = V>,
{
    type Output = Point2D<V>;

    fn add(self, other: &'b mut Point2D<U>) -> Self::Output {
        Point2D::at(&mut self.x + &mut other.x, &mut self.y + &mut other.y)
    }
}

impl<T, U> AddAssign<Point2D<U>> for Point2D<T>
where
    T: AddAssign<U>,
{
    fn add_assign(&mut self, other: Point2D<U>) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<'a, T, U> AddAssign<&'a Point2D<U>> for Point2D<T>
where
    T: AddAssign<&'a U>,
{
    fn add_assign(&mut self, other: &'a Point2D<U>) {
        self.x += &other.x;
        self.y += &other.y;
    }
}

impl<'a, T, U> AddAssign<&'a mut Point2D<U>> for Point2D<T>
where
    T: AddAssign<&'a mut U>,
{
    fn add_assign(&mut self, other: &'a mut Point2D<U>) {
        self.x += &mut other.x;
        self.y += &mut other.y;
    }
}

impl<'a, T, U> AddAssign<Point2D<U>> for &'a mut Point2D<T>
where
    T: AddAssign<U>,
{
    fn add_assign(&mut self, other: Point2D<U>) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<'a, 'b, T, U> AddAssign<&'b Point2D<U>> for &'a mut Point2D<T>
where
    T: AddAssign<&'b U>,
{
    fn add_assign(&mut self, other: &'b Point2D<U>) {
        self.x += &other.x;
        self.y += &other.y;
    }
}

impl<'a, 'b, T, U> AddAssign<&'b mut Point2D<U>> for &'a mut Point2D<T>
where
    T: AddAssign<&'b mut U>,
{
    fn add_assign(&mut self, other: &'b mut Point2D<U>) {
        self.x += &mut other.x;
        self.y += &mut other.y;
    }
}

impl<T, U, V> Div<U> for Point2D<T>
where
    T: Div<U, Output = V>,
    U: Clone,
{
    type Output = Point2D<V>;

    fn div(self, other: U) -> Self::Output {
        Point2D::at(self.x / other.clone(), self.y / other)
    }
}

impl<'a, T, U, V> Div<U> for &'a Point2D<T>
where
    &'a T: Div<U, Output = V>,
    U: Clone,
{
    type Output = Point2D<V>;

    fn div(self, other: U) -> Self::Output {
        Point2D::at(&self.x / other.clone(), &self.y / other)
    }
}

impl<'a, T, U, V> Div<U> for &'a mut Point2D<T>
where
    &'a mut T: Div<U, Output = V>,
    U: Clone,
{
    type Output = Point2D<V>;

    fn div(self, other: U) -> Self::Output {
        Point2D::at(&mut self.x / other.clone(), &mut self.y / other)
    }
}

impl<T, U> DivAssign<U> for Point2D<T>
where
    T: DivAssign<U>,
    U: Clone,
{
    fn div_assign(&mut self, other: U) {
        self.x /= other.clone();
        self.y /= other;
    }
}

impl<'a, T, U> DivAssign<U> for &'a mut Point2D<T>
where
    T: DivAssign<U>,
    U: Clone,
{
    fn div_assign(&mut self, other: U) {
        self.x /= other.clone();
        self.y /= other;
    }
}

impl<T, U, V> Mul<U> for Point2D<T>
where
    T: Mul<U, Output = V>,
    U: Clone,
{
    type Output = Point2D<V>;

    fn mul(self, other: U) -> Self::Output {
        Point2D::at(self.x * other.clone(), self.y * other)
    }
}

impl<'a, T, U, V> Mul<U> for &'a Point2D<T>
where
    &'a T: Mul<U, Output = V>,
    U: Clone,
{
    type Output = Point2D<V>;

    fn mul(self, other: U) -> Self::Output {
        Point2D::at(&self.x * other.clone(), &self.y * other)
    }
}

impl<'a, T, U, V> Mul<U> for &'a mut Point2D<T>
where
    &'a mut T: Mul<U, Output = V>,
    U: Clone,
{
    type Output = Point2D<V>;

    fn mul(self, other: U) -> Self::Output {
        Point2D::at(&mut self.x * other.clone(), &mut self.y * other)
    }
}

impl<T, U> MulAssign<U> for Point2D<T>
where
    T: MulAssign<U>,
    U: Clone,
{
    fn mul_assign(&mut self, other: U) {
        self.x *= other.clone();
        self.y *= other;
    }
}

impl<'a, T, U> MulAssign<U> for &'a mut Point2D<T>
where
    T: MulAssign<U>,
    U: Clone,
{
    fn mul_assign(&mut self, other: U) {
        self.x *= other.clone();
        self.y *= other;
    }
}

impl<T, U> Neg for Point2D<T>
where
    T: Neg<Output = U>,
{
    type Output = Point2D<U>;

    fn neg(self) -> Self::Output {
        Point2D::at(-self.x, -self.y)
    }
}

impl<'a, T, U> Neg for &'a Point2D<T>
where
    &'a T: Neg<Output = U>,
{
    type Output = Point2D<U>;

    fn neg(self) -> Self::Output {
        Point2D::at(-&self.x, -&self.y)
    }
}

impl<'a, T, U> Neg for &'a mut Point2D<T>
where
    &'a mut T: Neg<Output = U>,
{
    type Output = Point2D<U>;

    fn neg(self) -> Self::Output {
        Point2D::at(-&mut self.x, -&mut self.y)
    }
}

impl<T, U, V> Sub<Point2D<U>> for Point2D<T>
where
    T: Sub<U, Output = V>,
{
    type Output = Point2D<V>;

    fn sub(self, other: Point2D<U>) -> Self::Output {
        Point2D::at(self.x - other.x, self.y - other.y)
    }
}

impl<'a, T, U, V> Sub<&'a Point2D<U>> for Point2D<T>
where
    T: Sub<&'a U, Output = V>,
{
    type Output = Point2D<V>;

    fn sub(self, other: &'a Point2D<U>) -> Self::Output {
        Point2D::at(self.x - &other.x, self.y - &other.y)
    }
}

impl<'a, T, U, V> Sub<&'a mut Point2D<U>> for Point2D<T>
where
    T: Sub<&'a mut U, Output = V>,
{
    type Output = Point2D<V>;

    fn sub(self, other: &'a mut Point2D<U>) -> Self::Output {
        Point2D::at(self.x - &mut other.x, self.y - &mut other.y)
    }
}

impl<'a, T, U, V> Sub<Point2D<U>> for &'a Point2D<T>
where
    &'a T: Sub<U, Output = V>,
{
    type Output = Point2D<V>;

    fn sub(self, other: Point2D<U>) -> Self::Output {
        Point2D::at(&self.x - other.x, &self.y - other.y)
    }
}

impl<'a, 'b, T, U, V> Sub<&'b Point2D<U>> for &'a Point2D<T>
where
    &'a T: Sub<&'b U, Output = V>,
{
    type Output = Point2D<V>;

    fn sub(self, other: &'b Point2D<U>) -> Self::Output {
        Point2D::at(&self.x - &other.x, &self.y - &other.y)
    }
}

impl<'a, 'b, T, U, V> Sub<&'b mut Point2D<U>> for &'a Point2D<T>
where
    &'a T: Sub<&'b mut U, Output = V>,
{
    type Output = Point2D<V>;

    fn sub(self, other: &'b mut Point2D<U>) -> Self::Output {
        Point2D::at(&self.x - &mut other.x, &self.y - &mut other.y)
    }
}

impl<'a, T, U, V> Sub<Point2D<U>> for &'a mut Point2D<T>
where
    &'a mut T: Sub<U, Output = V>,
{
    type Output = Point2D<V>;

    fn sub(self, other: Point2D<U>) -> Self::Output {
        Point2D::at(&mut self.x - other.x, &mut self.y - other.y)
    }
}

impl<'a, 'b, T, U, V> Sub<&'b Point2D<U>> for &'a mut Point2D<T>
where
    &'a mut T: Sub<&'b U, Output = V>,
{
    type Output = Point2D<V>;

    fn sub(self, other: &'b Point2D<U>) -> Self::Output {
        Point2D::at(&mut self.x - &other.x, &mut self.y - &other.y)
    }
}

impl<'a, 'b, T, U, V> Sub<&'b mut Point2D<U>> for &'a mut Point2D<T>
where
    &'a mut T: Sub<&'b mut U, Output = V>,
{
    type Output = Point2D<V>;

    fn sub(self, other: &'b mut Point2D<U>) -> Self::Output {
        Point2D::at(&mut self.x - &mut other.x, &mut self.y - &mut other.y)
    }
}

impl<T, U> SubAssign<Point2D<U>> for Point2D<T>
where
    T: SubAssign<U>,
{
    fn sub_assign(&mut self, other: Point2D<U>) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl<'a, T, U> SubAssign<&'a Point2D<U>> for Point2D<T>
where
    T: SubAssign<&'a U>,
{
    fn sub_assign(&mut self, other: &'a Point2D<U>) {
        self.x -= &other.x;
        self.y -= &other.y;
    }
}

impl<'a, T, U> SubAssign<&'a mut Point2D<U>> for Point2D<T>
where
    T: SubAssign<&'a mut U>,
{
    fn sub_assign(&mut self, other: &'a mut Point2D<U>) {
        self.x -= &mut other.x;
        self.y -= &mut other.y;
    }
}

impl<'a, T, U> SubAssign<Point2D<U>> for &'a mut Point2D<T>
where
    T: SubAssign<U>,
{
    fn sub_assign(&mut self, other: Point2D<U>) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl<'a, 'b, T, U> SubAssign<&'b Point2D<U>> for &'a mut Point2D<T>
where
    T: SubAssign<&'b U>,
{
    fn sub_assign(&mut self, other: &'b Point2D<U>) {
        self.x -= &other.x;
        self.y -= &other.y;
    }
}

impl<'a, 'b, T, U> SubAssign<&'b mut Point2D<U>> for &'a mut Point2D<T>
where
    T: SubAssign<&'b mut U>,
{
    fn sub_assign(&mut self, other: &'b mut Point2D<U>) {
        self.x -= &mut other.x;
        self.y -= &mut other.y;
    }
}
