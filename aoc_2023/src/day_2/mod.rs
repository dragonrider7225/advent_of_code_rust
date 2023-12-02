use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use nom::{
    branch, bytes::complete as bytes, character::complete as character, combinator, multi, sequence,
};

#[derive(Clone, Copy, Default)]
struct Reveal {
    red: u32,
    green: u32,
    blue: u32,
}

impl Reveal {
    fn is_possible(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }

    fn nom_parse(s: &str) -> nom::IResult<&str, Self> {
        combinator::map(
            multi::separated_list1(
                bytes::tag(", "),
                sequence::separated_pair(
                    character::u32,
                    bytes::tag(" "),
                    branch::alt((bytes::tag("red"), bytes::tag("green"), bytes::tag("blue"))),
                ),
            ),
            |groups| {
                let mut res = Self::default();
                for group in groups {
                    match group {
                        (red, "red") => res.red = red,
                        (green, "green") => res.green = green,
                        (blue, "blue") => res.blue = blue,
                        _ => {}
                    }
                }
                res
            },
        )(s)
    }
}

fn parse_game(line: &str) -> io::Result<(u32, Vec<Reveal>)> {
    sequence::tuple((
        sequence::delimited(bytes::tag("Game "), character::u32, bytes::tag(": ")),
        multi::separated_list1(bytes::tag("; "), Reveal::nom_parse),
    ))(line)
    .map(|(_, res)| res)
    .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{e:?}")))
}

fn part1(input: &mut dyn BufRead) -> io::Result<u32> {
    input
        .lines()
        .map(|line| parse_game(&(line?)))
        .filter_map(|elem| {
            let (game, reveals) = match elem {
                Ok(t) => t,
                Err(e) => return Some(Err(e)),
            };
            if reveals.into_iter().all(|reveal| reveal.is_possible()) {
                Some(Ok(game))
            } else {
                None
            }
        })
        .try_fold(0, |acc, game| Ok(acc + game?))
}

fn part2(input: &mut dyn BufRead) -> io::Result<u32> {
    input
        .lines()
        .map(|line| parse_game(&(line?)))
        .map::<io::Result<_>, _>(|game| {
            let (_, reveals) = game?;
            Ok(reveals
                .into_iter()
                .fold(Reveal::default(), |mut acc, reveal| {
                    acc.red = acc.red.max(reveal.red);
                    acc.green = acc.green.max(reveal.green);
                    acc.blue = acc.blue.max(reveal.blue);
                    acc
                }))
        })
        .try_fold(0, |acc, cubes| Ok(acc + cubes?.power()))
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2023 Day 2 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2023_02.txt")?))?
        );
    }
    {
        println!("Year 2023 Day 2 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2023_02.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n",
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n",
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n",
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n",
        "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 8;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 2286;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
