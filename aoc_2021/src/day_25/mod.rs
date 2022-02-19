use std::{
    fmt::{self, Display, Formatter},
    fs::File,
    io::{self, BufRead, BufReader},
    mem,
    ops::{Index, IndexMut},
};

use aoc_util::nom_extended::NomParse;
use nom::{
    branch, bytes::complete as bytes, character::complete as character, combinator as comb, multi,
    sequence, IResult,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    East,
    South,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
struct Cell(Option<Direction>);

impl Cell {
    fn unwrap(self) -> Option<Direction> {
        self.0
    }
}

impl NomParse<&str> for Cell {
    fn nom_parse(input: &str) -> IResult<&str, Self> {
        branch::alt((
            comb::value(Self(None), bytes::tag(".")),
            comb::value(Self(Some(Direction::East)), bytes::tag(">")),
            comb::value(Self(Some(Direction::South)), bytes::tag("v")),
        ))(input)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Seafloor {
    cells: Vec<Option<Direction>>,
    num_rows: usize,
    num_columns: usize,
}

impl Seafloor {
    fn step(&mut self) -> bool {
        #[cfg(test)]
        println!("Before step: {}", self);
        let mut changed = false;
        let mut new_cells = self.clone();
        new_cells.cells.iter_mut().for_each(|cell| *cell = None);
        for row in 0..self.num_rows {
            for column in 0..self.num_columns {
                let current_space = (row, column);
                let current_value = self[current_space];
                match current_value {
                    None => {}
                    Some(Direction::South) => new_cells[current_space] = current_value,
                    Some(Direction::East) => {
                        let next_space = if column + 1 == self.num_columns {
                            (row, 0)
                        } else {
                            (row, column + 1)
                        };
                        #[cfg(test)]
                        println!(
                            "Found east-facing sea cucumber at {current_space:?} facing {next_space:?}"
                        );
                        if self[next_space].is_none() {
                            changed = true;
                            new_cells[current_space] = None;
                            new_cells[next_space] = current_value;
                        } else {
                            new_cells[current_space] = current_value;
                        }
                    }
                }
            }
        }
        mem::swap(self, &mut new_cells);
        #[cfg(test)]
        println!("After east step: {}", self);
        new_cells.cells.iter_mut().for_each(|cell| *cell = None);
        for row in 0..self.num_rows {
            for column in 0..self.num_columns {
                let current_space = (row, column);
                let current_value = self[current_space];
                match current_value {
                    None => {}
                    Some(Direction::East) => new_cells[current_space] = current_value,
                    Some(Direction::South) => {
                        let next_space = if row + 1 == self.num_rows {
                            (0, column)
                        } else {
                            (row + 1, column)
                        };
                        #[cfg(test)]
                        println!(
                            "Found south-facing sea cucumber at {current_space:?} facing {next_space:?}"
                        );
                        if self[next_space].is_none() {
                            changed = true;
                            new_cells[current_space] = None;
                            new_cells[next_space] = current_value;
                        } else {
                            new_cells[current_space] = current_value;
                        }
                    }
                }
            }
        }
        mem::swap(self, &mut new_cells);
        #[cfg(test)]
        println!("After south step: {}", self);
        changed
    }
}

impl Display for Seafloor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for row in 0..self.num_rows {
            for column in 0..self.num_columns {
                match self.cells[row * self.num_columns + column] {
                    None => write!(f, ".")?,
                    Some(Direction::East) => write!(f, ">")?,
                    Some(Direction::South) => write!(f, "v")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Index<(usize, usize)> for Seafloor {
    type Output = Option<Direction>;

    fn index(&self, (row, column): (usize, usize)) -> &Self::Output {
        if row >= self.num_rows {
            panic!("Tried to access row {} out of {}", row, self.num_rows);
        }
        if column >= self.num_columns {
            panic!(
                "Tried to access column {} out of {}",
                column, self.num_columns
            );
        }
        &self.cells[row * self.num_columns + column]
    }
}

impl IndexMut<(usize, usize)> for Seafloor {
    fn index_mut(&mut self, (row, column): (usize, usize)) -> &mut Self::Output {
        #[cfg(test)]
        println!("Attempting to access ({row}, {column}) mutably");
        if row >= self.num_rows {
            panic!("Tried to access row {} out of {}", row, self.num_rows);
        }
        if column >= self.num_columns {
            panic!(
                "Tried to access column {} out of {}",
                column, self.num_columns
            );
        }
        &mut self.cells[row * self.num_columns + column]
    }
}

impl NomParse<&str> for Seafloor {
    fn nom_parse(input: &str) -> IResult<&str, Self> {
        let (input, first_line) = multi::many1(Cell::nom_parse)(input)?;
        let num_columns = first_line.len();
        let (remainder, lines) = sequence::terminated(
            multi::many0(sequence::preceded(
                character::line_ending,
                multi::many_m_n(num_columns, num_columns, Cell::nom_parse),
            )),
            comb::opt(character::line_ending),
        )(input)?;
        let num_rows = 1 + lines.len();
        let mut cells = first_line.into_iter().map(Cell::unwrap).collect::<Vec<_>>();
        cells.extend(lines.into_iter().flatten().map(Cell::unwrap));
        assert_eq!(cells.len(), num_rows * num_columns);
        Ok((
            remainder,
            Self {
                cells,
                num_rows,
                num_columns,
            },
        ))
    }
}

aoc_util::impl_from_str_for_nom_parse!(Seafloor);

fn part1(input: &mut dyn BufRead) -> io::Result<u32> {
    let mut buf = String::new();
    input.read_to_string(&mut buf)?;
    let mut seafloor = buf
        .parse::<Seafloor>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    for i in 1.. {
        if !seafloor.step() {
            return Ok(i);
        }
    }
    Err(io::Error::new(io::ErrorKind::Other, "Ran out of numbers"))
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2021 Day 25 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2021_25.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        "...>...\n",
        ".......\n",
        "......>\n",
        "v.....>\n",
        "......>\n",
        ".......\n",
        "..vvv..\n",
    );

    const TEST_DATA_STEP1: &str = concat!(
        "..vv>..\n",
        ".......\n",
        ">......\n",
        "v.....>\n",
        ">......\n",
        ".......\n",
        "....v..\n",
    );

    const PART1_TEST_DATA: &str = concat!(
        "v...>>.vv>\n",
        ".vv>>.vv..\n",
        ">>.>v>...v\n",
        ">>v>>.>.v.\n",
        "v>v.vv.v..\n",
        ">.>>..v...\n",
        ".vv..>.>v.\n",
        "v.v..>>v.v\n",
        "....v..v.>\n",
    );

    #[test]
    #[ignore]
    fn test_seafloor_parse() {
        let expected = Seafloor {
            cells: vec![
                None,
                None,
                None,
                Some(Direction::East),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(Direction::East),
                Some(Direction::South),
                None,
                None,
                None,
                None,
                None,
                Some(Direction::East),
                None,
                None,
                None,
                None,
                None,
                None,
                Some(Direction::East),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(Direction::South),
                Some(Direction::South),
                Some(Direction::South),
                None,
                None,
            ],
            num_rows: 7,
            num_columns: 7,
        };
        let actual = TEST_DATA.parse::<Seafloor>().unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    #[ignore]
    fn test_step() {
        let expected = TEST_DATA_STEP1.parse::<Seafloor>().unwrap();
        let mut actual = TEST_DATA.parse::<Seafloor>().unwrap();
        assert!(actual.step(), "Step failed to modify seafloor");
        assert_eq!(expected, actual);
    }

    #[test]
    #[ignore]
    fn test_part1() -> io::Result<()> {
        let expected = 58;
        let actual = part1(&mut Cursor::new(PART1_TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
