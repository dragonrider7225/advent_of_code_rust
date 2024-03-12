use std::{
    cmp::Reverse,
    collections::HashMap,
    fmt::{self, Display, Formatter},
    fs::File,
    io::{self, BufRead, BufReader},
};

use aoc_util::{collections::PriorityQueue, geometry::Direction};

#[derive(Clone, Debug)]
struct City(Vec<Vec<usize>>);

impl City {
    fn least_heat_loss(&self, min_straight: usize, max_straight: usize) -> usize {
        #[derive(Clone, Copy, Debug)]
        struct Position {
            row_idx: usize,
            col_idx: usize,
            heat_loss: usize,
            last_direction: Option<Direction>,
            direction_count: usize,
        }
        impl Position {
            fn coordinates(&self) -> (usize, usize) {
                (self.row_idx, self.col_idx)
            }

            fn seen_key(&self) -> ((usize, usize), Option<Direction>, usize) {
                (
                    self.coordinates(),
                    self.last_direction,
                    self.direction_count,
                )
            }
        }
        macro_rules! priority_fn {
            () => {
                |&Position { heat_loss, .. }| {
                    Reverse(
                        // Somehow adding the manhattan distance to the final tile works fine for
                        // the example but causes my actual input to result in 685 instead of 684
                        heat_loss, // + (self.0.len() - row_idx - 1)
                                  // + (self.0[row_idx].len() - col_idx - 1),
                    )
                }
            };
        }
        type Coordinates = (usize, usize);
        type SeenType =
            HashMap<(Coordinates, Option<Direction>, usize), (usize, Option<Coordinates>)>;

        let add_position = |queue: &mut PriorityQueue<Reverse<usize>, Position>,
                            seen: &mut SeenType,
                            position: Position,
                            from: Option<(usize, usize)>| {
            if seen
                .get(&position.seen_key())
                .filter(|seen| seen.0 <= position.heat_loss)
                .is_none()
            {
                queue.insert_with_fn(position, priority_fn!());
                seen.insert(position.seen_key(), (position.heat_loss, from));
            }
        };

        let mut queue = PriorityQueue::new();
        let mut seen = HashMap::new();
        add_position(
            &mut queue,
            &mut seen,
            Position {
                row_idx: 0,
                col_idx: 0,
                heat_loss: 0,
                last_direction: None,
                direction_count: 0,
            },
            None,
        );
        while let Some(Position {
            row_idx,
            col_idx,
            heat_loss,
            last_direction,
            direction_count,
        }) = queue.pop()
        {
            if direction_count >= min_straight
                && row_idx + 1 == self.0.len()
                && col_idx + 1 == self.0[row_idx].len()
            {
                return heat_loss;
            }
            match last_direction {
                None => {
                    // We can only have no `last_direction` at our first Position, so we know we
                    // can't go up or left.
                    let new_row_idx = 1;
                    add_position(
                        &mut queue,
                        &mut seen,
                        Position {
                            row_idx: new_row_idx,
                            col_idx,
                            heat_loss: self.0[new_row_idx][col_idx],
                            last_direction: Some(Direction::Down),
                            direction_count: 1,
                        },
                        Some((row_idx, col_idx)),
                    );
                    let new_col_idx = 1;
                    add_position(
                        &mut queue,
                        &mut seen,
                        Position {
                            row_idx,
                            col_idx: new_col_idx,
                            heat_loss: self.0[row_idx][new_col_idx],
                            last_direction: Some(Direction::Right),
                            direction_count: 1,
                        },
                        Some((row_idx, col_idx)),
                    );
                }
                Some(Direction::Down) => {
                    if direction_count < max_straight && row_idx + 1 < self.0.len() {
                        let new_row_idx = row_idx + 1;
                        add_position(
                            &mut queue,
                            &mut seen,
                            Position {
                                row_idx: new_row_idx,
                                col_idx,
                                heat_loss: heat_loss + self.0[new_row_idx][col_idx],
                                last_direction,
                                direction_count: direction_count + 1,
                            },
                            Some((row_idx, col_idx)),
                        );
                    }
                    if direction_count >= min_straight && col_idx > 0 {
                        let new_col_idx = col_idx - 1;
                        add_position(
                            &mut queue,
                            &mut seen,
                            Position {
                                row_idx,
                                col_idx: new_col_idx,
                                heat_loss: heat_loss + self.0[row_idx][new_col_idx],
                                last_direction: Some(Direction::Left),
                                direction_count: 1,
                            },
                            Some((row_idx, col_idx)),
                        );
                    }
                    if direction_count >= min_straight && col_idx + 1 < self.0[row_idx].len() {
                        let new_col_idx = col_idx + 1;
                        add_position(
                            &mut queue,
                            &mut seen,
                            Position {
                                row_idx,
                                col_idx: new_col_idx,
                                heat_loss: heat_loss + self.0[row_idx][new_col_idx],
                                last_direction: Some(Direction::Right),
                                direction_count: 1,
                            },
                            Some((row_idx, col_idx)),
                        );
                    }
                }
                Some(Direction::Up) => {
                    if direction_count < max_straight && row_idx > 0 {
                        let new_row_idx = row_idx - 1;
                        add_position(
                            &mut queue,
                            &mut seen,
                            Position {
                                row_idx: new_row_idx,
                                col_idx,
                                heat_loss: heat_loss + self.0[new_row_idx][col_idx],
                                last_direction,
                                direction_count: direction_count + 1,
                            },
                            Some((row_idx, col_idx)),
                        );
                    }
                    if direction_count >= min_straight && col_idx > 0 {
                        let new_col_idx = col_idx - 1;
                        add_position(
                            &mut queue,
                            &mut seen,
                            Position {
                                row_idx,
                                col_idx: new_col_idx,
                                heat_loss: heat_loss + self.0[row_idx][new_col_idx],
                                last_direction: Some(Direction::Left),
                                direction_count: 1,
                            },
                            Some((row_idx, col_idx)),
                        );
                    }
                    if direction_count >= min_straight && col_idx + 1 < self.0[row_idx].len() {
                        let new_col_idx = col_idx + 1;
                        add_position(
                            &mut queue,
                            &mut seen,
                            Position {
                                row_idx,
                                col_idx: new_col_idx,
                                heat_loss: heat_loss + self.0[row_idx][new_col_idx],
                                last_direction: Some(Direction::Right),
                                direction_count: 1,
                            },
                            Some((row_idx, col_idx)),
                        );
                    }
                }
                Some(Direction::Left) => {
                    if direction_count < max_straight && col_idx > 0 {
                        let new_col_idx = col_idx - 1;
                        add_position(
                            &mut queue,
                            &mut seen,
                            Position {
                                row_idx,
                                col_idx: new_col_idx,
                                heat_loss: heat_loss + self.0[row_idx][new_col_idx],
                                last_direction,
                                direction_count: direction_count + 1,
                            },
                            Some((row_idx, col_idx)),
                        );
                    }
                    if direction_count >= min_straight && row_idx > 0 {
                        let new_row_idx = row_idx - 1;
                        add_position(
                            &mut queue,
                            &mut seen,
                            Position {
                                row_idx: new_row_idx,
                                col_idx,
                                heat_loss: heat_loss + self.0[new_row_idx][col_idx],
                                last_direction: Some(Direction::Up),
                                direction_count: 1,
                            },
                            Some((row_idx, col_idx)),
                        );
                    }
                    if direction_count >= min_straight && row_idx + 1 < self.0.len() {
                        let new_row_idx = row_idx + 1;
                        add_position(
                            &mut queue,
                            &mut seen,
                            Position {
                                row_idx: new_row_idx,
                                col_idx,
                                heat_loss: heat_loss + self.0[new_row_idx][col_idx],
                                last_direction: Some(Direction::Down),
                                direction_count: 1,
                            },
                            Some((row_idx, col_idx)),
                        );
                    }
                }
                Some(Direction::Right) => {
                    if direction_count < max_straight && col_idx + 1 < self.0[row_idx].len() {
                        let new_col_idx = col_idx + 1;
                        add_position(
                            &mut queue,
                            &mut seen,
                            Position {
                                row_idx,
                                col_idx: new_col_idx,
                                heat_loss: heat_loss + self.0[row_idx][new_col_idx],
                                last_direction,
                                direction_count: direction_count + 1,
                            },
                            Some((row_idx, col_idx)),
                        );
                    }
                    if direction_count >= min_straight && row_idx > 0 {
                        let new_row_idx = row_idx - 1;
                        add_position(
                            &mut queue,
                            &mut seen,
                            Position {
                                row_idx: new_row_idx,
                                col_idx,
                                heat_loss: heat_loss + self.0[new_row_idx][col_idx],
                                last_direction: Some(Direction::Up),
                                direction_count: 1,
                            },
                            Some((row_idx, col_idx)),
                        );
                    }
                    if direction_count >= min_straight && row_idx + 1 < self.0.len() {
                        let new_row_idx = row_idx + 1;
                        add_position(
                            &mut queue,
                            &mut seen,
                            Position {
                                row_idx: new_row_idx,
                                col_idx,
                                heat_loss: heat_loss + self.0[new_row_idx][col_idx],
                                last_direction: Some(Direction::Down),
                                direction_count: 1,
                            },
                            Some((row_idx, col_idx)),
                        );
                    }
                }
            }
        }
        unreachable!("Couldn't get to the bottom right of the rectangle")
    }
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in &self.0 {
            for &cell in row {
                write!(f, "{}", (cell as u8 + b'0') as char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl FromIterator<Vec<usize>> for City {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Vec<usize>>,
    {
        Self(iter.into_iter().collect())
    }
}

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    let city = input
        .lines()
        .map(|line| Ok(line?.bytes().map(|b| (b - b'0') as usize).collect()))
        .collect::<io::Result<City>>()?;
    Ok(city.least_heat_loss(1, 3))
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    let city = input
        .lines()
        .map(|line| Ok(line?.bytes().map(|b| (b - b'0') as usize).collect()))
        .collect::<io::Result<City>>()?;
    Ok(city.least_heat_loss(4, 10))
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2023 Day 17 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2023_17.txt")?))?
        );
    }
    {
        println!("Year 2023 Day 17 Part 2");
        println!("823 is too high on my input");
        println!("According to git@github.com:PhunkyBob/adventofcode.git, the correct answer for my input is 822");
        println!("I don't understand where the off-by-one error comes from, since my code works correctly on the examples");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2023_17.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        "2413432311323\n",
        "3215453535623\n",
        "3255245654254\n",
        "3446585845452\n",
        "4546657867536\n",
        "1438598798454\n",
        "4457876987766\n",
        "3637877979653\n",
        "4654967986887\n",
        "4564679986453\n",
        "1224686865563\n",
        "2546548887735\n",
        "4322674655533\n",
    );
    const TEST_DATA_2: &str = concat!(
        "111111111111\n",
        "999999999991\n",
        "999999999991\n",
        "999999999991\n",
        "999999999991\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 102;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 94;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        let expected = 71;
        let actual = part2(&mut Cursor::new(TEST_DATA_2))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
