use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn bank_max(bank: &[usize], num_batteries: usize, prefix: usize) -> Option<usize> {
    if bank.len() < num_batteries {
        return None;
    }
    if num_batteries == 0 {
        return Some(prefix);
    }
    (0..=9).rev().find_map(|digit| {
        let pos = bank.iter().position(|&d| d == digit)?;
        if pos + (num_batteries - 1) >= bank.len() {
            return None;
        }
        bank_max(&bank[(pos + 1)..], num_batteries - 1, prefix * 10 + digit)
    })
}

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    let banks = input
        .lines()
        .map(|line| {
            line.map(|line| {
                line.bytes()
                    .map(|b| (b - b'0') as usize)
                    .collect::<Vec<_>>()
            })
        })
        .collect::<io::Result<Vec<_>>>()?;
    eprintln!("{} banks of length {}", banks.len(), banks[0].len());
    Ok(banks
        .into_iter()
        .map(|bank| bank_max(&bank[..], 2, 0).unwrap_or_else(|| panic!("Bank {bank:?} too small")))
        .sum())
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    let banks = input
        .lines()
        .map(|line| {
            line.map(|line| {
                line.bytes()
                    .map(|b| (b - b'0') as usize)
                    .collect::<Vec<_>>()
            })
        })
        .collect::<io::Result<Vec<_>>>()?;
    Ok(banks
        .into_iter()
        .map(|bank| bank_max(&bank[..], 12, 0).unwrap_or_else(|| panic!("Bank {bank:?} too small")))
        .sum())
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2025 Day 3 Part 1");
        println!(
            "{:?}",
            part1(&mut BufReader::new(File::open("2025_03.txt")?))?
        );
    }
    {
        println!("Year 2025 Day 3 Part 2");
        println!(
            "{:?}",
            part2(&mut BufReader::new(File::open("2025_03.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = "987654321111111\n811111111111119\n234234234234278\n818181911112111\n";

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 357;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 3_121_910_778_619;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
