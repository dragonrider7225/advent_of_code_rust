use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
    fs::File,
    io::{self, BufRead, BufReader},
};

use nom::{
    branch, bytes::complete as bytes, character::complete as character, combinator, multi,
    sequence, IResult,
};

macro_rules! say {
    ($($tokens:tt)*) => {
        #[cfg(test)]
        println!($($tokens)*)
    };
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl Spring {
    fn nom_parse(s: &str) -> IResult<&str, Self> {
        branch::alt((
            combinator::value(Self::Operational, bytes::tag(".")),
            combinator::value(Self::Damaged, bytes::tag("#")),
            combinator::value(Self::Unknown, bytes::tag("?")),
        ))(s)
    }
}

impl Display for Spring {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Operational => write!(f, "."),
            Self::Damaged => write!(f, "#"),
            Self::Unknown => write!(f, "?"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SpringRow {
    springs: Vec<Spring>,
    damaged_groups: Vec<usize>,
}

impl SpringRow {
    fn nom_parse(s: &str) -> IResult<&str, Self> {
        combinator::map(
            sequence::separated_pair(
                multi::many1(Spring::nom_parse),
                bytes::tag(" "),
                multi::separated_list1(
                    bytes::tag(","),
                    combinator::map(character::u32, |n| n as usize),
                ),
            ),
            |(springs, damaged_groups)| Self {
                springs,
                damaged_groups,
            },
        )(s)
    }

    fn is_empty(&self) -> bool {
        self.springs.is_empty() && self.damaged_groups.is_empty()
    }

    fn count_sat(&self) -> usize {
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        struct Truncation {
            springs_start_idx: usize,
            damaged_groups_start_idx: usize,
        }
        impl Display for Truncation {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(
                    f,
                    "springs from {}, damaged groups from {}",
                    self.springs_start_idx, self.damaged_groups_start_idx
                )
            }
        }
        fn inner(
            this: &SpringRow,
            memoized: &mut HashMap<Truncation, usize>,
            truncation: Truncation,
        ) -> usize {
            if let Some(&n) = memoized.get(&truncation) {
                n
            } else {
                let ret = if truncation.damaged_groups_start_idx >= this.damaged_groups.len() {
                    say!("Out of groups");
                    if truncation.springs_start_idx >= this.springs.len()
                        || this.springs[truncation.springs_start_idx..]
                            .iter()
                            .all(|&spring| spring != Spring::Damaged)
                    {
                        say!("Out of definitely-damaged springs");
                        1
                    } else {
                        say!("Still have definitely-damaged springs");
                        0
                    }
                } else if truncation.springs_start_idx
                    + this.damaged_groups[truncation.damaged_groups_start_idx]
                    > this.springs.len()
                {
                    say!("Not enough springs for first remaining group");
                    0
                } else {
                    let current_group = this.damaged_groups[truncation.damaged_groups_start_idx];
                    let first_damaged = this.springs[truncation.springs_start_idx..]
                        .iter()
                        .take(current_group)
                        .position(|&spring| spring == Spring::Damaged)
                        .map(|first_damaged| first_damaged + truncation.springs_start_idx);
                    let last_operational = this.springs[truncation.springs_start_idx..]
                        .iter()
                        .take(current_group)
                        .rposition(|&spring| spring == Spring::Operational)
                        .map(|last_operational| last_operational + truncation.springs_start_idx);
                    match (first_damaged, last_operational) {
                        (Some(first_damaged), Some(last_operational))
                            if first_damaged < last_operational =>
                        {
                            say!("There is a damaged spring preceding an operational spring in the first {} springs", this.damaged_groups[truncation.damaged_groups_start_idx]);
                            0
                        }
                        (_, Some(last_operational)) => {
                            say!("Skip the first {last_operational} springs");
                            inner(
                                this,
                                memoized,
                                Truncation {
                                    springs_start_idx: last_operational + 1,
                                    ..truncation
                                },
                            )
                        }
                        (Some(first_damaged), None) => {
                            say!("Early damaged, no early operational");
                            (truncation.springs_start_idx..=first_damaged)
                                .take_while(|&group_start| {
                                    group_start + current_group <= this.springs.len()
                                        && this.springs[group_start..]
                                            .iter()
                                            .take(current_group)
                                            .all(|&spring| spring != Spring::Operational)
                                })
                                .filter(|&group_start| {
                                    this.springs
                                        .get(group_start + current_group)
                                        .filter(|&&spring| spring == Spring::Damaged)
                                        .is_none()
                                })
                                .map(|group_start| {
                                    say!("Group anchored at {first_damaged}, starting at {group_start}");
                                    inner(
                                        this,
                                        memoized,
                                        Truncation {
                                            springs_start_idx: group_start + current_group + 1,
                                            damaged_groups_start_idx: truncation
                                                .damaged_groups_start_idx
                                                + 1,
                                        },
                                    )
                                })
                                .sum()
                        }
                        (None, None) => {
                            if this.springs.len() == current_group + truncation.springs_start_idx {
                                inner(
                                    this,
                                    memoized,
                                    Truncation {
                                        springs_start_idx: this.springs.len(),
                                        damaged_groups_start_idx: truncation
                                            .damaged_groups_start_idx
                                            + 1,
                                    },
                                )
                            } else {
                                let leader = if this.springs
                                    [truncation.springs_start_idx + current_group]
                                    != Spring::Damaged
                                {
                                    inner(
                                        this,
                                        memoized,
                                        Truncation {
                                            springs_start_idx: truncation.springs_start_idx
                                                + current_group
                                                + 1,
                                            damaged_groups_start_idx: truncation
                                                .damaged_groups_start_idx
                                                + 1,
                                        },
                                    )
                                } else {
                                    0
                                };
                                leader
                                    + inner(
                                        this,
                                        memoized,
                                        Truncation {
                                            springs_start_idx: truncation.springs_start_idx + 1,
                                            ..truncation
                                        },
                                    )
                            }
                        }
                    }
                };
                say!("{truncation} @ {this}: {ret}");
                memoized.insert(truncation, ret);
                ret
            }
        }

        let mut memoized = HashMap::new();
        inner(
            self,
            &mut memoized,
            Truncation {
                springs_start_idx: 0,
                damaged_groups_start_idx: 0,
            },
        )
    }

    #[cfg(test)]
    fn matches(&self) -> bool {
        let mut group_idx = 0;
        let mut spring_idx = 0;
        while spring_idx < self.springs.len() {
            if self.springs[spring_idx] == Spring::Damaged {
                if spring_idx != 0 && self.springs[spring_idx - 1] != Spring::Operational
                    || group_idx >= self.damaged_groups.len()
                    || self.springs.len() < spring_idx + self.damaged_groups[group_idx]
                    || self.springs[spring_idx..(spring_idx + self.damaged_groups[group_idx])]
                        .iter()
                        .any(|&spring| spring != Spring::Damaged)
                {
                    return false;
                }
                spring_idx += self.damaged_groups[group_idx];
                group_idx += 1;
            } else {
                spring_idx += 1;
            }
        }
        group_idx == self.damaged_groups.len() && spring_idx == self.springs.len()
    }

    #[cfg(test)]
    fn count_sat_brute_force(&self) -> usize {
        let mut this = self.clone();
        let unknown_indices = this
            .springs
            .iter()
            .enumerate()
            .filter_map(|(idx, &spring)| {
                if spring == Spring::Unknown {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        (0..(1usize << unknown_indices.len()))
            .filter(|i| {
                (0..unknown_indices.len()).for_each(|j| {
                    this.springs[unknown_indices[j]] = match i & (1 << j) {
                        0 => Spring::Operational,
                        _ => Spring::Damaged,
                    };
                });
                this.matches()
            })
            .count()
    }

    fn unfold(&mut self) {
        use std::{iter, mem};

        self.springs = iter::repeat(mem::take(&mut self.springs).into_iter())
            .take(5)
            .fold(vec![], |mut acc, row| {
                if !acc.is_empty() {
                    acc.push(Spring::Unknown);
                }
                acc.extend(row);
                acc
            });
        self.damaged_groups = iter::repeat(mem::take(&mut self.damaged_groups).into_iter())
            .take(5)
            .flatten()
            .collect();
    }
}

impl Display for SpringRow {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if !self.is_empty() {
            for spring in self.springs.iter() {
                write!(f, "{spring}")?;
            }
            if !self.damaged_groups.is_empty() {
                write!(f, " {}", self.damaged_groups[0])?;
                for group in &self.damaged_groups[1..] {
                    write!(f, ",{group}")?;
                }
            }
        } else {
            write!(f, "<empty row>")?;
        }
        Ok(())
    }
}

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    let mut spring_rows = input
        .lines()
        .map(|line| {
            SpringRow::nom_parse(&line?)
                .map(|(_, row)| row)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
        })
        .collect::<io::Result<Vec<_>>>()?;
    Ok(spring_rows.iter_mut().map(|row| row.count_sat()).sum())
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    let mut spring_rows = input
        .lines()
        .map(|line| {
            SpringRow::nom_parse(&line?)
                .map(|(_, row)| row)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
        })
        .collect::<io::Result<Vec<_>>>()?;
    Ok(spring_rows
        .iter_mut()
        .enumerate()
        .map(|(row_idx, row)| {
            if row_idx % 100 == 0 {
                println!("{}0% complete", row_idx / 100);
            }
            row.unfold();
            row.count_sat()
        })
        .sum())
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2023 Day 12 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2023_12.txt")?))?
        );
    }
    {
        println!("Year 2023 Day 12 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2023_12.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA_1: &str = concat!(
        "#.#.### 1,1,3\n",
        ".#...#....###. 1,1,3\n",
        ".#.###.#.###### 1,3,1,6\n",
        "####.#...#... 4,1,1\n",
        "#....######..#####. 1,6,5\n",
        ".###.##....# 3,2,1\n",
    );

    const TEST_DATA_2: &str = concat!(
        "???.### 1,1,3\n",
        ".??..??...?##. 1,1,3\n",
        "?#?#?#?#?#?#?#? 1,3,1,6\n",
        "????.#...#... 4,1,1\n",
        "????.######..#####. 1,6,5\n",
        "?###???????? 3,2,1\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 6;
        let actual = part1(&mut Cursor::new(TEST_DATA_1))?;
        assert_eq!(expected, actual);
        let expected = 21;
        let actual = part1(&mut Cursor::new(TEST_DATA_2))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 525152;
        let actual = part2(&mut Cursor::new(TEST_DATA_2))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn current_case() {
        let mut row = SpringRow {
            springs: vec![
                Spring::Unknown,
                Spring::Unknown,
                Spring::Unknown,
                Spring::Operational,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Damaged,
            ],
            damaged_groups: vec![1, 1, 3],
        };
        row.unfold();
        assert_eq!(row.count_sat(), 1);
    }

    #[test]
    fn fragile_known_good() -> io::Result<()> {
        fn parse_line(s: &str) -> IResult<&str, (u32, u32)> {
            sequence::separated_pair(character::u32, bytes::tag(" "), character::u32)(s)
        }

        let input = match File::open("../2023_12_known_good.txt") {
            Ok(f) => f,
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                eprintln!("Couldn't open input file");
                eprintln!("Invoked in {:?}", std::env::current_dir());
                return Ok(());
            }
            Err(e) => return Err(e),
        };
        let mut puzzle_input = BufReader::new(File::open("../2023_12.txt")?).lines();
        let mut last_line = 0usize;
        for line in BufReader::new(input).lines() {
            let line = line?;
            if line.as_bytes()[0] == b'/' {
                continue;
            }
            let (next_line, expected_count) = parse_line(&line)
                .map(|(_, (next_line, expected_count))| {
                    (next_line as usize, expected_count as usize)
                })
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
            let line = puzzle_input
                .by_ref()
                .take(next_line - last_line)
                .last()
                .expect("Ran out of lines in puzzle input")?;
            last_line = next_line;
            let actual = part1(&mut Cursor::new(line))?;
            assert_eq!(
                expected_count, actual,
                "Line {last_line} had unexpected count {actual}"
            );
        }
        Ok(())
    }

    #[test]
    fn print_count_sat_brute_force() -> io::Result<()> {
        let input = BufReader::new(File::open("../2023_12.txt")?);
        for (i, line) in input.lines().enumerate() {
            let line = line?;
            let spring_row = SpringRow::nom_parse(&line)
                .map(|(_, row)| row)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
            println!("{} {}", i + 1, spring_row.count_sat_brute_force());
        }
        Ok(())
    }
}
