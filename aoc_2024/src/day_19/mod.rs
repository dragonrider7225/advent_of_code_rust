use std::{
    collections::{hash_map::Entry, BinaryHeap, HashMap, HashSet},
    fmt::{self, Display, Formatter},
    fs::File,
    io::{self, BufRead, BufReader},
};

use aoc_util::{
    impl_from_str_for_nom_parse,
    nom::{bytes::complete as bytes, character::complete as character, multi, IResult, Parser},
    nom_extended::NomParse,
    nom_supreme::ParserExt,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::White => write!(f, "w"),
            Self::Blue => write!(f, "u"),
            Self::Black => write!(f, "b"),
            Self::Red => write!(f, "r"),
            Self::Green => write!(f, "g"),
        }
    }
}

impl NomParse<&str> for Color {
    fn nom_parse(input: &str) -> IResult<&str, Self> {
        // White, Blue, Black, Red, Green,
        bytes::tag("w")
            .map(|_| Self::White)
            .or(bytes::tag("u").map(|_| Self::Blue))
            .or(bytes::tag("b").map(|_| Self::Black))
            .or(bytes::tag("r").map(|_| Self::Red))
            .or(bytes::tag("g").map(|_| Self::Green))
            .parse(input)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash)]
struct Colors<'c>(&'c [Color]);

impl Display for Colors<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for color in self.0 {
            write!(f, "{color}")?;
        }
        Ok(())
    }
}

impl Ord for Colors<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.len().cmp(&other.0.len())
    }
}

impl PartialEq for Colors<'_> {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.cmp(other), std::cmp::Ordering::Equal)
    }
}

impl PartialOrd for Colors<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct Input {
    available: Vec<Vec<Color>>,
    targets: Vec<Vec<Color>>,
}

impl NomParse<&str> for Input {
    fn nom_parse(input: &str) -> IResult<&str, Self> {
        multi::separated_list1(bytes::tag(", "), multi::many1(Color::nom_parse))
            .terminated(multi::count(character::line_ending, 2))
            .and(multi::many1(
                multi::many1(Color::nom_parse).terminated(character::line_ending),
            ))
            .map(|(available, targets)| Self { available, targets })
            .parse(input)
    }
}

impl_from_str_for_nom_parse!(Input);

fn has_towel_match(target: &[Color], towels: &[Vec<Color>]) -> bool {
    let mut tails = HashSet::<_>::from_iter([target]);
    let mut past_tails = tails.clone();

    while !tails.is_empty() {
        if tails.contains(&[][..]) {
            return true;
        }
        tails = tails
            .into_iter()
            .flat_map(|tail| {
                towels
                    .iter()
                    .filter(|towel| towel.len() <= tail.len())
                    .filter(|towel| towel.iter().zip(tail).all(|(towel, tail)| towel == tail))
                    .map(|towel| &tail[towel.len()..])
                    .filter(|&new_tail| past_tails.insert(new_tail))
                    .collect::<Vec<_>>()
            })
            .collect();
    }
    false
}

fn count_towel_matches(target: &[Color], towels: &[Vec<Color>]) -> usize {
    let mut tails = [target]
        .into_iter()
        .map(Colors)
        .collect::<BinaryHeap<Colors>>();
    let mut past_tails = tails
        .iter()
        .map(|&tail| (tail.0, 1))
        .collect::<HashMap<_, _>>();

    #[cfg(test)]
    eprintln!("Matching pattern {}", Colors(target));
    #[cfg(test)]
    let mut num_steps = 0;

    while let Some(Colors(tail)) = tails.pop() {
        tails.extend(
            towels
                .iter()
                .filter(|towel| towel.len() <= tail.len())
                .filter(|towel| towel.iter().zip(tail).all(|(towel, tail)| towel == tail))
                .map(|towel| &tail[towel.len()..])
                .filter(|&new_tail| {
                    let count = past_tails[tail];
                    match past_tails.entry(new_tail) {
                        Entry::Occupied(entry) => {
                            #[cfg(test)]
                            eprintln!("Found {count} more ways to get to {}", Colors(new_tail));
                            *entry.into_mut() += count;
                            false
                        }
                        Entry::Vacant(entry) => {
                            #[cfg(test)]
                            eprintln!("Found {count} ways to get to {}", Colors(new_tail));
                            entry.insert(count);
                            true
                        }
                    }
                })
                .map(Colors),
        );
        #[cfg(test)]
        {
            num_steps += 1;
            eprintln!("step {num_steps}:");
            eprint!("    tails: [");
            for &tail in &tails {
                eprint!("{},", tail);
            }
            eprintln!("]");
            eprintln!("    past_tails: {{");
            for (&tail, &paths) in &past_tails {
                eprintln!("        {}: {paths},", Colors(tail));
            }
            eprintln!("    }}");
        }
    }
    let ret = past_tails.get(&[][..]).copied().unwrap_or_default();
    #[cfg(test)]
    eprintln!(
        "Found {ret} ways to match {} with the available towels",
        Colors(target),
    );
    ret
}

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    let Input { available, targets } = io::read_to_string(input)?
        .parse()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(targets
        .into_iter()
        .enumerate()
        .filter(|(_, target)| has_towel_match(target, &available))
        .count())
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    let Input { available, targets } = io::read_to_string(input)?
        .parse()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(targets
        .into_iter()
        .map(|target| count_towel_matches(&target, &available))
        .sum())
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2024 Day 19 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2024_19.txt")?))?
        );
    }
    {
        println!("Year 2024 Day 19 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2024_19.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        "r, wr, b, g, bwu, rb, gb, br\n",
        "\n",
        "brwrr\n",
        "bggr\n",
        "gbbr\n",
        "rrbgbr\n",
        "ubwu\n",
        "bwurrg\n",
        "brgr\n",
        "bbrgwb\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 6;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 16;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
