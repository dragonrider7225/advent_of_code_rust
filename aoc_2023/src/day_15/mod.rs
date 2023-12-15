use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn run_hash_algorithm(s: impl IntoIterator<Item = u8>) -> u8 {
    s.into_iter()
        .fold(0, |acc, b| acc.wrapping_add(b).wrapping_mul(17))
}

fn part1(input: &mut dyn BufRead) -> io::Result<u32> {
    Ok({
        let mut s = String::new();
        input.read_to_string(&mut s)?;
        s
    }
    .trim()
    .split(',')
    .map(|step| run_hash_algorithm(step.bytes()))
    .map(u32::from)
    .sum())
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Lens {
    label: String,
    focal_length: u8,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct State {
    boxes: [Vec<Lens>; 256],
}

impl State {
    fn total_focusing_power(&self) -> u32 {
        self.boxes
            .iter()
            .enumerate()
            .map(|(box_idx, r#box)| {
                (box_idx as u32 + 1)
                    * r#box
                        .iter()
                        .enumerate()
                        .map(|(lens_idx, lens)| (lens_idx as u32 + 1) * lens.focal_length as u32)
                        .sum::<u32>()
            })
            .sum()
    }

    fn remove_lens(&mut self, label: &str) {
        let box_id = run_hash_algorithm(label.bytes());
        let r#box = &mut self.boxes[box_id as usize];
        if let Some(position) = r#box.iter().position(|lens| lens.label == label) {
            r#box.remove(position);
        }
    }

    fn insert_lens(&mut self, label: &str, focal_length: u8) {
        let box_id = run_hash_algorithm(label.bytes());
        let r#box = &mut self.boxes[box_id as usize];
        if let Some(position) = r#box.iter().position(|lens| lens.label == label) {
            r#box[position].focal_length = focal_length;
        } else {
            r#box.push(Lens {
                label: label.into(),
                focal_length,
            });
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            boxes: [const { Vec::new() }; 256],
        }
    }
}

fn part2(input: &mut dyn BufRead) -> io::Result<u32> {
    let steps = {
        let mut s = String::new();
        input.read_to_string(&mut s)?;
        s
    };
    let final_state = steps
        .trim()
        .split(',')
        .fold(State::default(), |mut acc, step| {
            let bytes = step.as_bytes();
            let label = &step[..bytes.iter().position(|b| !b.is_ascii_alphabetic()).unwrap()];
            let op = bytes[label.len()];
            match op {
                b'-' => acc.remove_lens(label),
                b'=' => acc.insert_lens(label, step[(label.len() + 1)..].parse().unwrap()),
                _ => unreachable!("The first non-alphabetic character must be - or ="),
            }
            acc
        });
    Ok(final_state.total_focusing_power())
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2023 Day 15 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2023_15.txt")?))?
        );
    }
    {
        println!("Year 2023 Day 15 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2023_15.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_hash_algorithm() {
        let expected = 52;
        let actual = run_hash_algorithm("HASH".bytes());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 1320;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 145;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
