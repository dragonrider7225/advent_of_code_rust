use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn part1(input: &mut dyn BufRead) -> io::Result<u32> {
    todo!("Year 2025 Day 1 Part 1")
}

fn part2(input: &mut dyn BufRead) -> io::Result<u32> {
    todo!("Year 2025 Day 1 Part 2")
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2025 Day 1 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2025_01.txt")?))?
        );
    }
    {
        println!("Year 2025 Day 1 Part 2");
        println!(
            "{:?}",
            part2(&mut BufReader::new(File::open("2025_01.txt")?))?
        );
    }
    Ok(())
}
