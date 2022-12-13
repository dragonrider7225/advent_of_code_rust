use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

type Card = u32;

fn part1(input: &mut dyn BufRead) -> io::Result<()> {
    let mut lines = input.lines();
    todo!("Year 2020 Day 22 Part 1")
}

fn part2(_input: &mut dyn BufRead) -> io::Result<()> {
    todo!("Year 2020 Day 22 Part 2")
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2020 Day 22 Part 1");
        println!(
            "{:?}",
            part1(&mut BufReader::new(File::open("2020_22.txt")?))?
        );
    }
    {
        println!("Year 2020 Day 22 Part 2");
        println!(
            "{:?}",
            part2(&mut BufReader::new(File::open("2020_22.txt")?))?
        );
    }
    Ok(())
}
