use crate::year_2019::intcode_interpreter::IntcodeInterpreter;

use std::{
    io::{self, BufRead, Cursor, Seek, Write},
    iter::Peekable,
    thread,
};

use extended_io::{
    self as eio,
    pipe::{self, PipeRead, PipeWrite},
};

struct Permutations3 {
    current_idx: usize,
    value: i64,
    inner: Peekable<<[[i64; 2]; 2] as IntoIterator>::IntoIter>,
}

impl Permutations3 {
    fn of(values: [i64; 3]) -> Self {
        Self {
            current_idx: 0,
            value: values[0],
            inner: [[values[1], values[2]], [values[2], values[1]]]
                .into_iter()
                .peekable(),
        }
    }
}

impl Iterator for Permutations3 {
    type Item = [i64; 3];

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_idx < 3 {
            let sub = self.inner.peek()?;
            let mut res = [0; 3];
            res[..self.current_idx].copy_from_slice(&sub[..self.current_idx]);
            res[self.current_idx] = self.value;
            res[(self.current_idx + 1)..].copy_from_slice(&sub[self.current_idx..]);
            self.current_idx += 1;
            Some(res)
        } else {
            self.current_idx = 0;
            let _ = self.inner.next();
            self.next()
        }
    }
}

struct Permutations4 {
    current_idx: usize,
    value: i64,
    inner: Peekable<Permutations3>,
}

impl Permutations4 {
    fn of(values: [i64; 4]) -> Self {
        Self {
            current_idx: 0,
            value: values[0],
            inner: Permutations3::of([values[1], values[2], values[3]]).peekable(),
        }
    }
}

impl Iterator for Permutations4 {
    type Item = [i64; 4];

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_idx < 4 {
            let sub = self.inner.peek()?;
            let mut res = [0; 4];
            res[..self.current_idx].copy_from_slice(&sub[..self.current_idx]);
            res[self.current_idx] = self.value;
            res[(self.current_idx + 1)..].copy_from_slice(&sub[self.current_idx..]);
            self.current_idx += 1;
            Some(res)
        } else {
            self.current_idx = 0;
            let _ = self.inner.next();
            self.next()
        }
    }
}

struct Permutations5 {
    current_idx: usize,
    value: i64,
    inner: Peekable<Permutations4>,
}

impl Permutations5 {
    fn of(values: [i64; 5]) -> Self {
        Self {
            current_idx: 0,
            value: values[0],
            inner: Permutations4::of([values[1], values[2], values[3], values[4]]).peekable(),
        }
    }
}

impl Iterator for Permutations5 {
    type Item = [i64; 5];

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_idx < 5 {
            let sub = self.inner.peek()?;
            let mut res = [0; 5];
            res[..self.current_idx].copy_from_slice(&sub[..self.current_idx]);
            res[self.current_idx] = self.value;
            res[(self.current_idx + 1)..].copy_from_slice(&sub[self.current_idx..]);
            self.current_idx += 1;
            Some(res)
        } else {
            self.current_idx = 0;
            let _ = self.inner.next();
            self.next()
        }
    }
}

pub(super) fn run() -> io::Result<()> {
    let amplifier_controller =
        IntcodeInterpreter::<PipeRead, PipeWrite>::read_from_file("2019_7.txt")?;
    {
        println!("Year 2019 Day 7 Part 1");
        let mut results = Cursor::new(vec![]);
        for perm in Permutations5::of([0, 1, 2, 3, 4]) {
            let (to_a_read, mut to_a_write) = pipe::mk_pipe();
            let (a_to_b_read, mut a_to_b_write) = pipe::mk_pipe();
            let (b_to_c_read, mut b_to_c_write) = pipe::mk_pipe();
            let (c_to_d_read, mut c_to_d_write) = pipe::mk_pipe();
            let (d_to_e_read, mut d_to_e_write) = pipe::mk_pipe();
            let (mut e_to_read, e_to_write) = pipe::mk_pipe();

            eio::write_i64(&mut to_a_write, perm[0])?;
            eio::write_i64(&mut to_a_write, 0)?;
            eio::write_i64(&mut a_to_b_write, perm[1])?;
            eio::write_i64(&mut b_to_c_write, perm[2])?;
            eio::write_i64(&mut c_to_d_write, perm[3])?;
            eio::write_i64(&mut d_to_e_write, perm[4])?;

            amplifier_controller
                .dup_with(to_a_read, a_to_b_write)
                .run_piped();
            amplifier_controller
                .dup_with(a_to_b_read, b_to_c_write)
                .run_piped();
            amplifier_controller
                .dup_with(b_to_c_read, c_to_d_write)
                .run_piped();
            amplifier_controller
                .dup_with(c_to_d_read, d_to_e_write)
                .run_piped();
            amplifier_controller
                .dup_with(d_to_e_read, e_to_write)
                .run_piped();

            let result = eio::read_i64(&mut e_to_read)?;
            writeln!(results, "Phase Sequence {perm:?}: {result}")?;
        }
        results.rewind()?;
        let mut lines = results
            .lines()
            .map(|s| s.expect("Ran into an error lines-ing `results`"))
            .map(|s| s.split(": ").map(|s| s.to_string()).collect::<Vec<_>>())
            .map(|mut v| {
                let name = v.remove(0);
                let speed = v.remove(0).parse::<u32>().unwrap();
                (name, speed)
            })
            .collect::<Vec<_>>();
        lines[..].sort_by_key(|(_, speed)| std::u32::MAX - speed);
        let (fastest, speed) = &lines[0];
        println!("{fastest}: {speed}");
    }
    {
        println!("Year 2019 Day 7 Part 2");
        let mut results = vec![];
        for perm in Permutations5::of([5, 6, 7, 8, 9]) {
            let (mut e_to_a_read, mut e_to_a_write) = pipe::mk_pipe();
            let (a_to_b_read, mut a_to_b_write) = pipe::mk_pipe();
            let (b_to_c_read, mut b_to_c_write) = pipe::mk_pipe();
            let (c_to_d_read, mut c_to_d_write) = pipe::mk_pipe();
            let (d_to_e_read, mut d_to_e_write) = pipe::mk_pipe();

            eio::write_i64(&mut e_to_a_write, perm[0])?;
            eio::write_i64(&mut e_to_a_write, 0)?;
            eio::write_i64(&mut a_to_b_write, perm[1])?;
            eio::write_i64(&mut b_to_c_write, perm[2])?;
            eio::write_i64(&mut c_to_d_write, perm[3])?;
            eio::write_i64(&mut d_to_e_write, perm[4])?;

            let amplifier_a = amplifier_controller.dup_with(e_to_a_read.clone(), a_to_b_write);
            let amplifier_b = amplifier_controller.dup_with(a_to_b_read, b_to_c_write);
            let amplifier_c = amplifier_controller.dup_with(b_to_c_read, c_to_d_write);
            let amplifier_d = amplifier_controller.dup_with(c_to_d_read, d_to_e_write);
            let amplifier_e = amplifier_controller.dup_with(d_to_e_read, e_to_a_write);

            let thread_a = thread::Builder::new()
                .name("2019::7::2::thread_a".to_string())
                .spawn(move || amplifier_a.run_piped())
                .unwrap();
            let thread_b = thread::Builder::new()
                .name("2019::7::2::thread_b".to_string())
                .spawn(move || amplifier_b.run_piped())
                .unwrap();
            let thread_c = thread::Builder::new()
                .name("2019::7::2::thread_c".to_string())
                .spawn(move || amplifier_c.run_piped())
                .unwrap();
            let thread_d = thread::Builder::new()
                .name("2019::7::2::thread_d".to_string())
                .spawn(move || amplifier_d.run_piped())
                .unwrap();
            let thread_e = thread::Builder::new()
                .name("2019::7::2::thread_e".to_string())
                .spawn(move || amplifier_e.run_piped())
                .unwrap();
            match thread_a.join() {
                Ok(_) => {}
                Err(e) => {
                    if e.is::<String>() {
                        panic!("[thread_a] {}", e.downcast_ref::<String>().unwrap(),);
                    } else {
                        panic!("[thread_a] {e:?}");
                    }
                }
            }
            match thread_b.join() {
                Ok(_) => {}
                Err(e) => {
                    if e.is::<String>() {
                        panic!("[thread_b] {}", e.downcast_ref::<String>().unwrap(),);
                    } else {
                        panic!("[thread_b] {e:?}");
                    }
                }
            }
            match thread_c.join() {
                Ok(_) => {}
                Err(e) => {
                    if e.is::<String>() {
                        panic!("[thread_c] {}", e.downcast_ref::<String>().unwrap(),);
                    } else {
                        panic!("[thread_c] {e:?}");
                    }
                }
            }
            match thread_d.join() {
                Ok(_) => {}
                Err(e) => {
                    if e.is::<String>() {
                        panic!("[thread_d] {}", e.downcast_ref::<String>().unwrap(),);
                    } else {
                        panic!("[thread_d] {e:?}");
                    }
                }
            }
            match thread_e.join() {
                Ok(_) => {}
                Err(e) => {
                    if e.is::<String>() {
                        panic!("[thread_e] {}", e.downcast_ref::<String>().unwrap(),);
                    } else {
                        panic!("[thread_e] {e:?}");
                    }
                }
            }
            let thrust = eio::read_i64(&mut e_to_a_read)?;
            results.push(perm[0]);
            results.push(perm[1]);
            results.push(perm[2]);
            results.push(perm[3]);
            results.push(perm[4]);
            results.push(thrust);
        }
        let results_chunks = results.chunks(6);
        let mut lines = results_chunks
            .map(|xs| {
                let name = [xs[0], xs[1], xs[2], xs[3], xs[4]];
                let thrust = xs[5];
                (name, thrust)
            })
            .collect::<Vec<_>>();
        lines[..].sort_by_key(|(_, speed)| *speed);
        let (fastest, speed) = lines
            .into_iter()
            .next_back()
            .expect("Ran at least one simulation");
        println!("{fastest:?}: {speed}");
    }
    Ok(())
}
