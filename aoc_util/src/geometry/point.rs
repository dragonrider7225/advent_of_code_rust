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

macro_rules! impl_manhattan_distance_2d_const {
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

impl_manhattan_distance_2d_const!(
    u8 u16 u32 u64 u128 usize
    i8 i16 i32 i64 i128 isize
);

macro_rules! impl_manhattan_distance_2d {
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

impl_manhattan_distance_2d!(f32 f64);

macro_rules! impl_checked_arithmetic {
    ($($t:ty)+) => {$(
        impl Point2D<$t> {
            /// Calculate `self + rhs` if that calculation would not overflow.
            pub fn checked_add(self, rhs: Self) -> Option<Self> {
                Some(Self {
                    x: self.x.checked_add(rhs.x)?,
                    y: self.y.checked_add(rhs.y)?,
                })
            }

            /// Calculate `self - rhs` if that calculation would not overflow.
            pub fn checked_sub(self, rhs: Self) -> Option<Self> {
                Some(Self {
                    x: self.x.checked_sub(rhs.x)?,
                    y: self.y.checked_sub(rhs.y)?,
                })
            }
        }
    )+};
}

impl_checked_arithmetic!(
    u8 u16 u32 u64 u128 usize
    i8 i16 i32 i64 i128 isize
);

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

impl<'a, T, U, V> Add<Point2D<U>> for &'a Point2D<T>
where
    &'a T: Add<U, Output = V>,
{
    type Output = Point2D<V>;

    fn add(self, other: Point2D<U>) -> Self::Output {
        Point2D::at(&self.x + other.x, &self.y + other.y)
    }
}

impl<T, U, V> Add<&Point2D<U>> for &Point2D<T>
where
    for<'a> &'a T: Add<&'a U, Output = V>,
{
    type Output = Point2D<V>;

    fn add(self, other: &Point2D<U>) -> Self::Output {
        Point2D::at(&self.x + &other.x, &self.y + &other.y)
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

impl<T, U, V> Sub<Point2D<U>> for Point2D<T>
where
    T: Sub<U, Output = V>,
{
    type Output = Point2D<V>;

    fn sub(self, other: Point2D<U>) -> Self::Output {
        Point2D::at(self.x - other.x, self.y - other.y)
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

impl<'a, T, U, V> Sub<&'a Point2D<U>> for Point2D<T>
where
    T: Sub<&'a U, Output = V>,
{
    type Output = Point2D<V>;

    fn sub(self, other: &'a Point2D<U>) -> Self::Output {
        Point2D::at(self.x - &other.x, self.y - &other.y)
    }
}

impl<T, U, V> Sub<&Point2D<U>> for &Point2D<T>
where
    for<'a> &'a T: Sub<&'a U, Output = V>,
{
    type Output = Point2D<V>;

    fn sub(self, other: &Point2D<U>) -> Self::Output {
        Point2D::at(&self.x - &other.x, &self.y - &other.y)
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

/// A 3-dimensional point.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Point3D<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Point3D<T> {
    /// Creates a new point with the given coordinates.
    pub const fn at(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    /// The x-coordinate of the point.
    pub const fn x(&self) -> &T {
        &self.x
    }

    /// The y-coordinate of the point.
    pub const fn y(&self) -> &T {
        &self.y
    }

    /// The z-coordinate of the point.
    pub const fn z(&self) -> &T {
        &self.z
    }

    /// Compute the dot product of `self` and `rhs`.
    ///
    /// Geometrically, the dot product of two vectors `u` and `v` with inner angle `theta` is
    /// `u.len() * v.len() * theta.cos()`.
    pub fn dot<U, V>(&self, rhs: &Point3D<U>) -> V
    where
        for<'a, 'b> &'a T: Mul<&'b U, Output = V>,
        V: Add<Output = V>,
    {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }

    /// Compute the cross product of `self` and `rhs`.
    ///
    /// Geometrically, the cross product of two vectors `u` and `v` with inner angle `theta` in
    /// 3-space has length `u.len() * v.len() * theta.cos()` and is perpendicular to both `u` and
    /// `v`. If `u.cross(&v)` points "up" then `u` is to the right of the angle `theta` and `v` is
    /// to the left.
    pub fn cross<U, V>(&self, rhs: &Point3D<U>) -> Point3D<V>
    where
        for<'a, 'b> &'a T: Mul<&'b U>,
        for<'a, 'b> <&'a T as Mul<&'b U>>::Output: std::ops::Sub<Output = V>,
    {
        Point3D {
            x: &self.y * &rhs.z - &self.z * &rhs.y,
            y: &self.z * &rhs.x - &self.x * &rhs.z,
            z: &self.x * &rhs.y - &self.y * &rhs.x,
        }
    }
}

macro_rules! impl_manhattan_distance_3d_const {
    ($($t:ty)+) => ($(
        impl Point3D<$t> {
            /// Calculates the sum of the distance between the x-coordinates, the distance between
            /// the y-coordinates, and the distance between the z-coordinates.
            pub const fn manhattan_distance(&self, rhs: &Self) -> $t {
                (self.x.abs_diff(rhs.x) + self.y.abs_diff(rhs.y) + self.z.abs_diff(rhs.z)) as $t
            }
        }
    )+)
}

impl_manhattan_distance_3d_const!(
    u8 u16 u32 u64 u128 usize
    i8 i16 i32 i64 i128 isize
);

macro_rules! impl_manhattan_distance_3d {
    ($($t:ty)+) => ($(
        impl Point3D<$t> {
            /// Calculates the sum of the distance between the x-coordinates, the distance between
            /// the y-coordinates, and the distance between the z-coordinates.
            pub fn manhattan_distance(&self, rhs: &Self) -> $t {
                (self.x - rhs.x).abs() + (self.y - rhs.y).abs() + (self.z - rhs.z).abs()
            }
        }
    )+)
}

impl_manhattan_distance_3d!(f32 f64);

impl<T, U, V> Add<Point3D<U>> for Point3D<T>
where
    T: Add<U, Output = V>,
{
    type Output = Point3D<V>;

    fn add(self, other: Point3D<U>) -> Self::Output {
        Point3D::at(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl<'a, T, U, V> Add<&'a Point3D<U>> for Point3D<T>
where
    T: Add<&'a U, Output = V>,
{
    type Output = Point3D<V>;

    fn add(self, other: &'a Point3D<U>) -> Self::Output {
        Point3D::at(self.x + &other.x, self.y + &other.y, self.z + &other.z)
    }
}

impl<'a, T, U, V> Add<Point3D<U>> for &'a Point3D<T>
where
    &'a T: Add<U, Output = V>,
{
    type Output = Point3D<V>;

    fn add(self, other: Point3D<U>) -> Self::Output {
        Point3D::at(&self.x + other.x, &self.y + other.y, &self.z + other.z)
    }
}

impl<T, U, V> Add<&Point3D<U>> for &Point3D<T>
where
    for<'a> &'a T: Add<&'a U, Output = V>,
{
    type Output = Point3D<V>;

    fn add(self, other: &Point3D<U>) -> Self::Output {
        Point3D::at(&self.x + &other.x, &self.y + &other.y, &self.z + &other.z)
    }
}

impl<T, U> AddAssign<Point3D<U>> for Point3D<T>
where
    T: AddAssign<U>,
{
    fn add_assign(&mut self, other: Point3D<U>) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<'a, T, U> AddAssign<&'a Point3D<U>> for Point3D<T>
where
    T: AddAssign<&'a U>,
{
    fn add_assign(&mut self, other: &'a Point3D<U>) {
        self.x += &other.x;
        self.y += &other.y;
        self.z += &other.z;
    }
}

impl<T, U, V> Div<U> for Point3D<T>
where
    T: Div<U, Output = V>,
    U: Clone,
{
    type Output = Point3D<V>;

    fn div(self, other: U) -> Self::Output {
        Point3D::at(
            self.x / other.clone(),
            self.y / other.clone(),
            self.z / other,
        )
    }
}

impl<'a, T, U, V> Div<U> for &'a Point3D<T>
where
    &'a T: Div<U, Output = V>,
    U: Clone,
{
    type Output = Point3D<V>;

    fn div(self, other: U) -> Self::Output {
        Point3D::at(
            &self.x / other.clone(),
            &self.y / other.clone(),
            &self.z / other,
        )
    }
}

impl<T, U> DivAssign<U> for Point3D<T>
where
    T: DivAssign<U>,
    U: Clone,
{
    fn div_assign(&mut self, other: U) {
        self.x /= other.clone();
        self.y /= other.clone();
        self.z /= other;
    }
}

impl<T, U, V> Mul<U> for Point3D<T>
where
    T: Mul<U, Output = V>,
    U: Clone,
{
    type Output = Point3D<V>;

    fn mul(self, other: U) -> Self::Output {
        Point3D::at(
            self.x * other.clone(),
            self.y * other.clone(),
            self.z * other,
        )
    }
}

impl<'a, T, U, V> Mul<U> for &'a Point3D<T>
where
    &'a T: Mul<U, Output = V>,
    U: Clone,
{
    type Output = Point3D<V>;

    fn mul(self, other: U) -> Self::Output {
        Point3D::at(
            &self.x * other.clone(),
            &self.y * other.clone(),
            &self.z * other,
        )
    }
}

impl<T, U> MulAssign<U> for Point3D<T>
where
    T: MulAssign<U>,
    U: Clone,
{
    fn mul_assign(&mut self, other: U) {
        self.x *= other.clone();
        self.y *= other.clone();
        self.z *= other;
    }
}

impl<T, U> Neg for Point3D<T>
where
    T: Neg<Output = U>,
{
    type Output = Point3D<U>;

    fn neg(self) -> Self::Output {
        Point3D::at(-self.x, -self.y, -self.z)
    }
}

impl<'a, T, U> Neg for &'a Point3D<T>
where
    &'a T: Neg<Output = U>,
{
    type Output = Point3D<U>;

    fn neg(self) -> Self::Output {
        Point3D::at(-&self.x, -&self.y, -&self.z)
    }
}

impl<T, U, V> Sub<Point3D<U>> for Point3D<T>
where
    T: Sub<U, Output = V>,
{
    type Output = Point3D<V>;

    fn sub(self, other: Point3D<U>) -> Self::Output {
        Point3D::at(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl<'a, T, U, V> Sub<&'a Point3D<U>> for Point3D<T>
where
    T: Sub<&'a U, Output = V>,
{
    type Output = Point3D<V>;

    fn sub(self, other: &'a Point3D<U>) -> Self::Output {
        Point3D::at(self.x - &other.x, self.y - &other.y, self.z - &other.z)
    }
}

impl<'a, T, U, V> Sub<Point3D<U>> for &'a Point3D<T>
where
    &'a T: Sub<U, Output = V>,
{
    type Output = Point3D<V>;

    fn sub(self, other: Point3D<U>) -> Self::Output {
        Point3D::at(&self.x - other.x, &self.y - other.y, &self.z - other.z)
    }
}

impl<T, U, V> Sub<&Point3D<U>> for &Point3D<T>
where
    for<'a> &'a T: Sub<&'a U, Output = V>,
{
    type Output = Point3D<V>;

    fn sub(self, other: &Point3D<U>) -> Self::Output {
        Point3D::at(&self.x - &other.x, &self.y - &other.y, &self.z - &other.z)
    }
}

impl<T, U> SubAssign<Point3D<U>> for Point3D<T>
where
    T: SubAssign<U>,
{
    fn sub_assign(&mut self, other: Point3D<U>) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl<'a, T, U> SubAssign<&'a Point3D<U>> for Point3D<T>
where
    T: SubAssign<&'a U>,
{
    fn sub_assign(&mut self, other: &'a Point3D<U>) {
        self.x -= &other.x;
        self.y -= &other.y;
        self.z -= &other.z;
    }
}
