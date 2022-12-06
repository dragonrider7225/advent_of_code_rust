use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn find_distinct(bytes: &[u8], num_distinct: usize) -> Option<usize> {
    let magic_number = num_distinct - 1;
    let mut i = magic_number;
    while i < bytes.len() {
        let marker = &bytes[(i - magic_number)..=i];
        if let Some(j) = (0..magic_number)
            .find(|j| marker[(magic_number - j)..].contains(&marker[magic_number - 1 - j]))
        {
            i += magic_number - j;
        } else {
            return Some(i + 1);
        }
    }
    None
}

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    let line = input.lines().next().expect("Missing data")?;
    find_distinct(line.as_bytes(), 4).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "Couldn't find start-of-packet marker",
        )
    })
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    let line = input.lines().next().expect("Missing data")?;
    find_distinct(line.as_bytes(), 14).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "Couldn't find start-of-message marker",
        )
    })
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2022 Day 6 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2022_06.txt")?))?
        );
    }
    {
        println!("Year 2022 Day 6 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2022_06.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA_0: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb\n";
    const TEST_DATA_1: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz\n";
    const TEST_DATA_2: &str = "nppdvjthqldpwncqszvftbrmjlhg\n";
    const TEST_DATA_3: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg\n";
    const TEST_DATA_4: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw\n";

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 7;
        let actual = part1(&mut Cursor::new(TEST_DATA_0))?;
        assert_eq!(expected, actual);
        let expected = 5;
        let actual = part1(&mut Cursor::new(TEST_DATA_1))?;
        assert_eq!(expected, actual);
        let expected = 6;
        let actual = part1(&mut Cursor::new(TEST_DATA_2))?;
        assert_eq!(expected, actual);
        let expected = 10;
        let actual = part1(&mut Cursor::new(TEST_DATA_3))?;
        assert_eq!(expected, actual);
        let expected = 11;
        let actual = part1(&mut Cursor::new(TEST_DATA_4))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 19;
        let actual = part2(&mut Cursor::new(TEST_DATA_0))?;
        assert_eq!(expected, actual);
        let expected = 23;
        let actual = part2(&mut Cursor::new(TEST_DATA_1))?;
        assert_eq!(expected, actual);
        let expected = 23;
        let actual = part2(&mut Cursor::new(TEST_DATA_2))?;
        assert_eq!(expected, actual);
        let expected = 29;
        let actual = part2(&mut Cursor::new(TEST_DATA_3))?;
        assert_eq!(expected, actual);
        let expected = 26;
        let actual = part2(&mut Cursor::new(TEST_DATA_4))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
