use std::{
    cmp::Ordering,
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn step_head(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Position {
                y: self.y + 1,
                ..*self
            },
            Direction::Right => Position {
                x: self.x + 1,
                ..*self
            },
            Direction::Down => Position {
                y: self.y - 1,
                ..*self
            },
            Direction::Left => Position {
                x: self.x - 1,
                ..*self
            },
        }
    }

    fn step_tail(&self, new_head: Position) -> Position {
        if (self.x - new_head.x).abs() > 1 || (self.y - new_head.y).abs() > 1 {
            match (new_head.x.cmp(&self.x), new_head.y.cmp(&self.y)) {
                (Ordering::Less, Ordering::Less) => Position {
                    x: self.x - 1,
                    y: self.y - 1,
                },
                (Ordering::Less, Ordering::Equal) => Position {
                    x: self.x - 1,
                    y: self.y,
                },
                (Ordering::Less, Ordering::Greater) => Position {
                    x: self.x - 1,
                    y: self.y + 1,
                },
                (Ordering::Equal, Ordering::Less) => Position {
                    x: self.x,
                    y: self.y - 1,
                },
                (Ordering::Equal, Ordering::Equal) => unreachable!(),
                (Ordering::Equal, Ordering::Greater) => Position {
                    x: self.x,
                    y: self.y + 1,
                },
                (Ordering::Greater, Ordering::Less) => Position {
                    x: self.x + 1,
                    y: self.y - 1,
                },
                (Ordering::Greater, Ordering::Equal) => Position {
                    x: self.x + 1,
                    y: self.y,
                },
                (Ordering::Greater, Ordering::Greater) => Position {
                    x: self.x + 1,
                    y: self.y + 1,
                },
            }
        } else {
            *self
        }
    }
}

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    let mut visited_cells = HashSet::from([Position::default()]);
    let mut current_head = Position::default();
    let mut current_tail = Position::default();
    for line in input.lines() {
        let line = line?;
        let (direction, distance) = {
            let bytes = line.as_bytes();
            (
                bytes[0],
                bytes[2..]
                    .iter()
                    .copied()
                    .fold(0, |acc, d| acc * 10 + (d - b'0') as usize),
            )
        };
        let direction = match direction {
            b'R' => Direction::Right,
            b'U' => Direction::Up,
            b'L' => Direction::Left,
            b'D' => Direction::Down,
            _ => unreachable!("Invalid direction {:?}", line.chars().next().unwrap()),
        };
        for _ in 0..distance {
            let new_head = current_head.step_head(direction);
            let new_tail = current_tail.step_tail(new_head);
            visited_cells.insert(new_tail);
            current_head = new_head;
            current_tail = new_tail;
        }
    }
    Ok(visited_cells.len())
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    let mut visited_cells = HashSet::from([Position::default()]);
    let mut current_head = Position::default();
    let mut current_knots = [Position { x: 0, y: 0 }; 9];
    let mut new_knots = Vec::with_capacity(9);
    for line in input.lines() {
        let line = line?;
        let (direction, distance) = {
            let bytes = line.as_bytes();
            (
                bytes[0],
                bytes[2..]
                    .iter()
                    .copied()
                    .fold(0, |acc, d| acc * 10 + (d - b'0') as usize),
            )
        };
        let direction = match direction {
            b'R' => Direction::Right,
            b'U' => Direction::Up,
            b'L' => Direction::Left,
            b'D' => Direction::Down,
            _ => unreachable!("Invalid direction {:?}", line.chars().next().unwrap()),
        };
        for _ in 0..distance {
            let new_head = current_head.step_head(direction);
            new_knots.push(current_knots[0].step_tail(new_head));
            for &current_knot in &current_knots[1..] {
                new_knots.push(current_knot.step_tail(*new_knots.last().unwrap()));
            }
            let new_tail = *new_knots.last().unwrap();
            visited_cells.insert(new_tail);
            current_head = new_head;
            current_knots.copy_from_slice(&new_knots[..]);
            new_knots.clear();
        }
    }
    Ok(visited_cells.len())
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2022 Day 9 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2022_09.txt")?))?
        );
    }
    {
        println!("Year 2022 Day 9 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2022_09.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2\n";
    const TEST_DATA_LARGE: &str = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20\n";

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 13;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 1;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        let expected = 36;
        let actual = part2(&mut Cursor::new(TEST_DATA_LARGE))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
