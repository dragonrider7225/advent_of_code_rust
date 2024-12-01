use aoc_util::{
    nom::{self, character::complete as character, Parser},
    nom_supreme::{final_parser, ParserExt},
};
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn read_lists(input: &mut dyn BufRead) -> io::Result<(Vec<u32>, Vec<u32>)> {
    input
        .lines()
        .map(|line| {
            let line = line?;
            final_parser::final_parser::<_, _, nom::error::Error<_>, nom::error::Error<&str>>(
                character::u32.and(character::u32.preceded_by(character::space1)),
            )(&line)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
        })
        .try_fold((vec![], vec![]), |mut acc, v| {
            let (l, r) = v?;
            acc.0.push(l);
            acc.1.push(r);
            io::Result::Ok(acc)
        })
}

fn part1(input: &mut dyn BufRead) -> io::Result<u32> {
    let (mut left, mut right) = read_lists(input)?;
    left.sort();
    right.sort();
    Ok(left
        .into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum())
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    let (mut left, mut right) = read_lists(input)?;
    left.sort();
    right.sort();
    let mut left_idx = 0;
    let mut right_idx = 0;
    let mut sum = 0;
    while left_idx < left.len() {
        let current_n = left[left_idx];
        let left_count = left[left_idx..]
            .iter()
            .take_while(|&&n| n == current_n)
            .count();
        left_idx += left_count;
        right_idx += right[right_idx..]
            .iter()
            .take_while(|&&n| n < current_n)
            .count();
        let right_count = right[right_idx..]
            .iter()
            .take_while(|&&n| n == current_n)
            .count();
        right_idx += right_count;
        sum += current_n as usize * left_count * right_count;
    }
    Ok(sum)
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2024 Day 1 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2024_01.txt")?))?
        );
    }
    {
        println!("Year 2024 Day 1 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2024_01.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use io::Cursor;

    use super::*;

    const TEST_DATA: &str = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3\n";

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 11;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 31;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
