use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    mem,
};

fn part1(input: &mut dyn BufRead) -> io::Result<u32> {
    let mut snack_elf_calories = 0;
    let mut current_elf_calories = 0;
    let mut insert_calories = |current_elf_calories: &mut u32| {
        snack_elf_calories = snack_elf_calories.max(mem::take(current_elf_calories));
    };
    for line in input.lines() {
        let line = line?;
        if line.trim().is_empty() {
            insert_calories(&mut current_elf_calories);
        } else {
            current_elf_calories += line
                .parse::<u32>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        }
    }
    insert_calories(&mut current_elf_calories);
    Ok(snack_elf_calories)
}

fn part2(input: &mut dyn BufRead) -> io::Result<u32> {
    let mut snack_elf_calories = [0; 3];
    let mut current_elf_calories = 0;
    let mut insert_calories = |current_elf_calories: &mut u32| {
        for place in snack_elf_calories.iter_mut() {
            if *place < *current_elf_calories {
                mem::swap(place, current_elf_calories);
            }
        }
        *current_elf_calories = 0;
    };
    for line in input.lines() {
        let line = line?;
        if line.trim().is_empty() {
            insert_calories(&mut current_elf_calories);
        } else {
            current_elf_calories += line
                .parse::<u32>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        }
    }
    insert_calories(&mut current_elf_calories);
    Ok(snack_elf_calories.into_iter().sum())
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2022 Day 1 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2022_01.txt")?))?
        );
    }
    {
        println!("Year 2022 Day 1 Part 2");
        println!(
            "{:?}",
            part2(&mut BufReader::new(File::open("2022_01.txt")?))?
        );
    }
    Ok(())
}
