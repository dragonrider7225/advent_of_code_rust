use std::{
    fmt::{self, Display, Formatter},
    fs::File,
    io::{self, BufRead, BufReader},
    ops::{Add, Index},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

impl Add<DeltaPosition> for Position {
    type Output = Option<Self>;

    fn add(self, rhs: DeltaPosition) -> Self::Output {
        Some(Self {
            x: self.x.checked_add_signed(rhs.x)?,
            y: self.y.checked_add_signed(rhs.y)?,
        })
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T> Index<Position> for Vec<Vec<T>> {
    type Output = T;

    fn index(&self, index: Position) -> &Self::Output {
        &self[index.y][index.x]
    }
}

trait TryIndex<Idx>: Index<Idx> {
    fn try_index(&self, index: Idx) -> Option<&Self::Output>;
}

trait TryIndexMut<Idx>: TryIndex<Idx> {
    fn try_index_mut(&mut self, index: Idx) -> Option<&mut Self::Output>;
}

impl<T> TryIndex<Position> for Vec<Vec<T>> {
    fn try_index(&self, index: Position) -> Option<&Self::Output> {
        self.get(index.y).and_then(|row| row.get(index.x))
    }
}

impl<T> TryIndexMut<Position> for Vec<Vec<T>> {
    fn try_index_mut(&mut self, index: Position) -> Option<&mut Self::Output> {
        self.get_mut(index.y).and_then(|row| row.get_mut(index.x))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct DeltaPosition {
    x: isize,
    y: isize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tile {
    Roll,
    Floor,
}

impl TryFrom<u8> for Tile {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'@' => Ok(Self::Roll),
            b'.' => Ok(Self::Floor),
            _ => Err(format!("Invalid tile character: {:?}", value as char)),
        }
    }
}

#[cfg(test)]
fn print_map(map: &[Vec<Tile>]) {
    for row in map {
        for &tile in row {
            match tile {
                Tile::Floor => eprint!("."),
                Tile::Roll => eprint!("@"),
            }
        }
        eprintln!();
    }
}

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    let map = input
        .lines()
        .map(|line| {
            line.and_then(|line| {
                line.bytes()
                    .map(Tile::try_from)
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            })
        })
        .collect::<io::Result<Vec<_>>>()?;
    #[cfg(test)]
    print_map(&map);
    Ok(map
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, &tile)| matches!(tile, Tile::Roll))
                .map(move |(col_idx, _)| Position {
                    x: col_idx,
                    y: row_idx,
                })
        })
        .filter(|&position| {
            let neighbors = [
                DeltaPosition { x: -1, y: -1 },
                DeltaPosition { x: -1, y: 0 },
                DeltaPosition { x: -1, y: 1 },
                DeltaPosition { x: 0, y: -1 },
                DeltaPosition { x: 0, y: 1 },
                DeltaPosition { x: 1, y: -1 },
                DeltaPosition { x: 1, y: 0 },
                DeltaPosition { x: 1, y: 1 },
            ]
            .into_iter()
            .filter(|&delta| {
                let Some(neighbor_position) = position + delta else {
                    return false;
                };
                matches!(map.try_index(neighbor_position).copied(), Some(Tile::Roll))
            })
            .count();
            if neighbors < 4 {
                if cfg!(test) {
                    eprintln!("Found movable roll at {position}");
                }
                true
            } else {
                false
            }
        })
        .count())
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    let mut map = input
        .lines()
        .map(|line| {
            line.and_then(|line| {
                line.bytes()
                    .map(Tile::try_from)
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            })
        })
        .collect::<io::Result<Vec<_>>>()?;
    let mut total = 0;
    loop {
        #[cfg(test)]
        print_map(&map);
        let to_remove = map
            .iter()
            .enumerate()
            .flat_map(|(row_idx, row)| {
                row.iter()
                    .enumerate()
                    .filter(|&(_, &tile)| matches!(tile, Tile::Roll))
                    .map(move |(col_idx, _)| Position {
                        x: col_idx,
                        y: row_idx,
                    })
            })
            .filter(|&position| {
                let neighbors = [
                    DeltaPosition { x: -1, y: -1 },
                    DeltaPosition { x: -1, y: 0 },
                    DeltaPosition { x: -1, y: 1 },
                    DeltaPosition { x: 0, y: -1 },
                    DeltaPosition { x: 0, y: 1 },
                    DeltaPosition { x: 1, y: -1 },
                    DeltaPosition { x: 1, y: 0 },
                    DeltaPosition { x: 1, y: 1 },
                ]
                .into_iter()
                .filter(|&delta| {
                    let Some(neighbor_position) = position + delta else {
                        return false;
                    };
                    matches!(map.try_index(neighbor_position).copied(), Some(Tile::Roll))
                })
                .count();
                if neighbors < 4 {
                    if cfg!(test) {
                        eprintln!("Found movable roll at {position}");
                    }
                    true
                } else {
                    false
                }
            })
            .collect::<Vec<_>>();
        match to_remove.len() {
            0 => return Ok(total),
            n => {
                #[cfg(test)]
                eprintln!("Removing {n} rolls");
                total += n
            }
        }
        to_remove
            .into_iter()
            .for_each(|p| *map.try_index_mut(p).unwrap() = Tile::Floor);
    }
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2025 Day 4 Part 1");
        println!(
            "{:?}",
            part1(&mut BufReader::new(File::open("2025_04.txt")?))?
        );
    }
    {
        println!("Year 2025 Day 4 Part 2");
        println!(
            "{:?}",
            part2(&mut BufReader::new(File::open("2025_04.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        "..@@.@@@@.\n",
        "@@@.@.@.@@\n",
        "@@@@@.@.@@\n",
        "@.@@@@..@.\n",
        "@@.@@@@.@@\n",
        ".@@@@@@@.@\n",
        ".@.@.@.@@@\n",
        "@.@@@.@@@@\n",
        ".@@@@@@@@.\n",
        "@.@.@@@.@.\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 13;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 43;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
