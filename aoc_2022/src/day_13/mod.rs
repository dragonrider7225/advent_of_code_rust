use std::{
    cmp::Ordering,
    fmt::{self, Debug, Display, Formatter},
    fs::File,
    io::{self, BufRead, BufReader},
    num::ParseIntError,
    str::FromStr,
};

use nom::{branch, character::complete as character, combinator as comb, multi, sequence};

#[derive(Clone, Eq, PartialEq)]
enum Packet {
    List(Vec<Self>),
    Int(u32),
}

// TODO: Switch to NomParse trait when it's moved into aoc_util
impl Packet {
    fn nom_parse<'s>(i: &'s str) -> nom::IResult<&'s str, Self> {
        branch::alt((
            comb::map(
                sequence::delimited(
                    character::char('['),
                    multi::separated_list0(character::char(','), Self::nom_parse),
                    character::char(']'),
                ),
                Self::List,
            ),
            comb::map_res(character::digit1, |s: &'s str| {
                Result::<_, ParseIntError>::Ok(Self::Int(s.parse()?))
            }),
        ))(i)
    }
}

impl Debug for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Packet({self})")
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(x) => write!(f, "{x}"),
            Self::List(xs) => match xs.len() {
                0 => write!(f, "[]"),
                1 => write!(f, "[{}]", &xs[0]),
                n => {
                    write!(f, "[{}", &xs[0])?;
                    for packet in &xs[1..n] {
                        write!(f, ",{packet}")?;
                    }
                    write!(f, "]")
                }
            },
        }
    }
}

impl FromStr for Packet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ::nom::Finish;

        Self::nom_parse(s)
            .finish()
            .map(|(_, res)| res)
            .map_err(|error| format!("{error:?}"))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Int(x), Self::Int(y)) => x.cmp(y),
            (Self::List(xs), Self::List(ys)) => xs.cmp(ys),
            (Self::Int(_), Self::List(_)) => Self::List(vec![self.clone()]).cmp(other),
            (Self::List(_), Self::Int(_)) => self.cmp(&Self::List(vec![other.clone()])),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(input: &mut dyn BufRead) -> io::Result<i32> {
    let mut total_correct = 0;
    let mut lines = input.lines();
    for i in 1.. {
        if let Some(line) = lines.next() {
            let first = line?
                .parse::<Packet>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            let second = lines
                .next()
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidData, "Missing second packet")
                })??
                .parse()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            if first < second {
                total_correct += i
            }
            match lines.next() {
                None => return Ok(total_correct),
                Some(Ok(line)) if line.is_empty() => {}
                Some(Ok(line)) => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Unexpected non-blank line {line:?}"),
                    ))
                }
                Some(Err(e)) => return Err(e),
            }
        } else {
            return Ok(total_correct);
        }
    }
    unreachable!("Too many lines")
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    let mut packets = input
        .lines()
        .filter_map(|line| {
            let line = match line {
                Ok(line) if line.is_empty() => return None,
                Ok(line) => line,
                Err(e) => return Some(Err(e)),
            };
            Some(
                line.parse::<Packet>()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)),
            )
        })
        .collect::<io::Result<Vec<_>>>()?;
    let first_divider = "[[2]]".parse::<Packet>().unwrap();
    let second_divider = "[[6]]".parse::<Packet>().unwrap();
    packets.extend([first_divider.clone(), second_divider.clone()]);
    packets.sort_unstable();
    let mut packets = packets.into_iter().enumerate();
    let first_divider = 1 + packets
        .find_map(|(idx, packet)| {
            if packet == first_divider {
                Some(idx)
            } else {
                None
            }
        })
        .unwrap();
    let second_divider = 1 + packets
        .find_map(|(idx, packet)| {
            if packet == second_divider {
                Some(idx)
            } else {
                None
            }
        })
        .unwrap();
    Ok(first_divider * second_divider)
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2022 Day 13 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2022_13.txt")?))?
        );
    }
    {
        println!("Year 2022 Day 13 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2022_13.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        "[1,1,3,1,1]\n",
        "[1,1,5,1,1]\n",
        "\n",
        "[[1],[2,3,4]]\n",
        "[[1],4]\n",
        "\n",
        "[9]\n",
        "[[8,7,6]]\n",
        "\n",
        "[[4,4],4,4]\n",
        "[[4,4],4,4,4]\n",
        "\n",
        "[7,7,7,7]\n",
        "[7,7,7]\n",
        "\n",
        "[]\n",
        "[3]\n",
        "\n",
        "[[[]]]\n",
        "[[]]\n",
        "\n",
        "[1,[2,[3,[4,[5,6,7]]]],8,9]\n",
        "[1,[2,[3,[4,[5,6,0]]]],8,9]\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 13;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 140;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
