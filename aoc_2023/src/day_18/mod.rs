use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use aoc_util::geometry::Direction;
use nom::{
    branch, bytes::complete as bytes, character::complete as character, combinator, multi,
    sequence, IResult,
};

fn parse_direction(s: &str) -> IResult<&str, Direction> {
    branch::alt((
        combinator::value(Direction::Up, bytes::tag("U")),
        combinator::value(Direction::Down, bytes::tag("D")),
        combinator::value(Direction::Left, bytes::tag("L")),
        combinator::value(Direction::Right, bytes::tag("R")),
    ))(s)
}

fn calculate_area(points: &[(i64, i64)]) -> i64 {
    let (double_signed_area, perimeter) = points
        .windows(2)
        .map(|points| match points {
            &[(x1, y1), (x2, y2)] => (x1 * y2 - y1 * x2, x1.abs_diff(x2) + y1.abs_diff(y2)),
            _ => unreachable!(),
        })
        .fold((0, 0), |total, delta| {
            (total.0 + delta.0, total.1 + delta.1)
        });
    (double_signed_area.abs() + perimeter as i64) / 2 + 1
}

fn parse_instructions<F>(input: &mut dyn BufRead, mut line_parser: F) -> io::Result<Vec<(i64, i64)>>
where
    F: FnMut(&str) -> io::Result<(Direction, i64)>,
{
    input
        .lines()
        .map(|line| line_parser(&line?[..]))
        .try_fold(vec![(0, 0)], |mut acc, parts| {
            let (direction, distance) = parts?;
            let mut position = *acc.last().unwrap();
            match direction {
                Direction::Down => position.1 -= distance,
                Direction::Left => position.0 -= distance,
                Direction::Right => position.0 += distance,
                Direction::Up => position.1 += distance,
            }
            acc.push(position);
            Ok(acc)
        })
}

fn part1(input: &mut dyn BufRead) -> io::Result<i64> {
    let pit = parse_instructions(input, |s| {
        sequence::terminated(
            sequence::separated_pair(parse_direction, bytes::tag(" "), character::i64),
            sequence::delimited(
                bytes::tag(" (#"),
                multi::many_m_n(6, 6, character::one_of("0123456789abcdef")),
                bytes::tag(")"),
            ),
        )(s)
        .map(|(_, x)| x)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
    })?;
    Ok(calculate_area(&pit))
}

fn part2(input: &mut dyn BufRead) -> io::Result<i64> {
    let pit = parse_instructions(input, |s| {
        sequence::delimited(
            sequence::tuple((
                parse_direction,
                bytes::tag(" "),
                character::i64,
                bytes::tag(" (#"),
            )),
            combinator::map(
                multi::many_m_n(6, 6, character::one_of("0123456789abcdef")),
                |chars| {
                    let ret = chars.into_iter().fold(0, |acc, c| {
                        if c.is_ascii_digit() {
                            acc * 16 + (c as u8 - b'0') as i64
                        } else {
                            acc * 16 + (c as u8 - b'a' + 10) as i64
                        }
                    });
                    let direction = match ret % 16 {
                        0 => Direction::Right,
                        1 => Direction::Down,
                        2 => Direction::Left,
                        3 => Direction::Up,
                        _ => unreachable!(),
                    };
                    (direction, ret / 16)
                },
            ),
            bytes::tag(")"),
        )(s)
        .map(|(_, x)| x)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
    })?;
    Ok(calculate_area(&pit))
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2023 Day 18 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2023_18.txt")?))?
        );
    }
    {
        println!("Year 2023 Day 18 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2023_18.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        "R 6 (#70c710)\n",
        "D 5 (#0dc571)\n",
        "L 2 (#5713f0)\n",
        "D 2 (#d2c081)\n",
        "R 2 (#59c680)\n",
        "D 2 (#411b91)\n",
        "L 5 (#8ceee2)\n",
        "U 2 (#caa173)\n",
        "L 1 (#1b58a2)\n",
        "U 2 (#caa171)\n",
        "R 2 (#7807d2)\n",
        "U 3 (#a77fa3)\n",
        "L 2 (#015232)\n",
        "U 2 (#7a21e3)\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 62;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 952_408_144_115;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
