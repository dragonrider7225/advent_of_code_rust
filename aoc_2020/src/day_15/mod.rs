use aoc_util::{
    nom::{bytes::complete as bytes, character::complete as character, multi, IResult, Parser},
    nom_extended::NomParse,
    nom_supreme::ParserExt,
};
use std::{
    collections::HashMap,
    fs, io,
    ops::{Add, Sub},
};

#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
struct Turn(u64);

impl Add<u64> for Turn {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Sub for Turn {
    type Output = u64;

    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

#[derive(Clone, Debug, Default)]
struct History {
    turn_number: Turn,
    last: u64,
    values: HashMap<u64, (Turn, Turn)>,
}

impl History {
    fn insert(&mut self, turn: Turn, value: u64) {
        assert_eq!(self.turn_number + 1, turn);
        let last = self
            .values
            .get(&value)
            .map(|&(last, _)| last)
            .unwrap_or(turn);
        self.values.insert(value, (turn, last));
        self.turn_number = turn;
        self.last = value;
    }

    fn step(&mut self) {
        let next_turn = self.turn_number + 1;
        let next_value = self
            .values
            .get(&self.last)
            .map(|&(last, second_last)| last - second_last)
            .unwrap_or(0);
        self.insert(next_turn, next_value);
    }

    /// Runs the game through turn `turn` then returns the last number said.
    fn run_to(&mut self, turn: Turn) -> u64 {
        while self.turn_number < turn {
            self.step();
        }
        self.last
    }
}

impl FromIterator<u64> for History {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = u64>,
    {
        let mut res = Self::default();
        (1..)
            .map(Turn)
            .zip(iter)
            .for_each(|(turn, value)| res.insert(turn, value));
        res
    }
}

aoc_util::impl_from_str_for_nom_parse!(History);

impl<'s> NomParse<&'s str> for History {
    fn nom_parse(s: &'s str) -> IResult<&'s str, Self> {
        multi::separated_list1(bytes::tag(","), character::u64)
            .terminated(character::line_ending.opt())
            .map(Vec::into_iter)
            .map(Iterator::collect)
            .parse(s)
    }
}

#[allow(unreachable_code)]
pub(super) fn run() -> io::Result<()> {
    let initial_values = fs::read_to_string("2020_15.txt")?
        .parse::<History>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    {
        println!("Year 2020 Day 15 Part 1");
        let value = initial_values.clone().run_to(Turn(2020));
        println!("The 2020th number is {value}");
    }
    {
        println!("Year 2020 Day 15 Part 2");
        let mut initial_values = initial_values;
        let value = initial_values.run_to(Turn(30_000_000));
        println!("The 30,000,000th number is {value}");
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[ignore]
    #[test]
    fn example_1() {
        let expected = 436;
        let actual = [0, 3, 6]
            .iter()
            .copied()
            .collect::<History>()
            .run_to(Turn(2020));
        assert_eq!(expected, actual);
    }
}
