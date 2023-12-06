use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use nom::{
    bytes::complete as bytes, character::complete as character, combinator, multi, sequence, Parser,
};

macro_rules! read_line {
    ($input:ident) => {{
        let mut line = String::new();
        $input.read_line(&mut line)?;
        io::Result::Ok(line)
    }};
}

fn nom_parse_nums(header: &str) -> impl Parser<&str, Vec<f64>, nom::error::Error<&str>> {
    sequence::preceded(
        sequence::tuple((bytes::tag(header), bytes::tag(":"), character::space1)),
        multi::separated_list1(
            multi::many1(bytes::tag(" ")),
            combinator::map(character::u32, f64::from),
        ),
    )
}

fn parse_times(s: &str) -> io::Result<Vec<f64>> {
    nom_parse_nums("Time")
        .parse(s)
        .map(|(_, times)| times)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
}

fn parse_distances(s: &str) -> io::Result<Vec<f64>> {
    nom_parse_nums("Distance")
        .parse(s)
        .map(|(_, times)| times)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
}

fn count_success_ways(time: f64, distance: f64) -> u64 {
    let discriminant = time.powi(2) - 4. * distance;
    let center_to_zero = discriminant.sqrt() / 2.;
    let discriminant_is_square =
        (discriminant.sqrt().trunc().powi(2) - discriminant).abs() < 1.0e-5;
    let discriminant_is_even = discriminant.trunc() as u32 % 2 == 0;
    if (time as u32) % 2 == 0 {
        let mut side_points = center_to_zero.trunc() as u64;
        if discriminant_is_square && discriminant_is_even {
            side_points -= 1;
        }
        1 + 2 * side_points
    } else {
        let mut side_points = (center_to_zero + 0.5).trunc() as u64;
        if discriminant_is_square && !discriminant_is_even {
            side_points -= 1;
        }
        2 * side_points
    }
}

fn part1(input: &mut dyn BufRead) -> io::Result<u64> {
    // wait x ms
    // spend n-x ms at x m/s
    // total distance: x(n-x) mm
    // maximum possible distance at 0.5n
    // n+1 ways to race, goal is number of latice points where x**2 - nx + d < 0
    // zeroes at x = (n \pm sqrt(n**2 - 4d))/2
    let times = parse_times(&read_line!(input)?)?;
    let distances = parse_distances(&read_line!(input)?)?;
    Ok(times
        .into_iter()
        .zip(distances)
        .fold(1, |acc, (time, distance)| {
            acc * count_success_ways(time, distance)
        }))
}

fn part2(input: &mut dyn BufRead) -> io::Result<u64> {
    fn parse_line(s: &str) -> f64 {
        s.bytes()
            .filter(u8::is_ascii_digit)
            .fold(0., |acc, b| acc * 10. + f64::from(b - b'0'))
    }

    let time = parse_line(&read_line!(input)?);
    let distance = parse_line(&read_line!(input)?);
    Ok(count_success_ways(time, distance))
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2023 Day 6 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2023_06.txt")?))?
        );
    }
    {
        println!("Year 2023 Day 6 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2023_06.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = "Time:      7  15   30\nDistance:  9  40  200\n";

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 288;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 71503;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
