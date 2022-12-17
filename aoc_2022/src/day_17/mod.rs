use std::{
    cmp::Reverse,
    collections::HashSet,
    fmt::{self, Display, Formatter},
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RockShape {
    Dash,
    Plus,
    Vee,
    Pipe,
    Square,
}

impl RockShape {
    const ORDER: [Self; 5] = [Self::Dash, Self::Plus, Self::Vee, Self::Pipe, Self::Square];
}

impl RockShape {
    fn left(&self, pos: &RockPosition) -> u32 {
        pos.left
    }

    fn right(&self, pos: &RockPosition) -> u32 {
        match self {
            Self::Dash => pos.left + 3,
            Self::Plus => pos.left + 2,
            Self::Vee => pos.left + 2,
            Self::Pipe => pos.left,
            Self::Square => pos.left + 1,
        }
    }

    fn top(&self, pos: &RockPosition) -> u32 {
        match self {
            Self::Dash => pos.bottom,
            Self::Plus => pos.bottom + 2,
            Self::Vee => pos.bottom + 2,
            Self::Pipe => pos.bottom + 3,
            Self::Square => pos.bottom + 1,
        }
    }

    fn bottom(&self, pos: &RockPosition) -> u32 {
        pos.bottom
    }

    fn parts(&self, pos: &RockPosition) -> impl Iterator<Item = (u32, u32)> + Clone {
        let deltas: &[_] = match self {
            Self::Dash => &[(0, 0), (1, 0), (2, 0), (3, 0)],
            Self::Plus => &[(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
            Self::Vee => &[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            Self::Pipe => &[(0, 0), (0, 1), (0, 2), (0, 3)],
            Self::Square => &[(0, 0), (0, 1), (1, 0), (1, 1)],
        };
        let pos = *pos;
        deltas
            .iter()
            .map(move |&(delta_x, delta_y)| (pos.left + delta_x, pos.bottom + delta_y))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct RockPosition {
    left: u32,
    bottom: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Winds {
    pattern: Vec<Direction>,
    idx: usize,
}

impl Winds {
    fn read(input: &mut dyn BufRead) -> io::Result<Self> {
        let pattern = input
            .lines()
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing input"))??
            .bytes()
            .map(|c| match c {
                b'>' => Ok(Direction::Right),
                b'<' => Ok(Direction::Left),
                _ => Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Invalid direction {:?}", char::from(c)),
                )),
            })
            .collect::<io::Result<_>>()?;
        Ok(Self { pattern, idx: 0 })
    }
}

impl Iterator for Winds {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.pattern[self.idx];
        self.idx = (self.idx + 1) % self.pattern.len();
        Some(val)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Screen {
    current_rock_idx: usize,
    current_rock_pos: RockPosition,
    column: HashSet<(u32, u32)>,
    winds: Winds,
    max_stopped_y: u32,
}

impl Screen {
    fn new(winds: Winds) -> Self {
        Self {
            current_rock_idx: 0,
            current_rock_pos: RockPosition { left: 2, bottom: 3 },
            column: HashSet::new(),
            winds,
            max_stopped_y: 0,
        }
    }

    fn current_rock(&self) -> RockShape {
        RockShape::ORDER[self.current_rock_idx]
    }

    fn finish_rock(&mut self) {
        self.column
            .extend(self.current_rock().parts(&self.current_rock_pos));
        let max_new_y = self
            .current_rock()
            .parts(&self.current_rock_pos)
            .map(|(_, y)| y)
            .max()
            .unwrap();
        if max_new_y > self.max_stopped_y {
            self.max_stopped_y = max_new_y;
        }
        self.current_rock_idx = (self.current_rock_idx + 1) % RockShape::ORDER.len();
        self.current_rock_pos = RockPosition {
            left: 2,
            bottom: self.max_stopped_y + 4,
        };
    }

    fn step(&mut self) {
        loop {
            match self.winds.next().unwrap() {
                Direction::Left => {
                    if self.current_rock_pos.left != 0 {
                        let next_position = RockPosition {
                            left: self.current_rock_pos.left - 1,
                            ..self.current_rock_pos
                        };
                        if next_position.bottom > self.max_stopped_y
                            || !self
                                .current_rock()
                                .parts(&next_position)
                                .any(|part| self.column.contains(&part))
                        {
                            self.current_rock_pos = next_position;
                        }
                    }
                }
                Direction::Right => {
                    if self.current_rock().right(&self.current_rock_pos) < 6 {
                        let next_position = RockPosition {
                            left: self.current_rock_pos.left + 1,
                            ..self.current_rock_pos
                        };
                        if next_position.bottom > self.max_stopped_y
                            || !self
                                .current_rock()
                                .parts(&next_position)
                                .any(|part| self.column.contains(&part))
                        {
                            self.current_rock_pos = next_position;
                        }
                    }
                }
            }
            if self.current_rock_pos.bottom == 0 {
                self.finish_rock();
                return;
            }
            let next_position = RockPosition {
                bottom: self.current_rock_pos.bottom - 1,
                ..self.current_rock_pos
            };
            if self
                .current_rock()
                .parts(&next_position)
                .any(|part| self.column.contains(&part))
            {
                self.finish_rock();
                return;
            } else {
                self.current_rock_pos = next_position;
            }
        }
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let max_y = self
            .current_rock()
            .top(&self.current_rock_pos)
            .max(self.max_stopped_y);
        let mut parts = self
            .column
            .iter()
            .map(|&(x, y)| ('#', x, y))
            .chain(
                self.current_rock()
                    .parts(&self.current_rock_pos)
                    .map(|(x, y)| ('@', x, y)),
            )
            .collect::<Vec<_>>();
        parts.sort_unstable_by_key(|&(_, x, _)| x);
        parts.sort_by_key(|&(_, _, y)| Reverse(y));
        let (last_y, mut acc) =
            parts
                .into_iter()
                .fold(Ok((max_y, "".to_string())), |acc, (c, x, y)| {
                    let (last_y, mut acc) = acc?;
                    if y < last_y {
                        acc.push_str(&" ".repeat(7usize - acc.len()));
                        writeln!(f, "|{acc}|")?;
                        for _ in (y + 1)..last_y {
                            writeln!(f, "|       |")?;
                        }
                        acc.clear();
                    }
                    acc.push_str(&" ".repeat(x as usize - acc.len()));
                    acc.push(c);
                    Ok((y, acc))
                })?;
        acc.push_str(&" ".repeat(7usize - acc.len()));
        writeln!(f, "|{acc}|")?;
        for _ in 0..last_y {
            writeln!(f, "|       |")?;
        }
        writeln!(f, "+-------+")
    }
}

fn part1(input: &mut dyn BufRead) -> io::Result<u32> {
    let winds = Winds::read(input)?;
    let mut screen = Screen::new(winds);
    for i in 0..2022 {
        screen.step();
        if i < 11 {
            println!("{screen}");
        }
    }
    Ok(screen.max_stopped_y + 1)
}

fn part2(_input: &mut dyn BufRead) -> io::Result<()> {
    todo!("Year 2022 Day 17 Part 2")
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2022 Day 17 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2022_17.txt")?))?
        );
    }
    {
        println!("Year 2022 Day 17 Part 2");
        println!(
            "{:?}",
            part2(&mut BufReader::new(File::open("2022_17.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>\n";

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 3068;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
