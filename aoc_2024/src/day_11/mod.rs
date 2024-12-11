use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
    iter,
    ops::{Deref, Div, Mul, Rem},
};

use aoc_util::{
    impl_from_str_for_nom_parse,
    nom::{character::complete as character, IResult, Parser},
    nom_extended::NomParse,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Stone(u128);

fn parse_stones(s: &str) -> Result<HashMap<Stone, usize>, String> {
    s.split_whitespace()
        .map(str::parse)
        .map(|r| r.map(|s| (s, 1)))
        .collect()
}

impl NomParse<&str> for Stone {
    fn nom_parse(input: &str) -> IResult<&str, Self> {
        character::u128.map(Self).parse(input)
    }
}

impl_from_str_for_nom_parse!(Stone);

impl Deref for Stone {
    type Target = u128;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Div<u128> for Stone {
    type Output = Self;

    fn div(self, rhs: u128) -> Self::Output {
        Self(*self / rhs)
    }
}

impl Mul<u128> for Stone {
    type Output = Self;

    fn mul(self, rhs: u128) -> Self::Output {
        Self(*self * rhs)
    }
}

impl Mul<Stone> for u128 {
    type Output = Stone;

    fn mul(self, rhs: Stone) -> Self::Output {
        rhs * self
    }
}

impl Rem<u128> for Stone {
    type Output = Self;

    fn rem(self, rhs: u128) -> Self::Output {
        Self(*self % rhs)
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct StoneBlinkMap(HashMap<Stone, (Stone, Option<Stone>)>);

impl StoneBlinkMap {
    fn cache_blink(&mut self, stones: impl Iterator<Item = Stone>) {
        for stone in stones {
            if self.0.contains_key(&stone) {
                continue;
            }
            let children = match stone.checked_ilog10() {
                None => (Stone(1), None),
                Some(n) if (n + 1) % 2 == 0 => {
                    let denominator = 10u128.pow((n + 1) / 2);
                    (stone / denominator, Some(stone % denominator))
                }
                _ => (stone * 2024, None),
            };
            self.0.insert(stone, children);
        }
    }

    fn children(&self, stone: Stone) -> impl Iterator<Item = Stone> {
        self.0
            .get(&stone)
            .into_iter()
            .flat_map(|(a, b)| iter::once(a).chain(b.as_ref()).copied())
    }

    fn cache_children(&mut self, stone: Stone) -> impl Iterator<Item = Stone> {
        self.cache_blink(iter::once(stone));
        self.children(stone)
    }
}

fn part1(input: &mut dyn BufRead, num_blinks: usize) -> io::Result<usize> {
    let stones = parse_stones(io::read_to_string(input)?.trim())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let cache = StoneBlinkMap(HashMap::new());
    Ok((0..num_blinks)
        .fold((cache, stones), |(mut cache, stones), blink| {
            if blink % 25 == 0 {
                eprintln!(
                    "After {blink} blinks there are {} stones",
                    stones.values().sum::<usize>()
                );
            }
            let children = stones
                .into_iter()
                .flat_map(|(stone, count)| {
                    cache
                        .cache_children(stone)
                        .map(|s| (s, count))
                        .collect::<Vec<_>>()
                })
                .fold(HashMap::new(), |mut acc, (stone, count)| {
                    *acc.entry(stone).or_default() += count;
                    acc
                });
            (cache, children)
        })
        .1
        .values()
        .sum())
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    part1(input, 75)
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2024 Day 11 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2024_11.txt")?), 25)?
        );
    }
    {
        println!("Year 2024 Day 11 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2024_11.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use io::Cursor;

    use super::*;

    const TEST_DATA_1: &str = "0 1 10 99 999\n";
    const TEST_DATA_2: &str = "125 17\n";

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 7;
        let actual = part1(&mut Cursor::new(TEST_DATA_1), 1)?;
        assert_eq!(expected, actual);
        let expected = 2;
        let actual = part1(&mut Cursor::new(TEST_DATA_2), 0)?;
        assert_eq!(expected, actual);
        let expected = 3;
        let actual = part1(&mut Cursor::new(TEST_DATA_2), 1)?;
        assert_eq!(expected, actual);
        let expected = 4;
        let actual = part1(&mut Cursor::new(TEST_DATA_2), 2)?;
        assert_eq!(expected, actual);
        let expected = 5;
        let actual = part1(&mut Cursor::new(TEST_DATA_2), 3)?;
        assert_eq!(expected, actual);
        let expected = 9;
        let actual = part1(&mut Cursor::new(TEST_DATA_2), 4)?;
        assert_eq!(expected, actual);
        let expected = 13;
        let actual = part1(&mut Cursor::new(TEST_DATA_2), 5)?;
        assert_eq!(expected, actual);
        let expected = 22;
        let actual = part1(&mut Cursor::new(TEST_DATA_2), 6)?;
        assert_eq!(expected, actual);
        let expected = 55312;
        let actual = part1(&mut Cursor::new(TEST_DATA_2), 25)?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 65601038650482;
        let actual = part2(&mut Cursor::new(TEST_DATA_2))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
