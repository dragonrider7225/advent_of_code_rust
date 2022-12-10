use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    str::FromStr,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Instruction {
    Addx(i32),
    Noop,
}

impl FromStr for Instruction {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[..4] {
            "addx" => {
                Ok(Self::Addx(s[5..].parse().map_err(|e| {
                    io::Error::new(io::ErrorKind::InvalidData, e)
                })?))
            }
            "noop" => Ok(Self::Noop),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Step {
    AddxPart1,
    AddxPart2(i32),
    Noop,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Registers {
    x: i32,
}

impl Default for Registers {
    fn default() -> Self {
        Self { x: 1 }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Cpu {
    cycle_num: i32,
    registers: Registers,
    // First step is `steps.last()`.
    steps: Vec<Step>,
}

impl Cpu {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            cycle_num: 1,
            registers: Registers::default(),
            steps: instructions
                .into_iter()
                .rev()
                .flat_map(|instruction| match instruction {
                    Instruction::Addx(v) => {
                        Box::new([Step::AddxPart1, Step::AddxPart2(v)].into_iter().rev())
                            as Box<dyn Iterator<Item = _>>
                    }
                    Instruction::Noop => Box::new([Step::Noop].into_iter()),
                })
                .collect(),
        }
    }

    /// Removes the next step, increases the cycle number, executes the step, and returns `true`. If
    /// no steps remained, returns `false` instead.
    fn step(&mut self) -> bool {
        let ret = match self.steps.pop() {
            None => false,
            Some(Step::AddxPart1 | Step::Noop) => true,
            Some(Step::AddxPart2(v)) => {
                self.registers.x += v;
                true
            }
        };
        if ret {
            self.cycle_num += 1;
        }
        ret
    }

    /// Returns the sum of the signal strengths during the cycles `c` such that `c % 20 == 0`.
    fn run_program(&mut self) -> i32 {
        let mut ret = 0;
        while self.step() {
            if self.cycle_num % 40 == 20 {
                ret += self.cycle_num * self.registers.x;
            }
        }
        ret
    }

    fn draw_sprite(&mut self) -> String {
        let mut ret = String::new();
        loop {
            let color = if (((self.cycle_num - 1) % 40) - self.registers.x).abs() <= 1 {
                '#'
            } else {
                '.'
            };
            if !self.step() {
                break;
            }
            ret.push(color);
            if (self.cycle_num - 1) % 40 == 0 {
                ret.push('\n');
            }
        }
        ret
    }
}

fn part1(input: &mut dyn BufRead) -> io::Result<i32> {
    let instructions = input
        .lines()
        .map(|line| {
            line?
                .parse()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let mut cpu = Cpu::new(instructions);
    Ok(cpu.run_program())
}

fn part2(input: &mut dyn BufRead) -> io::Result<String> {
    let instructions = input
        .lines()
        .map(|line| {
            line?
                .parse()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let mut cpu = Cpu::new(instructions);
    Ok(cpu.draw_sprite())
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2022 Day 10 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2022_10.txt")?))?
        );
    }
    {
        println!("Year 2022 Day 10 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2022_10.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        "addx 15\n",
        "addx -11\n",
        "addx 6\n",
        "addx -3\n",
        "addx 5\n",
        "addx -1\n",
        "addx -8\n",
        "addx 13\n",
        "addx 4\n",
        "noop\n",
        "addx -1\n",
        "addx 5\n",
        "addx -1\n",
        "addx 5\n",
        "addx -1\n",
        "addx 5\n",
        "addx -1\n",
        "addx 5\n",
        "addx -1\n",
        "addx -35\n",
        "addx 1\n",
        "addx 24\n",
        "addx -19\n",
        "addx 1\n",
        "addx 16\n",
        "addx -11\n",
        "noop\n",
        "noop\n",
        "addx 21\n",
        "addx -15\n",
        "noop\n",
        "noop\n",
        "addx -3\n",
        "addx 9\n",
        "addx 1\n",
        "addx -3\n",
        "addx 8\n",
        "addx 1\n",
        "addx 5\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "addx -36\n",
        "noop\n",
        "addx 1\n",
        "addx 7\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "addx 2\n",
        "addx 6\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "addx 1\n",
        "noop\n",
        "noop\n",
        "addx 7\n",
        "addx 1\n",
        "noop\n",
        "addx -13\n",
        "addx 13\n",
        "addx 7\n",
        "noop\n",
        "addx 1\n",
        "addx -33\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "addx 2\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "addx 8\n",
        "noop\n",
        "addx -1\n",
        "addx 2\n",
        "addx 1\n",
        "noop\n",
        "addx 17\n",
        "addx -9\n",
        "addx 1\n",
        "addx 1\n",
        "addx -3\n",
        "addx 11\n",
        "noop\n",
        "noop\n",
        "addx 1\n",
        "noop\n",
        "addx 1\n",
        "noop\n",
        "noop\n",
        "addx -13\n",
        "addx -19\n",
        "addx 1\n",
        "addx 3\n",
        "addx 26\n",
        "addx -30\n",
        "addx 12\n",
        "addx -1\n",
        "addx 3\n",
        "addx 1\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "addx -9\n",
        "addx 18\n",
        "addx 1\n",
        "addx 2\n",
        "noop\n",
        "noop\n",
        "addx 9\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "addx -1\n",
        "addx 2\n",
        "addx -37\n",
        "addx 1\n",
        "addx 3\n",
        "noop\n",
        "addx 15\n",
        "addx -21\n",
        "addx 22\n",
        "addx -6\n",
        "addx 1\n",
        "noop\n",
        "addx 2\n",
        "addx 1\n",
        "noop\n",
        "addx -10\n",
        "noop\n",
        "noop\n",
        "addx 20\n",
        "addx 1\n",
        "addx 2\n",
        "addx 2\n",
        "addx -6\n",
        "addx -11\n",
        "noop\n",
        "noop\n",
        "noop\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 13_140;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = concat!(
            "##..##..##..##..##..##..##..##..##..##..\n",
            "###...###...###...###...###...###...###.\n",
            "####....####....####....####....####....\n",
            "#####.....#####.....#####.....#####.....\n",
            "######......######......######......####\n",
            "#######.......#######.......#######.....\n",
        );
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
