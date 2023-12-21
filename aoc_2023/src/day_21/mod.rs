use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader},
    ops::{Index, IndexMut},
};

use aoc_util::{geometry::Point2D, nom_extended::NomParse};
use nom::{branch, bytes::complete as bytes, combinator, multi};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tile {
    Start,
    GardenPlot,
    Rock,
}

impl<'s> NomParse<&'s str> for Tile {
    fn nom_parse(input: &'s str) -> nom::IResult<&'s str, Self> {
        branch::alt((
            combinator::value(Self::Start, bytes::tag("S")),
            combinator::value(Self::GardenPlot, bytes::tag(".")),
            combinator::value(Self::Rock, bytes::tag("#")),
        ))(input)
    }
}

type Position = Point2D<usize>;
type IPosition = Point2D<isize>;

fn position_to_iposition(this: Position) -> IPosition {
    IPosition::at(*this.x() as isize, *this.y() as isize)
}

#[derive(Clone, Eq, PartialEq)]
struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    fn start(&self) -> Position {
        self.tiles
            .iter()
            .enumerate()
            .find_map(|(row_idx, row)| {
                row.iter()
                    .position(|&t| t == Tile::Start)
                    .map(|col_idx| Position::at(row_idx, col_idx))
            })
            .expect("Missing start position")
    }

    fn neighbors(&self, position: Position) -> Vec<Position> {
        let mut ret = vec![];
        if position.x() != &0 {
            let next_pos = position - Position::at(1, 0);
            if self[next_pos] != Tile::Rock {
                ret.push(next_pos);
            }
        }
        if position.y() != &0 {
            let next_pos = position - Position::at(0, 1);
            if self[next_pos] != Tile::Rock {
                ret.push(next_pos);
            }
        }
        if position.x() + 1 < self.tiles[0].len() {
            let next_pos = position + Position::at(1, 0);
            if self[next_pos] != Tile::Rock {
                ret.push(next_pos);
            }
        }
        if position.y() + 1 < self.tiles.len() {
            let next_pos = position + Position::at(0, 1);
            if self[next_pos] != Tile::Rock {
                ret.push(next_pos);
            }
        }
        ret
    }

    fn ineighbors(&self, position: IPosition) -> Vec<IPosition> {
        [
            IPosition::at(position.x() - 1, *position.y()),
            IPosition::at(position.x() + 1, *position.y()),
            IPosition::at(*position.x(), position.y() - 1),
            IPosition::at(*position.x(), position.y() + 1),
        ]
        .into_iter()
        .filter(|&ipos| self[ipos] != Tile::Rock)
        .collect()
    }
}

impl FromIterator<Vec<Tile>> for Map {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Vec<Tile>>,
    {
        Self {
            tiles: iter.into_iter().collect(),
        }
    }
}

impl Index<Position> for Map {
    type Output = Tile;

    fn index(&self, index: Position) -> &Self::Output {
        &self.tiles[*index.y()][*index.x()]
    }
}

impl IndexMut<Position> for Map {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        &mut self.tiles[*index.y()][*index.x()]
    }
}

impl Index<IPosition> for Map {
    type Output = Tile;

    fn index(&self, index: IPosition) -> &Self::Output {
        let mut x = index.x() % self.tiles[0].len() as isize;
        let mut y = index.y() % self.tiles.len() as isize;
        if x < 0 {
            x += self.tiles[0].len() as isize;
        }
        if y < 0 {
            y += self.tiles.len() as isize;
        }
        &self.tiles[y as usize][x as usize]
    }
}

impl IndexMut<IPosition> for Map {
    fn index_mut(&mut self, index: IPosition) -> &mut Self::Output {
        let mut x = index.x() % self.tiles[0].len() as isize;
        let mut y = index.y() % self.tiles.len() as isize;
        if x < 0 {
            x += self.tiles[0].len() as isize;
        }
        if y < 0 {
            y += self.tiles.len() as isize;
        }
        &mut self.tiles[y as usize][x as usize]
    }
}

fn part1(input: &mut dyn BufRead, num_steps: usize) -> io::Result<usize> {
    let map = input
        .lines()
        .map(|line| {
            let line = line?;
            #[allow(clippy::let_and_return)]
            let ret = multi::many1(Tile::nom_parse)(&line)
                .map(|(_, x)| x)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()));
            ret
        })
        .collect::<io::Result<Map>>()?;
    let mut positions = HashSet::new();
    positions.insert(map.start());
    for _ in 0..num_steps {
        positions = positions
            .into_iter()
            .flat_map(|position| map.neighbors(position))
            .collect();
    }
    Ok(positions.len())
}

fn part2(input: &mut dyn BufRead, num_steps: usize) -> io::Result<usize> {
    let map = input
        .lines()
        .map(|line| {
            let line = line?;
            #[allow(clippy::let_and_return)]
            let ret = multi::many1(Tile::nom_parse)(&line)
                .map(|(_, x)| x)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()));
            ret
        })
        .collect::<io::Result<Map>>()?;
    let mut positions = HashSet::new();
    positions.insert(position_to_iposition(map.start()));
    for _ in 0..num_steps {
        positions = positions
            .into_iter()
            .flat_map(|position| map.ineighbors(position))
            .collect();
    }
    Ok(positions.len())
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2023 Day 21 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2023_21.txt")?), 64)?
        );
    }
    {
        println!("Year 2023 Day 21 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2023_21.txt")?), 26_501_365)?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        "...........\n",
        ".....###.#.\n",
        ".###.##..#.\n",
        "..#.#...#..\n",
        "....#.#....\n",
        ".##..S####.\n",
        ".##..#...#.\n",
        ".......##..\n",
        ".##.#.####.\n",
        ".##..##.##.\n",
        "...........\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 16;
        let actual = part1(&mut Cursor::new(TEST_DATA), 6)?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 16;
        let actual = part2(&mut Cursor::new(TEST_DATA), 6)?;
        assert_eq!(expected, actual);
        let expected = 50;
        let actual = part2(&mut Cursor::new(TEST_DATA), 10)?;
        assert_eq!(expected, actual);
        let expected = 1594;
        let actual = part2(&mut Cursor::new(TEST_DATA), 50)?;
        assert_eq!(expected, actual);
        let expected = 6536;
        let actual = part2(&mut Cursor::new(TEST_DATA), 100)?;
        assert_eq!(expected, actual);
        let expected = 167_004;
        let actual = part2(&mut Cursor::new(TEST_DATA), 500)?;
        assert_eq!(expected, actual);
        let expected = 668_697;
        let actual = part2(&mut Cursor::new(TEST_DATA), 1000)?;
        assert_eq!(expected, actual);
        let expected = 16_733_044;
        let actual = part2(&mut Cursor::new(TEST_DATA), 5000)?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
