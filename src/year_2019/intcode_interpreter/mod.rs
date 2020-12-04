use std::{
    convert::{TryFrom, TryInto},
    io::{self, BufRead, Write},
    ops::{Index, IndexMut},
    path::Path,
    str::FromStr,
};

use crate::parse::NomParse;

use nom::{
    bytes::complete as bytes,
    character::complete as character,
    combinator as comb,
    multi,
    sequence,
    IResult,
};

use extended_io::{self as eio, pipe::{PipeRead, PipeWrite}};

enum ParamMode {
    Address,
    Immediate,
    Relative,
}

impl TryFrom<i64> for ParamMode {
    type Error = String;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ParamMode::Address),
            1 => Ok(ParamMode::Immediate),
            2 => Ok(ParamMode::Relative),
            _ => Err(format!("Invalid parameter mode {}", value)),
        }
    }
}

enum Instruction {
    Add(ParamMode, ParamMode, ParamMode),
    Mul(ParamMode, ParamMode, ParamMode),
    Read(ParamMode),
    Write(ParamMode),
    JmpIfTrue(ParamMode, ParamMode),
    JmpIfFalse(ParamMode, ParamMode),
    LessThan(ParamMode, ParamMode, ParamMode),
    Equal(ParamMode, ParamMode, ParamMode),
    MRB(ParamMode),
    Halt,
}

impl TryFrom<i64> for Instruction {
    type Error = String;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value % 100 {
            1 => {
                let par1_mode = ParamMode::try_from((value / 100) % 10)?;
                let par2_mode = ParamMode::try_from((value / 1000) % 10)?;
                let out_mode = ParamMode::try_from((value / 10_000) % 10)?;
                if let ParamMode::Immediate = out_mode {
                    Err("Invalid parameter mode for Add".to_string())
                } else {
                    Ok(Instruction::Add(par1_mode, par2_mode, out_mode))
                }
            }
            2 => {
                let par1_mode = ParamMode::try_from((value / 100) % 10)?;
                let par2_mode = ParamMode::try_from((value / 1000) % 10)?;
                let out_mode = ParamMode::try_from((value / 10_000) % 10)?;
                if let ParamMode::Immediate = out_mode {
                    Err("Invalid parameter mode for Mul".to_string())
                } else {
                    Ok(Instruction::Mul(par1_mode, par2_mode, out_mode))
                }
            }
            3 => {
                let par_mode = ParamMode::try_from((value / 100) % 10)?;
                if let ParamMode::Immediate = par_mode {
                    Err("Invalid parameter mode for Read".to_string())
                } else {
                    Ok(Instruction::Read(par_mode))
                }
            }
            4 => {
                let par_mode = ParamMode::try_from((value / 100) % 10)?;
                Ok(Instruction::Write(par_mode))
            }
            5 => {
                let par1_mode = ParamMode::try_from((value / 100) % 10)?;
                let par2_mode = ParamMode::try_from((value / 1000) % 10)?;
                Ok(Instruction::JmpIfTrue(par1_mode, par2_mode))
            }
            6 => {
                let par1_mode = ParamMode::try_from((value / 100) % 10)?;
                let par2_mode = ParamMode::try_from((value / 1000) % 10)?;
                Ok(Instruction::JmpIfFalse(par1_mode, par2_mode))
            }
            7 => {
                let par1_mode = ParamMode::try_from((value / 100) % 10)?;
                let par2_mode = ParamMode::try_from((value / 1000) % 10)?;
                let out_mode = ParamMode::try_from((value / 10_000) % 10)?;
                if let ParamMode::Immediate = out_mode {
                    Err("Invalid parameter mode for LessThan".to_string())
                } else {
                    Ok(Instruction::LessThan(par1_mode, par2_mode, out_mode))
                }
            }
            8 => {
                let par1_mode = ParamMode::try_from((value / 100) % 10)?;
                let par2_mode = ParamMode::try_from((value / 1000) % 10)?;
                let out_mode = ParamMode::try_from((value / 10_000) % 10)?;
                if let ParamMode::Immediate = out_mode {
                    Err("Invalid parameter mode for Equal".to_string())
                } else {
                    Ok(Instruction::Equal(par1_mode, par2_mode, out_mode))
                }
            }
            9 => {
                let par_mode = ParamMode::try_from((value / 100) % 10)?;
                Ok(Instruction::MRB(par_mode))
            }
            99 => Ok(Instruction::Halt),
            opcode => Err(format!("Invalid opcode {}", opcode)),
        }
    }
}

#[derive(Clone)]
pub struct IntcodeProgram {
    values: Vec<i64>,
}

impl IntcodeProgram {
    pub fn new(values: Vec<i64>) -> Self {
        IntcodeProgram { values }
    }
}

impl From<Vec<i64>> for IntcodeProgram {
    fn from(values: Vec<i64>) -> Self {
        Self::new(values)
    }
}

impl Index<usize> for IntcodeProgram {
    type Output = i64;

    fn index(&self, index: usize) -> &Self::Output {
        // This is memory-safe as long as the `Vec` referred to by `values`
        // (`self.values`) is not accessed except through `values` until
        // `values` is dropped because the pointer is a reference to a
        // `Vec<i64>` which lives longer than `values` does.
        let values: &mut _ = unsafe {
            let ptr = &self.values as *const Vec<i64> as *mut  Vec<i64>;
            ptr.as_mut().unwrap()
        };
        if values.len() <= index {
            values.resize_with(index + 1, Default::default);
        }
        std::mem::drop(values);
        &self.values[index]
    }
}

impl IndexMut<usize> for IntcodeProgram {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if self.values.len() <= index {
            self.values.resize_with(index + 1, Default::default);
        }
        &mut self.values[index]
    }
}

pub struct IntcodeInterpreter<R = PipeRead, W = PipeWrite>
where
  R: BufRead + Sized,
  W: Write + Sized,
{
    pc: usize,
    prog: IntcodeProgram,
    input: Option<R>,
    output: Option<W>,
    relative_base: i64,
    debug: bool,
}

impl IntcodeInterpreter<PipeRead, PipeWrite> {
    pub fn run_piped(mut self) -> i64 {
        loop {
            let instr = self.prog[self.pc];
            if self.debug {
                println!("Executing instruction {} at {}", instr, self.pc);
            }
            match Instruction::try_from(instr).unwrap() {
                Instruction::Add(par1_mode, par2_mode, out_mode) => {
                    let par1 = self.prog[self.pc + 1];
                    let par1 = self.get_input_parameter(par1_mode, par1);
                    let par2 = self.prog[self.pc + 2];
                    let par2 = self.get_input_parameter(par2_mode, par2);
                    let out = self.prog[self.pc + 3];
                    let out = self.get_output_parameter(out_mode, out);
                    *out = par1 + par2;
                    self.pc += 4;
                }
                Instruction::Mul(par1_mode, par2_mode, out_mode) => {
                    let par1 = self.prog[self.pc + 1];
                    let par1 = self.get_input_parameter(par1_mode, par1);
                    let par2 = self.prog[self.pc + 2];
                    let par2 = self.get_input_parameter(par2_mode, par2);
                    let out = self.prog[self.pc + 3];
                    let out = self.get_output_parameter(out_mode, out);
                    *out = par1 * par2;
                    self.pc += 4;
                }
                Instruction::Read(out_mode) => {
                    let value = self.input.as_mut()
                        .map(|r| eio::read_i64(r).expect("Errored on read"))
                        .unwrap_or_else(|| {
                            let mut line = String::new();
                            io::stdin().lock().read_line(&mut line).unwrap();
                            line.parse().unwrap()
                        });
                    let out = self.prog[self.pc + 1];
                    let out = self.get_output_parameter(out_mode, out);
                    *out = value;
                    self.pc += 2;
                }
                Instruction::Write(par_mode) => {
                    let par = self.prog[self.pc + 1];
                    let par = self.get_input_parameter(par_mode, par);
                    self.output.as_mut().map(|w| eio::write_i64(w, par).expect("Error on write"))
                        .unwrap_or_else(|| println!("{}\n", par));
                    self.pc += 2;
                }
                Instruction::JmpIfTrue(par1_mode, par2_mode) => {
                    let par1 = self.prog[self.pc + 1];
                    let par1 = self.get_input_parameter(par1_mode, par1);
                    if par1 != 0 {
                        let par2 = self.prog[self.pc + 2];
                        let par2 = self.get_input_parameter(par2_mode, par2);
                        self.pc = par2.try_into().unwrap();
                    } else {
                        self.pc += 3;
                    }
                }
                Instruction::JmpIfFalse(par1_mode, par2_mode) => {
                    let par1 = self.prog[self.pc + 1];
                    let par1 = self.get_input_parameter(par1_mode, par1);
                    if par1 == 0 {
                        let par2 = self.prog[self.pc + 2];
                        let par2 = self.get_input_parameter(par2_mode, par2);
                        self.pc = par2.try_into().unwrap();
                    } else {
                        self.pc += 3;
                    }
                }
                Instruction::LessThan(par1_mode, par2_mode, out_mode) => {
                    let par1 = self.prog[self.pc + 1];
                    let par1 = self.get_input_parameter(par1_mode, par1);
                    let par2 = self.prog[self.pc + 2];
                    let par2 = self.get_input_parameter(par2_mode, par2);
                    let out = self.prog[self.pc + 3];
                    let out = self.get_output_parameter(out_mode, out);
                    *out = if par1 < par2 {
                        1
                    } else {
                        0
                    };
                    self.pc += 4;
                }
                Instruction::Equal(par1_mode, par2_mode, out_mode) => {
                    let par1 = self.prog[self.pc + 1];
                    let par1 = self.get_input_parameter(par1_mode, par1);
                    let par2 = self.prog[self.pc + 2];
                    let par2 = self.get_input_parameter(par2_mode, par2);
                    let out = self.prog[self.pc + 3];
                    let out = self.get_output_parameter(out_mode, out);
                    *out = if par1 == par2 {
                        1
                    } else {
                        0
                    };
                    self.pc += 4;
                }
                Instruction::MRB(par_mode) => {
                    let par = self.prog[self.pc + 1];
                    let par = self.get_input_parameter(par_mode, par);
                    self.relative_base += par;
                    self.pc += 2;
                }
                Instruction::Halt => return self.prog[0],
            }
        }
    }
}

impl<R, W> IntcodeInterpreter<R, W>
where
  R: BufRead + Sized,
  W: Write + Sized,
{
    pub fn new(prog: IntcodeProgram) -> Self {
        Self::with_streams(prog, None, None)
    }

    pub fn with_streams(prog: IntcodeProgram, input: Option<R>, output: Option<W>) -> Self {
        Self {
            pc: 0,
            prog,
            input,
            output,
            relative_base: 0,
            debug: false,
        }
    }

    pub fn read_from_file<P>(path: P) -> io::Result<Self>
    where
      P: AsRef<Path>,
    {
        std::fs::read_to_string(path)?.parse().map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }

    pub fn dup<R1, W1>(&self) -> IntcodeInterpreter<R1, W1>
    where
      R1: BufRead + Sized,
      W1: Write + Sized,
    {
        let mut ret = IntcodeInterpreter::new(self.prog.clone());
        ret.set_debug(self.debug);
        ret
    }

    pub fn dup_with<R1, W1>(&self, input: R1, output: W1) -> IntcodeInterpreter<R1, W1>
    where
      R1: BufRead + Sized,
      W1: Write + Sized,
    {
        let mut ret = self.dup();
        ret.set_input_stream(input);
        ret.set_output_stream(output);
        ret
    }

    pub fn get_program(&self) -> IntcodeProgram {
        self.prog.clone()
    }

    fn get_input_parameter(&self, par_mode: ParamMode, par: i64) -> i64 {
        match par_mode {
            ParamMode::Address => {
                let address: usize = par.try_into().unwrap();
                self.prog[address]
            }
            ParamMode::Immediate => par,
            ParamMode::Relative => {
                let address: usize = (par + self.relative_base).try_into().unwrap();
                self.prog[address]
            }
        }
    }

    pub fn set_input_stream(&mut self, input: R) {
        self.input = Some(input);
    }

    pub fn set_output_stream(&mut self, output: W) {
        self.output = Some(output);
    }

    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }

    fn get_output_parameter(&mut self, par_mode: ParamMode, par: i64) -> &mut i64 {
        match par_mode {
            ParamMode::Address => {
                let address: usize = par.try_into().unwrap();
                &mut self.prog[address]
            }
            ParamMode::Immediate => {
                panic!("Can't write to immediate");
            }
            ParamMode::Relative => {
                let address: usize = (par + self.relative_base).try_into().unwrap();
                &mut self.prog[address]
            }
        }
    }

    pub fn run(mut self) -> i64 {
        loop {
            let instr = self.prog[self.pc];
            if self.debug {
                println!("Executing instruction {} at {}", instr, self.pc);
            }
            match Instruction::try_from(instr).unwrap() {
                Instruction::Add(par1_mode, par2_mode, out_mode) => {
                    let par1 = self.prog[self.pc + 1];
                    let par1 = self.get_input_parameter(par1_mode, par1);
                    let par2 = self.prog[self.pc + 2];
                    let par2 = self.get_input_parameter(par2_mode, par2);
                    let out = self.prog[self.pc + 3];
                    let out = self.get_output_parameter(out_mode, out);
                    *out = par1 + par2;
                    self.pc += 4;
                }
                Instruction::Mul(par1_mode, par2_mode, out_mode) => {
                    let par1 = self.prog[self.pc + 1];
                    let par1 = self.get_input_parameter(par1_mode, par1);
                    let par2 = self.prog[self.pc + 2];
                    let par2 = self.get_input_parameter(par2_mode, par2);
                    let out = self.prog[self.pc + 3];
                    let out = self.get_output_parameter(out_mode, out);
                    *out = par1 * par2;
                    self.pc += 4;
                }
                Instruction::Read(out_mode) => {
                    let mut line = String::new();
                    self.input.as_mut()
                        .map(|r| {
                            match r.read_line(&mut line) {
                                Ok(0) => panic!("Ran out of input"),
                                Ok(n) => n,
                                Err(e) => panic!("Errored on read: {}", e),
                            }
                        })
                        .unwrap_or_else(|| io::stdin().lock().read_line(&mut line).unwrap());
                    let out = self.prog[self.pc + 1];
                    let out = self.get_output_parameter(out_mode, out);
                    *out = line.trim().parse().unwrap();
                    self.pc += 2;
                }
                Instruction::Write(par_mode) => {
                    let par = self.prog[self.pc + 1];
                    let par = self.get_input_parameter(par_mode, par);
                    let args = format!("{}\n", par);
                    match self.output.as_mut() {
                        Some(out) => write!(out, "{}", args),
                        None => write!(io::stdout().lock(), "{}", args),
                    }.unwrap();
                    self.pc += 2;
                }
                Instruction::JmpIfTrue(par1_mode, par2_mode) => {
                    let par1 = self.prog[self.pc + 1];
                    let par1 = self.get_input_parameter(par1_mode, par1);
                    if par1 != 0 {
                        let par2 = self.prog[self.pc + 2];
                        let par2 = self.get_input_parameter(par2_mode, par2);
                        self.pc = par2.try_into().unwrap();
                    } else {
                        self.pc += 3;
                    }
                }
                Instruction::JmpIfFalse(par1_mode, par2_mode) => {
                    let par1 = self.prog[self.pc + 1];
                    let par1 = self.get_input_parameter(par1_mode, par1);
                    if par1 == 0 {
                        let par2 = self.prog[self.pc + 2];
                        let par2 = self.get_input_parameter(par2_mode, par2);
                        self.pc = par2.try_into().unwrap();
                    } else {
                        self.pc += 3;
                    }
                }
                Instruction::LessThan(par1_mode, par2_mode, out_mode) => {
                    let par1 = self.prog[self.pc + 1];
                    let par1 = self.get_input_parameter(par1_mode, par1);
                    let par2 = self.prog[self.pc + 2];
                    let par2 = self.get_input_parameter(par2_mode, par2);
                    let out = self.prog[self.pc + 3];
                    let out = self.get_output_parameter(out_mode, out);
                    *out = if par1 < par2 {
                        1
                    } else {
                        0
                    };
                    self.pc += 4;
                }
                Instruction::Equal(par1_mode, par2_mode, out_mode) => {
                    let par1 = self.prog[self.pc + 1];
                    let par1 = self.get_input_parameter(par1_mode, par1);
                    let par2 = self.prog[self.pc + 2];
                    let par2 = self.get_input_parameter(par2_mode, par2);
                    let out = self.prog[self.pc + 3];
                    let out = self.get_output_parameter(out_mode, out);
                    *out = if par1 == par2 {
                        1
                    } else {
                        0
                    };
                    self.pc += 4;
                }
                Instruction::MRB(par_mode) => {
                    let par = self.prog[self.pc + 1];
                    let par = self.get_input_parameter(par_mode, par);
                    self.relative_base += par;
                    self.pc += 2;
                }
                Instruction::Halt => return self.prog[0],
            }
        }
    }
}

impl<R, W> From<IntcodeProgram> for IntcodeInterpreter<R, W>
where
  R: BufRead + Sized,
  W: Write + Sized,
{
    fn from(prog: IntcodeProgram) -> Self {
        Self::new(prog)
    }
}

impl<R, W> From<Vec<i64>> for IntcodeInterpreter<R, W>
where
  R: BufRead + Sized,
  W: Write + Sized,
{
    fn from(prog: Vec<i64>) -> Self {
        Self::new(IntcodeProgram::new(prog))
    }
}

impl<'s, R, W> NomParse<'s> for IntcodeInterpreter<R, W>
where
  R: BufRead + Sized,
  W: Write + Sized,
{
    fn nom_parse(s: &str) -> IResult<&str, Self> {
        let parse_i64 = comb::map(
            comb::recognize(sequence::pair(comb::opt(bytes::tag("-")), character::digit1)),
            |s: &str| s.parse::<i64>().expect("Invalid i64"),
        );
        let snl = multi::separated_list1;
        comb::map(snl(bytes::tag(","), parse_i64), Self::from)(s)
    }
}

impl<R, W> FromStr for IntcodeInterpreter<R, W>
where
  R: BufRead + Sized,
  W: Write + Sized,
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        comb::cut(Self::nom_parse)(s).map(|(_, x)| x).map_err(|e| format!("{:?}", e))
    }
}
