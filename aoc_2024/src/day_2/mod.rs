use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use aoc_util::{
    impl_from_str_for_nom_parse,
    nom::{bytes::complete as bytes, character::complete as character, multi, IResult, Parser},
    nom_extended::NomParse,
};

#[derive(Clone, Debug)]
struct Report {
    levels: Vec<u32>,
}

impl NomParse<&str> for Report {
    fn nom_parse(input: &str) -> IResult<&str, Self> {
        multi::separated_list1(bytes::tag(" "), character::u32)
            .map(|levels| Self { levels })
            .parse(input)
    }
}

impl_from_str_for_nom_parse!(Report);

fn safe_report(levels: &[u32]) -> bool {
    let monotonic = if levels[0] < levels[1] {
        levels.windows(2).all(|window| window[0] < window[1])
    } else {
        levels.windows(2).all(|window| window[0] > window[1])
    };
    monotonic
        && levels
            .windows(2)
            .all(|window| window[0].abs_diff(window[1]) < 4)
}

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    let reports = input
        .lines()
        .map(|line| {
            line?
                .parse::<Report>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        })
        .collect::<io::Result<Vec<_>>>()?;
    Ok(reports
        .iter()
        .filter(|report| safe_report(&report.levels))
        .count())
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    let reports = input
        .lines()
        .map(|line| {
            line?
                .parse::<Report>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        })
        .collect::<io::Result<Vec<_>>>()?;
    Ok(reports
        .iter()
        .filter(|report| {
            (0..report.levels.len())
                .map(|i| {
                    report.levels[..i]
                        .iter()
                        .chain(&report.levels[(i + 1)..])
                        .copied()
                        .collect::<Vec<_>>()
                })
                .any(|levels| safe_report(&levels))
        })
        .count())
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2024 Day 2 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2024_02.txt")?))?
        );
    }
    {
        println!("Year 2024 Day 2 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2024_02.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        "7 6 4 2 1\n",
        "1 2 7 8 9\n",
        "9 7 6 2 1\n",
        "1 3 2 4 5\n",
        "8 6 4 4 1\n",
        "1 3 6 7 9\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 2;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 4;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
