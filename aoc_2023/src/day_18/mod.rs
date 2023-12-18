use std::{
    fmt::{self, Display, Formatter},
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
    iter, mem,
    path::Path,
};

use aoc_util::geometry::Direction;
use nom::{
    branch, bytes::complete as bytes, character::complete as character, combinator, multi,
    sequence, IResult,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tile {
    Ground,
    Hole,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ground => write!(f, "."),
            Self::Hole => write!(f, "#"),
        }
    }
}

#[derive(Clone, Debug)]
struct Pit {
    tiles: Vec<Vec<Tile>>,
    position: (usize, usize),
}

impl Pit {
    fn new() -> Self {
        Self {
            tiles: vec![vec![Tile::Hole]],
            position: (0, 0),
        }
    }

    fn write_ppm(&self, out: &mut dyn Write) -> io::Result<()> {
        writeln!(out, "P6\n{} {}\n1", self.tiles[0].len(), self.tiles.len())?;
        for (row_idx, row) in self.tiles.iter().enumerate() {
            for (col_idx, tile) in row.iter().enumerate() {
                if (row_idx, col_idx) == self.position {
                    out.write_all(&[1, 0, 0])?;
                } else {
                    match tile {
                        Tile::Ground => out.write_all(&[1, 1, 1])?,
                        Tile::Hole => out.write_all(&[0, 0, 0])?,
                    }
                }
            }
        }
        Ok(())
    }

    fn dig(&mut self, direction: Direction, distance: usize) {
        match direction {
            Direction::Down => {
                let max_row = self.position.0 + distance;
                if self.tiles.len() <= max_row {
                    self.tiles[self.position.0..]
                        .iter_mut()
                        .for_each(|row| row[self.position.1] = Tile::Hole);
                    let mut new_row = vec![Tile::Ground; self.tiles[0].len()];
                    new_row[self.position.1] = Tile::Hole;
                    let new_rows = iter::repeat(new_row).take(max_row - (self.tiles.len() - 1));
                    self.tiles.extend(new_rows);
                } else {
                    self.tiles[self.position.0..=max_row]
                        .iter_mut()
                        .for_each(|row| row[self.position.1] = Tile::Hole);
                }
                self.position.0 = max_row;
            }
            Direction::Up => {
                if self.position.0 < distance {
                    self.tiles[..self.position.0]
                        .iter_mut()
                        .for_each(|row| row[self.position.1] = Tile::Hole);
                    let num_new_rows = distance - self.position.0;
                    let mut new_row = vec![Tile::Ground; self.tiles[0].len()];
                    new_row[self.position.1] = Tile::Hole;
                    let old_rows = mem::replace(
                        &mut self.tiles,
                        iter::repeat(new_row).take(num_new_rows).collect(),
                    );
                    self.tiles.extend(old_rows);
                    self.position.0 = 0;
                } else {
                    let min_row = self.position.0 - distance;
                    self.tiles[min_row..self.position.0]
                        .iter_mut()
                        .for_each(|row| row[self.position.1] = Tile::Hole);
                    self.position.0 = min_row;
                }
            }
            Direction::Left => {
                if self.position.1 < distance {
                    self.tiles[self.position.0][..self.position.1].fill(Tile::Hole);
                    let num_new_cols = distance - self.position.1;
                    let new_cols = vec![Tile::Ground; num_new_cols];
                    self.tiles
                        .iter_mut()
                        .enumerate()
                        .for_each(|(row_idx, row)| {
                            let old_row = if row_idx == self.position.0 {
                                mem::replace(row, vec![Tile::Hole; num_new_cols])
                            } else {
                                mem::replace(row, new_cols.clone())
                            };
                            row.extend(old_row);
                        });
                    self.position.1 = 0;
                } else {
                    let min_col = self.position.1 - distance;
                    self.tiles[self.position.0][min_col..self.position.1].fill(Tile::Hole);
                    self.position.1 = min_col;
                }
            }
            Direction::Right => {
                let max_col = self.position.1 + distance;
                if self.tiles[self.position.0].len() <= max_col {
                    self.tiles[self.position.0][self.position.1..].fill(Tile::Hole);
                    let num_new_cols = max_col - (self.tiles[self.position.0].len() - 1);
                    let new_cols = vec![Tile::Ground; num_new_cols];
                    self.tiles
                        .iter_mut()
                        .enumerate()
                        .for_each(|(row_idx, row)| {
                            if row_idx == self.position.0 {
                                row.extend(vec![Tile::Hole; num_new_cols]);
                            } else {
                                row.extend(new_cols.clone());
                            }
                        });
                    self.position.1 = self.tiles[self.position.0].len() - 1;
                } else {
                    self.tiles[self.position.0][self.position.1..=max_col].fill(Tile::Hole);
                    self.position.1 = max_col;
                }
            }
        }
    }
}

impl Default for Pit {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Pit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (row_idx, row) in self.tiles.iter().enumerate() {
            for (col_idx, tile) in row.iter().enumerate() {
                if (row_idx, col_idx) == self.position {
                    write!(f, "X")?;
                } else {
                    write!(f, "{}", tile)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse_direction(s: &str) -> IResult<&str, Direction> {
    branch::alt((
        combinator::value(Direction::Up, bytes::tag("U")),
        combinator::value(Direction::Down, bytes::tag("D")),
        combinator::value(Direction::Left, bytes::tag("L")),
        combinator::value(Direction::Right, bytes::tag("R")),
    ))(s)
}

fn open_file_for_writing(p: impl AsRef<Path>) -> io::Result<File> {
    OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(p)
}

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    let mut pit = input
        .lines()
        .map(|line| {
            let line = line?;
            #[allow(clippy::let_and_return)]
            let ret = sequence::terminated(
                sequence::separated_pair(
                    parse_direction,
                    bytes::tag(" "),
                    combinator::map(character::u32, |n| n as usize),
                ),
                sequence::delimited(
                    bytes::tag(" (#"),
                    multi::many_m_n(6, 6, character::one_of("0123456789abcdef")),
                    bytes::tag(")"),
                ),
            )(&line[..])
            .map(|(_, x)| x)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()));
            ret
        })
        .try_fold::<_, _, io::Result<_>>(Pit::default(), |mut acc, parts| {
            let (direction, distance) = parts?;
            acc.dig(direction, distance);
            Ok(acc)
        })?;
    pit.write_ppm(&mut open_file_for_writing("2023_18-border.ppm")?)?;
    for row_idx in 1..(pit.tiles.len() - 1) {
        let mut inside = false;
        let mut first_hole_idx = pit.tiles[row_idx]
            .iter()
            .position(|&tile| tile == Tile::Hole)
            .expect("Pit contains row without any holes");
        while let Some((ground_idx, _)) = pit.tiles[row_idx]
            .iter()
            .enumerate()
            .skip(first_hole_idx)
            .find(|&(_, &tile)| tile == Tile::Ground)
        {
            let last_hole_idx = ground_idx - 1;
            if first_hole_idx == last_hole_idx
                || pit.tiles[row_idx + 1][first_hole_idx] != pit.tiles[row_idx + 1][last_hole_idx]
            {
                // This was not the top or bottom part of a U-turn:
                // ###
                // #.#
                // or
                // ###
                // ...
                // so we've crossed the boundry.
                inside = !inside;
            }
            let next_hole_idx = match pit.tiles[row_idx]
                .iter()
                .skip(ground_idx)
                .position(|&tile| tile == Tile::Hole)
            {
                Some(next_hole_position) => next_hole_position + ground_idx,
                None => {
                    if inside {
                        panic!("The right side of the structure includes at least one ground tile inside of the lagoon in row {row_idx}")
                    }
                    break;
                }
            };
            if inside {
                pit.tiles[row_idx][ground_idx..next_hole_idx].fill(Tile::Hole);
            }
            first_hole_idx = next_hole_idx;
        }
    }
    pit.write_ppm(&mut open_file_for_writing("2023_18-filled.ppm")?)?;
    Ok(pit
        .tiles
        .into_iter()
        .flatten()
        .filter(|&tile| tile == Tile::Hole)
        .count())
}

fn part2(_input: &mut dyn BufRead) -> io::Result<()> {
    todo!("Year 2023 Day 18 Part 2")
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2023 Day 18 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2023_18.txt")?))?
        );
    }
    {
        println!("Year 2023 Day 18 Part 2");
        println!(
            "{:?}",
            part2(&mut BufReader::new(File::open("2023_18.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        "R 6 (#70c710)\n",
        "D 5 (#0dc571)\n",
        "L 2 (#5713f0)\n",
        "D 2 (#d2c081)\n",
        "R 2 (#59c680)\n",
        "D 2 (#411b91)\n",
        "L 5 (#8ceee2)\n",
        "U 2 (#caa173)\n",
        "L 1 (#1b58a2)\n",
        "U 2 (#caa171)\n",
        "R 2 (#7807d2)\n",
        "U 3 (#a77fa3)\n",
        "L 2 (#015232)\n",
        "U 2 (#7a21e3)\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 62;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
