use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader},
};

fn part1(input: &mut dyn BufRead) -> io::Result<u32> {
    let mut compartment_one = HashSet::new();
    let mut compartment_two = HashSet::new();
    let mut total_error = 0;
    for line in input.lines() {
        let line = line?;
        let num_items = line.len();
        for (i, item) in line.bytes().enumerate() {
            let compartment = if i < num_items / 2 {
                &mut compartment_one
            } else {
                &mut compartment_two
            };
            let priority = match item {
                b'a'..=b'z' => item - b'a' + 1,
                b'A'..=b'Z' => item - b'A' + 27,
                _ => unreachable!(),
            };
            compartment.insert(priority.into());
        }
        total_error += compartment_one
            .iter()
            .filter(|&item| compartment_two.contains(item))
            .sum::<u32>();
        compartment_one.clear();
        compartment_two.clear();
    }
    Ok(total_error)
}

fn part2(input: &mut dyn BufRead) -> io::Result<u32> {
    let mut compartments = [
        (HashSet::new(), HashSet::new()),
        (HashSet::new(), HashSet::new()),
        (HashSet::new(), HashSet::new()),
    ];
    let mut elf_index = 0;
    let mut total_badge = 0;
    for line in input.lines() {
        let line = line?;
        let num_items = line.len();
        for (i, item) in line.bytes().enumerate() {
            let compartment = if i < num_items / 2 {
                &mut compartments[elf_index].0
            } else {
                &mut compartments[elf_index].1
            };
            let priority = match item {
                b'a'..=b'z' => item - b'a' + 1,
                b'A'..=b'Z' => item - b'A' + 27,
                _ => unreachable!(),
            };
            compartment.insert(priority.into());
        }
        if elf_index == 2 {
            total_badge += compartments
                .iter()
                .map(|(compartment_one, compartment_two)| {
                    compartment_one
                        .union(compartment_two)
                        .copied()
                        .collect::<HashSet<u32>>()
                })
                .reduce(|left, right| left.intersection(&right).copied().collect())
                .unwrap()
                .into_iter()
                .sum::<u32>();
            compartments[0].0.clear();
            compartments[0].1.clear();
            compartments[1].0.clear();
            compartments[1].1.clear();
            compartments[2].0.clear();
            compartments[2].1.clear();
            elf_index = 0;
        } else {
            elf_index += 1;
        }
    }
    Ok(total_badge)
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2022 Day 3 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2022_03.txt")?))?
        );
    }
    {
        println!("Year 2022 Day 3 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2022_03.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        "vJrwpWtwJgWrhcsFMMfFFhFp\n",
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n",
        "PmmdzqPrVvPwwTWBwg\n",
        "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n",
        "ttgJtRGJQctTZtZT\n",
        "CrZsJsPPZsGzwwsLwLmpwMDw\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 157;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 70;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
