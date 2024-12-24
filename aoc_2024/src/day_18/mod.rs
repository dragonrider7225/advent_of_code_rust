use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader},
};

use aoc_util::{
    geometry::{Direction, Point2D},
    nom::{bytes::complete as bytes, character::complete as character, IResult, Parser},
    nom_supreme::ParserExt,
};

type Byte = Point2D<usize>;

fn parse_byte(input: &str) -> IResult<&str, Byte> {
    character::u32
        .and(character::u32.preceded_by(bytes::tag(",")))
        .map(|(x, y)| Byte::at(x as _, y as _))
        .parse(input)
}

fn path_length(corrupted_bytes: &[Byte], max_byte: usize) -> Option<usize> {
    let mut to_check = HashSet::<_>::from_iter([Byte::at(0, 0)]);
    let mut visited = to_check.clone();
    for step_num in 0.. {
        if to_check.is_empty() {
            break;
        }
        if to_check.contains(&Byte::at(max_byte, max_byte)) {
            return Some(step_num);
        }
        to_check = to_check
            .into_iter()
            .flat_map(|front| {
                Direction::values()
                    .iter()
                    .filter_map(move |&direction| match direction {
                        Direction::Left if *front.x() == 0 => None,
                        Direction::Down if *front.y() == 0 => None,
                        Direction::Right if *front.x() == max_byte => None,
                        Direction::Up if *front.y() == max_byte => None,
                        _ => Some(front + direction),
                    })
            })
            .filter(|next| !corrupted_bytes.contains(next))
            .filter(|&next| visited.insert(next))
            .collect();
    }
    None
}

fn part1(input: &mut dyn BufRead, max_byte: usize) -> io::Result<usize> {
    let num_corrupted_bytes = if max_byte < 70 { 12 } else { 1024 };
    let corrupted_bytes = input
        .lines()
        .take(num_corrupted_bytes)
        .map(|line| {
            parse_byte(&(line?))
                .map(|(_, b)| b)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
        })
        .collect::<io::Result<Vec<_>>>()?;
    path_length(&corrupted_bytes[..], max_byte)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Couldn't reach exit"))
}

fn part2(input: &mut dyn BufRead, max_byte: usize) -> io::Result<Byte> {
    let corrupted_bytes = input
        .lines()
        .map(|line| {
            parse_byte(&(line?))
                .map(|(_, b)| b)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
        })
        .collect::<io::Result<Vec<_>>>()?;
    let mut num_corrupted_bytes = if max_byte < 70 { 12 } else { 1024 };
    let mut step_size = (corrupted_bytes.len() - num_corrupted_bytes) / 2;
    while step_size > 1 {
        if path_length(&corrupted_bytes[..num_corrupted_bytes], max_byte).is_some() {
            eprintln!("Path still possible after {num_corrupted_bytes} corrupted bytes");
            num_corrupted_bytes += step_size;
        } else {
            eprintln!("Path not possible after {num_corrupted_bytes} corrupted bytes");
            num_corrupted_bytes -= step_size;
        }
        step_size /= 2;
    }
    let num_corrupted_bytes =
        if path_length(&corrupted_bytes[..num_corrupted_bytes], max_byte).is_some() {
            ((num_corrupted_bytes + 1)..corrupted_bytes.len())
                .find(|&num_corrupted_bytes| {
                    path_length(&corrupted_bytes[..num_corrupted_bytes], max_byte).is_none()
                })
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidData, "Path to exit never blocked")
                })?
        } else {
            (0..num_corrupted_bytes)
                .rev()
                .find(|&num_corrupted_bytes| {
                    path_length(&corrupted_bytes[..num_corrupted_bytes], max_byte).is_some()
                })
                .map(|idx| idx + 1)
                .unwrap_or_else(|| {
                    unreachable!("There will always be a path before any bytes are corrupted")
                })
        };
    eprintln!("The exit is no longer reachable after {num_corrupted_bytes} bytes have fallen");
    Ok(corrupted_bytes[num_corrupted_bytes - 1])
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2024 Day 18 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2024_18.txt")?), 70)?
        );
    }
    {
        println!("Year 2024 Day 18 Part 2");
        let output = part2(&mut BufReader::new(File::open("2024_18.txt")?), 70)?;
        println!("{},{}", *output.x(), *output.y(),);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        "5,4\n", "4,2\n", "4,5\n", "3,0\n", "2,1\n", "6,3\n", "2,4\n", "1,5\n", "0,6\n", "3,3\n",
        "2,6\n", "5,1\n", "1,2\n", "5,5\n", "2,5\n", "6,5\n", "1,4\n", "0,4\n", "6,4\n", "1,1\n",
        "6,1\n", "1,0\n", "0,5\n", "1,6\n", "2,0\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let break_point = 12 * 4;
        assert_eq!(TEST_DATA.as_bytes()[break_point - 1], b'\n');
        let expected = 22;
        let actual = part1(&mut Cursor::new(&TEST_DATA[..break_point]), 6)?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = Byte::at(6, 1);
        let actual = part2(&mut Cursor::new(&TEST_DATA), 6)?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
