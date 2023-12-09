use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    iter,
};

fn parse_lines(input: &mut dyn BufRead) -> impl Iterator<Item = io::Result<Vec<i32>>> + '_ {
    input.lines().map(|line| {
        line.map(|line| {
            line.split_ascii_whitespace()
                .map(|digits| {
                    digits
                        .parse()
                        .expect("Input contains non-number non-whitespace character")
                })
                .collect::<Vec<_>>()
        })
    })
}

fn calculate_differences(#[allow(clippy::ptr_arg)] values: &Vec<i32>) -> Option<Vec<i32>> {
    Some(
        values
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect::<Vec<_>>(),
    )
    .filter(|ret| !ret.iter().all(|&d| d == 0))
}

fn part1(input: &mut dyn BufRead) -> io::Result<i32> {
    parse_lines(input)
        .map(|history| {
            Ok(iter::successors(Some(history?), calculate_differences)
                .collect::<Vec<_>>()
                .into_iter()
                .rfold(0, |acc, values| values.last().unwrap() + acc))
        })
        .sum()
}

fn part2(input: &mut dyn BufRead) -> io::Result<i32> {
    parse_lines(input)
        .map(|history| {
            Ok(iter::successors(Some(history?), calculate_differences)
                .collect::<Vec<_>>()
                .into_iter()
                .rfold(0, |acc, values| values[0] - acc))
        })
        .sum()
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2023 Day 9 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2023_09.txt")?))?
        );
    }
    {
        println!("Year 2023 Day 9 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2023_09.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!("0 3 6 9 12 15\n", "1 3 6 10 15 21\n", "10 13 16 21 30 45\n",);

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 114;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 2;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
