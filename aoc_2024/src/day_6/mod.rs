use aoc_util::geometry::direction::Direction2D;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, read_to_string, BufRead, BufReader},
};

use aoc_util::{
    geometry::Point2D,
    nom::{
        branch, bytes::complete as bytes, character::complete as character, multi, IResult, Parser,
    },
    nom_extended::NomParse,
    nom_supreme::ParserExt,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tile {
    Floor,
    Guard,
    Obstruction,
}

impl NomParse<&str> for Tile {
    fn nom_parse(input: &str) -> IResult<&str, Self> {
        branch::alt((
            bytes::tag(".").map(|_| Self::Floor),
            bytes::tag("^").map(|_| Self::Guard),
            bytes::tag("#").map(|_| Self::Obstruction),
        ))(input)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Map {
    tiles: Vec<Vec<Tile>>,
    guard_position: Option<Point2D<usize>>,
    guard_facing: Direction2D,
    visited: HashMap<Point2D<usize>, HashSet<Direction2D>>,
}

impl Map {
    fn new(tiles: Vec<Vec<Tile>>) -> Self {
        let guard_position = tiles
            .iter()
            .enumerate()
            .filter_map(|(row_idx, row)| {
                row.iter()
                    .position(|tile| matches!(tile, Tile::Guard))
                    .map(|col_idx| (row_idx, col_idx))
            })
            .next()
            .map(|(y, x)| Point2D::at(x, y));
        let visited = guard_position
            .iter()
            .copied()
            .map(|p| (p, [Direction2D::Up].into_iter().collect()))
            .collect();
        Self {
            tiles,
            guard_position,
            guard_facing: Direction2D::Up,
            visited,
        }
    }

    /// Moves the guard one tick forward. If the tile immediately in front of the guard is
    /// [`Floor`], she will move into that tile. Otherwise she will turn 90 degrees to her right.
    ///
    /// [`Floor`]: [Tile::Floor]
    fn step(&mut self) -> StepResult {
        let guard_position = self.guard_position.take();
        let next_position = match self.guard_facing {
            Direction2D::Up => guard_position
                .filter(|p| *p.y() != 0)
                .map(|p| p - Point2D::at(0, 1)),
            Direction2D::Right => guard_position
                .filter(|p| *p.x() + 1 < self.tiles[0].len())
                .map(|p| p + Point2D::at(1, 0)),
            Direction2D::Down => guard_position
                .filter(|p| *p.y() + 1 < self.tiles.len())
                .map(|p| p + Point2D::at(0, 1)),
            Direction2D::Left => guard_position
                .filter(|p| *p.x() != 0)
                .map(|p| p - Point2D::at(1, 0)),
        };
        if let Some(p) = next_position {
            if matches!(self.tiles[*p.y()][*p.x()], Tile::Obstruction) {
                self.guard_position = guard_position;
                self.guard_facing = self.guard_facing.rotate_clockwise();
            } else {
                let visited = self.visited.entry(p).or_default();
                if !visited.insert(self.guard_facing) {
                    return StepResult::Loop;
                }
                self.guard_position = Some(p);
            }
            StepResult::Continue
        } else {
            StepResult::OffMap
        }
    }
}

impl NomParse<&str> for Map {
    fn nom_parse(input: &str) -> IResult<&str, Self> {
        multi::many1(multi::many1(Tile::nom_parse).terminated(character::line_ending))
            .map(Self::new)
            .parse(input)
    }
}

/// The return value of [`Map::step()`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum StepResult {
    /// The guard has reached a tile that she visited previously.
    Loop,
    /// The guard has moved off the edge of the map.
    OffMap,
    /// The guard has not yet exited the map or entered a loop.
    Continue,
}

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    let input = read_to_string(input)?;
    let (_, mut map) = Map::nom_parse(&input)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
    while matches!(map.step(), StepResult::Continue) {}
    Ok(map.visited.len())
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    let input = read_to_string(input)?;
    let (_, map) = Map::nom_parse(&input)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
    let num_rows = map.tiles.len();
    let num_cols = map.tiles[0].len();
    let loop_count = (0..num_rows)
        .flat_map(|row_idx| {
            if row_idx % 10 == 0 {
                eprintln!(
                    "Trying obstruction in rows {row_idx}-{} of {num_rows}",
                    row_idx + 9
                );
            }
            let map = map.clone();
            (0..num_cols).filter(move |&col_idx| {
                let mut map = map.clone();
                if matches!(map.tiles[row_idx][col_idx], Tile::Floor) {
                    map.tiles[row_idx][col_idx] = Tile::Obstruction;
                } else {
                    return false;
                }
                loop {
                    match map.step() {
                        StepResult::Loop => return true,
                        StepResult::OffMap => return false,
                        StepResult::Continue => {}
                    }
                }
            })
        })
        .count();
    Ok(loop_count)
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2024 Day 6 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2024_06.txt")?))?
        );
    }
    {
        println!("Year 2024 Day 6 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2024_06.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        "....#.....\n",
        ".........#\n",
        "..........\n",
        "..#.......\n",
        ".......#..\n",
        "..........\n",
        ".#..^.....\n",
        "........#.\n",
        "#.........\n",
        "......#...\n",
    );

    #[test]
    fn test_map_parse() {
        let expected = Ok((
            "",
            Map {
                tiles: vec![
                    vec![
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Obstruction,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                    ],
                    vec![
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Obstruction,
                    ],
                    vec![
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                    ],
                    vec![
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Obstruction,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                    ],
                    vec![
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Obstruction,
                        Tile::Floor,
                        Tile::Floor,
                    ],
                    vec![
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                    ],
                    vec![
                        Tile::Floor,
                        Tile::Obstruction,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Guard,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                    ],
                    vec![
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Obstruction,
                        Tile::Floor,
                    ],
                    vec![
                        Tile::Obstruction,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                    ],
                    vec![
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Obstruction,
                        Tile::Floor,
                        Tile::Floor,
                        Tile::Floor,
                    ],
                ],
                guard_position: Some(Point2D::at(4, 6)),
                guard_facing: Direction2D::Up,
                visited: [(Point2D::at(4, 6), [Direction2D::Up].into_iter().collect())]
                    .into_iter()
                    .collect(),
            },
        ));
        let actual = Map::nom_parse(TEST_DATA);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 41;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 6;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
