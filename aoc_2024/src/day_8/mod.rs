use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead, BufReader},
    ops::Deref,
};

use aoc_util::geometry::Point2D;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Frequency(u8);

impl Deref for Frequency {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct Map {
    positions: HashMap<Frequency, HashSet<Point2D<i32>>>,
    width: i32,
    height: i32,
}

impl Map {
    fn read(input: &mut dyn BufRead) -> io::Result<Self> {
        let mut width = 0;
        let positions = input
            .lines()
            .enumerate()
            .map(|(row_idx, line)| {
                let line = line?;
                width = width.max(line.len() as _);
                Ok(line
                    .bytes()
                    .enumerate()
                    .filter(|(_, b)| b.is_ascii_alphanumeric())
                    .map(|(col_idx, b)| (Frequency(b), Point2D::at(col_idx as _, row_idx as _)))
                    .collect::<Vec<_>>())
            })
            .collect::<io::Result<Vec<_>>>()?;
        let height = positions.len() as _;
        let positions = positions.into_iter().flatten().fold(
            HashMap::<Frequency, HashSet<_>>::new(),
            |mut acc, (freq, pos)| {
                acc.entry(freq).or_default().insert(pos);
                acc
            },
        );
        Ok(Self {
            positions,
            width,
            height,
        })
    }

    fn is_in_bounds(&self, position: &Point2D<i32>) -> bool {
        (0..self.width).contains(position.x()) && (0..self.height).contains(position.y())
    }
}

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    let map = Map::read(input)?;
    Ok(map
        .positions
        .values()
        .flat_map(|positions| {
            positions.iter().flat_map(|position1| {
                positions
                    .iter()
                    .filter(move |&position2| position1 != position2)
                    .flat_map(move |position2| {
                        [
                            position1
                                .checked_sub(*position2)
                                .map(|direction| position1 + direction),
                            position2
                                .checked_sub(*position1)
                                .map(|direction| position2 + direction),
                        ]
                    })
                    .flatten()
                    .filter(|antinode| {
                        (0..map.width).contains(antinode.x())
                            && (0..map.height).contains(antinode.y())
                    })
            })
        })
        .collect::<HashSet<_>>()
        .len())
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    let map = Map::read(input)?;
    let map_ref = &map;
    let ret = map
        .positions
        .values()
        .flat_map(|positions| {
            positions.iter().flat_map(|position1| {
                positions
                    .iter()
                    .filter(move |&position2| position1 != position2)
                    .flat_map(move |position2| {
                        let direction = position1.checked_sub(*position2);
                        (0..)
                            .map(move |i| {
                                direction.and_then(|direction| position1.checked_add(direction * i))
                            })
                            .take_while(|antinode| {
                                antinode
                                    .filter(|antinode| map_ref.is_in_bounds(antinode))
                                    .is_some()
                            })
                            .chain(
                                (0..)
                                    .map(move |i| {
                                        direction.and_then(|direction| {
                                            position1.checked_sub(direction * i)
                                        })
                                    })
                                    .take_while(|antinode| {
                                        antinode
                                            .filter(|antinode| map_ref.is_in_bounds(antinode))
                                            .is_some()
                                    }),
                            )
                    })
                    .flatten()
                    .filter(|antinode| {
                        (0..map.width).contains(antinode.x())
                            && (0..map.height).contains(antinode.y())
                    })
            })
        })
        .collect::<HashSet<_>>()
        .len();
    Ok(ret)
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2024 Day 8 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2024_08.txt")?))?
        );
    }
    {
        println!("Year 2024 Day 8 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2024_08.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        "............\n",
        "........0...\n",
        ".....0......\n",
        ".......0....\n",
        "....0.......\n",
        "......A.....\n",
        "............\n",
        "............\n",
        "........A...\n",
        ".........A..\n",
        "............\n",
        "............\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 14;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 34;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
