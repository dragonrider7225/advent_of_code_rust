use std::{
    collections::HashSet,
    io,
};

pub fn run() -> io::Result<()> {
    {
        // Part 1
        let freq: i32 = super::parse_lines::<i32, _>("1.txt")?.sum();
        println!("Final frequency is {}", freq);
    }
    {
        // Part 2
        let changes_vec: Vec<_> = super::parse_lines("1.txt")?.collect();
        let mut changes = changes_vec.iter().cycle();
        let mut freqs = HashSet::new();
        let mut freq = 0i32;
        while freqs.insert(freq) {
            freq += changes.next().expect("Can't get None from non-empty cycle");
        }
        println!("First doubled frequency is {}", freq);
    }
    Ok(())
}

