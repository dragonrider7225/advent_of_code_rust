use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
};

use aoc_util::{
    impl_from_str_for_nom_parse,
    nom_extended::{self, NomParse},
};

use nom::{branch, bytes::complete as bytes, combinator as comb, sequence, IResult};

#[derive(Clone, Debug, Default)]
struct Alu {
    variables: HashMap<Variable, i128>,
}

impl Alu {
    fn read_variable(&self, variable: Variable) -> i128 {
        self.variables.get(&variable).copied().unwrap_or_default()
    }
}

impl Alu {
    fn run_program(
        &mut self,
        instructions: impl IntoIterator<Item = Instruction>,
        input: &mut dyn Iterator<Item = i128>,
    ) {
        for instruction in instructions {
            match instruction {
                Instruction::Inp(variable) => {
                    let value = input.next().expect("Couldn't get input value");
                    self.variables.insert(variable, value);
                }
                Instruction::Add { dest, addend } => {
                    let new_value = self.read_variable(dest) + addend.unwrap(self);
                    self.variables.insert(dest, new_value);
                }
                Instruction::Mul { dest, multiplicand } => {
                    let new_value = self.read_variable(dest) * multiplicand.unwrap(self);
                    self.variables.insert(dest, new_value);
                }
                Instruction::Div { dest, denominator } => {
                    let new_value = self.read_variable(dest) / denominator.unwrap(self);
                    self.variables.insert(dest, new_value);
                }
                Instruction::Mod { dest, denominator } => {
                    let dest_value = self.read_variable(dest);
                    let denominator = denominator.unwrap(self);
                    assert!(
                        dest_value >= 0,
                        "Tried to compute modulo of negative number: {} % {}",
                        dest_value,
                        denominator
                    );
                    assert!(
                        denominator > 0,
                        "Tried to compute modulo non-positive number: {} % {}",
                        dest_value,
                        denominator
                    );
                    self.variables.insert(dest, dest_value % denominator);
                }
                Instruction::Eql { dest, rhs } => {
                    let new_value = u32::from(self.read_variable(dest) == rhs.unwrap(self)).into();
                    self.variables.insert(dest, new_value);
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Variable {
    W,
    X,
    Y,
    Z,
}

impl NomParse<&'_ str> for Variable {
    fn nom_parse(s: &str) -> IResult<&str, Self> {
        branch::alt((
            comb::value(Self::W, bytes::tag("w")),
            comb::value(Self::X, bytes::tag("x")),
            comb::value(Self::Y, bytes::tag("y")),
            comb::value(Self::Z, bytes::tag("z")),
        ))(s)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Value {
    Variable(Variable),
    Literal(i128),
}

impl NomParse<&'_ str> for Value {
    fn nom_parse(s: &str) -> IResult<&str, Self> {
        branch::alt((
            comb::map(Variable::nom_parse, Self::Variable),
            comb::map(nom_extended::recognize_i128, Self::Literal),
        ))(s)
    }
}

impl Value {
    fn unwrap(&self, alu: &Alu) -> i128 {
        match *self {
            Self::Variable(v) => alu.read_variable(v),
            Self::Literal(x) => x,
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
enum Instruction {
    Inp(Variable),
    Add { dest: Variable, addend: Value },
    Mul { dest: Variable, multiplicand: Value },
    Div { dest: Variable, denominator: Value },
    Mod { dest: Variable, denominator: Value },
    Eql { dest: Variable, rhs: Value },
}

impl NomParse<&'_ str> for Instruction {
    fn nom_parse(s: &str) -> IResult<&str, Self> {
        branch::alt((
            comb::map(
                sequence::preceded(bytes::tag("inp "), Variable::nom_parse),
                Self::Inp,
            ),
            comb::map(
                sequence::preceded(
                    bytes::tag("add "),
                    sequence::separated_pair(
                        Variable::nom_parse,
                        bytes::tag(" "),
                        Value::nom_parse,
                    ),
                ),
                |(dest, addend)| Self::Add { dest, addend },
            ),
            comb::map(
                sequence::preceded(
                    bytes::tag("mul "),
                    sequence::separated_pair(
                        Variable::nom_parse,
                        bytes::tag(" "),
                        Value::nom_parse,
                    ),
                ),
                |(dest, multiplicand)| Self::Mul { dest, multiplicand },
            ),
            comb::map(
                sequence::preceded(
                    bytes::tag("div "),
                    sequence::separated_pair(
                        Variable::nom_parse,
                        bytes::tag(" "),
                        Value::nom_parse,
                    ),
                ),
                |(dest, denominator)| Self::Div { dest, denominator },
            ),
            comb::map(
                sequence::preceded(
                    bytes::tag("mod "),
                    sequence::separated_pair(
                        Variable::nom_parse,
                        bytes::tag(" "),
                        Value::nom_parse,
                    ),
                ),
                |(dest, denominator)| Self::Mod { dest, denominator },
            ),
            comb::map(
                sequence::preceded(
                    bytes::tag("eql "),
                    sequence::separated_pair(
                        Variable::nom_parse,
                        bytes::tag(" "),
                        Value::nom_parse,
                    ),
                ),
                |(dest, rhs)| Self::Eql { dest, rhs },
            ),
        ))(s)
    }
}

impl_from_str_for_nom_parse!(Instruction);

fn read_program(input: &mut dyn BufRead) -> io::Result<Vec<Instruction>> {
    input
        .lines()
        .map(|line| {
            line?
                .trim()
                .parse::<Instruction>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        })
        .collect()
}

fn fold_num(digits: impl IntoIterator<Item = u64>) -> u64 {
    digits.into_iter().fold(0, |acc, digit| acc * 10 + digit)
}

fn part1(input: &mut dyn BufRead) -> io::Result<u64> {
    let mut alu = Alu::default();
    let program = read_program(input)?;
    let digits = [6, 9, 9, 1, 4, 9, 9, 9, 9, 7, 5, 3, 6, 9];
    alu.run_program(
        program.iter().cloned(),
        &mut digits.iter().copied().map(i128::from),
    );
    assert_eq!(0, alu.read_variable(Variable::Z));
    Ok(fold_num(digits))
}

fn part2(input: &mut dyn BufRead) -> io::Result<u64> {
    let mut alu = Alu::default();
    let program = read_program(input)?;
    let digits = [1, 4, 9, 1, 1, 6, 7, 5, 3, 1, 1, 1, 1, 4];
    alu.run_program(
        program.iter().cloned(),
        &mut digits.iter().copied().map(i128::from),
    );
    assert_eq!(0, alu.read_variable(Variable::Z));
    Ok(fold_num(digits))
}

#[allow(unreachable_code)]
pub(super) fn run() -> io::Result<()> {
    println!("This problem was solved by manually stepping through the fourteen segments of the program and keeping track of exactly what the output would be for any possible input sequence. As such, this \"solution\" works only for my specific input");
    {
        println!("Year 2021 Day 24 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2021_24.txt")?))?
        );
    }
    {
        println!("Year 2021 Day 24 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2021_24.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    #[ignore]
    fn test_negation() -> io::Result<()> {
        let program = "inp x\nmul x -1\n";
        let input = [3];
        let expected = -3;
        let mut alu = Alu::default();
        alu.run_program(
            read_program(&mut Cursor::new(program))?,
            &mut input.into_iter(),
        );
        let actual = alu.read_variable(Variable::X);
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_comparison() -> io::Result<()> {
        let program = "inp z\ninp x\nmul z 3\neql z x\n";
        let input = [3, 9];
        let expected = 1;
        let mut alu = Alu::default();
        alu.run_program(
            read_program(&mut Cursor::new(program))?,
            &mut input.into_iter(),
        );
        let actual = alu.read_variable(Variable::Z);
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_bit_storage() -> io::Result<()> {
        let program = concat!(
            "inp w\n",
            "add z w\n",
            "mod z 2\n",
            "div w 2\n",
            "add y w\n",
            "mod y 2\n",
            "div w 2\n",
            "add x w\n",
            "mod x 2\n",
            "div w 2\n",
            "mod w 2\n",
        );
        let input = [3, 9];
        let expected = 1;
        let mut alu = Alu::default();
        alu.run_program(
            read_program(&mut Cursor::new(program))?,
            &mut input.into_iter(),
        );
        let actual = alu.read_variable(Variable::Z);
        assert_eq!(expected, actual);
        Ok(())
    }
}
