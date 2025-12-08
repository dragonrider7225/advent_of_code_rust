use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead, BufReader},
};

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    input
        .lines()
        .try_fold((0, HashSet::new()), |(num_splits, beams), line| {
            line.map(|line| {
                let line_bytes = line.as_bytes();
                let (num_splits, mut beams) = beams.into_iter().fold(
                    (num_splits, HashSet::new()),
                    |(mut num_splits, mut beams), old_beam| {
                        match line_bytes[old_beam] {
                            b'.' => {
                                beams.insert(old_beam);
                            }
                            b'^' => {
                                beams.insert(old_beam - 1);
                                beams.insert(old_beam + 1);
                                num_splits += 1;
                            }
                            b'S' => eprintln!("Found unexpected start tile in column {old_beam}"),
                            b => {
                                eprintln!("Got unexpected character '\\x{b:02x}' in tachyon manifold diagram");
                            }
                        }
                        (num_splits, beams)
                    },
                );
                if beams.is_empty() {
                    beams.extend(
                        line_bytes
                            .iter()
                            .copied()
                            .enumerate()
                            .filter_map(|(column, tile)| Some(column).filter(|_| tile == b'S'))
                    );
                }
                {
                    let line = line
                        .chars()
                        .enumerate()
                        .map(|(col, c)| {
                            if c == '.' && beams.contains(&col) {
                                '|'
                            } else {
                                c
                            }
                        })
                        .collect::<String>();
                    eprintln!("{line}");
                }
                (num_splits, beams)
            })
        })
        .map(|(num_splits, _beams)| num_splits)
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    input
        .lines()
        .try_fold(HashMap::<usize, usize>::new(), |beams, line| {
            line.map(|line| {
                let line_bytes = line.as_bytes();
                let mut beams =
                    beams
                        .into_iter()
                        .fold(HashMap::new(), |mut beams, (column, count)| {
                            match line_bytes[column] {
                                b'.' => *beams.entry(column).or_default() += count,
                                b'^' => {
                                    for delta in [-1, 1] {
                                        *beams
                                            .entry(column.strict_add_signed(delta))
                                            .or_default() += count
                                    }
                                }
                                b'S' => eprintln!("Found unexpected start tile"),
                                b => {
                                    eprintln!("Got unexpected character '\\x{b:02x}' in tachyon manifold diagram")
                                }
                            }
                            beams
                        });
                if beams.is_empty() {
                    beams.extend(
                        line_bytes
                            .iter()
                            .copied()
                            .enumerate()
                            .filter_map(|(column, tile)| {
                                Some((column, 1)).filter(|_| tile == b'S')
                            }),
                    );
                }
                beams
            })
        })
        .map(|beams| beams.values().sum())
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2025 Day 7 Part 1");
        println!(
            "{:?}",
            part1(&mut BufReader::new(File::open("2025_07.txt")?))?
        );
    }
    {
        println!("Year 2025 Day 7 Part 2");
        println!(
            "{:?}",
            part2(&mut BufReader::new(File::open("2025_07.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        ".......S.......\n",
        "...............\n",
        ".......^.......\n",
        "...............\n",
        "......^.^......\n",
        "...............\n",
        ".....^.^.^.....\n",
        "...............\n",
        "....^.^...^....\n",
        "...............\n",
        "...^.^...^.^...\n",
        "...............\n",
        "..^...^.....^..\n",
        "...............\n",
        ".^.^.^.^.^...^.\n",
        "...............\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 21;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 40;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
