use aoc_util::nom_extended::NomParse;

use std::{
    cmp::Ordering,
    fs::File,
    io::{self, BufRead, BufReader},
    ops::{Add, AddAssign},
};

use nom::{
    bytes::complete as bytes, character::complete as character, combinator as comb, sequence,
    IResult,
};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Vec3 {
    x: i16,
    y: i16,
    z: i16,
}

impl Vec3 {
    fn new(x: i16, y: i16, z: i16) -> Self {
        Self { x, y, z }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Add<&'_ Self> for Vec3 {
    type Output = Self;

    fn add(self, other: &Self) -> Self::Output {
        self + *other
    }
}

impl Add<&'_ mut Self> for Vec3 {
    type Output = Self;

    fn add(self, other: &mut Self) -> Self::Output {
        self + *other
    }
}

impl Add<Vec3> for &'_ Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Self::Output {
        other + self
    }
}

impl Add<&'_ Vec3> for &'_ Vec3 {
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Self::Output {
        *self + other
    }
}

impl Add<&'_ mut Vec3> for &'_ Vec3 {
    type Output = Vec3;

    fn add(self, other: &mut Vec3) -> Self::Output {
        *self + other
    }
}

impl Add<Vec3> for &'_ mut Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Self::Output {
        other + self
    }
}

impl Add<&'_ Vec3> for &'_ mut Vec3 {
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Self::Output {
        *self + other
    }
}

impl Add<&'_ mut Vec3> for &'_ mut Vec3 {
    type Output = Vec3;

    fn add(self, other: &mut Vec3) -> Self::Output {
        *self + other
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl AddAssign<&'_ Self> for Vec3 {
    fn add_assign(&mut self, other: &Self) {
        *self += *other;
    }
}

impl AddAssign<&'_ mut Self> for Vec3 {
    fn add_assign(&mut self, other: &mut Self) {
        *self += *other;
    }
}

impl AddAssign<Vec3> for &'_ mut Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        **self += other;
    }
}

impl AddAssign<&'_ Vec3> for &'_ mut Vec3 {
    fn add_assign(&mut self, other: &Vec3) {
        **self += other;
    }
}

impl AddAssign<&'_ mut Vec3> for &'_ mut Vec3 {
    fn add_assign(&mut self, other: &mut Vec3) {
        **self += other;
    }
}

impl<'s> NomParse<&'s str> for Vec3 {
    fn nom_parse(s: &'s str) -> IResult<&'s str, Self> {
        comb::map(
            sequence::delimited(
                bytes::tag("<"),
                sequence::separated_pair(
                    sequence::preceded(bytes::tag("x="), character::i16),
                    bytes::tag(", "),
                    sequence::separated_pair(
                        sequence::preceded(bytes::tag("y="), character::i16),
                        bytes::tag(", "),
                        sequence::preceded(bytes::tag("z="), character::i16),
                    ),
                ),
                bytes::tag(">"),
            ),
            |(x, (y, z))| Vec3::new(x, y, z),
        )(s)
    }
}

aoc_util::impl_from_str_for_nom_parse!(Vec3);

pub(super) fn run() -> io::Result<()> {
    let initial_xv = BufReader::new(File::open("2019_12.txt")?)
        .lines()
        .map(|s| {
            s?.parse::<Vec3>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        })
        .map(|v| Ok((v?, Vec3::default())))
        .collect::<io::Result<Vec<_>>>()?;
    {
        println!("Year 2019 Day 12 Part 1");
        let mut xv1 = initial_xv.clone();
        for _ in 0..1000 {
            let xv2 = xv1.clone();
            for (i, (ref moon1_x, moon1_v)) in xv1.iter_mut().enumerate() {
                for (_, (moon2_x, _)) in xv2.iter().enumerate().filter(|&(j, _)| i != j) {
                    match moon1_x.x.cmp(&moon2_x.x) {
                        Ordering::Less => moon1_v.x += 1,
                        Ordering::Equal => {}
                        Ordering::Greater => moon1_v.x -= 1,
                    }
                    match moon1_x.y.cmp(&moon2_x.y) {
                        Ordering::Less => moon1_v.y += 1,
                        Ordering::Equal => {}
                        Ordering::Greater => moon1_v.y -= 1,
                    }
                    match moon1_x.z.cmp(&moon2_x.z) {
                        Ordering::Less => moon1_v.z += 1,
                        Ordering::Equal => {}
                        Ordering::Greater => moon1_v.z -= 1,
                    }
                }
            }
            for moon in xv1.iter_mut() {
                moon.0 += &moon.1;
            }
        }
        fn potential_energy(moon_x: Vec3) -> i16 {
            moon_x.x.abs() + moon_x.y.abs() + moon_x.z.abs()
        }
        fn kinetic_energy(moon_v: Vec3) -> i16 {
            moon_v.x.abs() + moon_v.y.abs() + moon_v.z.abs()
        }
        fn total_energy((moon_x, moon_v): (Vec3, Vec3)) -> i16 {
            potential_energy(moon_x) * kinetic_energy(moon_v)
        }
        println!(
            "The total energy is {}",
            xv1.into_iter().map(total_energy).sum::<i16>(),
        );
    }
    {
        println!("Year 2019 Day 12 Part 2");
        let mut steps = 0u128;
        let mut overflows = 0u128;
        let mut xv1 = initial_xv.clone();
        loop {
            if steps % 100_000 == 0 {
                println!("Reached {steps} distinct states since last overflow",);
            }
            let xv2 = xv1.clone();
            for (i, (ref moon1_x, moon1_v)) in xv1.iter_mut().enumerate() {
                for (_, (moon2_x, _)) in xv2.iter().enumerate().filter(|&(j, _)| i != j) {
                    match moon1_x.x.cmp(&moon2_x.x) {
                        Ordering::Less => moon1_v.x += 1,
                        Ordering::Equal => {}
                        Ordering::Greater => moon1_v.x -= 1,
                    }
                    match moon1_x.y.cmp(&moon2_x.y) {
                        Ordering::Less => moon1_v.y += 1,
                        Ordering::Equal => {}
                        Ordering::Greater => moon1_v.y -= 1,
                    }
                    match moon1_x.z.cmp(&moon2_x.z) {
                        Ordering::Less => moon1_v.z += 1,
                        Ordering::Equal => {}
                        Ordering::Greater => moon1_v.z -= 1,
                    }
                }
            }
            for moon in xv1.iter_mut() {
                moon.0 += &moon.1;
            }
            let (steps1, overflowed) = steps.overflowing_add(1);
            if overflowed {
                overflows += 1;
                steps = 1;
                println!("Overflowed {overflows} times before returning to the initial state.",);
                continue;
            }
            steps = steps1;
            if xv1 == initial_xv {
                break;
            }
        }
        println!(
            "The moons returned to their initial state after {overflows}*{}+{steps} steps",
            u128::MAX,
        );
    }
    Ok(())
}
