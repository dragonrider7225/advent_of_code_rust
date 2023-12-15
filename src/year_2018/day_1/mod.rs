use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader},
};

pub fn run() -> io::Result<()> {
    {
        // Part 1
        let freq = BufReader::new(File::open("2018_01.txt")?)
            .lines()
            .map(|line| {
                line?
                    .parse::<i32>()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            })
            .sum::<io::Result<i32>>()?;
        println!("Final frequency is {freq}");
    }
    {
        // Part 2
        let changes_vec = BufReader::new(File::open("2018_01.txt")?)
            .lines()
            .map(|line| {
                line?
                    .parse::<i32>()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            })
            .collect::<io::Result<Vec<_>>>()?;
        let mut changes = changes_vec.iter().cycle();
        let mut freqs = HashSet::new();
        let mut freq = 0i32;
        while freqs.insert(freq) {
            freq += changes.next().expect("Can't get None from non-empty cycle");
        }
        println!("First doubled frequency is {freq}");
    }
    Ok(())
}
