use std::{
    fmt::{self, Display, Formatter},
    fs::File,
    io::{self, BufRead, BufReader},
    iter,
};

use aoc_util::{
    impl_from_str_for_nom_parse,
    nom::{
        bytes::complete as bytes, character::complete as character, multi, sequence, IResult,
        Parser,
    },
    nom_extended::NomParse,
    nom_supreme::ParserExt,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ComboOperand {
    _0,
    _1,
    _2,
    _3,
    RegisterA,
    RegisterB,
    RegisterC,
}

impl Display for ComboOperand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::_0 => write!(f, "0"),
            Self::_1 => write!(f, "1"),
            Self::_2 => write!(f, "2"),
            Self::_3 => write!(f, "3"),
            Self::RegisterA => write!(f, "A"),
            Self::RegisterB => write!(f, "B"),
            Self::RegisterC => write!(f, "C"),
        }
    }
}

impl TryFrom<usize> for ComboOperand {
    type Error = String;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::_0),
            1 => Ok(Self::_1),
            2 => Ok(Self::_2),
            3 => Ok(Self::_3),
            4 => Ok(Self::RegisterA),
            5 => Ok(Self::RegisterB),
            6 => Ok(Self::RegisterC),
            _ => Err(format!("Invalid combo operand {value}")),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Opcode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Display for Opcode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Adv => write!(f, "adv"),
            Self::Bxl => write!(f, "bxl"),
            Self::Bst => write!(f, "bst"),
            Self::Jnz => write!(f, "jnz"),
            Self::Bxc => write!(f, "bxc"),
            Self::Out => write!(f, "out"),
            Self::Bdv => write!(f, "bdv"),
            Self::Cdv => write!(f, "cdv"),
        }
    }
}

impl TryFrom<usize> for Opcode {
    type Error = String;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Adv),
            1 => Ok(Self::Bxl),
            2 => Ok(Self::Bst),
            3 => Ok(Self::Jnz),
            4 => Ok(Self::Bxc),
            5 => Ok(Self::Out),
            6 => Ok(Self::Bdv),
            7 => Ok(Self::Cdv),
            _ => Err(format!("Invalid opcode {value}")),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Instruction(Opcode, usize);

impl Instruction {
    fn from_values(values: &[usize]) -> impl Iterator<Item = Self> {
        values
            .windows(2)
            .map(|window| Self(window[0].try_into().unwrap(), window[1]))
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.0, self.1)
    }
}

impl TryFrom<(usize, usize)> for Instruction {
    type Error = String;

    fn try_from((opcode, operand): (usize, usize)) -> Result<Self, Self::Error> {
        Ok(Self(opcode.try_into()?, operand))
    }
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    character::u32.map(|n| n as usize).parse(input)
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Cpu {
    instruction_pointer: usize,
    register_a: usize,
    register_b: usize,
    register_c: usize,
    values: Vec<usize>,
    instructions: Vec<Instruction>,
}

impl Cpu {
    #[cfg(test)]
    fn set_values(&mut self, values: Vec<usize>) {
        self.instructions = Instruction::from_values(&values).collect();
        self.values = values;
    }

    fn resolve_combo_operand(&self, combo_operand: ComboOperand) -> usize {
        match combo_operand {
            ComboOperand::_0 => 0,
            ComboOperand::_1 => 1,
            ComboOperand::_2 => 2,
            ComboOperand::_3 => 3,
            ComboOperand::RegisterA => self.register_a,
            ComboOperand::RegisterB => self.register_b,
            ComboOperand::RegisterC => self.register_c,
        }
    }

    fn step(&mut self) -> Result<Option<usize>, ()> {
        macro_rules! combo {
            ($operand:expr) => {
                match ComboOperand::try_from($operand) {
                    Ok(op) => self.resolve_combo_operand(op),
                    Err(e) => {
                        eprintln!("{e}");
                        self.instruction_pointer = self.values.len();
                        return Err(());
                    }
                }
            };
        }
        let Some(&Instruction(opcode, operand)) = self.instructions.get(self.instruction_pointer)
        else {
            return Err(());
        };
        self.instruction_pointer += 2;
        match opcode {
            Opcode::Adv => {
                self.register_a = self
                    .register_a
                    .checked_shr(combo!(operand) as _)
                    .unwrap_or(0)
            }
            Opcode::Bxl => self.register_b ^= operand,
            Opcode::Bst => self.register_b = combo!(operand) & 0b111,
            Opcode::Jnz => {
                if self.register_a != 0 {
                    self.instruction_pointer = operand
                }
            }
            Opcode::Bxc => self.register_b ^= self.register_c,
            Opcode::Out => {
                return Ok(Some(combo!(operand) % 8));
            }
            Opcode::Bdv => {
                self.register_b = self
                    .register_a
                    .checked_shr(combo!(operand) as _)
                    .unwrap_or(0)
            }
            Opcode::Cdv => {
                self.register_c = self
                    .register_a
                    .checked_shr(combo!(operand) as _)
                    .unwrap_or(0)
            }
        }
        Ok(None)
    }

    fn next_output(&mut self) -> Option<usize> {
        loop {
            if let Some(value) = self.step().ok()? {
                return Some(value);
            }
        }
    }
}

impl NomParse<&str> for Cpu {
    fn nom_parse(input: &str) -> IResult<&str, Self> {
        sequence::tuple((
            parse_usize
                .preceded_by(bytes::tag("Register A: "))
                .terminated(character::line_ending),
            parse_usize
                .preceded_by(bytes::tag("Register B: "))
                .terminated(character::line_ending),
            parse_usize
                .preceded_by(bytes::tag("Register C: "))
                .terminated(character::line_ending),
        ))
        .and(
            multi::separated_list1(bytes::tag(","), parse_usize)
                .map(|values| {
                    let instructions = Instruction::from_values(&values).collect::<Vec<_>>();
                    (values, instructions)
                })
                .preceded_by(bytes::tag("Program: ").preceded_by(character::line_ending))
                .terminated(character::line_ending),
        )
        .map(
            |((register_a, register_b, register_c), (values, instructions))| Self {
                instruction_pointer: 0,
                register_a,
                register_b,
                register_c,
                values,
                instructions,
            },
        )
        .parse(input)
    }
}

impl_from_str_for_nom_parse!(Cpu);

fn part1(input: &mut dyn BufRead) -> io::Result<String> {
    let mut cpu = io::read_to_string(input)?
        .parse::<Cpu>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let output = iter::from_fn(|| cpu.next_output())
        .map(|n| n.to_string())
        .collect::<Vec<_>>();
    Ok(output.join(","))
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    let mut cpu = io::read_to_string(input)?
        .parse::<Cpu>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let init_cpu = cpu.clone();
    let ip_always_even = cpu.instructions.iter().step_by(2).all(|instruction| !matches!(instruction, Instruction(Opcode::Jnz, target) if *target != 0 && *target != 2));
    if ip_always_even {
        let mut saved_as = vec![0];
        for value in cpu.values.iter().copied().rev() {
            let cpu = cpu.clone();
            saved_as = saved_as
                .into_iter()
                .flat_map(move |saved_a| {
                    let mut cpu = cpu.clone();
                    (0..8).filter_map(move |next| {
                        let new_a = saved_a * 8 + next;
                        cpu.register_a = new_a;
                        cpu.register_b = 0;
                        cpu.register_c = 0;
                        cpu.instruction_pointer = 0;
                        cpu.next_output()
                            .filter(|&output| output == value)
                            .map(|_| new_a)
                    })
                })
                .collect();
        }
        saved_as.sort();
        return Ok(saved_as[0]);
    }
    'a_start: for a in 8usize.pow(cpu.values.len() as u32 - 1)..8usize.pow(cpu.values.len() as _) {
        if a % 1_000_000 == 0 {
            eprintln!("Testing A: {a}");
        }
        cpu = Cpu {
            register_a: a,
            ..init_cpu.clone()
        };
        for i in 0..=cpu.values.len() {
            if cpu.values.get(i).copied() != cpu.next_output() {
                continue 'a_start;
            }
        }
        return Ok(a);
    }
    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "No value in usize range",
    ))
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2024 Day 17 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2024_17.txt")?))?
        );
    }
    {
        println!("Year 2024 Day 17 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2024_17.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use io::Cursor;

    use super::*;

    #[test]
    fn test_cpu() {
        let values = vec![2, 6];
        let mut cpu = Cpu {
            instruction_pointer: 0,
            register_a: 0,
            register_b: 0,
            register_c: 9,
            instructions: Instruction::from_values(&values).collect(),
            values,
        };
        assert_eq!(None, cpu.next_output());
        assert_eq!(cpu.register_b, 1);
        cpu.instruction_pointer = 0;
        cpu.register_a = 10;
        cpu.set_values(vec![5, 0, 5, 1, 5, 4]);
        assert_eq!(
            [0, 1, 2],
            iter::from_fn(|| cpu.next_output()).collect::<Vec<_>>()[..],
        );
        cpu.instruction_pointer = 0;
        cpu.register_a = 2024;
        cpu.set_values(vec![0, 1, 5, 4, 3, 0]);
        assert_eq!(
            [4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0],
            iter::from_fn(|| cpu.next_output()).collect::<Vec<_>>()[..],
        );
        assert_eq!(cpu.register_a, 0);
        cpu.instruction_pointer = 0;
        cpu.register_b = 29;
        cpu.set_values(vec![1, 7]);
        assert_eq!(None, cpu.next_output());
        assert_eq!(cpu.register_b, 26);
        cpu.instruction_pointer = 0;
        cpu.register_b = 2024;
        cpu.register_c = 43690;
        cpu.set_values(vec![4, 0]);
        assert_eq!(None, cpu.next_output());
        assert_eq!(cpu.register_b, 44354);
    }

    #[test]
    fn extra_cpu_test() {
        let values = vec![2, 4, 1, 1, 7, 2, 1, 5, 1, 0, 0, 3, 5, 5, 3, 0];
        let mut cpu = Cpu {
            instruction_pointer: 0,
            register_a: 64_197_994,
            register_b: 0,
            register_c: 0,
            instructions: values
                .windows(2)
                .map(|window| Instruction(window[0].try_into().unwrap(), window[1]))
                .collect(),
            values,
        };
        assert_eq!(
            [6, 1, 1, 6, 5, 3, 0, 2, 7],
            iter::from_fn(|| cpu.next_output()).collect::<Vec<_>>()[..],
        );
        assert_eq!(cpu.register_a, 0);
        assert_eq!(cpu.register_b, 7);
        assert_eq!(cpu.register_c, 0);
    }

    const TEST_DATA: &str = concat!(
        "Register A: 729\n",
        "Register B: 0\n",
        "Register C: 0\n",
        "\n",
        "Program: 0,1,5,4,3,0\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = "4,6,3,5,6,3,5,2,1,0";
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 117_440;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
