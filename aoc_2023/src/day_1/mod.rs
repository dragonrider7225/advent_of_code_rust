use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn part1(input: &mut dyn BufRead) -> io::Result<u32> {
    input
        .lines()
        .map(|line| {
            let line = line?;
            let first_digit = line
                .bytes()
                .find(|b| b.is_ascii_digit())
                .unwrap_or_else(|| panic!("Line {line:?} doesn't contain any digits"))
                - b'0';
            let last_digit = line
                .bytes()
                .rfind(|b| b.is_ascii_digit())
                .unwrap_or_else(|| panic!("Line {line:?} doesn't contain any digits"))
                - b'0';
            let value = first_digit as u32 * 10 + last_digit as u32;
            Ok(value)
        })
        .try_fold(0, |acc, elem: io::Result<_>| Ok(acc + elem?))
}

fn part2(input: &mut dyn BufRead) -> io::Result<u32> {
    todo!("Year 2023 Day 1 Part 2")
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2023 Day 1 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2023_01.txt")?))?
        );
    }
    {
        println!("Year 2023 Day 1 Part 2");
        println!(
            "{:?}",
            part2(&mut BufReader::new(File::open("2023_01.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!("1abc2\n", "pqr3stu8vwx\n", "a1b2c3d4e5f\n", "treb7uchet\n");

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 142;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
