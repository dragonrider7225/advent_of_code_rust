use crate::parse::NomParse;
use nom::{
    branch, bytes::complete as bytes, character::complete as character, combinator as comb, multi,
    sequence, IResult,
};
use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
    fs, io, iter,
    ops::{BitAnd, BitOr, BitXor, Not},
};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Value(u8, u32);

impl Value {
    const MAX: Self = Self((1 << 4) - 1, u32::MAX);

    fn high_bits(value: u64) -> u8 {
        ((value >> 32) & 0xF).try_into().unwrap()
    }

    fn low_bits(value: u64) -> u32 {
        (value & u32::MAX.into(): u64).try_into().unwrap()
    }

    fn unwrap(self) -> u64 {
        ((self.0.into(): u64) << 32) + (self.1.into(): u64)
    }
}

impl BitAnd for Value {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0, self.1 & rhs.1)
    }
}

impl BitOr for Value {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0, self.1 | rhs.1)
    }
}

impl BitXor for Value {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0, self.1 ^ rhs.1)
    }
}

impl<'s> NomParse<'s> for Value {
    fn nom_parse(s: &str) -> IResult<&str, Self> {
        comb::map_res(u64::nom_parse, Self::try_from)(s)
    }
}

impl Not for Value {
    type Output = Self;

    fn not(self) -> Self::Output {
        self ^ Self::MAX
    }
}

impl TryFrom<u64> for Value {
    type Error = ();

    fn try_from(old: u64) -> Result<Self, Self::Error> {
        if old > Self::MAX.unwrap() {
            Err(())
        } else {
            Ok(Self(Self::high_bits(old), Self::low_bits(old)))
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
struct Mask {
    bits_set: Value,
    bits_unset: Value,
}

impl Mask {
    fn apply(&self, rhs: Value) -> Value {
        (rhs & !self.bits_unset) | self.bits_set
    }

    fn mask_idx(&self, idx: Value) -> impl Iterator<Item = Value> {
        fn float_bit(value: Value, bit: Value) -> impl Iterator<Item = Value> {
            vec![value & !bit, value | bit].into_iter()
        }

        let mut res: Box<dyn Iterator<Item = Value>> = box iter::once(idx | self.bits_set);
        for i in 0..36 {
            let bit = Value::try_from(1 << i).unwrap();
            if (self.bits_set | self.bits_unset) & bit == Value(0, 0) {
                res = box res.flat_map(move |value| float_bit(value, bit));
            }
        }
        res
    }
}

impl_from_str_for_nom_parse!(Mask);

impl<'s> NomParse<'s> for Mask {
    fn nom_parse(s: &str) -> IResult<&str, Self> {
        fn build(bits: Vec<char>) -> Result<Mask, <Value as TryFrom<u64>>::Error> {
            let (bits_set, bits_unset) =
                bits.into_iter()
                    .fold((0, 0), |(mut bits_set, mut bits_unset), c| {
                        bits_set <<= 1;
                        bits_unset <<= 1;
                        match c {
                            '0' => bits_unset |= 1,
                            '1' => bits_set |= 1,
                            'X' => {}
                            _ => unreachable!("{}", c),
                        }
                        (bits_set, bits_unset)
                    });
            Ok(Mask {
                bits_set: Value::try_from(bits_set)?,
                bits_unset: Value::try_from(bits_unset)?,
            })
        }

        comb::map_res(multi::many_m_n(36, 36, character::one_of("X01")), build)(s)
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Instruction {
    SetMask(Mask),
    SetValue { idx: Value, value: Value },
}

impl Instruction {
    fn execute(&self, mask: &mut Mask, values: &mut ProgramMemory) {
        match *self {
            Self::SetMask(new_mask) => *mask = new_mask,
            Self::SetValue { idx, value } => {
                values.set(idx, mask.apply(value));
            }
        }
    }

    fn execute_v2(&self, mask: &mut Mask, values: &mut ProgramMemory) {
        match *self {
            Self::SetMask(new_mask) => *mask = new_mask,
            Self::SetValue { idx, value } => {
                values.set_masked_idx(mask.mask_idx(idx), value);
            }
        }
    }
}

impl_from_str_for_nom_parse!(Instruction);

impl<'s> NomParse<'s> for Instruction {
    fn nom_parse(s: &str) -> IResult<&str, Self> {
        branch::alt((
            comb::map(
                sequence::preceded(bytes::tag("mask = "), Mask::nom_parse),
                Self::SetMask,
            ),
            comb::map(
                sequence::preceded(
                    bytes::tag("mem["),
                    sequence::separated_pair(
                        Value::nom_parse,
                        bytes::tag("] = "),
                        Value::nom_parse,
                    ),
                ),
                |(idx, value)| Self::SetValue { idx, value },
            ),
        ))(s)
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct ProgramMemory(HashMap<Value, Value>);

impl ProgramMemory {
    fn total(&self) -> u64 {
        self.0
            .values()
            .copied()
            .map(Value::unwrap)
            .filter(|&value| value != 0)
            .sum()
    }

    fn set(&mut self, idx: Value, value: Value) {
        self.0.insert(idx, value);
    }

    fn set_masked_idx(&mut self, idx: impl Iterator<Item = Value>, value: Value) {
        idx.for_each(|idx| self.set(idx, value));
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    fn run(self) -> ProgramMemory {
        let mut values = ProgramMemory(HashMap::new());
        let mut mask = Mask::default();
        for instruction in self.instructions {
            instruction.execute(&mut mask, &mut values);
        }
        values
    }

    fn run_v2(self) -> ProgramMemory {
        let mut values = ProgramMemory(HashMap::new());
        let mut mask = Mask::default();
        for instruction in self.instructions {
            instruction.execute_v2(&mut mask, &mut values);
        }
        values
    }
}

impl_from_str_for_nom_parse!(Program);

impl<'s> NomParse<'s> for Program {
    fn nom_parse(s: &str) -> IResult<&str, Self> {
        comb::map(
            sequence::terminated(
                multi::separated_list0(character::line_ending, Instruction::nom_parse),
                comb::opt(character::line_ending),
            ),
            |instructions| Self { instructions },
        )(s)
    }
}

pub(super) fn run() -> io::Result<()> {
    let program = fs::read_to_string("2020_14.txt")?
        .parse::<Program>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    {
        println!("Year 2020 Day 14 Part 1");
        let total = program.clone().run().total();
        println!(
            "The total of all values remaining after running the initialization program is {total}",
        );
    }
    {
        println!("Year 2020 Day 14 Part 2");
        let total = program.run_v2().total();
        println!(
            "The total of all values remaining after running the initialization program v2 is {total}",
        );
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[ignore]
    #[test]
    fn set_mask_parses() {
        let expected = Ok(Instruction::SetMask(Mask {
            bits_set: Value::try_from(0b1000000).unwrap(),
            bits_unset: Value::try_from(0b10).unwrap(),
        }));
        let actual = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".parse::<Instruction>();
        assert_eq!(expected, actual);
    }

    #[ignore]
    #[test]
    fn set_value_parses() {
        let expected = Ok(Instruction::SetValue {
            idx: Value::try_from(8).unwrap(),
            value: Value::try_from(11).unwrap(),
        });
        let actual = "mem[8] = 11".parse::<Instruction>();
        assert_eq!(expected, actual);
    }

    #[ignore]
    #[test]
    fn program_parses() {
        let expected = Ok(Program {
            instructions: vec![
                Instruction::SetMask(Mask {
                    bits_set: Value::try_from(0b1000000).unwrap(),
                    bits_unset: Value::try_from(0b10).unwrap(),
                }),
                Instruction::SetValue {
                    idx: Value::try_from(8).unwrap(),
                    value: Value::try_from(11).unwrap(),
                },
                Instruction::SetValue {
                    idx: Value::try_from(7).unwrap(),
                    value: Value::try_from(101).unwrap(),
                },
                Instruction::SetValue {
                    idx: Value::try_from(8).unwrap(),
                    value: Value::try_from(0).unwrap(),
                },
            ],
        });
        let actual = concat!(
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\n",
            "mem[8] = 11\n",
            "mem[7] = 101\n",
            "mem[8] = 0\n",
        )
        .parse::<Program>();
        assert_eq!(expected, actual);
    }

    #[ignore]
    #[test]
    fn program_runs_correctly() {
        let program = Program {
            instructions: vec![
                Instruction::SetMask(Mask {
                    bits_set: Value::try_from(0b1000000).unwrap(),
                    bits_unset: Value::try_from(0b10).unwrap(),
                }),
                Instruction::SetValue {
                    idx: Value::try_from(8).unwrap(),
                    value: Value::try_from(11).unwrap(),
                },
                Instruction::SetValue {
                    idx: Value::try_from(7).unwrap(),
                    value: Value::try_from(101).unwrap(),
                },
                Instruction::SetValue {
                    idx: Value::try_from(8).unwrap(),
                    value: Value::try_from(0).unwrap(),
                },
            ],
        };
        let expected = 165;
        let actual = program.run().total();
        assert_eq!(expected, actual);
    }

    #[ignore]
    #[test]
    fn program_v2_masks_address_correctly() {
        let expected = vec![26, 27, 58, 59];

        let address = Value::try_from(42).unwrap();
        let mask = Mask {
            bits_set: Value::try_from(0b10010).unwrap(),
            bits_unset: !Value::try_from(0b110011).unwrap(),
        };
        let mut actual = mask
            .mask_idx(address)
            .map(Value::unwrap)
            .collect::<Vec<_>>();
        actual.sort();
        assert_eq!(expected, actual);
    }

    #[ignore]
    #[test]
    fn mask_v2_sets_memory_correctly() {
        let mut memory = ProgramMemory(HashMap::new());
        memory.set_masked_idx(
            Mask {
                bits_set: Value::try_from(0b10010).unwrap(),
                bits_unset: !Value::try_from(0b110011).unwrap(),
            }
            .mask_idx(Value::try_from(42).unwrap()),
            Value::try_from(100).unwrap(),
        );
        assert_eq!(memory.0[&Value::try_from(26).unwrap()].unwrap(), 100);
        assert_eq!(memory.0[&Value::try_from(27).unwrap()].unwrap(), 100);
        assert_eq!(memory.0[&Value::try_from(58).unwrap()].unwrap(), 100);
        assert_eq!(memory.0[&Value::try_from(59).unwrap()].unwrap(), 100);
    }

    #[ignore]
    #[test]
    fn program_v2_masks_address_correctly_2() {
        let expected = vec![16, 17, 18, 19, 24, 25, 26, 27];

        let address = Value::try_from(26).unwrap();
        let mask = Mask {
            bits_set: Value::try_from(0).unwrap(),
            bits_unset: !Value::try_from(0b1011).unwrap(),
        };
        let mut actual = mask
            .mask_idx(address)
            .map(Value::unwrap)
            .collect::<Vec<_>>();
        actual.sort();
        assert_eq!(expected, actual);
    }

    #[ignore]
    #[test]
    fn program_v2_runs_correctly() {
        let program = Program {
            instructions: vec![
                Instruction::SetMask(Mask {
                    bits_set: Value::try_from(0b10010).unwrap(),
                    bits_unset: !Value::try_from(0b110011).unwrap(),
                }),
                Instruction::SetValue {
                    idx: Value::try_from(42).unwrap(),
                    value: Value::try_from(100).unwrap(),
                },
                Instruction::SetMask(Mask {
                    bits_set: Value::try_from(0).unwrap(),
                    bits_unset: !Value::try_from(0b1011).unwrap(),
                }),
                Instruction::SetValue {
                    idx: Value::try_from(26).unwrap(),
                    value: Value::try_from(1).unwrap(),
                },
            ],
        };
        let expected = 208;
        let memory = program.run_v2();
        let actual = memory.total();
        assert_eq!(expected, actual);
    }
}
