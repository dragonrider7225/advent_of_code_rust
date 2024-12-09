use std::{
    cmp::Ordering,
    fmt::{self, Display, Formatter},
    fs::File,
    io::{self, BufRead, BufReader},
    mem,
};

use aoc_util::{
    impl_from_str_for_nom_parse,
    nom::{character::complete as character, multi, IResult, Parser},
    nom_extended::NomParse,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum Block {
    #[default]
    Empty,
    FileId(usize),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct DiskMapEntry(Block, usize);

#[derive(Clone, Debug, Eq, PartialEq)]
struct DiskMap {
    contents: Vec<DiskMapEntry>,
}

impl DiskMap {
    fn checksum(&self) -> usize {
        self.contents
            .iter()
            .fold((0, 0), |(idx, total), &DiskMapEntry(block, count)| {
                let next_idx = idx + count;
                match block {
                    Block::Empty => (next_idx, total),
                    Block::FileId(file_id) => {
                        let positions = (idx..(idx + count)).sum::<usize>();
                        (next_idx, total + positions * file_id)
                    }
                }
            })
            .1
    }

    fn compactify(&mut self) {
        let mut i = 0;
        while i < self.contents.len() {
            let mut popped_empty = false;
            while self
                .contents
                .last()
                .filter(|DiskMapEntry(block, _)| matches!(block, Block::Empty))
                .is_some()
            {
                self.contents.pop();
                popped_empty = true;
            }
            if popped_empty {
                continue;
            }
            if matches!(self.contents[i].0, Block::Empty) {
                let available_space = self.contents[i].1;
                match available_space.cmp(&self.contents.last().unwrap().1) {
                    Ordering::Less => {
                        let last = self.contents.last_mut().unwrap();
                        last.1 -= available_space;
                        let last_block_id = last.0;
                        self.contents[i].0 = last_block_id;
                    }
                    Ordering::Equal => {
                        self.contents[i] = self.contents.pop().unwrap();
                    }
                    Ordering::Greater => {
                        let last = self.contents.pop().unwrap();
                        self.contents[i].1 -= last.1;
                        self.contents.insert(i, last);
                    }
                }
            }
            i += 1;
            continue;
        }
    }

    fn compactify_no_fragment(&mut self) {
        let last_file_id = *self
            .contents
            .iter()
            .rev()
            .find_map(|DiskMapEntry(block, _)| {
                if let Block::FileId(file_id) = block {
                    Some(file_id)
                } else {
                    None
                }
            })
            .expect("Disk map should have at least one file");
        for file_id in (0..=last_file_id).rev() {
            let Some(file_idx) = self
                .contents
                .iter()
                .rposition(|&DiskMapEntry(block, _)| Block::FileId(file_id) == block)
            else {
                eprintln!("Missing file id {file_id}");
                continue;
            };
            let file_length = self.contents[file_idx].1;
            let Some(empty_idx) =
                self.contents
                    .iter()
                    .take(file_idx)
                    .position(|&DiskMapEntry(block, length)| {
                        Block::Empty == block && length >= file_length
                    })
            else {
                continue;
            };
            let file_block = mem::take(&mut self.contents[file_idx].0);
            if self.contents[empty_idx].1 > file_length {
                self.contents[empty_idx].1 -= file_length;
                self.contents
                    .insert(empty_idx, DiskMapEntry(file_block, file_length));
            } else {
                // self.contents[empty_idx].1 == file_length
                self.contents[empty_idx].0 = file_block;
            }
        }
    }
}

impl Display for DiskMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for &DiskMapEntry(block, length) in &self.contents {
            match block {
                Block::Empty => {
                    for _ in 0..length {
                        write!(f, ".")?;
                    }
                }
                Block::FileId(file_id) => {
                    let s = format!("{{{file_id}}}");
                    for _ in 0..length {
                        write!(f, "{s}")?;
                    }
                }
            }
        }
        Ok(())
    }
}

impl NomParse<&str> for DiskMap {
    fn nom_parse(input: &str) -> IResult<&str, Self> {
        fn segment_length(s: &str) -> IResult<&str, usize> {
            character::one_of("0123456789")
                .map(|c| (c as u8 - b'0') as usize)
                .parse(s)
        }

        multi::fold_many1(
            segment_length,
            Vec::<DiskMapEntry>::new,
            |mut acc, segment_length| {
                let block = if acc.is_empty() {
                    Block::FileId(0)
                } else if acc.last().unwrap().0 == Block::Empty {
                    let Block::FileId(last_file_id) = acc[acc.len() - 2].0 else {
                        unreachable!("Accumulated two empty segments in a row")
                    };
                    Block::FileId(last_file_id + 1)
                } else {
                    Block::Empty
                };
                acc.push(DiskMapEntry(block, segment_length));
                acc
            },
        )
        .map(|contents| Self { contents })
        .parse(input)
    }
}

impl_from_str_for_nom_parse!(DiskMap);

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    let mut disk_map = io::read_to_string(input)?
        .trim()
        .parse::<DiskMap>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    disk_map.compactify();
    Ok(disk_map.checksum())
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    let mut disk_map = io::read_to_string(input)?
        .trim()
        .parse::<DiskMap>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    disk_map.compactify_no_fragment();
    Ok(disk_map.checksum())
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2024 Day 9 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2024_09.txt")?))?
        );
    }
    {
        println!("Year 2024 Day 9 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2024_09.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use io::Cursor;

    use super::*;

    const TEST_DATA: &str = "2333133121414131402\n";

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 1928;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 2858;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
