use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn part1(input: &mut dyn BufRead) -> io::Result<u32> {
    let mut num_increases = 0;
    let mut last_depth = None;
    for line in input.lines() {
        let depth = line?
            .parse::<u32>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        match last_depth {
            Some(last_depth) if last_depth < depth => num_increases += 1,
            _ => {}
        }
        last_depth = Some(depth);
    }
    Ok(num_increases)
}

fn part2(input: &mut dyn BufRead) -> io::Result<u32> {
    let mut num_increases = 0;
    let mut last_depths = [None, None, None];
    for line in input.lines() {
        let depth = line?
            .parse::<u32>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        match last_depths[0] {
            // `x` only exists if the other two elements of last_depths also exist.
            Some(x) if x < depth => num_increases += 1,
            _ => {}
        }
        let new_depths = [last_depths[1], last_depths[2], Some(depth)];
        last_depths = new_depths;
    }
    Ok(num_increases)
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2021 Day 1 Part 1");
        let input = File::open("2021_01.txt")?;
        let num_increases = part1(&mut BufReader::new(input))?;
        println!("{}", num_increases);
    }
    {
        println!("Year 2021 Day 1 Part 2");
        let input = File::open("2021_01.txt")?;
        let num_increases = part2(&mut BufReader::new(input))?;
        println!("{}", num_increases);
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use super::*;

    #[test]
    #[ignore]
    fn test_part1() -> io::Result<()> {
        let expected = 7;
        let actual = part1(&mut Cursor::new(
            "199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n",
        ))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part2() -> io::Result<()> {
        let expected = 5;
        let actual = part2(&mut Cursor::new(
            "199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n",
        ))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
