use crate::year_2019::intcode_interpreter::IntcodeInterpreter;

use std::io;

use extended_io::pipe::{PipeRead, PipeWrite};

pub(super) fn run() -> io::Result<()> {
    let prog = IntcodeInterpreter::<PipeRead, PipeWrite>::read_from_file("2019_5.txt")?;
    {
        println!("Day 5 Part 1");
        prog.dup::<PipeRead, PipeWrite>().run();
    }
    {
        println!("Day 5 Part 2");
        prog.run();
    }
    Ok(())
}
