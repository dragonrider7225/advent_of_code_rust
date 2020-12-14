use crate::parse::NomParse;
use nom::{
    branch, bytes::complete as bytes, character::complete as character, combinator as comb,
    sequence, IResult,
};
use std::{collections::HashSet, convert::TryFrom, fs, io};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Instruction {
    NoOp(isize),
    Accumulate(i32),
    Jump(isize),
}

impl_from_str_for_nom_parse!(Instruction);

impl<'s> NomParse<'s> for Instruction {
    fn nom_parse(s: &str) -> IResult<&str, Self> {
        branch::alt((
            comb::map(
                sequence::pair(
                    bytes::tag("nop "),
                    comb::map(
                        sequence::pair(character::one_of("+-"), u32::nom_parse),
                        |(sign, val)| {
                            let val = isize::try_from(val).unwrap();
                            if sign == '-' {
                                -val
                            } else {
                                val
                            }
                        },
                    ),
                ),
                |(_, val)| Self::NoOp(val),
            ),
            comb::map(
                sequence::pair(
                    bytes::tag("acc "),
                    comb::map(
                        sequence::pair(character::one_of("+-"), u32::nom_parse),
                        |(sign, val)| {
                            let val = i32::try_from(val).unwrap();
                            if sign == '-' {
                                -val
                            } else {
                                val
                            }
                        },
                    ),
                ),
                |(_, val)| Self::Accumulate(val),
            ),
            comb::map(
                sequence::pair(
                    bytes::tag("jmp "),
                    comb::map(
                        sequence::pair(character::one_of("+-"), u32::nom_parse),
                        |(sign, val)| {
                            let val = isize::try_from(val).unwrap();
                            if sign == '-' {
                                -val
                            } else {
                                val
                            }
                        },
                    ),
                ),
                |(_, val)| Self::Jump(val),
            ),
        ))(s)
    }
}

#[derive(Clone, Copy, Debug)]
struct State<'instructions> {
    instructions: &'instructions [Instruction],
    instruction_pointer: usize,
    accumulator: i32,
}

impl<'instructions> State<'instructions> {
    fn new(instructions: &'instructions [Instruction]) -> Self {
        Self {
            instructions,
            instruction_pointer: 0,
            accumulator: 0,
        }
    }

    /// Execute instructions until the next instruction to execute either has been previously
    /// executed or is after the end of the instruction slice. Returns `Ok(accumulator)` if the
    /// program ran out of instructions and `Err(accumulator)` if the program would have entered an
    /// infinite loop.
    fn run(mut self) -> Result<i32, i32> {
        let mut instructions_executed = HashSet::new();
        while self.instruction_pointer < self.instructions.len()
            && !instructions_executed.contains(&self.instruction_pointer)
        {
            instructions_executed.insert(self.instruction_pointer);
            let mut jumped = false;
            match self.instructions[self.instruction_pointer] {
                Instruction::NoOp(_) => {}
                Instruction::Accumulate(delta) => self.accumulator += delta,
                Instruction::Jump(delta) => {
                    if delta < 0 {
                        self.instruction_pointer -= usize::try_from(-delta).unwrap();
                        jumped = true;
                    } else {
                        self.instruction_pointer += usize::try_from(delta).unwrap();
                        jumped = true;
                    }
                }
            }
            if !jumped {
                self.instruction_pointer += 1;
            }
        }
        if self.instruction_pointer < self.instructions.len() {
            Err(self.accumulator)
        } else {
            Ok(self.accumulator)
        }
    }

    /// Count the number of noops, accumulates, and jumps.
    fn _count_instructions(&self) -> (usize, usize, usize) {
        self.instructions.iter().fold(
            (0, 0, 0),
            |(mut noops, mut accumulates, mut jumps), &instruction| {
                match instruction {
                    Instruction::NoOp(_) => noops += 1,
                    Instruction::Accumulate(_) => accumulates += 1,
                    Instruction::Jump(_) => jumps += 1,
                }
                (noops, accumulates, jumps)
            },
        )
    }
}

#[allow(unreachable_code)]
pub(super) fn run() -> io::Result<()> {
    let instructions = fs::read_to_string("2020_08.txt")?
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let state = State::new(&instructions);
    {
        println!("Year 2020 Day 8 Part 1");
        println!(
            "Immediately before an instruction is first executed for the second time, the value of the accumulator is {}",
            state.run().expect_err("Program ran out of instructions before looping"),
        );
    }
    {
        println!("Year 2020 Day 8 Part 2");
        let mut local_instructions = instructions.clone();
        let res = (0..instructions.len())
            .filter_map(|idx| match instructions[idx] {
                Instruction::NoOp(delta) => Some((idx, Instruction::Jump(delta))),
                Instruction::Accumulate(_) => None,
                Instruction::Jump(delta) => Some((idx, Instruction::NoOp(delta))),
            })
            .fold(None, |acc, (idx, replacement)| {
                acc.or_else(|| {
                    local_instructions[idx] = replacement;
                    let res = State::new(&local_instructions).run().ok();
                    local_instructions[idx] = instructions[idx];
                    res
                })
            })
            .expect("No single no-op or jump instruction found to remove the infinite loop");
        println!("The program terminates with {} in the accumulator", res);
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn noop_parses() {
        let expected = Ok(Instruction::NoOp(0));
        let actual = "nop +0".parse();
        assert_eq!(expected, actual);
    }

    #[test]
    fn noop_can_have_any_argument() {
        let expected = Ok(Instruction::NoOp(7));
        assert_eq!(expected, "nop +7".parse());
        let expected = Ok(Instruction::NoOp(-32));
        assert_eq!(expected, "nop -32".parse());
    }

    #[test]
    fn accumulate_parses() {
        let expected = Ok(Instruction::Accumulate(5));
        let actual = "acc +5".parse();
        assert_eq!(expected, actual);
    }

    #[test]
    fn accumulate_negative_parses() {
        let expected = Ok(Instruction::Accumulate(-5));
        let actual = "acc -5".parse();
        assert_eq!(expected, actual);
    }

    #[test]
    fn state_runs_correctly() {
        use Instruction::{Accumulate, Jump, NoOp};

        let instructions = [
            NoOp(0),
            Accumulate(1),
            Jump(4),
            Accumulate(3),
            Jump(-3),
            Accumulate(-99),
            Accumulate(1),
            Jump(-4),
            Accumulate(6),
        ];
        let state = State::new(&instructions);
        let expected = Err(5);
        let actual = state.run();
        assert_eq!(expected, actual);
    }
}
