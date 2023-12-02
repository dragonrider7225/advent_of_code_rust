use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn part1(_input: &mut dyn BufRead) -> io::Result<()> {
    todo!("Year 2023 Day 7 Part 1")
}

fn part2(_input: &mut dyn BufRead) -> io::Result<()> {
    todo!("Year 2023 Day 7 Part 2")
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2023 Day 7 Part 1");
        println!(
            "{:?}",
            part1(&mut BufReader::new(File::open("2023_07.txt")?))?
        );
    }
    {
        println!("Year 2023 Day 7 Part 2");
        println!(
            "{:?}",
            part2(&mut BufReader::new(File::open("2023_07.txt")?))?
        );
    }
    Ok(())
}
