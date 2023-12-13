use std::{
    fmt::{self, Display, Formatter},
    fs::File,
    io::{self, BufRead, BufReader},
};

use nom::{
    branch, bytes::complete as bytes, character::complete as character, combinator, multi,
    sequence, IResult,
};

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

    fn required_springs(&self) -> usize {
        if self.damaged_groups.is_empty() {
            0
        } else {
            self.damaged_groups.len() - 1 + self.damaged_groups.iter().sum::<usize>()
        }
    }

    fn count_sat(&self) -> usize {
        fn num_weak_compositions(n: usize, max_parts: usize) -> usize {
            fn num_k_compositions(n: usize, k: usize) -> usize {
                if k > n {
                    0
                } else {
                    choose(n - 1, k - 1)
                }
            }

            fn choose(n: usize, k: usize) -> usize {
                if n - k < k {
                    choose(n, n - k)
                } else {
                    (1..=k).fold(1, |acc, x| acc * (n - k) / x + acc)
                }
            }

            if max_parts == 0 {
                0
            } else if n == 0 {
                1
            } else {
                // A k-composition is a composition with exactly `k` positive terms. Since we are
                // counting weak compositions with up to `max_parts` terms, we need to add the
                // k-compositions for all k in `1..=max_parts` with the appropriate multiplicity,
                // derived from the number of ways to have `max_parts - k` empty "buckets".
                (1..=max_parts)
                    .map(|k| choose(max_parts, k) * num_k_compositions(n, k))
                    .sum()
            }
        }

        let ret = if self.damaged_groups.is_empty() {
            if self
                .springs
                .iter()
                .all(|spring| !matches!(spring, Spring::Damaged))
            {
                1
            } else {
                0
            }
        } else if self.required_springs() > self.springs.len() {
            0
        } else if self
            .springs
            .iter()
            .all(|spring| matches!(spring, Spring::Unknown))
        {
            let num_undamaged = self.springs.len() - self.required_springs();
            let num_groups = self.damaged_groups.len() + 1;
            num_weak_compositions(num_undamaged, num_groups)
        } else if self.damaged_groups.len() == 1 && !self.springs.contains(&Spring::Operational) {
            self.springs.len() - self.damaged_groups[0] + 1
        } else {
            let num_leading_unknowns = self
                .springs
                .iter()
                .take_while(|spring| matches!(spring, Spring::Unknown))
                .count();
            let num_leading_possible = self
                .springs
                .iter()
                .take_while(|spring| !matches!(spring, Spring::Operational))
                .count();
            let leading_group_len = self.damaged_groups[0];
            (leading_group_len..=num_leading_possible.min(num_leading_unknowns + leading_group_len))
                .map(|num_removed| {
                    let mut sub = Self {
                        springs: self.springs[num_removed..].to_owned(),
                        damaged_groups: self.damaged_groups[1..].to_owned(),
                    };
                    if let Some(spring) = sub.springs.first_mut() {
                        *spring = Spring::Operational;
                    }
                    sub.apply_by_inspection();
                    sub.count_sat()
                })
                .sum()
        };
        ret
    }

    fn trim_leading_operational(&mut self) {
        for i in 0..self.springs.len() {
            if !matches!(self.springs[i], Spring::Operational) {
                self.springs.drain(..i);
                return;
            }
        }
        self.springs.clear();
    }

    fn trim_leading_groups(&mut self) {
        self.trim_leading_operational();
        for i in 0..self.damaged_groups.len() {
            let group_len = self.damaged_groups[i];
            loop {
                // Remove any springs that *must* precede the first group of damaged springs
                let mut should_break = true;
                let num_damaged = self
                    .springs
                    .iter()
                    .skip(group_len)
                    .take_while(|spring| matches!(spring, Spring::Damaged))
                    .count();
                self.springs.drain(..num_damaged);
                if num_damaged > 0 {
                    should_break = false;
                }
                if let Some(undamaged_idx) = self
                    .springs
                    .iter()
                    .take(group_len)
                    .rposition(|spring| matches!(spring, Spring::Operational))
                {
                    should_break = false;
                    self.springs.drain(..=undamaged_idx);
                }
                if should_break {
                    break;
                }
            }
            if self.springs.len() < group_len {
                self.springs.clear();
                return;
            }
            if self.springs.len() == group_len {
                self.springs.clear();
                self.damaged_groups.drain(..=i);
                return;
            } else if matches!(self.springs.first(), Some(Spring::Damaged))
                || self.springs[..group_len]
                    .iter()
                    .any(|spring| matches!(spring, Spring::Damaged))
                    && matches!(self.springs[group_len], Spring::Operational)
            {
                // The first group of damaged springs is definitely shoved up against the
                // beginning of the row of springs.
                self.springs.drain(..group_len);
                if let Some(spring) = self.springs.first_mut() {
                    *spring = Spring::Operational;
                }
                self.trim_leading_operational();
                continue;
            } else if let Some(first_damaged) = self
                .springs
                .iter()
                .take(group_len)
                .position(|spring| matches!(spring, Spring::Damaged))
            {
                // The first group of damaged springs definitely includes at least one of the
                // first `group_len` springs, so we can confirm that all of the first
                // `group_len` springs that follow the damaged spring are also damaged.
                for spring in self.springs[first_damaged..group_len].iter_mut() {
                    *spring = Spring::Damaged;
                }
            }
            // Keep going only if a group of damaged springs was removed.
            self.damaged_groups.drain(..i);
            return;
        }
        self.damaged_groups.clear();
    }

    fn trim_trailing_operational(&mut self) {
        for i in (0..self.springs.len()).rev() {
            if !matches!(self.springs[i], Spring::Operational) {
                self.springs.truncate(i + 1);
                return;
            }
        }
        self.springs.clear();
    }

    fn trim_trailing_groups(&mut self) {
        self.trim_trailing_operational();
        for i in (0..self.damaged_groups.len()).rev() {
            debug_assert_eq!(i + 1, self.damaged_groups.len());
            let group_len = self.damaged_groups[i];
            loop {
                // Remove all springs that *must* follow the last group of damaged springs
                let mut should_break = true;
                let num_damaged = self
                    .springs
                    .iter()
                    .rev()
                    .skip(group_len)
                    .take_while(|spring| matches!(spring, Spring::Damaged))
                    .count();
                self.springs.drain((self.springs.len() - num_damaged)..);
                if num_damaged > 0 {
                    should_break = false;
                }
                self.trim_trailing_operational();
                if let Some(operational_idx) = self
                    .springs
                    .iter()
                    .rev()
                    .take(group_len)
                    .rposition(|spring| matches!(spring, Spring::Operational))
                    .map(|idx| self.springs.len() - 1 - idx)
                {
                    self.springs.drain(operational_idx..);
                    should_break = false;
                }
                if should_break {
                    break;
                }
            }
            if self.springs.len() < group_len {
                self.springs.clear();
                return;
            }
            if self.springs.len() == group_len {
                self.springs.clear();
                self.damaged_groups.remove(i);
                return;
            } else if matches!(self.springs.last(), Some(Spring::Damaged))
                || self.springs[(self.springs.len() - group_len)..]
                    .iter()
                    .any(|spring| matches!(spring, Spring::Damaged))
                    && matches!(
                        self.springs[self.springs.len() - 1 - group_len],
                        Spring::Operational
                    )
            {
                // The last group of damaged springs is definitely shoved up against the end of
                // the row of springs.
                self.springs.truncate(self.springs.len() - group_len);
                if let Some(spring) = self.springs.last_mut() {
                    *spring = Spring::Operational;
                }
                self.trim_trailing_operational();
                self.damaged_groups.remove(i);
                continue;
            } else if let Some(last_damaged) = self
                .springs
                .iter()
                .rev()
                .take(group_len)
                .position(|spring| matches!(spring, Spring::Damaged))
            {
                // The last group of damaged springs definitely includes at least one of the
                // last `group_len` springs, so we can confirm that all of the last `group_len`
                // springs that precede the damaged spring are also damaged.
                let last_damaged = self.springs.len() - 1 - last_damaged;
                let last_group_start = self.springs.len() - group_len;
                for spring in self.springs[last_group_start..last_damaged].iter_mut() {
                    *spring = Spring::Damaged;
                }
            }
            // Keep going only if a group of damaged springs was removed (reducing the length of
            // the list of groups of damaged springs)
            break;
        }
    }

    /// Applies all transformations that "by inspection" do not change the number of possible ways
    /// to resolve the unknown springs:
    /// * Converts unknown springs adjacent to known complete groups of damaged springs into
    ///   operational springs
    /// * Trims leading and trailing groups of operational springs
    /// * Trims leading and trailing complete groups of damaged springs
    /// * Replaces each group of multiple consecutive operational springs with a single operational
    ///   spring
    fn apply_by_inspection(&mut self) {
        self.trim_trailing_groups();
        self.trim_leading_groups();
        if !self.damaged_groups.is_empty()
            && self.damaged_groups[1..]
                .iter()
                .fold(self.damaged_groups[0], |acc, group_len| acc + 1 + group_len)
                == self.springs.len()
        {
            self.damaged_groups.clear();
            self.springs.clear();
        }
        if self.damaged_groups.is_empty()
            && self
                .springs
                .iter()
                .all(|spring| !matches!(spring, Spring::Damaged))
        {
            self.springs.clear();
        }
        if let [group_len] = self.damaged_groups[..] {
            if let Some(last_damaged) = self
                .springs
                .iter()
                .rposition(|spring| matches!(spring, Spring::Damaged))
            {
                let mut changed = false;
                if last_damaged < self.springs.len() - group_len {
                    self.springs.drain((last_damaged + group_len)..);
                    changed = true;
                }
                let first_damaged = self
                    .springs
                    .iter()
                    .position(|spring| matches!(spring, Spring::Damaged))
                    .unwrap();
                if first_damaged >= group_len {
                    self.springs.drain(..(first_damaged - (group_len - 1)));
                    changed = true;
                }
                if changed {
                    self.apply_by_inspection();
                }
            }
        }
        for i in (1..self.springs.len()).rev() {
            if let (Spring::Operational, Spring::Operational) =
                (self.springs[i - 1], self.springs[i])
            {
                self.springs.remove(i);
            }
        }
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
    Ok(spring_rows
        .iter_mut()
        .map(|row| {
            row.apply_by_inspection();
            row.count_sat()
        })
        .sum())
}

fn part2(_input: &mut dyn BufRead) -> io::Result<()> {
    todo!("Year 2023 Day 12 Part 2")
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
            "{:?}",
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
    fn random_sample() -> io::Result<()> {
        const TEST_DATA: &str = concat!(
            "??#??#?.??? 6,2\n",
            "??##???##???#.#?.?.# 11,1,1,1,1\n",
            "?#????.#??.????#.? 4,1,3,2,1,1\n",
            "???#?????.#..??#???? 8,1,4\n",
            "??#???#??????#.????? 12,3\n",
            "???.???##?##???.???? 10,1\n",
            ".???#??????#???.???# 5,5,1,4\n",
            "??.????##..?#.??# 1,3,2,2,2\n",
            "?.?????.????#? 4,2,1\n",
            "?????????? 1,1,1\n",
            ".??????#?????.# 1,1,5,1,1\n",
            "????###???#???? 1,9,1\n",
            "?#??#?#?????#??..?. 7,3\n",
            "?.?#.?.#???..? 1,4\n",
        );
        let expected = 4 + 1 + 2 + 6 + 9 + 8 + 4 + 2 + 4 + 56 + 5 + 10 + 6 + 1;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn current_case() {
        let mut row = SpringRow {
            springs: vec![
                Spring::Unknown,
                Spring::Operational,
                Spring::Unknown,
                Spring::Damaged,
                Spring::Operational,
                Spring::Unknown,
                Spring::Operational,
                Spring::Damaged,
                Spring::Unknown,
                Spring::Unknown,
                Spring::Unknown,
                Spring::Operational,
                Spring::Operational,
                Spring::Unknown,
            ],
            damaged_groups: vec![1, 4],
        };
        row.apply_by_inspection();
        assert_eq!(row.count_sat(), 1);
    }
}
