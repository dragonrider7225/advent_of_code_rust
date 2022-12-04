use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn part1(input: &mut dyn BufRead) -> io::Result<u32> {
    let mut total_containment = 0;
    for line in input.lines() {
        let line = line?;
        let (left, right) = line
            .split_once(',')
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, line.clone()))?;
        let (left_low, left_high) = left
            .split_once('-')
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, left.to_string()))?;
        let (right_low, right_high) = right
            .split_once('-')
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, right.to_string()))?;
        let left_low = left_low
            .parse::<u32>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let left_high = left_high
            .parse::<u32>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let right_low = right_low
            .parse::<u32>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let right_high = right_high
            .parse::<u32>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let left = left_low..=left_high;
        let right = right_low..=right_high;
        if left.contains(&right_low) && left.contains(&right_high)
            || right.contains(&left_low) && right.contains(&left_high)
        {
            total_containment += 1;
        }
    }
    Ok(total_containment)
}

fn part2(input: &mut dyn BufRead) -> io::Result<u32> {
    let mut total_overlaps = 0;
    for line in input.lines() {
        let line = line?;
        let (left, right) = line
            .split_once(',')
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, line.clone()))?;
        let (left_low, left_high) = left
            .split_once('-')
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, left.to_string()))?;
        let (right_low, right_high) = right
            .split_once('-')
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, right.to_string()))?;
        let left_low = left_low
            .parse::<u32>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let left_high = left_high
            .parse::<u32>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let right_low = right_low
            .parse::<u32>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let right_high = right_high
            .parse::<u32>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let left = left_low..=left_high;
        let right = right_low..=right_high;
        if (left.contains(&right_low) || left.contains(&right_high))
            || (right.contains(&left_low) || right.contains(&left_high))
        {
            total_overlaps += 1;
        }
    }
    Ok(total_overlaps)
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2022 Day 4 Part 1");
        println!(
            "{:?}",
            part1(&mut BufReader::new(File::open("2022_04.txt")?))?
        );
    }
    {
        println!("Year 2022 Day 4 Part 2");
        println!(
            "{:?}",
            part2(&mut BufReader::new(File::open("2022_04.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        "2-4,6-8\n",
        "2-3,4-5\n",
        "5-7,7-9\n",
        "2-8,3-7\n",
        "6-6,4-6\n",
        "2-6,4-8\n",
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
