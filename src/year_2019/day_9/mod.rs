use std::io::{self, Write};

use extended_io::pipe::{PipeRead, PipeWrite};

use super::intcode_interpreter::IntcodeInterpreter;

pub(super) fn run() -> io::Result<()> {
    let prog = IntcodeInterpreter::<PipeRead, PipeWrite>::read_from_file("2019_9.txt")?;
    {
        println!("Year 2019 Day 9 Part 1");
        print!("Enter mode id: ");
        io::stdout().flush()?;
        prog.dup::<PipeRead, PipeWrite>().run();
    }
    {
        println!("Year 2019 Day 9 Part 2");
        print!("Enter mode id: ");
        io::stdout().flush()?;
        prog.run();
    }
    Ok(())
}
