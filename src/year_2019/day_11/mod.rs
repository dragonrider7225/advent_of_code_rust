use std::{io, thread};

use crate::year_2019::{
    intcode_interpreter::IntcodeInterpreter,
    robot::{Color, Robot},
};

use extended_io::{self as eio, pipe::{self, PipeRead, PipeWrite}};

pub(super) fn run() -> io::Result<()> {
    let prog = IntcodeInterpreter::<PipeRead, PipeWrite>::read_from_file("2019_11.txt")?;
    {
        println!("Year 2019 Day 11 Part 1");
        let (robot_to_prog_read, robot_to_prog_write) = pipe::mk_pipe();
        let (prog_to_robot_read, mut prog_to_robot_write) = pipe::mk_pipe();
        let prog = prog.dup_with(robot_to_prog_read, prog_to_robot_write.clone());
        let mut robot = Robot::new(prog_to_robot_read, robot_to_prog_write);
        let prog_thread = thread::spawn(move || prog.run_piped());
        let robot_thread = thread::spawn(move || {
            robot.run();
            robot.num_panels()
        });
        prog_thread.join().unwrap();
        eio::write_i64(&mut prog_to_robot_write, 2)?;
        let num_panels = robot_thread.join().unwrap();
        println!("The robot painted {} panels", num_panels);
    }
    {
        println!("Year 2019 Day 11 Part 2");
        let (robot_to_prog_read, robot_to_prog_write) = pipe::mk_pipe();
        let (prog_to_robot_read, mut prog_to_robot_write) = pipe::mk_pipe();
        let prog = prog.dup_with(robot_to_prog_read, prog_to_robot_write.clone());
        let mut robot = Robot::new(prog_to_robot_read, robot_to_prog_write);
        let prog_thread = thread::spawn(move || prog.run_piped());
        let robot_thread = thread::spawn(move || {
            robot.set(Default::default(), Color::White);
            robot.run();
            robot.print_field();
        });
        prog_thread.join().unwrap();
        eio::write_i64(&mut prog_to_robot_write, 2)?;
        robot_thread.join().unwrap();
    }
    Ok(())
}
