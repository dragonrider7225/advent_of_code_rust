use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead, BufReader},
    iter,
};

use aoc_util::{geometry::Direction, nom_extended::NomParse};
use nom::{branch, bytes::complete as bytes, combinator, multi, IResult};

#[derive(Clone, Copy, Debug)]
enum Tile {
    Empty,
    PositiveMirror,
    NegativeMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

impl Tile {
    fn output(
        &self,
        cell: (usize, usize),
        direction: Direction,
    ) -> impl Iterator<Item = ((usize, usize), Direction)> {
        type Out = Box<dyn Iterator<Item = ((usize, usize), Direction)>>;
        match self {
            Self::Empty => match direction {
                Direction::Up if cell.0 == 0 => Box::new(iter::empty()) as Out,
                Direction::Up => Box::new(iter::once(((cell.0 - 1, cell.1), direction))),
                Direction::Down => Box::new(iter::once(((cell.0 + 1, cell.1), direction))),
                Direction::Left if cell.1 == 0 => Box::new(iter::empty()),
                Direction::Left => Box::new(iter::once(((cell.0, cell.1 - 1), direction))),
                Direction::Right => Box::new(iter::once(((cell.0, cell.1 + 1), direction))),
            },
            Self::PositiveMirror => match direction {
                Direction::Down if cell.1 == 0 => Box::new(iter::empty()) as Out,
                Direction::Down => Box::new(iter::once(((cell.0, cell.1 - 1), Direction::Left))),
                Direction::Right if cell.0 == 0 => Box::new(iter::empty()),
                Direction::Right => Box::new(iter::once(((cell.0 - 1, cell.1), Direction::Up))),
                Direction::Left => Box::new(iter::once(((cell.0 + 1, cell.1), Direction::Down))),
                Direction::Up => Box::new(iter::once(((cell.0, cell.1 + 1), Direction::Right))),
            },
            Self::NegativeMirror => match direction {
                Direction::Left if cell.0 == 0 => Box::new(iter::empty()) as Out,
                Direction::Left => Box::new(iter::once(((cell.0 - 1, cell.1), Direction::Up))),
                Direction::Up if cell.1 == 0 => Box::new(iter::empty()),
                Direction::Up => Box::new(iter::once(((cell.0, cell.1 - 1), Direction::Left))),
                Direction::Down => Box::new(iter::once(((cell.0, cell.1 + 1), Direction::Right))),
                Direction::Right => Box::new(iter::once(((cell.0 + 1, cell.1), Direction::Down))),
            },
            Self::VerticalSplitter => match direction {
                Direction::Up if cell.0 == 0 => Box::new(iter::empty()) as Out,
                Direction::Up => Box::new(iter::once(((cell.0 - 1, cell.1), direction))),
                Direction::Down => Box::new(iter::once(((cell.0 + 1, cell.1), direction))),
                _ if cell.0 == 0 => Box::new(iter::once(((cell.0 + 1, cell.1), Direction::Down))),
                _ => Box::new(
                    [
                        ((cell.0 - 1, cell.1), Direction::Up),
                        ((cell.0 + 1, cell.1), Direction::Down),
                    ]
                    .into_iter(),
                ),
            },
            Self::HorizontalSplitter => match direction {
                Direction::Left if cell.1 == 0 => Box::new(iter::empty()) as Out,
                Direction::Left => Box::new(iter::once(((cell.0, cell.1 - 1), direction))),
                Direction::Right => Box::new(iter::once(((cell.0, cell.1 + 1), direction))),
                _ if cell.1 == 0 => Box::new(iter::once(((cell.0, cell.1 + 1), Direction::Right))),
                _ => Box::new(
                    [
                        ((cell.0, cell.1 - 1), Direction::Left),
                        ((cell.0, cell.1 + 1), Direction::Right),
                    ]
                    .into_iter(),
                ),
            },
        }
    }
}

impl<'s> NomParse<&'s str> for Tile {
    fn nom_parse(input: &'s str) -> IResult<&'s str, Self> {
        branch::alt((
            combinator::value(Self::Empty, bytes::tag(".")),
            combinator::value(Self::PositiveMirror, bytes::tag("/")),
            combinator::value(Self::NegativeMirror, bytes::tag("\\")),
            combinator::value(Self::VerticalSplitter, bytes::tag("|")),
            combinator::value(Self::HorizontalSplitter, bytes::tag("-")),
        ))(input)
    }
}

#[derive(Clone, Debug)]
struct Contraption(Vec<Vec<Tile>>);

impl Contraption {
    fn read(input: &mut dyn BufRead) -> io::Result<Self> {
        input
            .lines()
            .map(|line| {
                let line = line?;
                #[allow(clippy::let_and_return)]
                let ret = multi::many1(Tile::nom_parse)(&line)
                    .map(|(_, row)| row)
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()));
                ret
            })
            .collect()
    }

    fn count_energized_from(&self, cell: (usize, usize), direction: Direction) -> usize {
        let mut cells_to_check = vec![(cell, direction)];
        let mut cells_energized = HashMap::new();
        while let Some((cell, direction)) = cells_to_check.pop() {
            let directions = cells_energized.entry(cell).or_insert(HashSet::new());
            if directions.insert(direction) {
                cells_to_check.extend(
                    self.0[cell.0][cell.1]
                        .output(cell, direction)
                        .filter(|(cell, _)| cell.0 < self.0.len() && cell.1 < self.0[cell.0].len()),
                );
            }
        }
        cells_energized.keys().count()
    }
}

impl FromIterator<Vec<Tile>> for Contraption {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Vec<Tile>>,
    {
        Self(iter.into_iter().collect())
    }
}

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    let contraption = Contraption::read(input)?;
    Ok(contraption.count_energized_from((0, 0), Direction::Right))
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    let contraption = Contraption::read(input)?;
    let max_energized = (0..contraption.0.len())
        .flat_map(|row_idx| {
            [
                ((row_idx, 0), Direction::Right),
                ((row_idx, contraption.0[row_idx].len() - 1), Direction::Left),
            ]
        })
        .chain((0..contraption.0[0].len()).flat_map(|col_idx| {
            [
                ((0, col_idx), Direction::Down),
                ((contraption.0.len() - 1, col_idx), Direction::Up),
            ]
        }))
        .map(|(cell, direction)| contraption.count_energized_from(cell, direction))
        .max()
        .unwrap();
    Ok(max_energized)
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2023 Day 16 Part 1");
        println!(
            "{:?}",
            part1(&mut BufReader::new(File::open("2023_16.txt")?))?
        );
    }
    {
        println!("Year 2023 Day 16 Part 2");
        println!(
            "{:?}",
            part2(&mut BufReader::new(File::open("2023_16.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        ".|...\\....\n",
        "|.-.\\.....\n",
        ".....|-...\n",
        "........|.\n",
        "..........\n",
        ".........\\\n",
        "..../.\\\\..\n",
        ".-.-/..|..\n",
        ".|....-|.\\\n",
        "..//.|....\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 46;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 51;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
