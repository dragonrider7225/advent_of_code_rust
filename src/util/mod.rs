use std::ops::{
    Add,
    AddAssign,
    Div,
    DivAssign,
    Mul,
    MulAssign,
    Neg,
    Sub,
    SubAssign,
};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    pub fn at(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> &T {
        &self.x
    }

    pub fn y(&self) -> &T {
        &self.y
    }
}

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
