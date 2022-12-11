use std::{
    cmp::Reverse,
    collections::VecDeque,
    fmt::Debug,
    fs::File,
    io::{self, BufRead, BufReader},
    num::ParseIntError,
    str::FromStr,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Val {
    X(Worry),
    Old,
}

impl Val {
    fn unwrap_or(self, old: Worry) -> Worry {
        match self {
            Self::X(x) => x,
            Self::Old => old,
        }
    }
}

impl FromStr for Val {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if "old" == s {
            Ok(Self::Old)
        } else {
            Ok(Self::X(s.parse()?))
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Operator {
    Add,
    Mul,
}

impl Operator {
    fn apply(&self, a: Worry, b: Worry) -> Worry {
        match self {
            Self::Add => a + b,
            Self::Mul => a * b,
        }
    }
}

impl FromStr for Operator {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "*" => Ok(Self::Mul),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Only + and * operators are supported",
            )),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Expr {
    left: Val,
    operator: Operator,
    right: Val,
}

impl Expr {
    fn eval(&self, old: Worry) -> Worry {
        self.operator
            .apply(self.left.unwrap_or(old), self.right.unwrap_or(old))
    }
}

type MonkeyId = usize;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct NextMonkey {
    test_denominator: Worry,
    success: MonkeyId,
    failure: MonkeyId,
}

impl NextMonkey {
    fn test(&self, worry: Worry) -> MonkeyId {
        if worry % self.test_denominator == 0 {
            self.success
        } else {
            self.failure
        }
    }
}

type Worry = u64;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Monkey {
    worry_levels: VecDeque<Worry>,
    operation: Expr,
    next_monkey: NextMonkey,
}

impl Monkey {
    fn read(input: &mut dyn BufRead) -> io::Result<Self> {
        let mut lines = input.lines();
        let worry_levels = {
            let starting_items = lines.next().ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Ran out of input before first line",
                )
            })??;
            if let Some(worry_levels) = starting_items.strip_prefix("  Starting items: ") {
                worry_levels
                    .split(", ")
                    .map(|level| {
                        level
                            .parse()
                            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
                    })
                    .collect::<io::Result<_>>()?
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    r#"First line does not start with "Starting items""#,
                ));
            }
        };
        let operation = {
            let line = lines.next().ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "Operation line missing")
            })??;
            if let Some(operation) = line.strip_prefix("  Operation: new = ") {
                let mut bits = operation.split_whitespace();
                let left = bits
                    .next()
                    .ok_or_else(|| {
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            "right side of operation missing",
                        )
                    })?
                    .parse::<Val>()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                let operator = bits
                    .next()
                    .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "operator missing"))?
                    .parse::<Operator>()?;
                let right = bits
                    .next()
                    .ok_or_else(|| {
                        io::Error::new(io::ErrorKind::InvalidData, "second argument missing")
                    })?
                    .parse::<Val>()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                Expr {
                    left,
                    operator,
                    right,
                }
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid operation line",
                ));
            }
        };
        let test_denominator = {
            let line = lines.next().ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "Missing monkey test")
            })??;
            if let Some(next_monkey_test) = line.strip_prefix("  Test: divisible by ") {
                next_monkey_test.parse::<Worry>().map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Invalid test value {next_monkey_test:?}: {e:?}"),
                    )
                })?
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Malformed monkey test",
                ));
            }
        };
        let success = {
            let line = lines.next().ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "Missing test success line")
            })??;
            if let Some(next_monkey_true) = line.strip_prefix("    If true: throw to monkey ") {
                next_monkey_true
                    .parse::<MonkeyId>()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid test success line",
                ));
            }
        };
        let failure = {
            let line = lines.next().ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "Missing test failure line")
            })??;
            if let Some(next_monkey_false) = line.strip_prefix("    If false: throw to monkey ") {
                next_monkey_false
                    .parse::<MonkeyId>()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid test failure line",
                ));
            }
        };
        let next_monkey = NextMonkey {
            test_denominator,
            success,
            failure,
        };
        Ok(Self {
            worry_levels,
            operation,
            next_monkey,
        })
    }

    fn inspect_all(&mut self, do_relief: bool) -> impl Iterator<Item = (MonkeyId, Worry)> + '_ {
        let operation = self.operation;
        let next_monkey = self.next_monkey;
        self.worry_levels.drain(..).map(move |old| {
            let mut new = operation.eval(old);
            if do_relief {
                new /= 3;
            }
            (next_monkey.test(new), new)
        })
    }

    fn catch(&mut self, worry: Worry) {
        self.worry_levels.push_back(worry);
    }
}

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    let mut monkeys = vec![];
    loop {
        if let Some(monkey_num) = input.lines().next() {
            if let Some(monkey_num) = monkey_num?
                .strip_prefix("Monkey ")
                .and_then(|monkey_num| monkey_num.strip_suffix(':'))
            {
                if monkeys.len()
                    != monkey_num
                        .parse::<MonkeyId>()
                        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
                {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Monkeys got scrambled!",
                    ));
                } else {
                    monkeys.push(Monkey::read(input)?);
                    if let Some(empty_line) = input.lines().next() {
                        let empty_line = empty_line?;
                        if !empty_line.is_empty() {
                            return Err(io::Error::new(
                                io::ErrorKind::InvalidData,
                                format!("Unexpected non-empty line after monkey: {empty_line:?}"),
                            ));
                        } else {
                            continue;
                        }
                    } else {
                        break;
                    }
                }
            } else {
                break;
            }
        }
    }
    let mut num_inspections = vec![0; monkeys.len()];
    for _round in 0..20 {
        for monkey_idx in 0..monkeys.len() {
            let targets = monkeys[monkey_idx].inspect_all(true).collect::<Vec<_>>();
            num_inspections[monkey_idx] += targets.len();
            for (idx, worry) in targets {
                monkeys[idx].catch(worry);
            }
        }
    }
    num_inspections.sort_unstable_by_key(|&num_inspections| Reverse(num_inspections));
    Ok(num_inspections.into_iter().take(2).product())
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    let mut monkeys = vec![];
    loop {
        if let Some(monkey_num) = input.lines().next() {
            if let Some(monkey_num) = monkey_num?
                .strip_prefix("Monkey ")
                .and_then(|monkey_num| monkey_num.strip_suffix(':'))
            {
                if monkeys.len()
                    != monkey_num
                        .parse::<MonkeyId>()
                        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
                {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Monkeys got scrambled!",
                    ));
                } else {
                    monkeys.push(Monkey::read(input)?);
                    if let Some(empty_line) = input.lines().next() {
                        let empty_line = empty_line?;
                        if !empty_line.is_empty() {
                            return Err(io::Error::new(
                                io::ErrorKind::InvalidData,
                                format!("Unexpected non-empty line after monkey: {empty_line:?}"),
                            ));
                        } else {
                            continue;
                        }
                    } else {
                        break;
                    }
                }
            } else {
                break;
            }
        }
    }
    let test_product = monkeys
        .iter()
        .map(|monkey| monkey.next_monkey.test_denominator)
        .product::<Worry>();
    let mut num_inspections = vec![0; monkeys.len()];
    for _round in 0..10_000 {
        for monkey_idx in 0..monkeys.len() {
            let targets = monkeys[monkey_idx].inspect_all(false).collect::<Vec<_>>();
            num_inspections[monkey_idx] += targets.len();
            for (idx, worry) in targets {
                // This modulus won't change the result because
                // `x * y % b == (x % b) * (y % b) % b`.
                monkeys[idx].catch(worry % test_product);
            }
        }
    }
    num_inspections.sort_unstable_by_key(|&num_inspections| Reverse(num_inspections));
    Ok(num_inspections.into_iter().take(2).product())
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2022 Day 11 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2022_11.txt")?))?
        );
    }
    {
        println!("Year 2022 Day 11 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2022_11.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        "Monkey 0:\n",
        "  Starting items: 79, 98\n",
        "  Operation: new = old * 19\n",
        "  Test: divisible by 23\n",
        "    If true: throw to monkey 2\n",
        "    If false: throw to monkey 3\n",
        "\n",
        "Monkey 1:\n",
        "  Starting items: 54, 65, 75, 74\n",
        "  Operation: new = old + 6\n",
        "  Test: divisible by 19\n",
        "    If true: throw to monkey 2\n",
        "    If false: throw to monkey 0\n",
        "\n",
        "Monkey 2:\n",
        "  Starting items: 79, 60, 97\n",
        "  Operation: new = old * old\n",
        "  Test: divisible by 13\n",
        "    If true: throw to monkey 1\n",
        "    If false: throw to monkey 3\n",
        "\n",
        "Monkey 3:\n",
        "  Starting items: 74\n",
        "  Operation: new = old + 3\n",
        "  Test: divisible by 17\n",
        "    If true: throw to monkey 0\n",
        "    If false: throw to monkey 1\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 10_605;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 2_713_310_158;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
