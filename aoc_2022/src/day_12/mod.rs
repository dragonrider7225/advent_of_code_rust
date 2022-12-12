use std::{
    cmp::Reverse,
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader},
    ops::Index,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn neighbors(&self) -> impl Iterator<Item = Self> + '_ {
        (0..4).filter_map(|i| match i {
            0 => {
                if self.x != 0 {
                    Some(Self {
                        x: self.x - 1,
                        ..*self
                    })
                } else {
                    None
                }
            }
            1 => {
                if self.y != 0 {
                    Some(Self {
                        y: self.y - 1,
                        ..*self
                    })
                } else {
                    None
                }
            }
            2 => Some(Self {
                x: self.x + 1,
                ..*self
            }),
            3 => Some(Self {
                y: self.y + 1,
                ..*self
            }),
            _ => unreachable!(),
        })
    }
}

impl Default for Pos {
    fn default() -> Self {
        Self {
            x: usize::MAX,
            y: usize::MAX,
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Map {
    heights: Vec<Vec<u32>>,
    start: Pos,
    end: Pos,
}

impl Map {
    fn read(input: &mut dyn BufRead) -> io::Result<Self> {
        let ret = input
            .lines()
            .fold(Ok(Self::default()), |acc: io::Result<_>, line| {
                let mut acc = acc?;
                let line = line?;
                let bytes = line.as_bytes();
                let row = bytes
                    .iter()
                    .copied()
                    .enumerate()
                    .map(|(i, c)| {
                        let actual_height = match c {
                            b'S' => {
                                acc.start = Pos {
                                    x: i,
                                    y: acc.heights.len(),
                                };
                                b'a'
                            }
                            b'E' => {
                                acc.end = Pos {
                                    x: i,
                                    y: acc.heights.len(),
                                };
                                b'z'
                            }
                            b'a'..=b'z' => c,
                            _ => {
                                return Err(io::Error::new(
                                    io::ErrorKind::InvalidData,
                                    format!("Invalid height {c:?}"),
                                ))
                            }
                        } - b'a';
                        Ok(actual_height as u32)
                    })
                    .collect::<io::Result<_>>()?;
                acc.heights.push(row);
                Ok(acc)
            })?;
        if ret.start.y >= ret.heights.len() || ret.start.x >= ret.heights[0].len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Couldn't find start point",
            ));
        }
        if ret.end.y >= ret.heights.len() || ret.end.x >= ret.heights[0].len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Couldn't find end point",
            ));
        }
        Ok(ret)
    }

    fn has_pos(&self, pos: Pos) -> bool {
        self.heights.len() > pos.y && self.heights[0].len() > pos.x
    }
}

impl Index<Pos> for Map {
    type Output = u32;

    fn index(&self, index: Pos) -> &Self::Output {
        &self.heights[index.y][index.x]
    }
}

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    let map = Map::read(input)?;
    let mut current_positions = vec![(map.start, 0)];
    let mut visited: HashSet<Pos> =
        HashSet::from_iter(current_positions.iter().map(|&(pos, _)| pos));
    loop {
        current_positions.sort_unstable_by_key(|&(_, steps_so_far)| Reverse(steps_so_far));
        if let Some((next_step_from, steps_so_far)) = current_positions.pop() {
            if next_step_from == map.end {
                return Ok(steps_so_far);
            }
            let neighbors = next_step_from
                .neighbors()
                .filter(|neighbor| !visited.contains(neighbor))
                .filter(|&neighbor| map.has_pos(neighbor))
                .filter(|&neighbor| map[neighbor] <= map[next_step_from] + 1)
                .collect::<Vec<_>>();
            visited.extend(neighbors.iter().copied());
            current_positions.extend(
                neighbors
                    .into_iter()
                    .map(|neighbor| (neighbor, steps_so_far + 1)),
            );
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Couldn't reach end",
            ));
        }
    }
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    let map = Map::read(input)?;
    let mut current_positions = vec![(map.end, 0)];
    let mut visited: HashSet<Pos> =
        HashSet::from_iter(current_positions.iter().map(|&(pos, _)| pos));
    loop {
        current_positions.sort_unstable_by_key(|&(_, steps_so_far)| Reverse(steps_so_far));
        if let Some((next_step_from, steps_so_far)) = current_positions.pop() {
            if map[next_step_from] == 0 {
                return Ok(steps_so_far);
            }
            let neighbors = next_step_from
                .neighbors()
                .filter(|neighbor| !visited.contains(neighbor))
                .filter(|&neighbor| map.has_pos(neighbor))
                .filter(|&neighbor| map[neighbor] + 1 >= map[next_step_from])
                .collect::<Vec<_>>();
            visited.extend(neighbors.iter().copied());
            current_positions.extend(
                neighbors
                    .into_iter()
                    .map(|neighbor| (neighbor, steps_so_far + 1)),
            );
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Couldn't reach elevation 0",
            ));
        }
    }
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2022 Day 12 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2022_12.txt")?))?
        );
    }
    {
        println!("Year 2022 Day 12 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2022_12.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        "Sabqponm\n",
        "abcryxxl\n",
        "accszExk\n",
        "acctuvwj\n",
        "abdefghi\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 31;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 29;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
