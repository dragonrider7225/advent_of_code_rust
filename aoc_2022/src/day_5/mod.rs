use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    str::FromStr,
};

type Stack<T> = Vec<T>;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Warehouse {
    stacks: [Stack<u8>; 9],
    num_stacks: usize,
}

impl Warehouse {
    fn new() -> Self {
        Self {
            stacks: [
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
            ],
            num_stacks: 0,
        }
    }

    fn read(input: &mut dyn BufRead) -> io::Result<Self> {
        // A warehouse consists of 0 or more lines of "crates" followed by a line with the numbers
        // from 1 to 9 directly below the crate labels, truncated to the same length as the lines
        // of crates. All lines of crates are the same number of bytes and there is a space between
        // adjacent crates or in the place of a crate if the stack at that position is shorter than
        // the current height. A crate consists of a '[', an uppercase letter, and a ']'.
        let mut ret = Self::new();
        let mut lines = input.lines();
        let mut next_line = || {
            lines
                .next()
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        "Failed to reach bottom of warehouse",
                    )
                })
                .map_or_else(Err, |x| x)
        };
        loop {
            let line = next_line()?;
            if line.is_empty() {
                break;
            }
            let bytes = line.as_bytes();
            if bytes[1] == b'1' {
                let next_line = next_line()?;
                if !next_line.is_empty() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Unexpected non-empty line after warehouse",
                    ));
                }
                break;
            }
            ret.num_stacks = (bytes.len() + 1) / 4;
            for i in 0..ret.num_stacks {
                match bytes[4 * i + 1] {
                    b' ' => {}
                    label => ret.stacks[i].push(label),
                }
            }
        }
        for stack in 0..ret.num_stacks {
            ret.stacks[stack].reverse();
        }
        Ok(ret)
    }

    fn top_crates(&self) -> String {
        String::from_utf8(
            self.stacks[..self.num_stacks]
                .iter()
                .map(|stack| stack.last().copied().unwrap_or(b' '))
                .collect(),
        )
        .unwrap()
    }

    /// `combined` represents whether the top `r#move.count` boxes should be moved one at a time or
    /// all at once.
    fn move_crates(&mut self, r#move: Move, combined: bool) {
        if r#move.from > self.num_stacks {
            panic!("Can't move crates from beyond the last stack");
        }
        if r#move.to > self.num_stacks {
            panic!("Can't move crates to beyond the last stack");
        }
        let from = r#move.from - 1;
        let to = r#move.to - 1;
        if combined {
            let from_stack = &mut self.stacks[from];
            let bottom_moved_crate = from_stack.len() - r#move.count;
            from_stack[bottom_moved_crate..].reverse();
        }
        for _ in 0..r#move.count {
            let current_crate = self.stacks[from]
                .pop()
                .expect("Can't move crates from empty stack");
            self.stacks[to].push(current_crate);
        }
    }
}

struct Move {
    /// 1-indexed
    from: usize,
    /// 1-indexed
    to: usize,
    count: usize,
}

impl FromStr for Move {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn mk_error(msg: String) -> io::Error {
            io::Error::new(io::ErrorKind::InvalidData, msg)
        }
        let bytes = s.as_bytes();
        let log_num_crates = bytes.len() - 17;
        if log_num_crates == 0 {
            return Err(mk_error(format!(
                "Line is too short to be a valid move: {s:?}"
            )));
        }
        if (b"move " != &bytes[..5])
            || !(0..log_num_crates).all(|i| bytes[i + 5].is_ascii_digit())
            || (b" from " != &bytes[(5 + log_num_crates)..(11 + log_num_crates)])
            || !bytes[11 + log_num_crates].is_ascii_digit()
            || (b" to " != &bytes[(12 + log_num_crates)..(16 + log_num_crates)])
            || !bytes[16 + log_num_crates].is_ascii_digit()
        {
            return Err(mk_error(
                r#"Move line must match "move \d\+ from \d to \d"#.to_string(),
            ));
        }
        let count = bytes[5..(5 + log_num_crates)]
            .iter()
            .fold(0, |acc, d| acc * 10 + d - b'0')
            .into();
        let from = usize::from(bytes[11 + log_num_crates] - b'0');
        let to = usize::from(bytes[16 + log_num_crates] - b'0');
        Ok(Self { from, to, count })
    }
}

fn part1(input: &mut dyn BufRead) -> io::Result<String> {
    let mut warehouse = Warehouse::read(input)?;
    for line in input.lines() {
        warehouse.move_crates(line?.parse()?, false);
    }
    Ok(warehouse.top_crates())
}

fn part2(input: &mut dyn BufRead) -> io::Result<String> {
    let mut warehouse = Warehouse::read(input)?;
    for line in input.lines() {
        warehouse.move_crates(line?.parse()?, true);
    }
    Ok(warehouse.top_crates())
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2022 Day 5 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2022_05.txt")?))?
        );
    }
    {
        println!("Year 2022 Day 5 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2022_05.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        "    [D]    \n",
        "[N] [C]    \n",
        "[Z] [M] [P]\n",
        " 1   2   3 \n",
        "\n",
        "move 1 from 2 to 1\n",
        "move 3 from 1 to 3\n",
        "move 2 from 2 to 1\n",
        "move 1 from 1 to 2\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = "CMZ";
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = "MCD";
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
