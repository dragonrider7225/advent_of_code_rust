use std::{
    collections::HashMap,
    fs::File,
    hash::Hash,
    io::{self, BufRead, BufReader},
    mem,
};

struct Polymer {
    len: u64,
    pairs: HashMap<(char, char), u64>,
    counts: HashMap<char, u64>,
    rules: HashMap<(char, char), char>,
}

impl Polymer {
    fn read(input: &mut dyn BufRead) -> io::Result<Self> {
        let mut buf = String::new();
        let elements = {
            input.read_line(&mut buf)?;
            buf.drain(..).filter(|&c| c != '\n').collect::<Vec<_>>()
        };
        let len = elements.len() as u64;
        let pairs = elements.windows(2).fold(HashMap::new(), |mut acc, pair| {
            *acc.entry((pair[0], pair[1])).or_default() += 1;
            acc
        });
        let counts = elements
            .into_iter()
            .fold(HashMap::new(), |mut acc, element| {
                *acc.entry(element).or_default() += 1;
                acc
            });
        input.read_line(&mut buf)?;
        assert_eq!(buf, "\n");
        let rules = input
            .lines()
            .map(|line| {
                let line = line?;
                let mk_error = || {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Invalid pair insertion rule: {line:?}"),
                    )
                };
                let (pair, result) = line.trim().split_once(" -> ").ok_or_else(mk_error)?;
                match (pair.len(), result.len()) {
                    (2, 1) => {}
                    _ => return Err(mk_error()),
                }
                let mut pair_chars = pair.chars();
                let left = pair_chars.next().ok_or_else(mk_error)?;
                let right = pair_chars.next().ok_or_else(mk_error)?;
                let result = result.chars().next().ok_or_else(mk_error)?;
                Ok(((left, right), result))
            })
            .collect::<io::Result<_>>()?;
        Ok(Self {
            len,
            pairs,
            counts,
            rules,
        })
    }
}

impl Polymer {
    fn counts(&self) -> &HashMap<char, u64> {
        &self.counts
    }
}

impl Polymer {
    fn polymerize(&mut self) {
        fn update_entry<K>(map: &mut HashMap<K, u64>, key: K, count: u64)
        where
            K: Eq + Hash,
        {
            *map.entry(key).or_default() += count;
        }

        for ((left, right), count) in mem::take(&mut self.pairs) {
            let center = self.rules.get(&(left, right)).copied();
            match center {
                None => update_entry(&mut self.pairs, (left, right), count),
                Some(center) => {
                    update_entry(&mut self.pairs, (left, center), count);
                    update_entry(&mut self.pairs, (center, right), count);
                    update_entry(&mut self.counts, center, count);
                    self.len += count;
                }
            }
        }
    }
}

fn part1(input: &mut dyn BufRead) -> io::Result<u64> {
    let mut polymer = Polymer::read(input)?;
    for _ in 0..10 {
        polymer.polymerize();
    }
    Ok(polymer.counts().values().max().unwrap() - polymer.counts().values().min().unwrap())
}

fn part2(input: &mut dyn BufRead) -> io::Result<u64> {
    let mut polymer = Polymer::read(input)?;
    for _ in 0..40 {
        polymer.polymerize();
    }
    Ok(polymer.counts().values().max().unwrap() - polymer.counts().values().min().unwrap())
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2021 Day 14 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2021_14.txt")?))?
        );
    }
    {
        println!("Year 2021 Day 14 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2021_14.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        "NNCB\n",
        "\n",
        "CH -> B\n",
        "HH -> N\n",
        "CB -> H\n",
        "NH -> C\n",
        "HB -> C\n",
        "HC -> B\n",
        "HN -> C\n",
        "NN -> C\n",
        "BH -> H\n",
        "NC -> B\n",
        "NB -> B\n",
        "BN -> B\n",
        "BB -> N\n",
        "BC -> B\n",
        "CC -> N\n",
        "CN -> C\n",
    );

    #[test]
    #[ignore]
    fn test_part1() -> io::Result<()> {
        let expected = 1588;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part2() -> io::Result<()> {
        let expected = 2_188_189_693_529;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
