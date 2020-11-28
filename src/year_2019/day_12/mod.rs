use crate::parse::NomParse;

use std::{
    cmp::Ordering,
    io,
    ops::{Add, AddAssign},
    str::FromStr,
};

use nom::{bytes::complete as bytes, combinator as comb, sequence, IResult};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Vec3<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Vec3<T> {
    fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T, U, V> Add<Vec3<U>> for Vec3<T>
where
  T: Add<U, Output = V>,
{
    type Output = Vec3<V>;

    fn add(self, other: Vec3<U>) -> Self::Output {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl<'a, T, U, V> Add<&'a Vec3<U>> for Vec3<T>
where
  T: Add<U, Output = V>,
  U: Clone,
{
    type Output = Vec3<V>;

    fn add(self, other: &'a Vec3<U>) -> Self::Output {
        Vec3::new(
            self.x + other.x.clone(),
            self.y + other.y.clone(),
            self.z + other.z.clone(),
        )
    }
}

impl<'a, T, U, V> Add<&'a mut Vec3<U>> for Vec3<T>
where
  T: Add<&'a mut U, Output = V>,
{
    type Output = Vec3<V>;

    fn add(self, other: &'a mut Vec3<U>) -> Self::Output {
        Vec3::new(
            self.x + &mut other.x,
            self.y + &mut other.y,
            self.z + &mut other.z,
        )
    }
}

impl<'a, T, U, V> Add<Vec3<U>> for &'a Vec3<T>
where
  &'a T: Add<U, Output = V>,
{
    type Output = Vec3<V>;

    fn add(self, other: Vec3<U>) -> Self::Output {
        Vec3::new(&self.x + other.x, &self.y + other.y, &self.z + other.z)
    }
}

impl<'a, 'b, T, U, V> Add<&'b Vec3<U>> for &'a Vec3<T>
where
  &'a T: Add<&'b U, Output = V>,
{
    type Output = Vec3<V>;

    fn add(self, other: &'b Vec3<U>) -> Self::Output {
        Vec3::new(&self.x + &other.x, &self.y + &other.y, &self.z + &other.z)
    }
}

impl<'a, 'b, T, U, V> Add<&'b mut Vec3<U>> for &'a Vec3<T>
where
  &'a T: Add<&'b mut U, Output = V>,
{
    type Output = Vec3<V>;

    fn add(self, other: &'b mut Vec3<U>) -> Self::Output {
        Vec3::new(
            &self.x + &mut other.x,
            &self.y + &mut other.y,
            &self.z + &mut other.z,
        )
    }
}

impl<'a, T, U, V> Add<Vec3<U>> for &'a mut Vec3<T>
where
  &'a mut T: Add<U, Output = V>,
{
    type Output = Vec3<V>;

    fn add(self, other: Vec3<U>) -> Self::Output {
        Vec3::new(
            &mut self.x + other.x,
            &mut self.y + other.y,
            &mut self.z + other.z,
        )
    }
}

impl<'a, 'b, T, U, V> Add<&'b Vec3<U>> for &'a mut Vec3<T>
where
  &'a mut T: Add<&'b U, Output = V>,
{
    type Output = Vec3<V>;

    fn add(self, other: &'b Vec3<U>) -> Self::Output {
        Vec3::new(
            &mut self.x + &other.x,
            &mut self.y + &other.y,
            &mut self.z + &other.z,
        )
    }
}

impl<'a, 'b, T, U, V> Add<&'b mut Vec3<U>> for &'a mut Vec3<T>
where
  &'a mut T: Add<&'b mut U, Output = V>,
{
    type Output = Vec3<V>;

    fn add(self, other: &'b mut Vec3<U>) -> Self::Output {
        Vec3::new(
            &mut self.x + &mut other.x,
            &mut self.y + &mut other.y,
            &mut self.z + &mut other.z,
        )
    }
}

impl<T, U> AddAssign<Vec3<U>> for Vec3<T>
where
  T: AddAssign<U>,
{
    fn add_assign(&mut self, other: Vec3<U>) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<'a, T, U> AddAssign<&'a Vec3<U>> for Vec3<T>
where
  T: AddAssign<U>,
  U: Clone,
{
    fn add_assign(&mut self, other: &'a Vec3<U>) {
        self.x += other.x.clone();
        self.y += other.y.clone();
        self.z += other.z.clone();
    }
}

impl<'a, T, U> AddAssign<&'a mut Vec3<U>> for Vec3<T>
where
  T: AddAssign<U>,
  U: Clone,
{
    fn add_assign(&mut self, other: &'a mut Vec3<U>) {
        self.x += other.x.clone();
        self.y += other.y.clone();
        self.z += other.z.clone();
    }
}

impl<'a, T, U> AddAssign<Vec3<U>> for &'a mut Vec3<T>
where
  T: AddAssign<U>,
{
    fn add_assign(&mut self, other: Vec3<U>) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<'a, 'b, T, U> AddAssign<&'b Vec3<U>> for &'a mut Vec3<T>
where
  T: AddAssign<U>,
  U: Clone,
{
    fn add_assign(&mut self, other: &'b Vec3<U>) {
        self.x += other.x.clone();
        self.y += other.y.clone();
        self.z += other.z.clone();
    }
}

impl<'a, 'b, T, U> AddAssign<&'b mut Vec3<U>> for &'a mut Vec3<T>
where
  T: AddAssign<U>,
  U: Clone,
{
    fn add_assign(&mut self, other: &'b mut Vec3<U>) {
        self.x += other.x.clone();
        self.y += other.y.clone();
        self.z += other.z.clone();
    }
}

impl<T> NomParse for Vec3<T>
where
  T: NomParse,
{
    fn nom_parse(s: &str) -> IResult<&str, Self> {
        comb::map(
            sequence::delimited(
                bytes::tag("<"),
                sequence::separated_pair(
                    sequence::preceded(bytes::tag("x="), NomParse::nom_parse),
                    bytes::tag(", "),
                    sequence::separated_pair(
                        sequence::preceded(
                            bytes::tag("y="),
                            NomParse::nom_parse,
                        ),
                        bytes::tag(", "),
                        sequence::preceded(
                            bytes::tag("z="),
                            NomParse::nom_parse,
                        ),
                    ),
                ),
                bytes::tag(">"),
            ),
            |(x, (y, z))| Vec3::new(x, y, z),
        )(s)
    }
}

impl<T> FromStr for Vec3<T>
where
  T: NomParse,
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::nom_parse(s).map(|(_, x)| x).map_err(|e| format!("{:?}", e))
    }
}

pub(super) fn run() -> io::Result<()> {
    let initial_xv = crate::get_lines("2019_12.txt")?
        .map(|s| s.parse().unwrap())
        .map(|v: Vec3<i16>| (v, Vec3::default()))
        .collect::<Vec<_>>();
    {
        println!("Year 2019 Day 12 Part 1");
        let mut xv1 = initial_xv.clone();
        for _ in 0..1000 {
            let xv2 = xv1.clone();
            for i in 0..xv1.len() {
                for j in 0..xv2.len() {
                    if i == j {
                        continue;
                    }
                    let moon1 = &mut xv1[i];
                    let moon1_x = &moon1.0;
                    let moon1_v = &mut moon1.1;
                    let (moon2_x, _) = &xv2[j];
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
        fn potential_energy(moon_x: Vec3<i16>) -> i16 {
            moon_x.x.abs() + moon_x.y.abs() + moon_x.z.abs()
        }
        fn kinetic_energy(moon_v: Vec3<i16>) -> i16 {
            moon_v.x.abs() + moon_v.y.abs() + moon_v.z.abs()
        }
        fn total_energy((moon_x, moon_v): (Vec3<i16>, Vec3<i16>)) -> i16 {
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
                println!(
                    "Reached {} distinct states since last overflow",
                    steps,
                );
            }
            let xv2 = xv1.clone();
            for i in 0..xv1.len() {
                for j in 0..xv2.len() {
                    if i == j {
                        continue;
                    }
                    let moon1 = &mut xv1[i];
                    let moon1_x = &moon1.0;
                    let moon1_v = &mut moon1.1;
                    let (moon2_x, _) = &xv2[j];
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
                println!(
                    "Overflowed {} times before returning to the initial state.",
                    overflows,
                );
                continue;
            }
            steps = steps1;
            if xv1 == initial_xv {
                break;
            }
        }
        println!(
            "The moons returned to their initial state after {}*{}+{} steps",
            overflows, std::u128::MAX, steps,
        );
    }
    Ok(())
}
