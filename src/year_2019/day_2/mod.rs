use super::intcode_interpreter::IntcodeInterpreter;

use std::io;

use extended_io::pipe::{PipeRead, PipeWrite};

pub(super) fn run() -> io::Result<()> {
    {
        // Part 1
        let mut prog = crate::get_lines("2019_2.txt")?.next().unwrap().split(",")
            .map(|s| s.parse().unwrap())
            .collect::<Vec<_>>();
        prog[1] = 12;
        prog[2] = 2;
        let result = IntcodeInterpreter::<PipeRead, PipeWrite>::from(prog).run();
        println!("The final value in position 0 is {}", result);
    }
    {
        // Part 2
        let mut prog = crate::get_lines("2019_2.txt")?.next().unwrap().split(",")
            .map(|s| s.parse().expect(&format!(r#"Invalid line: "{}""#, s)))
            .collect::<Vec<_>>();
        for noun in 0..100 {
            prog[1] = noun;
            for verb in 0..100 {
                prog[2] = verb;
                let result = IntcodeInterpreter::<PipeRead, PipeWrite>::from(prog.clone()).run();
                if result == 19690720 {
                    println!("noun = {}, verb = {}", noun, verb);
                    return Ok(());
                }
            }
        }
    }
    Ok(())
}
