use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    let instructions = input
        .lines()
        .map(|line| {
            line.and_then(|line| match line.as_bytes()[0] {
                b'L' => line[1..]
                    .parse::<usize>()
                    .map(|n| -(n as isize))
                    .map_err(|e| {
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("{e:?}: {line:?} is not a valid instruction"),
                        )
                    }),
                b'R' => line[1..].parse::<usize>().map(|n| n as isize).map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("{e:?}: {line:?} is not a valid instruction"),
                    )
                }),
                b => Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("'\\u{b:x}' does not start any valid instruction"),
                )),
            })
        })
        .collect::<io::Result<Vec<_>>>()?;
    Ok(instructions
        .into_iter()
        .fold((0, 50), |(count, current), instr| {
            let next = (current + instr) % 100;
            (count + usize::from(next == 0), next)
        })
        .0)
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    let instructions = input
        .lines()
        .map(|line| {
            line.and_then(|line| match line.as_bytes()[0] {
                b'L' => line[1..]
                    .parse::<usize>()
                    .map(|n| -(n as isize))
                    .map_err(|e| {
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("{e:?}: {line:?} is not a valid instruction"),
                        )
                    }),
                b'R' => line[1..].parse::<usize>().map(|n| n as isize).map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("{e:?}: {line:?} is not a valid instruction"),
                    )
                }),
                b => Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("'\\u{b:x}' does not start any valid instruction"),
                )),
            })
        })
        .collect::<io::Result<Vec<_>>>()?;
    Ok(instructions
        .into_iter()
        .fold((0, 50), |(count, current), instr| {
            let full_cycles = (instr / 100).abs_diff(0);
            let extras = instr % 100;
            let unclamped = current + extras;
            let next = unclamped.rem_euclid(100);
            (
                count
                    + full_cycles
                    + ((next - unclamped) / 100).abs_diff(0)
                    + usize::from(unclamped == 0 && extras < 0)
                    - usize::from(current == 0 && extras < 0),
                next,
            )
        })
        .0)
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2025 Day 1 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2025_01.txt")?))?
        );
    }
    {
        println!("Year 2025 Day 1 Part 2");
        println!(
            "{:?}",
            part2(&mut BufReader::new(File::open("2025_01.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::{self, Cursor};

    const TEST_DATA: &str = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82\n";

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 3;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 6;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
