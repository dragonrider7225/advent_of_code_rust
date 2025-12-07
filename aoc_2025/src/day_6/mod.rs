use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    str::FromStr,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Op {
    Add,
    Mul,
}

impl FromStr for Op {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "*" => Ok(Self::Mul),
            _ => Err(format!("{s:?} is not a valid operator")),
        }
    }
}

impl TryFrom<char> for Op {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Self::Add),
            '*' => Ok(Self::Mul),
            _ => Err(format!("Invalid operation: {value:?}")),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Field {
    Num(u64),
    Op(Op),
}

impl FromStr for Field {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<Op>().map(Self::Op).or_else(|_| {
            s.parse::<u64>()
                .map(Self::Num)
                .map_err(|e| format!("{e:?}: Cannot parse {s:?} as Field"))
        })
    }
}

fn part1(input: &mut dyn BufRead) -> io::Result<u64> {
    input
        .lines()
        .try_fold(vec![], |acc, line| {
            line.and_then(|line| {
                if acc.is_empty() {
                    line.split_whitespace()
                        .map(|n| {
                            n.parse::<u64>().map_err(|e| {
                                io::Error::new(io::ErrorKind::InvalidData, format!("{e:?}: {n:?}"))
                            })
                        })
                        .map(|n| n.map(|n| (None, n, n)))
                        .collect::<io::Result<Vec<_>>>()
                } else {
                    line.split_whitespace()
                        .zip(acc)
                        .map(|(n, (op, sum, product))| match n.parse::<Field>() {
                            Ok(Field::Num(n)) => Ok((op, sum + n, product * n)),
                            Ok(Field::Op(op)) => Ok((Some(op), sum, product)),
                            Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e)),
                        })
                        .collect::<io::Result<Vec<_>>>()
                }
            })
        })
        .and_then(|values| {
            values
                .into_iter()
                .map(|(op, sum, product)| match op {
                    Some(Op::Add) => Ok(sum),
                    Some(Op::Mul) => Ok(product),
                    None => Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Encountered problem without operator",
                    )),
                })
                .try_fold(0, |acc, n| n.map(|n| acc + n))
        })
}

fn part2(input: &mut dyn BufRead) -> io::Result<u64> {
    input
        .lines()
        .map(|line| {
            line.and_then(|line| {
                line.bytes()
                    .map(|b| match b {
                        b'0'..=b'9' => Ok(Some(Field::Num((b - b'0') as _))),
                        b'+' => Ok(Some(Field::Op(Op::Add))),
                        b'*' => Ok(Some(Field::Op(Op::Mul))),
                        b' ' => Ok(None),
                        _ => Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("'\\x{b:x}' is not a valid character in Cephalopod math"),
                        )),
                    })
                    .collect::<io::Result<Vec<_>>>()
            })
        })
        .try_fold(vec![], |acc, line| {
            line.map(|line| {
                if acc.is_empty() {
                    line.into_iter()
                        .map(|field| {
                            field.map(|f| match f {
                                Field::Num(n) => n,
                                Field::Op(_) => panic!("Operator in first row"),
                            })
                        })
                        .collect::<Vec<_>>()
                } else {
                    assert_eq!(acc.len(), line.len());
                    acc.into_iter()
                        .zip(line)
                        .fold(
                            (None, vec![]),
                            |(mut calculating, mut acc): (_, Vec<Option<_>>), (col, field)| {
                                let value = match (col, field) {
                                    (Some(n), Some(Field::Num(d))) => Some(n * 10 + d),
                                    (Some(n), Some(Field::Op(op))) => {
                                        calculating = Some(op);
                                        Some(n)
                                    }
                                    (Some(col), None) => {
                                        if let Some(op) = calculating {
                                            let total = acc.pop().unwrap().unwrap();
                                            match op {
                                                Op::Add => Some(total + col),
                                                Op::Mul => Some(total * col),
                                            }
                                        } else {
                                            Some(col)
                                        }
                                    }
                                    (None, Some(Field::Num(n))) => Some(n),
                                    (None, Some(Field::Op(_))) => {
                                        panic!("Column contains only operator")
                                    }
                                    (None, None) => {
                                        calculating = None;
                                        None
                                    }
                                };
                                acc.push(value);
                                (calculating, acc)
                            },
                        )
                        .1
                }
            })
        })
        .map(|totals| totals.into_iter().flatten().sum())
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2025 Day 6 Part 1");
        println!(
            "{:?}",
            part1(&mut BufReader::new(File::open("2025_06.txt")?))?
        );
    }
    {
        println!("Year 2025 Day 6 Part 2");
        println!(
            "{:?}",
            part2(&mut BufReader::new(File::open("2025_06.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        "123 328  51 64 \n",
        " 45 64  387 23 \n",
        "  6 98  215 314\n",
        "*   +   *   +  \n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 4_277_556;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 3_263_827;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
