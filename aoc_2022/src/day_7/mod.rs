use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

fn part1(input: &mut dyn BufRead) -> io::Result<u32> {
    let mut current_directory = PathBuf::new();
    let mut total_sizes = HashMap::new();
    total_sizes.insert(PathBuf::new(), Some(0));
    for line in input.lines() {
        let line = line?;
        if let Some(target) = line.strip_prefix("$ cd ") {
            match target {
                ".." => {
                    current_directory.pop();
                }
                "/" => current_directory.clear(),
                x => current_directory.push(x),
            }
        } else if "$ ls" == line {
            // We already read the output of this command automatically
        } else if line.starts_with('$') {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Unknown command {line:?}"),
            ));
        } else {
            let (size, name) = line.split_once(' ').ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "Invalid output line {line:?}")
            })?;
            if "dir" == size {
                let mut full_name = current_directory.clone();
                full_name.push(name);
                if total_sizes.contains_key(&full_name) {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Listed contents of /{current_directory:?} multiple times"),
                    ));
                }
                total_sizes.insert(full_name, Some(0));
            } else {
                let size = size.parse::<u32>().map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Invalid size of file {name:?} in directory {current_directory:?}: {e:?}"),
                    )
                })?;
                let mut parent = current_directory.clone();
                loop {
                    let total_size = total_sizes.get_mut(&parent).ok_or_else(|| {
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("Changed blindly into {current_directory:?}"),
                        )
                    })?;
                    match total_size {
                        None => {}
                        Some(total) => {
                            *total += size;
                            if *total > 100_000 {
                                *total_size = None;
                            }
                        }
                    }
                    if !parent.pop() {
                        break;
                    }
                }
            }
        }
    }
    Ok(total_sizes.values().copied().flatten().sum())
}

fn part2(input: &mut dyn BufRead) -> io::Result<u32> {
    let mut current_directory = PathBuf::new();
    let mut total_sizes = HashMap::new();
    total_sizes.insert(PathBuf::new(), 0);
    for line in input.lines() {
        let line = line?;
        if let Some(target) = line.strip_prefix("$ cd ") {
            match target {
                ".." => {
                    current_directory.pop();
                }
                "/" => current_directory.clear(),
                x => current_directory.push(x),
            }
        } else if "$ ls" == line {
            // We already read the output of this command automatically
        } else if line.starts_with('$') {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Unknown command {line:?}"),
            ));
        } else {
            let (size, name) = line.split_once(' ').ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "Invalid output line {line:?}")
            })?;
            if "dir" == size {
                let mut full_name = current_directory.clone();
                full_name.push(name);
                if total_sizes.contains_key(&full_name) {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Listed contents of /{current_directory:?} multiple times"),
                    ));
                }
                total_sizes.insert(full_name, 0);
            } else {
                let size = size.parse::<u32>().map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Invalid size of file {name:?} in directory {current_directory:?}: {e:?}"),
                    )
                })?;
                let mut parent = current_directory.clone();
                loop {
                    let total_size = total_sizes.get_mut(&parent).ok_or_else(|| {
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("Changed blindly into {current_directory:?}"),
                        )
                    })?;
                    *total_size += size;
                    if !parent.pop() {
                        break;
                    }
                }
            }
        }
    }
    const TOTAL_SPACE: u32 = 70_000_000;
    const REQUIRED_SPACE: u32 = 30_000_000;
    let mut total_sizes = total_sizes.values().copied().collect::<Vec<_>>();
    total_sizes.sort_unstable();
    let used_space = total_sizes.last().expect("Empty filesystem");
    let remaining_space = TOTAL_SPACE - used_space;
    let space_to_free = REQUIRED_SPACE - remaining_space;
    total_sizes
        .into_iter()
        .find(|&size| size >= space_to_free)
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "70_000_000 < 30_000_000".to_string(),
            )
        })
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2022 Day 7 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2022_07.txt")?))?
        );
    }
    {
        println!("Year 2022 Day 7 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2022_07.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        "$ cd /\n",
        "$ ls\n",
        "dir a\n",
        "14848514 b.txt\n",
        "8504156 c.dat\n",
        "dir d\n",
        "$ cd a\n",
        "$ ls\n",
        "dir e\n",
        "29116 f\n",
        "2557 g\n",
        "62596 h.lst\n",
        "$ cd e\n",
        "$ ls\n",
        "584 i\n",
        "$ cd ..\n",
        "$ cd ..\n",
        "$ cd d\n",
        "$ ls\n",
        "4060174 j\n",
        "8033020 d.log\n",
        "5626152 d.ext\n",
        "7214296 k\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 95_437;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 24_933_642;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
