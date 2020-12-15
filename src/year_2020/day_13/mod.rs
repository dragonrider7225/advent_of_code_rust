use crate::parse::NomParse;
use nom::{branch, character::complete as character, combinator as comb, multi, IResult};
use std::{
    convert::TryFrom,
    fmt::{self, Display, Formatter},
    fs, io, mem,
    ops::{Add, AddAssign, Rem, Sub},
};

fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b > 0 && a != b {
        a %= b;
        mem::swap(&mut a, &mut b);
    }
    a
}

fn lcm(a: u128, b: u128) -> u128 {
    if a == 0 {
        b
    } else if b == 0 {
        a
    } else {
        let gcd = gcd(a, b);
        a * (b / gcd)
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Duration(u128);

impl TryFrom<usize> for Duration {
    type Error = <u128 as TryFrom<usize>>::Error;

    fn try_from(old: usize) -> Result<Self, Self::Error> {
        Ok(Self(u128::try_from(old)?))
    }
}

impl Rem<u128> for Duration {
    type Output = Duration;

    fn rem(self, rhs: u128) -> Self {
        Self(self.0 % rhs)
    }
}

impl Sub for Duration {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Timestamp(u128);

impl Add<Duration> for Timestamp {
    type Output = Timestamp;

    fn add(self, rhs: Duration) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign<Duration> for Timestamp {
    fn add_assign(&mut self, rhs: Duration) {
        self.0 += rhs.0;
    }
}

impl Display for Timestamp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl_from_str_for_nom_parse!(Timestamp);

impl<'s> NomParse<'s> for Timestamp {
    fn nom_parse(s: &str) -> IResult<&str, Self> {
        comb::map(u128::nom_parse, Self)(s)
    }
}

impl Rem<u128> for Timestamp {
    type Output = Duration;

    fn rem(self, rhs: u128) -> Self::Output {
        Duration(self.0 % rhs)
    }
}

impl TryFrom<usize> for Timestamp {
    type Error = <u128 as TryFrom<usize>>::Error;

    fn try_from(old: usize) -> Result<Self, Self::Error> {
        Ok(Self(u128::try_from(old)?))
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct BusNumber(u128);

impl BusNumber {
    fn runs_at(&self, time: Timestamp) -> bool {
        time.0 % self.0 == 0
    }

    fn wait(&self, start_time: Timestamp) -> Duration {
        (0..)
            .map(Duration)
            .find(|&delay| self.runs_at(start_time + delay))
            .unwrap()
    }
}

impl Display for BusNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'s> NomParse<'s> for BusNumber {
    fn nom_parse(s: &str) -> IResult<&str, Self> {
        comb::map(u128::nom_parse, Self)(s)
    }
}

struct BusSchedule {
    buses: Vec<BusNumber>,
}

impl BusSchedule {
    /// Gets a pair of `(next_bus, delay)` for the least delay.
    fn next_bus(&self, time: Timestamp) -> (BusNumber, Duration) {
        self.buses
            .iter()
            .filter(|&&bus| bus != BusNumber(0))
            .map(|&bus| (bus, bus.wait(time)))
            .min_by_key(|&(_, delay)| delay)
            .expect("No buses")
    }

    fn first_diagonal(&self) -> Timestamp {
        let bus_offsets = self
            .buses
            .iter()
            .enumerate()
            .filter(|&(_, &bus_number)| bus_number != BusNumber(0))
            .map(|(offset, &bus_number)| (Duration::try_from(offset).unwrap(), bus_number))
            .collect::<Vec<_>>();
        let max_step = bus_offsets
            .iter()
            .map(|&(_, bus_number)| bus_number.0)
            .fold(1, lcm);
        println!("The time between diagonals is {}", max_step);
        let mut buses_satisfied = 1;
        let mut step = self.buses[0].0;
        let mut time = Timestamp(0u128);
        loop {
            if let Some((new_idx, extra_skip)) = bus_offsets[buses_satisfied..]
                .iter()
                .zip(buses_satisfied..)
                .take_while(|&(&(offset, bus_number), _)| bus_number.runs_at(time + offset))
                .fold(None, |acc, (&(_, bus_number), idx)| {
                    acc.or(Some((idx, 1)))
                        .map(|(_, base)| (idx, lcm(base, bus_number.0)))
                })
            {
                if new_idx == bus_offsets.len() - 1 {
                    return time;
                } else if new_idx > buses_satisfied {
                    buses_satisfied = new_idx;
                    let new_step = lcm(step, extra_skip);
                    step = new_step;
                }
            }

            time += Duration(step);
        }
    }
}

impl_from_str_for_nom_parse!(BusSchedule);

impl<'s> NomParse<'s> for BusSchedule {
    fn nom_parse(s: &str) -> IResult<&str, Self> {
        comb::map(
            multi::separated_list1(
                character::char(','),
                branch::alt((u128::nom_parse, comb::value(0, character::char('x')))),
            ),
            |buses| Self {
                buses: buses.into_iter().map(BusNumber).collect(),
            },
        )(s)
    }
}

#[allow(unreachable_code)]
pub(super) fn run() -> io::Result<()> {
    let notes = fs::read_to_string("2020_13.txt")?;
    let mut lines = notes.lines();
    let time = lines
        .next()
        .expect("Missing time")
        .parse::<Timestamp>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let schedule = lines
        .next()
        .expect("Missing schedule")
        .parse::<BusSchedule>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    {
        println!("Year 2020 Day 13 Part 1");
        let (first_bus, delay) = schedule.next_bus(time);
        println!("The first available bus is {}", first_bus);
        println!("The result is {}", delay.0 * first_bus.0);
    }
    {
        println!("Year 2020 Day 13 Part 2");
        let first_diagonal = schedule.first_diagonal();
        println!(
            "The first time that starts a diagonal is {}",
            first_diagonal
        );
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[ignore]
    #[test]
    fn finds_correct_time() {
        let schedule = BusSchedule {
            buses: [7, 13, 0, 0, 59, 0, 31, 19]
                .iter()
                .copied()
                .map(BusNumber)
                .collect(),
        };
        let expected = Timestamp(1068781);
        let actual = schedule.first_diagonal();
        assert_eq!(expected, actual);
    }
}
