use std::{
    cmp::Ordering,
    fmt::{self, Display, Formatter},
    num::NonZeroI128,
    ops::{Add, Div, Mul, Neg, Sub},
};

fn gcd(mut a: i128, mut b: i128) -> i128 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Fraction {
    numerator: i128,
    denominator: NonZeroI128,
}

impl Fraction {
    pub fn new(numerator: i128, denominator: NonZeroI128) -> Self {
        if numerator == 0 {
            return Self::from(0);
        }
        let gcd = gcd(numerator.abs(), denominator.get().abs());
        let numerator = numerator / gcd;
        let denominator = NonZeroI128::new(denominator.get() / gcd).unwrap();
        if denominator.get() < 0 {
            Self {
                numerator: -numerator,
                denominator: -denominator,
            }
        } else {
            Self {
                numerator,
                denominator,
            }
        }
    }

    pub fn trunc(&self) -> i128 {
        self.numerator / self.denominator.get()
    }

    pub fn fract(&self) -> f64 {
        let self_rem = self.numerator % self.denominator.get();
        self_rem as f64 / self.denominator.get() as f64
    }

    pub const fn signum(&self) -> i128 {
        self.numerator.signum()
    }

    pub fn is_integral(&self) -> bool {
        self.numerator % self.denominator.get() == 0
    }
}

impl Add for Fraction {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.numerator * rhs.denominator.get() + rhs.numerator * self.denominator.get(),
            NonZeroI128::new(self.denominator.get() * rhs.denominator.get()).unwrap(),
        )
    }
}

impl Add<&'_ Self> for Fraction {
    type Output = Self;

    fn add(self, rhs: &'_ Self) -> Self::Output {
        self + *rhs
    }
}

impl Add<Fraction> for &'_ Fraction {
    type Output = Fraction;

    fn add(self, rhs: Fraction) -> Self::Output {
        *self + rhs
    }
}

impl Add<&'_ Fraction> for &'_ Fraction {
    type Output = Fraction;

    fn add(self, rhs: &'_ Fraction) -> Self::Output {
        *self + *rhs
    }
}

impl Add<i128> for Fraction {
    type Output = Self;

    fn add(mut self, rhs: i128) -> Self::Output {
        self.numerator += rhs * self.denominator.get();
        self
    }
}

impl Add<&'_ i128> for Fraction {
    type Output = Self;

    fn add(self, rhs: &'_ i128) -> Self::Output {
        self + *rhs
    }
}

impl Add<i128> for &'_ Fraction {
    type Output = Fraction;

    fn add(self, rhs: i128) -> Self::Output {
        *self + rhs
    }
}

impl Add<&'_ i128> for &'_ Fraction {
    type Output = Fraction;

    fn add(self, rhs: &'_ i128) -> Self::Output {
        *self + *rhs
    }
}

impl Add<Fraction> for i128 {
    type Output = Fraction;

    fn add(self, rhs: Fraction) -> Self::Output {
        rhs + self
    }
}

impl Add<&'_ Fraction> for i128 {
    type Output = Fraction;

    fn add(self, rhs: &'_ Fraction) -> Self::Output {
        self + *rhs
    }
}

impl Add<Fraction> for &'_ i128 {
    type Output = Fraction;

    fn add(self, rhs: Fraction) -> Self::Output {
        *self + rhs
    }
}

impl Add<&'_ Fraction> for &'_ i128 {
    type Output = Fraction;

    fn add(self, rhs: &'_ Fraction) -> Self::Output {
        *self + *rhs
    }
}

impl Display for Fraction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.numerator)?;
        if self.denominator.get() != 1 {
            write!(f, "/{}", self.denominator.get())?;
        }
        Ok(())
    }
}

impl Div for Fraction {
    type Output = Self;

    #[expect(
        clippy::suspicious_arithmetic_impl,
        reason = "division is multiplication by reciprocal"
    )]
    fn div(self, rhs: Self) -> Self::Output {
        self * Self::new(
            rhs.denominator.get(),
            NonZeroI128::new(rhs.numerator).expect("Cannot divide by 0"),
        )
    }
}

impl Div<&'_ Self> for Fraction {
    type Output = Self;

    fn div(self, rhs: &'_ Self) -> Self::Output {
        self / *rhs
    }
}

impl Div<Fraction> for &'_ Fraction {
    type Output = Fraction;

    fn div(self, rhs: Fraction) -> Self::Output {
        *self / rhs
    }
}

impl Div<&'_ Fraction> for &'_ Fraction {
    type Output = Fraction;

    fn div(self, rhs: &'_ Fraction) -> Self::Output {
        *self / *rhs
    }
}

impl From<i128> for Fraction {
    fn from(value: i128) -> Self {
        Self {
            numerator: value,
            denominator: NonZeroI128::new(1).unwrap(),
        }
    }
}

impl Mul for Fraction {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let self_rhs = Self::new(self.numerator, rhs.denominator);
        let rhs_self = Self::new(rhs.numerator, self.denominator);
        Self {
            numerator: self_rhs.numerator * rhs_self.numerator,
            denominator: NonZeroI128::new(self_rhs.denominator.get() * rhs_self.denominator.get())
                .unwrap(),
        }
    }
}

impl Mul<&'_ Self> for Fraction {
    type Output = Self;

    fn mul(self, rhs: &'_ Self) -> Self::Output {
        self * *rhs
    }
}

impl Mul<Fraction> for &'_ Fraction {
    type Output = Fraction;

    fn mul(self, rhs: Fraction) -> Self::Output {
        *self * rhs
    }
}

impl Mul<&'_ Fraction> for &'_ Fraction {
    type Output = Fraction;

    fn mul(self, rhs: &'_ Fraction) -> Self::Output {
        *self * *rhs
    }
}

impl Neg for Fraction {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.numerator = -self.numerator;
        self
    }
}

impl Neg for &'_ Fraction {
    type Output = Fraction;

    fn neg(self) -> Self::Output {
        -*self
    }
}

impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.denominator == other.denominator {
            self.numerator.cmp(&other.numerator)
        } else {
            self.trunc()
                .cmp(&other.trunc())
                .then(self.fract().total_cmp(&other.fract()))
        }
    }
}

impl PartialEq<Fraction> for i128 {
    fn eq(&self, other: &Fraction) -> bool {
        other.eq(self)
    }
}

impl PartialEq<i128> for Fraction {
    fn eq(&self, other: &i128) -> bool {
        self.is_integral() && self.trunc().eq(other)
    }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd<i128> for Fraction {
    fn partial_cmp(&self, other: &i128) -> Option<Ordering> {
        if self.is_integral() {
            self.trunc().partial_cmp(other)
        } else {
            match self.numerator.signum() {
                1 => Some(self.trunc().cmp(other).then(Ordering::Greater)),
                -1 => Some(self.trunc().cmp(other).then(Ordering::Less)),
                0 => unreachable!("0 numerator is always integral"),
                _ => unreachable!("signum always returns -1, 0, or 1"),
            }
        }
    }
}

impl PartialOrd<Fraction> for i128 {
    fn partial_cmp(&self, other: &Fraction) -> Option<Ordering> {
        other.partial_cmp(self).map(|o| o.reverse())
    }
}

impl Sub for Fraction {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.add(-rhs)
    }
}

impl Sub<&'_ Self> for Fraction {
    type Output = Self;

    fn sub(self, rhs: &'_ Self) -> Self::Output {
        self - *rhs
    }
}

impl Sub<Fraction> for &'_ Fraction {
    type Output = Fraction;

    fn sub(self, rhs: Fraction) -> Self::Output {
        *self - rhs
    }
}

impl Sub<&'_ Fraction> for &'_ Fraction {
    type Output = Fraction;

    fn sub(self, rhs: &'_ Fraction) -> Self::Output {
        *self - *rhs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partial_ord() {
        let x = Fraction {
            numerator: -6,
            denominator: NonZeroI128::new(1).unwrap(),
        };
        assert!(x < 7);
        assert!(7 > x);
        assert!(!(7..=27).contains(&x));
    }
}
