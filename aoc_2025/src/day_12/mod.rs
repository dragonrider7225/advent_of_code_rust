use std::{
    fmt::{self, Debug, Display, Formatter},
    fs::File,
    io::{self, BufRead, BufReader},
    iter,
    ops::{Add, ControlFlow, Sub},
};

#[derive(Clone, Copy, Eq, PartialEq)]
struct Position {
    row: usize,
    column: usize,
}

impl Position {
    /// Rotates the positive column axis to point in the direction of the positive row axis keeping
    /// `center` fixed.
    pub fn rotate_about(self, center: Self) -> Self {
        let delta = self - center;
        let new_delta = Delta {
            delta_row: delta.delta_column,
            delta_column: -delta.delta_row,
        };
        (center + new_delta).unwrap()
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "<{}, {}>", self.column, self.row)
    }
}

impl Add<Delta> for Position {
    type Output = Option<Self>;

    fn add(self, rhs: Delta) -> Self::Output {
        Some(Self {
            row: self.row.checked_add_signed(rhs.delta_row)?,
            column: self.column.checked_add_signed(rhs.delta_column)?,
        })
    }
}

impl Sub for Position {
    type Output = Delta;

    fn sub(self, rhs: Self) -> Self::Output {
        Delta {
            delta_row: {
                let delta_unsigned = self.row.abs_diff(rhs.row) as isize;
                if self.row < rhs.row {
                    -delta_unsigned
                } else {
                    delta_unsigned
                }
            },
            delta_column: {
                let delta_unsigned = self.column.abs_diff(rhs.column) as isize;
                if self.column < rhs.column {
                    -delta_unsigned
                } else {
                    delta_unsigned
                }
            },
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct Delta {
    delta_row: isize,
    delta_column: isize,
}

impl Debug for Delta {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "<{}, {}>", self.delta_column, self.delta_row)
    }
}

#[derive(Clone, Debug, Default, Eq)]
struct Present {
    cells: Vec<Position>,
    width: usize,
    height: usize,
}

impl Present {
    fn parse_prefix(lines: impl Iterator<Item = (usize, io::Result<String>)>) -> io::Result<Self> {
        lines
            .take_while(|(_, line)| line.as_ref().map_or_else(|_| true, |line| !line.is_empty()))
            .enumerate()
            .map(|(row_number, (line_num, line))| {
                line.and_then(|line| {
                    line.bytes()
                        .enumerate()
                        .filter_map(|(column_number, b)| match b {
                            b'#' => Some(Ok(Position {
                                row: row_number,
                                column: column_number,
                            })),
                            b'.' => None,
                            _ => Some(Err(io::Error::new(
                                io::ErrorKind::InvalidData,
                                format!("{line_num}: Invalid present character {:?}", b as char),
                            ))),
                        })
                        .collect::<io::Result<Vec<_>>>()
                })
            })
            .flat_map(|res| match res {
                Ok(row) => row.into_iter().map(Ok).collect(),
                Err(e) => vec![Err(e)],
            })
            .collect()
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn variants(&self) -> Vec<Self> {
        let mut ret = vec![];
        let mut operand = self.clone();
        for _num_flips in 0..2 {
            for _num_rotations in 0..4 {
                if !ret.contains(&operand) {
                    ret.push(operand.clone());
                }
                operand = operand.rotate();
            }
            operand = operand.flip();
        }
        ret
    }

    pub fn rotate(&self) -> Self {
        let even_width = self.width().is_multiple_of(2);
        let even_height = self.height().is_multiple_of(2);
        let center = Position {
            row: (self.height() * (1 + even_height as usize) - 1) / 2,
            column: (self.width() * (1 + even_width as usize) - 1) / 2,
        };
        let cells = self
            .cells
            .iter()
            .copied()
            .map(|cell| Position {
                column: cell.column * (1 + even_width as usize),
                ..cell
            })
            .map(|cell| Position {
                row: cell.row * (1 + even_height as usize),
                ..cell
            })
            .map(|cell| cell.rotate_about(center))
            .map(|cell| Position {
                column: cell.column / (1 + even_height as usize),
                ..cell
            })
            .map(|cell| Position {
                row: cell.row / (1 + even_width as usize),
                ..cell
            });
        Self {
            cells: cells.collect(),
            width: self.height,
            height: self.width,
        }
    }

    pub fn flip(&self) -> Self {
        let cells = self
            .cells
            .iter()
            .copied()
            .map(|cell| Position {
                column: self.width - 1 - cell.column,
                ..cell
            })
            .collect();
        Self { cells, ..*self }
    }

    /// Shifts every cell in `self` by `shift`, dropping any that go negative.
    pub fn shifted_cells(&self, shift: Delta) -> impl Iterator<Item = Position> {
        self.cells.iter().flat_map(move |&cell| cell + shift)
    }
}

impl Display for Present {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in 0..self.height() {
            for column in 0..self.width() {
                if self.cells.contains(&Position { row, column }) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl FromIterator<Position> for Present {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Position>,
    {
        iter.into_iter().fold(Self::default(), |mut acc, cell| {
            acc.width = acc.width.max(cell.column + 1);
            acc.height = acc.height.max(cell.row + 1);
            acc.cells.push(cell);
            acc
        })
    }
}

impl PartialEq for Present {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width
            && self.height == other.height
            && self.cells.len() == other.cells.len()
            && self.cells.iter().all(|cell| other.cells.contains(cell))
    }
}

#[cfg(test)]
#[derive(Debug)]
struct Presents(Vec<Present>);

#[cfg(test)]
impl Display for Presents {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.0.is_empty() {
            return Ok(());
        }
        let options = f.options();
        let mut linesed = self
            .0
            .iter()
            .map(|present| {
                let mut s = String::new();
                Display::fmt(&present, &mut options.create_formatter(&mut s))?;
                Ok(s.lines().map(str::to_string).collect::<Vec<_>>())
            })
            .filter(|lines| lines.is_err() || matches!(lines, Ok(lines) if !lines.is_empty()))
            .collect::<Result<Vec<_>, _>>()?;
        debug_assert!(linesed
            .iter()
            .all(|lines| lines.iter().all(|line| line.len() == lines[0].len())));
        debug_assert!(linesed.iter().all(|lines| lines
            .iter()
            .all(|line| matches!(line.chars().next(), Some('#' | '.')))));
        let Some(max_height) = linesed.iter().map(|lines| lines.len()).max() else {
            unreachable!(
                "We already checked at the top of the function that there is at least one present"
            );
        };
        linesed.iter_mut().for_each(|lines| {
            if lines.len() < max_height {
                let width = lines[0].len();
                let line = format!("{:.width$}", "");
                lines.extend(iter::repeat_n(line, max_height - lines.len()));
            }
        });
        let formatted = linesed
            .into_iter()
            .reduce(|mut left, right| {
                left.iter_mut().zip(right).for_each(|(left, right)| {
                    left.push(' ');
                    left.push_str(&right);
                });
                left
            })
            .unwrap()
            .into_iter()
            .map(|mut line| {
                line.push('\n');
                line
            })
            .reduce(|mut top, bottom| {
                top.push_str(&bottom);
                top
            })
            .unwrap();
        write!(f, "{formatted}")
    }
}

#[cfg(test)]
impl FromIterator<Present> for Presents {
    fn from_iter<T: IntoIterator<Item = Present>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

#[derive(Clone, Debug)]
struct Region {
    width: usize,
    height: usize,
    counts: Vec<usize>,
}

impl Region {
    fn is_possible(&self, presents: &[Present]) -> bool {
        #[derive(Clone, Copy)]
        struct Flags {
            first_checked_present_bounds: usize,
        }

        struct Board {
            cells: Vec<Vec<bool>>,
            width: usize,
            height: usize,
        }

        impl Board {
            pub fn new(width: usize, height: usize) -> Self {
                Self {
                    cells: vec![vec![false; width]; height],
                    width,
                    height,
                }
            }

            pub fn is_used(&self, cell: Position) -> bool {
                self.cells[cell.row][cell.column]
            }

            fn fill_cell(&mut self, cell: Position) {
                self.cells[cell.row][cell.column] = true;
            }

            fn unfill_cell(&mut self, cell: Position) {
                self.cells[cell.row][cell.column] = false;
            }

            pub fn try_fill(&mut self, present: &Present, shift: Delta) -> Result<(), ()> {
                for (idx, cell) in present.shifted_cells(shift).enumerate() {
                    if self.is_used(cell) {
                        present
                            .shifted_cells(shift)
                            .take(idx)
                            .for_each(|cell| self.unfill_cell(cell));
                        return Err(());
                    }
                    self.fill_cell(cell);
                }
                Ok(())
            }

            pub fn unfill(&mut self, present: &Present, shift: Delta) {
                present
                    .shifted_cells(shift)
                    .for_each(|cell| self.cells[cell.row][cell.column] = false);
            }
        }

        impl Display for Board {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                for row in &self.cells {
                    for &cell in row {
                        if cell {
                            write!(f, "#")?;
                        } else {
                            write!(f, ".")?;
                        }
                    }
                    writeln!(f)?;
                }
                Ok(())
            }
        }

        fn go(
            remaining_presents: &mut [(Vec<Present>, usize)],
            board: &mut Board,
            mut flags: Flags,
        ) -> ControlFlow<()> {
            match remaining_presents.last().cloned() {
                None => ControlFlow::Break(()),
                Some((present_variants, _)) => {
                    let present = present_variants[0].clone();
                    if flags.first_checked_present_bounds < remaining_presents.len()
                        && (present.width() > board.width || present.height() > board.height)
                        && (present.height() > board.width || present.width() > board.height)
                    {
                        // The present can't fit into a {width}x{height} area, even if it is
                        // the only present present.
                        return ControlFlow::Break(());
                    }
                    flags.first_checked_present_bounds = remaining_presents.len();
                    remaining_presents.last_mut().unwrap().1 -= 1;
                    let sub_presents = if remaining_presents.last().unwrap().1 == 0 {
                        remaining_presents.split_last_mut().unwrap().1
                    } else {
                        &mut remaining_presents[..]
                    };
                    for oriented_present in present_variants {
                        let anchor_bound = Position {
                            row: board.height + 1 - oriented_present.height,
                            column: board.width + 1 - oriented_present.width,
                        };
                        for delta_row in 0..(anchor_bound.row as isize) {
                            for delta_column in 0..(anchor_bound.column as isize) {
                                let anchor = Delta {
                                    delta_row,
                                    delta_column,
                                };
                                if board.try_fill(&oriented_present, anchor).is_err() {
                                    continue;
                                }
                                go(sub_presents, board, flags)?;
                                board.unfill(&oriented_present, anchor);
                            }
                        }
                    }
                    remaining_presents.last_mut().unwrap().1 += 1;
                    ControlFlow::Continue(())
                }
            }
        }

        let present_area = presents
            .iter()
            .zip(self.counts.iter().copied())
            .map(|(present, count)| present.cells.len() * count)
            .sum::<usize>();
        present_area <= self.width * self.height && {
            let mut remaining_presents = presents
                .iter()
                .map(|present| {
                    let variants = present.variants();
                    cfg_select! {
                        test => {
                            let presents = Presents(variants);
                            eprintln!("{presents}");
                            presents.0
                        }
                        _ => variants,
                    }
                })
                .zip(self.counts.iter().copied())
                .filter(|&(_, count)| count > 0)
                .collect::<Vec<_>>();
            let flags = Flags {
                first_checked_present_bounds: remaining_presents.len(),
            };
            go(
                &mut remaining_presents[..],
                &mut Board::new(self.width, self.height),
                flags,
            )
            .is_break()
        }
    }
}

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    let mut presents = vec![];
    let mut lines = input.lines().enumerate();
    let regions_iter;
    loop {
        let Some((line_num, line)) = lines.next() else {
            eprintln!("Input file contains no regions");
            return Ok(0);
        };
        let line = line?;
        let Some((_idx, suffix)) = line.split_once(':') else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Unexpected line {line:?}. Expected present index or region"),
            ));
        };
        if suffix.is_empty() {
            presents.push(Present::parse_prefix(&mut lines)?);
        } else {
            regions_iter = iter::once((line_num, Ok(line))).chain(&mut lines);
            break;
        }
    }
    let mut first_region_line = 0;
    regions_iter
        .map(|(line_num, line)| {
            if first_region_line == 0 {
                first_region_line = line_num;
            }
            let line = line?;
            let Some((size, counts)) = line.split_once(": ") else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Invalid region line: {line:?}"),
                ));
            };
            let Some((width, height)) = size.split_once('x') else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Invalid region size: {size:?}"),
                ));
            };
            let width = width.parse().map_err(|e| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("{line_num}: Invalid width: {width:?}: {e:?}"),
                )
            })?;
            let height = height.parse().map_err(|e| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("{line_num}: Invalid height: {height:?}: {e:?}"),
                )
            })?;
            let counts = counts
                .split_ascii_whitespace()
                .map(|count| {
                    count.parse::<usize>().map_err(|e| {
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("{line_num}: Invalid present count: {count:?}: {e:?}"),
                        )
                    })
                })
                .collect::<io::Result<Vec<_>>>()?;
            let ret = usize::from(
                Region {
                    width,
                    height,
                    counts,
                }
                .is_possible(&presents),
            );
            if (line_num - first_region_line).count_ones() == 1 {
                eprintln!("Checked {} regions", line_num - first_region_line);
            }
            Ok(ret)
        })
        .sum()
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2025 Day 12 Part 1");
        println!(
            "{:?}",
            part1(&mut BufReader::new(File::open("2025_12.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_present_rotation() {
        const PRESENT: &str = concat!(".##\n", "##.\n", ".#.\n");
        const ROTATED_PRESENT: &str = concat!(".#.\n", "###\n", "..#\n");

        fn parse_present(s: &str) -> io::Result<Present> {
            Present::parse_prefix(s.lines().map(str::to_string).map(Ok).enumerate())
        }

        let present = parse_present(PRESENT).unwrap();
        let rotated_present = parse_present(ROTATED_PRESENT).unwrap();
        assert_eq!(rotated_present, present.rotate());
    }

    #[test]
    fn test_part1() -> io::Result<()> {
        const TEST_DATA: &str = concat!(
            "0:\n",
            "###\n",
            "##.\n",
            "##.\n",
            "\n",
            "1:\n",
            "###\n",
            "##.\n",
            ".##\n",
            "\n",
            "2:\n",
            ".##\n",
            "###\n",
            "##.\n",
            "\n",
            "3:\n",
            "##.\n",
            "###\n",
            "##.\n",
            "\n",
            "4:\n",
            "###\n",
            "#..\n",
            "###\n",
            "\n",
            "5:\n",
            "###\n",
            ".#.\n",
            "###\n",
            "\n",
            "4x4: 0 0 0 0 2 0\n",
            "12x5: 1 0 1 0 2 2\n",
            "12x5: 1 0 1 0 3 2\n",
        );

        let expected = 2;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
