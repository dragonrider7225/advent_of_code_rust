use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn part1(_input: &mut dyn BufRead) -> io::Result<()> {
    todo!("Year ???? Day 12 Part 1")
}

fn part2(_input: &mut dyn BufRead) -> io::Result<()> {
    todo!("Year ???? Day 12 Part 2")
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year ???? Day 12 Part 1");
        println!(
            "{:?}",
            part1(&mut BufReader::new(File::open("????_12.txt")?))?
        );
    }
    {
        println!("Year ???? Day 12 Part 2");
        println!(
            "{:?}",
            part2(&mut BufReader::new(File::open("????_12.txt")?))?
        );
    }
    Ok(())
}
