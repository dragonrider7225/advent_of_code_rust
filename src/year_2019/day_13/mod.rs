use crate::year_2019::intcode_interpreter::IntcodeInterpreter;

use std::{convert::TryInto, fmt::{self, Display, Formatter}, io, thread};

use extended_io::{self as eio, pipe::{self, PipeRead, PipeWrite}};

struct Screen {
    tiles: Vec<Vec<u8>>,
    num_blocks: u8,
    score: u64,
}

impl Screen {
    fn new() -> Self {
        Self {
            tiles: vec![vec![0]],
            num_blocks: 0,
            score: 0,
        }
    }

    fn set(&mut self, (x, y): (usize, usize), tile: u8) {
        if tile > 4 {
            panic!("Invalid tile: {}", tile);
        }
        if y >= self.tiles.len() {
            self.tiles.extend(vec![vec![0; self.tiles[0].len()]; self.tiles.len() - y + 1]);
        }
        if x >= self.tiles[y].len() {
            let missing = self.tiles[y].len() - x + 1;
            for col in self.tiles.iter_mut() {
                col.extend(vec![0; missing]);
            }
        }
        self.tiles[y][x] = tile;
    }

    fn set_score(&mut self, score: u64) {
        self.score = score;
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "Score: {}", self.score)?;
        for row in &self.tiles {
            for col in row {
                write!(f, "{}", match col {
                    0 => ' ',
                    1 => 'W',
                    2 => 'B',
                    3 => 'P',
                    4 => 'o',
                    n => panic!("Invalid tile: {}", n),
                })?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

pub(super) fn run() -> io::Result<()> {
    let prog = IntcodeInterpreter::<PipeRead, PipeWrite>::read_from_file("2019_13.txt")?;
    {
        println!("Year 2019 Day 12 Part 1");
        let (mut prog_to_screen_read, prog_to_screen_write) = pipe::mk_pipe();
        let (screen_to_prog_read, _) = pipe::mk_pipe();
        prog.dup_with(screen_to_prog_read, prog_to_screen_write).run_piped();
        let mut num_blocks = 0;
        while let Ok(_) = eio::read_i64(&mut prog_to_screen_read) {
            let _ = eio::read_i64(&mut prog_to_screen_read)?;
            if let Ok(2) = eio::read_i64(&mut prog_to_screen_read) {
                num_blocks += 1;
            }
        }
        println!("The game exits with {} blocks on screen", num_blocks);
    }
    {
        println!("Year 2019 Day 12 Part 2");
        let unimplemented = true;
        if unimplemented {
            println!("Not implemented");
            return Ok(());
        }
        let mut prog = prog.get_program();
        prog[0] = 2;
        let (mut prog_to_screen_read, prog_to_screen_write) = pipe::mk_pipe();
        let (screen_to_prog_read, mut screen_to_prog_write) = pipe::mk_pipe();
        let prog = IntcodeInterpreter::with_streams(
            prog,
            Some(screen_to_prog_read),
            Some(prog_to_screen_write),
        );
        // prog.set_debug(true);
        let prog_thread = thread::spawn(move || prog.run_piped());
        let mut screen = Screen::new();
        let mut blanking = false;
        loop {
            // TODO: implement interrupts.
            let num_blocks = screen.num_blocks;
            let x = match eio::read_i64(&mut prog_to_screen_read)? {
                -1 => {
                    blanking = true;
                    0
                }
                n => n.try_into().unwrap(),
            };
            let y = eio::read_i64(&mut prog_to_screen_read)?.try_into()
                .expect("Invalid y coordinate");
            let tile = eio::read_i64(&mut prog_to_screen_read)?;
            if blanking {
                screen.set_score(tile.try_into().expect("Invalid score"));
                println!("{}", screen);
                eio::write_i64(
                    &mut screen_to_prog_write,
                    eio::prompt("Enter joystick position (left: -1, right: 1): ")?,
                )?;
                blanking = false;
            } else {
                screen.set((x, y), tile.try_into().expect("Invalid tile"));
                if num_blocks > screen.num_blocks && screen.num_blocks == 0 {
                    println!("{}", screen);
                    break;
                }
            }
        }
        prog_thread.join().expect("prog_thread panicked");
    }
    Ok(())
}
