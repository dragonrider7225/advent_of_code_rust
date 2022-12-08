use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

type TreeHeight = u8;

#[derive(Clone, Copy, Debug, Default)]
struct MaxHeights {
    north: TreeHeight,
    south: TreeHeight,
    west: TreeHeight,
    east: TreeHeight,
}

#[derive(Clone, Copy, Debug, Default)]
struct ViewDistances {
    north: usize,
    south: usize,
    east: usize,
    west: usize,
}

#[derive(Clone, Debug, Default)]
struct Forest {
    rows: Vec<Vec<(TreeHeight, MaxHeights, ViewDistances)>>,
}

impl Forest {
    fn num_visible_trees(&self) -> usize {
        self.rows
            .iter()
            .flatten()
            .copied()
            .filter(|&(height, heights, ..)| {
                height > heights.north
                    || height > heights.south
                    || height > heights.east
                    || height > heights.west
            })
            .count()
    }

    fn max_scenic_score(&self) -> usize {
        self.rows
            .iter()
            .flatten()
            .map(|(_, _, views)| views.north * views.south * views.east * views.west)
            .max()
            .unwrap_or(0)
    }

    fn add_row(&mut self, mut row: Vec<TreeHeight>) {
        // Fix the length of `row` if it doesn't match the extant rows.
        if !self.rows.is_empty() {
            if row.len() < self.rows[0].len() {
                row.extend(std::iter::repeat(1).take(self.rows[0].len() - row.len()));
            } else {
                row.truncate(self.rows[0].len());
            }
        }
        // Update the `MaxHeights` for the extant rows to account for the new row south of them
        // all.
        for i in 0..self.rows.len() {
            let check_row = &mut self.rows[i];
            for j in 0..check_row.len() {
                let old_south = check_row[j].1.south;
                if old_south < row[j] {
                    check_row[j].1.south = row[j];
                }
            }
        }
        // Update the `ViewDistances` for the extant rows to account for the new row south of them
        // all.
        for i in 0..self.rows.len() {
            for j in 0..self.rows[i].len() {
                if self.rows[i][j].2.south + i + 1 == self.rows.len()
                    && (i + 1 == self.rows.len()
                        || self.rows.last().unwrap()[j].0 < self.rows[i][j].0)
                {
                    self.rows[i][j].2.south += 1;
                }
            }
        }
        // Add the metadata to the new row.
        let mut actual_row: Vec<(TreeHeight, MaxHeights, ViewDistances)> = vec![];
        for (j, next_height) in row.iter().copied().enumerate() {
            // Build the `MaxHeights` for column `j` of the new row.
            let max_heights = MaxHeights {
                north: self
                    .rows
                    .last()
                    .map(|row| row[j].0.max(row[j].1.north))
                    .unwrap_or(0),
                south: 0,
                west: actual_row
                    .last()
                    .map(|&(height, heights, ..)| height.max(heights.west))
                    .unwrap_or(0),
                east: 0,
            };
            // Update the `MaxHeights` for the columns `0..j` to account for column `j`.
            for heights in actual_row.iter_mut().map(|(_, heights, ..)| heights) {
                heights.east = heights.east.max(next_height);
            }
            // Build the `ViewDistances` for column `j` of the new row.
            let view_distances = ViewDistances {
                north: if self.rows.is_empty() {
                    0
                } else {
                    let num_rows = self.rows.len();
                    self.rows
                        .iter()
                        .map(|row| row[j].0)
                        .enumerate()
                        .rev()
                        .find_map(|(i, height_ij)| {
                            if height_ij >= next_height {
                                Some(num_rows - i)
                            } else {
                                None
                            }
                        })
                        .unwrap_or(num_rows)
                },
                south: 0,
                east: 0,
                west: if j == 0 {
                    0
                } else {
                    actual_row
                        .iter()
                        .map(|&(height_ij1, ..)| height_ij1)
                        .enumerate()
                        .rev()
                        .find_map(|(j1, height_ij1)| {
                            if height_ij1 >= next_height {
                                Some(j - j1)
                            } else {
                                None
                            }
                        })
                        .unwrap_or(j)
                },
            };
            // Update the `ViewDistances` for columns `0..j` to account for column `j`.
            if let Some(&(last_height, ..)) = actual_row.last() {
                actual_row
                    .iter_mut()
                    .map(|(height_ij1, _, view_distances, ..)| (*height_ij1, view_distances))
                    .enumerate()
                    .filter_map(|(j1, (height_ij1, view_distances))| {
                        if j1 + 1 == j
                            || view_distances.east + j1 + 1 == j && last_height < height_ij1
                        {
                            Some(view_distances)
                        } else {
                            None
                        }
                    })
                    .for_each(|view_distances| view_distances.east += 1);
            }
            actual_row.push((next_height, max_heights, view_distances));
        }
        self.rows.push(actual_row);
    }
}

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    let forest = input
        .lines()
        .fold(Ok(Forest::default()), |forest: io::Result<_>, line| {
            let mut forest = forest?;
            forest.add_row(line?.bytes().map(|b| b - b'0' + 1).collect());
            Ok(forest)
        })?;
    Ok(forest.num_visible_trees())
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    let forest = input
        .lines()
        .fold(Ok(Forest::default()), |forest: io::Result<_>, line| {
            let mut forest = forest?;
            forest.add_row(line?.bytes().map(|b| b - b'0' + 1).collect());
            Ok(forest)
        })?;
    Ok(forest.max_scenic_score())
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2022 Day 8 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2022_08.txt")?))?
        );
    }
    {
        println!("Year 2022 Day 8 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2022_08.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = "30373\n25512\n65332\n33549\n35390\n";

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 21;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 8;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
