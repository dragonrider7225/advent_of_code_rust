use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use nom::{branch, bytes::complete as bytes, combinator, multi};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl Pipe {
    fn nom_parse(s: &str) -> nom::IResult<&str, Self> {
        branch::alt((
            combinator::value(Self::Vertical, bytes::tag("|")),
            combinator::value(Self::Horizontal, bytes::tag("-")),
            combinator::value(Self::NorthEast, bytes::tag("L")),
            combinator::value(Self::NorthWest, bytes::tag("J")),
            combinator::value(Self::SouthWest, bytes::tag("7")),
            combinator::value(Self::SouthEast, bytes::tag("F")),
            combinator::value(Self::Ground, bytes::tag(".")),
            combinator::value(Self::Start, bytes::tag("S")),
        ))(s)
    }

    /// Checks whether this pipe connects to the start pipe at `target`.
    fn connects_to_start(&self, pos: (usize, usize), start_pos: (usize, usize)) -> bool {
        match self {
            Self::Vertical => pos.1 == start_pos.1 && pos.0.abs_diff(start_pos.0) == 1,
            Self::Horizontal => pos.0 == start_pos.0 && pos.1.abs_diff(start_pos.1) == 1,
            Self::NorthEast => {
                pos == (start_pos.0 + 1, start_pos.1) || (pos.0, pos.1 + 1) == start_pos
            }
            Self::NorthWest => {
                pos == (start_pos.0 + 1, start_pos.1) || pos == (start_pos.0, start_pos.1 + 1)
            }
            Self::SouthWest => {
                (pos.0 + 1, pos.1) == start_pos || pos == (start_pos.0, start_pos.1 + 1)
            }
            Self::SouthEast => (pos.0 + 1, pos.1) == start_pos || (pos.0, pos.1 + 1) == start_pos,
            Self::Ground => false,
            Self::Start => false,
        }
    }

    fn next_location(
        &self,
        current_location: (usize, usize),
        previous_location: (usize, usize),
    ) -> (usize, usize) {
        match self {
            Self::Vertical => {
                if previous_location.0 < current_location.0 {
                    (current_location.0 + 1, current_location.1)
                } else {
                    (current_location.0 - 1, current_location.1)
                }
            }
            Self::Horizontal => {
                if previous_location.1 < current_location.1 {
                    (current_location.0, current_location.1 + 1)
                } else {
                    (current_location.0, current_location.1 - 1)
                }
            }
            Self::NorthEast => {
                if previous_location.0 < current_location.0 {
                    (current_location.0, current_location.1 + 1)
                } else {
                    (current_location.0 - 1, current_location.1)
                }
            }
            Self::NorthWest => {
                if previous_location.0 < current_location.0 {
                    (current_location.0, current_location.1 - 1)
                } else {
                    (current_location.0 - 1, current_location.1)
                }
            }
            Self::SouthWest => {
                if previous_location.0 > current_location.0 {
                    (current_location.0, current_location.1 - 1)
                } else {
                    (current_location.0 + 1, current_location.1)
                }
            }
            Self::SouthEast => {
                if previous_location.0 > current_location.0 {
                    (current_location.0, current_location.1 + 1)
                } else {
                    (current_location.0 + 1, current_location.1)
                }
            }
            Self::Ground => unreachable!("Ground is not a usable pipe"),
            Self::Start => unreachable!("We started at the start"),
        }
    }
}

fn move_north((row, col): (usize, usize)) -> Option<(usize, usize)> {
    if row == 0 {
        None
    } else {
        Some((row - 1, col))
    }
}

fn move_south((row, col): (usize, usize)) -> Option<(usize, usize)> {
    Some((row + 1, col))
}

fn move_west((row, col): (usize, usize)) -> Option<(usize, usize)> {
    if col == 0 {
        None
    } else {
        Some((row, col - 1))
    }
}

fn move_east((row, col): (usize, usize)) -> Option<(usize, usize)> {
    Some((row, col + 1))
}

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    let sketch = input
        .lines()
        .map(|line| {
            let line = line?;
            // TODO: This is required to fix a "dropped while borrowed" error on `line`.
            #[allow(clippy::let_and_return)]
            let ret = multi::many1(Pipe::nom_parse)(&line)
                .map(|(_, pipes)| pipes)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()));
            ret
        })
        .collect::<io::Result<Vec<_>>>()?;
    let start_coords = sketch
        .iter()
        .enumerate()
        .find_map(|(row_idx, row)| {
            row.iter()
                .position(|pipe| matches!(pipe, Pipe::Start))
                .map(|col_idx| (row_idx, col_idx))
        })
        .expect("No start pipe found");
    let (left, right) = [
        move_north(start_coords),
        move_east(start_coords),
        move_south(start_coords),
        move_west(start_coords),
    ]
    .into_iter()
    .flatten()
    .fold((None, None), |acc, (row, col)| match sketch[row][col] {
        pipe if pipe.connects_to_start((row, col), start_coords) => match acc {
            (None, None) => (Some((row, col)), None),
            (left, None) => (left, Some((row, col))),
            (None, _) => unreachable!(),
            _ => acc,
        },
        _ => acc,
    });
    let (mut old_left, mut old_right) = (start_coords, start_coords);
    let (mut left, mut right) = (
        left.expect("Start doesn't connect to any pipes"),
        right.expect("Start only connects to one pipe"),
    );
    let mut num_steps = 1;
    while left != right {
        let new_left = sketch[left.0][left.1].next_location(left, old_left);
        let new_right = sketch[right.0][right.1].next_location(right, old_right);
        (old_left, old_right, left, right) = (left, right, new_left, new_right);
        num_steps += 1;
    }
    Ok(num_steps)
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    let sketch = input
        .lines()
        .map(|line| {
            let line = line?;
            // TODO: This is required to fix a "dropped while borrowed" error on `line`.
            #[allow(clippy::let_and_return)]
            let ret = multi::many1(Pipe::nom_parse)(&line)
                .map(|(_, pipes)| pipes)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()));
            ret
        })
        .collect::<io::Result<Vec<_>>>()?;
    let start_coords = sketch
        .iter()
        .enumerate()
        .find_map(|(row_idx, row)| {
            row.iter()
                .position(|pipe| matches!(pipe, Pipe::Start))
                .map(|col_idx| (row_idx, col_idx))
        })
        .expect("No start pipe found");
    let (init_left, init_right) = [
        move_north(start_coords),
        move_east(start_coords),
        move_south(start_coords),
        move_west(start_coords),
    ]
    .into_iter()
    .flatten()
    .fold((None, None), |acc, (row, col)| match sketch[row][col] {
        pipe if pipe.connects_to_start((row, col), start_coords) => match acc {
            (None, None) => (Some((row, col)), None),
            (left, None) => (left, Some((row, col))),
            (None, _) => unreachable!(),
            _ => acc,
        },
        _ => acc,
    });
    let actual_start = match init_left {
        left if left == move_north(start_coords) => match init_right {
            right if right == move_east(start_coords) => Pipe::NorthEast,
            right if right == move_south(start_coords) => Pipe::Vertical,
            right => {
                debug_assert_eq!(right, move_west(start_coords));
                Pipe::NorthWest
            }
        },
        left if left == move_east(start_coords) => match init_right {
            right if right == move_south(start_coords) => Pipe::SouthEast,
            right => {
                debug_assert_eq!(right, move_west(start_coords));
                Pipe::Horizontal
            }
        },
        left => {
            debug_assert_eq!(left, move_south(start_coords));
            debug_assert_eq!(init_right, move_west(start_coords));
            Pipe::SouthWest
        }
    };
    let sketch = {
        let mut sketch = sketch;
        sketch[start_coords.0][start_coords.1] = actual_start;
        sketch
    };
    let (init_left, init_right) = (
        init_left.expect("Start doesn't connect to any pipes"),
        init_right.expect("Start only connects to one pipe"),
    );
    let mut loop_tiles = vec![start_coords];
    let (mut old_left, mut old_right) = (start_coords, start_coords);
    let (mut left, mut right) = (init_left, init_right);
    while left != right {
        loop_tiles.extend([left, right]);
        let new_left = sketch[left.0][left.1].next_location(left, old_left);
        let new_right = sketch[right.0][right.1].next_location(right, old_right);
        (old_left, old_right, left, right) = (left, right, new_left, new_right);
    }
    loop_tiles.push(left);
    let mut num_enclosed_tiles = 0;
    for (row_idx, row) in sketch.iter().enumerate() {
        let mut inside_loop = false;
        let mut last_corner = None;
        for (col_idx, tile) in row.iter().copied().enumerate() {
            if let Some(idx) = loop_tiles.iter().position(|&pos| pos == (row_idx, col_idx)) {
                match tile {
                    Pipe::NorthEast | Pipe::SouthEast => {
                        debug_assert!(last_corner.is_none());
                        last_corner = Some(tile);
                    }
                    Pipe::Horizontal => debug_assert!(last_corner.is_some()),
                    Pipe::NorthWest => {
                        debug_assert!(last_corner.is_some());
                        match last_corner.take() {
                            Some(Pipe::SouthEast) => {
                                inside_loop = !inside_loop;
                            }
                            Some(Pipe::NorthEast) => {}
                            _ => unreachable!("Invalid last_corner"),
                        }
                    }
                    Pipe::SouthWest => {
                        debug_assert!(last_corner.is_some());
                        match last_corner.take() {
                            Some(Pipe::NorthEast) => {
                                inside_loop = !inside_loop;
                            }
                            Some(Pipe::SouthEast) => {}
                            _ => unreachable!("Invalid last_corner"),
                        }
                    }
                    Pipe::Vertical => inside_loop = !inside_loop,
                    _ => unreachable!("Invalid loop tile {tile:?}"),
                }
                loop_tiles.swap_remove(idx);
            } else if inside_loop {
                debug_assert!(last_corner.is_none());
                num_enclosed_tiles += 1;
            }
        }
    }
    Ok(num_enclosed_tiles)
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2023 Day 10 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2023_10.txt")?))?
        );
    }
    {
        println!("Year 2023 Day 10 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2023_10.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA_1: &str = concat!("-L|F7\n", "7S-7|\n", "L|7||\n", "-L-J|\n", "L|-JF\n",);
    const TEST_DATA_2: &str = concat!("7-F7-\n", ".FJ|7\n", "SJLL7\n", "|F--J\n", "LJ.LJ\n",);
    const TEST_DATA_3: &str = concat!(
        "...........\n",
        ".S-------7.\n",
        ".|F-----7|.\n",
        ".||.....||.\n",
        ".||.....||.\n",
        ".|L-7.F-J|.\n",
        ".|..|.|..|.\n",
        ".L--J.L--J.\n",
        "...........\n",
    );
    const TEST_DATA_4: &str = concat!(
        "..........\n",
        ".S------7.\n",
        ".|F----7|.\n",
        ".||....||.\n",
        ".||....||.\n",
        ".|L-7F-J|.\n",
        ".|..||..|.\n",
        ".L--JL--J.\n",
        "..........\n",
    );
    const TEST_DATA_5: &str = concat!(
        ".F----7F7F7F7F-7....\n",
        ".|F--7||||||||FJ....\n",
        ".||.FJ||||||||L7....\n",
        "FJL7L7LJLJ||LJ.L-7..\n",
        "L--J.L7...LJS7F-7L7.\n",
        "....F-J..F7FJ|L7L7L7\n",
        "....L7.F7||L7|.L7L7|\n",
        ".....|FJLJ|FJ|F7|.LJ\n",
        "....FJL-7.||.||||...\n",
        "....L---J.LJ.LJLJ...\n",
    );
    const TEST_DATA_6: &str = concat!(
        "FF7FSF7F7F7F7F7F---7\n",
        "L|LJ||||||||||||F--J\n",
        "FL-7LJLJ||||||LJL-77\n",
        "F--JF--7||LJLJ7F7FJ-\n",
        "L---JF-JLJ.||-FJLJJ7\n",
        "|F|F-JF---7F7-L7L|7|\n",
        "|FFJF7L7F-JF7|JL---7\n",
        "7-L-JL7||F7|L7F-7F7|\n",
        "L.L7LFJ|||||FJL7||LJ\n",
        "L7JLJL-JLJLJL--JLJ.L\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 4;
        let actual = part1(&mut Cursor::new(TEST_DATA_1))?;
        assert_eq!(expected, actual);
        let expected = 8;
        let actual = part1(&mut Cursor::new(TEST_DATA_2))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 4;
        let actual = part2(&mut Cursor::new(TEST_DATA_3))?;
        assert_eq!(expected, actual);
        let expected = 4;
        let actual = part2(&mut Cursor::new(TEST_DATA_4))?;
        assert_eq!(expected, actual);
        let expected = 8;
        let actual = part2(&mut Cursor::new(TEST_DATA_5))?;
        assert_eq!(expected, actual);
        let expected = 10;
        let actual = part2(&mut Cursor::new(TEST_DATA_6))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
