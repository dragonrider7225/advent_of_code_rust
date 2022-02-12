use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Clone, Debug, Default)]
struct Connections {
    connections: HashMap<String, HashSet<String>>,
}

impl Connections {
    fn read(input: &mut dyn BufRead) -> io::Result<Self> {
        let connections =
            input
                .lines()
                .try_fold(HashMap::<_, HashSet<_>>::new(), |mut acc, line| {
                    let line = line?;
                    let (left, right) = line.split_once('-').ok_or_else(|| {
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("Invalid connection: {:?}", line),
                        )
                    })?;
                    let left = left.to_owned();
                    let right = right.to_owned();
                    acc.entry(left.clone()).or_default().insert(right.clone());
                    acc.entry(right).or_default().insert(left);
                    io::Result::Ok(acc)
                })?;
        Ok(Self { connections })
    }
}

impl Connections {
    fn num_paths(&self) -> u32 {
        fn paths_impl<'this>(
            this: &'this Connections,
            current_cave: &'this str,
            explored_caves: &mut HashSet<&'this str>,
        ) -> u32 {
            let num_paths = if current_cave == "end" {
                1
            } else {
                let is_small_cave = current_cave.chars().next().unwrap().is_lowercase();
                if is_small_cave {
                    explored_caves.insert(current_cave);
                }
                let mut num_paths = 0;
                for cave in &this.connections[current_cave] {
                    let cave = &**cave;
                    if explored_caves.contains(cave) {
                        continue;
                    }
                    num_paths += paths_impl(this, cave, &mut *explored_caves);
                }
                if is_small_cave {
                    explored_caves.remove(current_cave);
                }
                num_paths
            };
            num_paths
        }
        paths_impl(self, "start", &mut HashSet::new())
    }

    fn num_longer_paths(&self) -> u32 {
        fn paths_impl<'this>(
            this: &'this Connections,
            current_cave: &'this str,
            explored_caves: &mut HashSet<&'this str>,
            doubled_small_cave: bool,
        ) -> u32 {
            let num_paths = if current_cave == "end" {
                1
            } else {
                let is_small_cave = current_cave.chars().next().unwrap().is_lowercase();
                let cave_doubled = if is_small_cave {
                    !explored_caves.insert(current_cave)
                } else {
                    false
                };
                let doubled_next = doubled_small_cave || cave_doubled;
                let mut num_paths = 0;
                for cave in &this.connections[current_cave] {
                    let cave = &**cave;
                    if cave == "start" {
                        continue;
                    }
                    if explored_caves.contains(cave) && doubled_next {
                        continue;
                    }
                    num_paths += paths_impl(this, cave, &mut *explored_caves, doubled_next);
                }
                if is_small_cave && !cave_doubled {
                    explored_caves.remove(current_cave);
                }
                num_paths
            };
            num_paths
        }
        paths_impl(self, "start", &mut HashSet::new(), false)
    }
}

fn part1(input: &mut dyn BufRead) -> io::Result<u32> {
    let connections = Connections::read(input)?;
    Ok(connections.num_paths())
}

fn part2(input: &mut dyn BufRead) -> io::Result<u32> {
    let connections = Connections::read(input)?;
    Ok(connections.num_longer_paths())
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2021 Day 12 Part 1");
        println!(
            "The total number of valid paths is {}",
            part1(&mut BufReader::new(File::open("2021_12.txt")?))?
        );
    }
    {
        println!("Year 2021 Day 12 Part 2");
        println!(
            "The total number of longer paths is {}",
            part2(&mut BufReader::new(File::open("2021_12.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const SHORT_EXAMPLE: &'static str = "start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end\n";
    const MEDIUM_EXAMPLE: &'static str = concat!(
        "dc-end\n",
        "HN-start\n",
        "start-kj\n",
        "dc-start\n",
        "dc-HN\n",
        "LN-dc\n",
        "HN-end\n",
        "kj-sa\n",
        "kj-HN\n",
        "kj-dc\n"
    );
    const LONG_EXAMPLE: &'static str = concat!(
        "fs-end\n",
        "he-DX\n",
        "fs-he\n",
        "start-DX\n",
        "pj-DX\n",
        "end-zg\n",
        "zg-sl\n",
        "zg-pj\n",
        "pj-he\n",
        "RW-he\n",
        "fs-DX\n",
        "pj-RW\n",
        "zg-RW\n",
        "start-pj\n",
        "he-WI\n",
        "zg-he\n",
        "pj-fs\n",
        "start-RW\n"
    );

    #[test]
    #[ignore]
    fn test_part1_short() -> io::Result<()> {
        let s = SHORT_EXAMPLE;
        let expected = 10;
        let actual = part1(&mut Cursor::new(s))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part1_medium() -> io::Result<()> {
        let s = MEDIUM_EXAMPLE;
        let expected = 19;
        let actual = part1(&mut Cursor::new(s))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part1_long() -> io::Result<()> {
        let s = LONG_EXAMPLE;
        let expected = 226;
        let actual = part1(&mut Cursor::new(s))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part2_short() -> io::Result<()> {
        let s = SHORT_EXAMPLE;
        let expected = 36;
        let actual = part2(&mut Cursor::new(s))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part2_medium() -> io::Result<()> {
        let s = MEDIUM_EXAMPLE;
        let expected = 103;
        let actual = part2(&mut Cursor::new(s))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part2_long() -> io::Result<()> {
        let s = LONG_EXAMPLE;
        let expected = 3509;
        let actual = part2(&mut Cursor::new(s))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
