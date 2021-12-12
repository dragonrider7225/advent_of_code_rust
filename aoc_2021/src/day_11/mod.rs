use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader},
    mem,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct Octopuses {
    energy_levels: [[u32; 10]; 10],
}

impl Octopuses {
    fn neighbors((row_idx, col_idx): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        let prev_row = row_idx.checked_sub(1);
        let curr_row = Some(row_idx);
        let next_row = Some(row_idx + 1).filter(|&row_idx| row_idx < 10);
        let prev_col = col_idx.checked_sub(1);
        let curr_col = Some(col_idx);
        let next_col = Some(col_idx + 1).filter(|&col_idx| col_idx < 10);
        [
            prev_row.and_then(|row_idx| Some((row_idx, prev_col?))),
            prev_row.and_then(|row_idx| Some((row_idx, curr_col?))),
            prev_row.and_then(|row_idx| Some((row_idx, next_col?))),
            curr_row.and_then(|row_idx| Some((row_idx, prev_col?))),
            curr_row.and_then(|row_idx| Some((row_idx, curr_col?))),
            curr_row.and_then(|row_idx| Some((row_idx, next_col?))),
            next_row.and_then(|row_idx| Some((row_idx, prev_col?))),
            next_row.and_then(|row_idx| Some((row_idx, curr_col?))),
            next_row.and_then(|row_idx| Some((row_idx, next_col?))),
        ]
        .into_iter()
        .filter_map(|x| x)
    }
}

impl Octopuses {
    fn read(input: &mut dyn BufRead) -> io::Result<Self> {
        let mut ret = Self::default();
        for (row_idx, line) in input.lines().enumerate() {
            let row = &mut ret.energy_levels[row_idx];
            let line = line?;
            for (col_idx, c) in line.char_indices() {
                row[col_idx] = c.to_digit(10).ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Invalid energy level in row {}: {:?}", row_idx, c),
                    )
                })?;
            }
        }
        Ok(ret)
    }
}

impl Octopuses {
    fn energize(&mut self, (row_idx, col_idx): (usize, usize)) -> bool {
        self.energy_levels[row_idx][col_idx] += 1;
        self.energy_levels[row_idx][col_idx] == 10
    }

    /// Returns the number of octopuses that flashed on this update.
    fn update(&mut self) -> usize {
        let mut thresholds = HashSet::new();
        let mut new_thresholds = HashSet::new();
        for pos in (0..10).flat_map(|row_idx| (0..10).map(move |col_idx| (row_idx, col_idx))) {
            if self.energize(pos) {
                new_thresholds.insert(pos);
            }
        }
        let mut tmp_thresholds = HashSet::new();
        while !new_thresholds.is_empty() {
            for pos in new_thresholds.drain() {
                thresholds.insert(pos);
                tmp_thresholds.extend(
                    Self::neighbors(pos).filter(|neighbor| {
                        !thresholds.contains(neighbor) && self.energize(*neighbor)
                    }),
                );
            }
            mem::swap(&mut new_thresholds, &mut tmp_thresholds);
        }
        let ret = thresholds.len();
        for (row_idx, col_idx) in thresholds {
            self.energy_levels[row_idx][col_idx] = 0;
        }
        ret
    }
}

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    let mut octopuses = Octopuses::read(input)?;
    Ok((0..100).map(|_| octopuses.update()).sum())
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    let mut octopuses = Octopuses::read(input)?;
    Ok((1..).find(|_: &usize| octopuses.update() == 100).unwrap())
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2021 Day 11 Part 1");
        println!(
            "There are {} flashes in the first 100 steps",
            part1(&mut BufReader::new(File::open("2021_11.txt")?))?
        );
    }
    {
        println!("Year 2021 Day 11 Part 2");
        println!(
            "The first step where all 100 octopuses flash is {}",
            part2(&mut BufReader::new(File::open("2021_11.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    #[ignore]
    fn test_part1() -> io::Result<()> {
        let s = "5483143223\n2745854711\n5264556173\n6141336146\n6357385478\n4167524645\n2176841721\n6882881134\n4846848554\n5283751526";
        let expected = 1656;
        let actual = part1(&mut Cursor::new(s))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part2() -> io::Result<()> {
        let s = "5483143223\n2745854711\n5264556173\n6141336146\n6357385478\n4167524645\n2176841721\n6882881134\n4846848554\n5283751526";
        let expected = 195;
        let actual = part2(&mut Cursor::new(s))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
