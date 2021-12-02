use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    num::ParseIntError,
    str::FromStr,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Motion {
    Forward(u32),
    Up(u32),
    Down(u32),
}

impl FromStr for Motion {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let to_string = |e: ParseIntError| e.to_string();
        let (direction, distance) = s
            .split_once(' ')
            .ok_or(format!("Missing space in {:?}", s))?;
        match direction {
            "forward" => Ok(Self::Forward(distance.parse().map_err(to_string)?)),
            "up" => Ok(Self::Up(distance.parse().map_err(to_string)?)),
            "down" => Ok(Self::Down(distance.parse().map_err(to_string)?)),
            direction => Err(format!("Unknown direction: {:?}", direction)),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct Position {
    x: u32,
    depth: u32,
}

impl FromIterator<Motion> for Position {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Motion>,
    {
        iter.into_iter().fold(Self::default(), |mut acc, x| {
            match x {
                Motion::Forward(distance) => acc.x += distance,
                Motion::Up(distance) => acc.depth -= distance,
                Motion::Down(distance) => acc.depth += distance,
            }
            acc
        })
    }
}

fn part1(input: &mut dyn BufRead) -> io::Result<Position> {
    input
        .lines()
        .map(|e| {
            Ok(e?
                .parse::<Motion>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?)
        })
        .collect()
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct Ray {
    pos: Position,
    aim: i32,
}

impl FromIterator<Motion> for Ray {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Motion>,
    {
        iter.into_iter().fold(Self::default(), |mut acc, motion| {
            match motion {
                Motion::Forward(distance) => {
                    acc.pos.x += distance;
                    if acc.aim < 0 {
                        acc.pos.depth -= distance * (-acc.aim) as u32
                    } else {
                        acc.pos.depth += distance * acc.aim as u32
                    }
                }
                Motion::Up(distance) => acc.aim -= distance as i32,
                Motion::Down(distance) => acc.aim += distance as i32,
            }
            acc
        })
    }
}

fn part2(input: &mut dyn BufRead) -> io::Result<Position> {
    Ok(input
        .lines()
        .map(|s| {
            s?.parse::<Motion>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        })
        .collect::<io::Result<Ray>>()?
        .pos)
}

#[allow(unreachable_code)]
pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2021 Day 2 Part 1");
        let mut input = BufReader::new(File::open("2021_02.txt")?);
        let final_position = part1(&mut input)?;
        println!(
            "Final position is {} units forward by {} units deep ({})",
            final_position.x,
            final_position.depth,
            final_position.x * final_position.depth
        );
    }
    {
        println!("Year 2021 Day 2 Part 2");
        let mut input = BufReader::new(File::open("2021_02.txt")?);
        let final_position = part2(&mut input)?;
        println!(
            "Final position is {} units forward by {} units deep ({})",
            final_position.x,
            final_position.depth,
            final_position.x * final_position.depth
        );
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use super::*;

    #[test]
    #[ignore]
    fn test_part1() -> io::Result<()> {
        let s = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2\n";
        let expected = Position { x: 15, depth: 10 };
        let actual = part1(&mut Cursor::new(s))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part2() -> io::Result<()> {
        let s = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2\n";
        let expected = Position { x: 15, depth: 60 };
        let actual = part2(&mut Cursor::new(s))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
