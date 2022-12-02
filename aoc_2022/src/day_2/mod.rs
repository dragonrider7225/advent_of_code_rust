use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    str::FromStr,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RpsChoice {
    Rock,
    Paper,
    Scissors,
}

impl RpsChoice {
    fn r#match(self, other: Self) -> RpsResult {
        match (self, other) {
            (Self::Rock, Self::Paper)
            | (Self::Paper, Self::Scissors)
            | (Self::Scissors, Self::Rock) => RpsResult::Lose,
            (Self::Paper, Self::Rock)
            | (Self::Scissors, Self::Paper)
            | (Self::Rock, Self::Scissors) => RpsResult::Win,
            _ => RpsResult::Tie,
        }
    }
}

impl FromStr for RpsChoice {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, s)),
        }
    }
}

impl From<RpsChoice> for u32 {
    fn from(val: RpsChoice) -> Self {
        match val {
            RpsChoice::Rock => 1,
            RpsChoice::Paper => 2,
            RpsChoice::Scissors => 3,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RpsResult {
    Win,
    Tie,
    Lose,
}

impl RpsResult {
    fn my_choice(self, opponent: RpsChoice) -> RpsChoice {
        match (self, opponent) {
            (Self::Tie, _) => opponent,
            (Self::Win, RpsChoice::Rock) | (Self::Lose, RpsChoice::Scissors) => RpsChoice::Paper,
            (Self::Win, RpsChoice::Paper) | (Self::Lose, RpsChoice::Rock) => RpsChoice::Scissors,
            (Self::Win, RpsChoice::Scissors) | (Self::Lose, RpsChoice::Paper) => RpsChoice::Rock,
        }
    }
}

impl FromStr for RpsResult {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Tie),
            "Z" => Ok(Self::Win),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, s)),
        }
    }
}

impl From<RpsResult> for u32 {
    fn from(val: RpsResult) -> Self {
        match val {
            RpsResult::Win => 6,
            RpsResult::Tie => 3,
            RpsResult::Lose => 0,
        }
    }
}

fn part1(input: &mut dyn BufRead) -> io::Result<u32> {
    let mut score = 0;
    for line in input.lines() {
        let line = line?;
        let (opponent, me) = line
            .split_once(' ')
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, line.clone()))?;
        let opponent: RpsChoice = opponent.parse()?;
        let me: RpsChoice = me.parse()?;
        let result = me.r#match(opponent);
        let choice_score: u32 = me.into();
        let match_score: u32 = result.into();
        score += choice_score + match_score;
    }
    Ok(score)
}

fn part2(input: &mut dyn BufRead) -> io::Result<u32> {
    let mut score = 0;
    for line in input.lines() {
        let line = line?;
        let (opponent, result) = line
            .split_once(' ')
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, line.clone()))?;
        let opponent: RpsChoice = opponent.parse()?;
        let result: RpsResult = result.parse()?;
        let me = result.my_choice(opponent);
        let choice_score: u32 = me.into();
        let match_score: u32 = result.into();
        score += choice_score + match_score;
    }
    Ok(score)
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2022 Day 2 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2022_02.txt")?))?
        );
    }
    {
        println!("Year 2022 Day 2 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2022_02.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = "A Y\nB X\nC Z\n";

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 15;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 12;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
