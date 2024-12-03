use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    iter,
};

use aoc_util::{
    nom::{bytes::complete as bytes, character::complete as character, sequence, IResult, Parser},
    nom_supreme::ParserExt,
};

fn number(input: &[u8]) -> IResult<&[u8], u32> {
    character::u32.verify(|&n| n < 1000).parse(input)
}

fn mul(input: &[u8]) -> IResult<&[u8], u32> {
    sequence::delimited(
        bytes::tag(b"mul("),
        number
            .and(number.preceded_by(bytes::tag(b",")))
            .map(|(a, b)| a * b),
        bytes::tag(b")"),
    )(input)
}

fn do_or_dont(input: &[u8]) -> IResult<&[u8], bool> {
    bytes::tag(b"do()")
        .map(|_| true)
        .or(bytes::tag(b"don't()").map(|_| false))
        .parse(input)
}

fn part1(input: &mut dyn BufRead) -> io::Result<u32> {
    let input = {
        let mut buf = String::new();
        input.read_to_string(&mut buf)?;
        buf
    };
    Ok(
        iter::successors(Some((0, input.as_bytes())), |(acc, bytes)| {
            let i = bytes.iter().position(|&c| c == b'm')?;
            let bytes = &bytes[i..];
            let (bytes, product) = mul(bytes).unwrap_or((&bytes[1..], 0));
            Some((acc + product, bytes))
        })
        .last()
        .unwrap()
        .0,
    )
}

fn part2(input: &mut dyn BufRead) -> io::Result<u32> {
    let input = {
        let mut buf = String::new();
        input.read_to_string(&mut buf)?;
        buf
    };
    Ok(iter::successors(
        Some(((0, true), input.as_bytes())),
        |&((sum, should_do), bytes)| {
            let i = bytes.iter().position(|c| matches!(c, b'd' | b'm'))?;
            let bytes = &bytes[i..];
            let (bytes, (product, should_do)) = mul
                .map(|n| (n * should_do as u32, should_do))
                .or(do_or_dont.map(|should_do| (0, should_do)))
                .parse(bytes)
                .unwrap_or((&bytes[1..], (0, should_do)));
            Some(((sum + product, should_do), bytes))
        },
    )
    .last()
    .unwrap()
    .0
     .0)
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2024 Day 3 Part 1");
        println!(
            "{:?}",
            part1(&mut BufReader::new(File::open("2024_03.txt")?))?
        );
    }
    {
        println!("Year 2024 Day 3 Part 2");
        println!(
            "{:?}",
            part2(&mut BufReader::new(File::open("2024_03.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use io::Cursor;

    use super::*;

    const TEST_DATA_1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const TEST_DATA_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 161;
        let actual = part1(&mut Cursor::new(TEST_DATA_1))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 48;
        let actual = part2(&mut Cursor::new(TEST_DATA_2))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
