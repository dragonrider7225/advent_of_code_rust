use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use aoc_util::{
    nom::{bytes::complete as bytes, character::complete as character, multi, IResult, Parser},
    nom_supreme::ParserExt,
};

type Number = u64;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

impl Operator {
    fn pop2(n: Number) -> (Number, Operator) {
        let operator = if n.is_multiple_of(2) {
            Self::Add
        } else {
            Self::Multiply
        };
        (n / 2, operator)
    }

    fn pop3(n: Number) -> (Number, Operator) {
        let operator = match n % 3 {
            0 => Operator::Add,
            1 => Operator::Multiply,
            _ => Operator::Concatenate,
        };
        (n / 3, operator)
    }

    fn apply(&self, left: Number, right: Number) -> Number {
        match self {
            Self::Add => left + right,
            Self::Multiply => left * right,
            Self::Concatenate => left * (10 as Number).pow(right.ilog10() + 1) + right,
        }
    }
}

fn number(s: &str) -> IResult<&str, Number> {
    character::u64(s)
}

fn part1(input: &mut dyn BufRead) -> io::Result<Number> {
    let equations = input
        .lines()
        .map(|line| {
            let line = line?;
            number
                .and(multi::many1(number.preceded_by(bytes::tag(" "))).preceded_by(bytes::tag(":")))
                .parse(&*line)
                .map(|(_, parsed)| parsed)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
        })
        .collect::<io::Result<Vec<_>>>()?;
    Ok(equations
        .into_iter()
        .filter(|&(result, ref operands)| {
            (0..(2 << (operands.len() as u32 - 1))).any(|operators| {
                result
                    == operands[1..]
                        .iter()
                        .fold((operands[0], operators), |(total, operators), &operand| {
                            let (operators, operator) = Operator::pop2(operators);
                            (operator.apply(total, operand), operators)
                        })
                        .0
            })
        })
        .map(|(result, _)| result)
        .sum())
}

fn part2(input: &mut dyn BufRead) -> io::Result<Number> {
    let equations = input
        .lines()
        .map(|line| {
            let line = line?;
            number
                .and(multi::many1(number.preceded_by(bytes::tag(" "))).preceded_by(bytes::tag(":")))
                .parse(&*line)
                .map(|(_, parsed)| parsed)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
        })
        .collect::<io::Result<Vec<_>>>()?;
    Ok(equations
        .into_iter()
        .filter(|&(result, ref operands)| {
            (0..((3 as Number).pow(operands.len() as u32 - 1))).any(|operators| {
                result
                    == operands[1..]
                        .iter()
                        .fold((operands[0], operators), |(total, operators), &operand| {
                            let (operators, operator) = Operator::pop3(operators);
                            (operator.apply(total, operand), operators)
                        })
                        .0
            })
        })
        .map(|(result, _)| result)
        .sum())
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2024 Day 7 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2024_07.txt")?))?
        );
    }
    {
        println!("Year 2024 Day 7 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2024_07.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        "190: 10 19\n",
        "3267: 81 40 27\n",
        "83: 17 5\n",
        "156: 15 6\n",
        "7290: 6 8 6 15\n",
        "161011: 16 10 13\n",
        "192: 17 8 14\n",
        "21037: 9 7 18 13\n",
        "292: 11 6 16 20\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 3749;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 11387;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
