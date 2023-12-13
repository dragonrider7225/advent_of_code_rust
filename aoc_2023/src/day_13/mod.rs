use std::{
    fmt::{self, Display, Formatter},
    fs::File,
    io::{self, BufRead, BufReader},
};

use nom::{branch, bytes::complete as bytes, combinator, multi, sequence, IResult};

fn newline(s: &str) -> IResult<&str, ()> {
    combinator::value((), branch::alt((bytes::tag("\n"), bytes::tag("\r\n"))))(s)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tile {
    Ash,
    Rocks,
}

impl Tile {
    fn nom_parse(s: &str) -> IResult<&str, Self> {
        branch::alt((
            combinator::value(Self::Ash, bytes::tag(".")),
            combinator::value(Self::Rocks, bytes::tag("#")),
        ))(s)
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ash => write!(f, "."),
            Self::Rocks => write!(f, "#"),
        }
    }
}

#[derive(Clone, Debug)]
struct Array {
    rows: Vec<Vec<Tile>>,
}

impl Array {
    fn nom_parse(s: &str) -> IResult<&str, Self> {
        combinator::map(
            multi::many1(sequence::terminated(multi::many1(Tile::nom_parse), newline)),
            |rows| Self { rows },
        )(s)
    }

    fn find_mirror(&self) -> usize {
        for i in 1..self.rows.len() {
            if (1..=i.min(self.rows.len() - i)).all(|di| self.rows[i - di] == self.rows[i + di - 1])
            {
                return 100 * i;
            }
        }
        for j in 1..self.rows[0].len() {
            if (1..=j.min(self.rows[0].len() - j)).all(|dj| {
                (0..self.rows.len()).all(|i| self.rows[i][j - dj] == self.rows[i][j + dj - 1])
            }) {
                return j;
            }
        }
        unreachable!("Every array must have a mirror");
    }

    fn find_smudged_mirror(&self) -> usize {
        for i in 1..self.rows.len() {
            if let Ok(1) = (1..=i.min(self.rows.len() - i)).try_fold(0, |acc, di| {
                let num_smudges = self.rows[i - di]
                    .iter()
                    .zip(&self.rows[i + di - 1])
                    .filter(|(t1, t2)| t1 != t2)
                    .count();
                match acc + num_smudges {
                    num_smudges @ 0 | num_smudges @ 1 => Ok(num_smudges),
                    _ => Err(()),
                }
            }) {
                return i * 100;
            }
        }
        for j in 1..self.rows[0].len() {
            if let Ok(1) = (1..=j.min(self.rows[0].len() - j)).try_fold(0, |acc, dj| {
                let num_smudges = (0..self.rows.len())
                    .filter(|&i| self.rows[i][j - dj] != self.rows[i][j + dj - 1])
                    .count();
                match acc + num_smudges {
                    num_smudges @ 0 | num_smudges @ 1 => Ok(num_smudges),
                    _ => Err(()),
                }
            }) {
                return j;
            }
        }
        todo!("Vertical mirror")
    }
}

impl Display for Array {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in &self.rows {
            for tile in row {
                write!(f, "{tile}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    let input = {
        let mut s = String::new();
        input.read_to_string(&mut s)?;
        s
    };
    let arrays = multi::separated_list1(newline, Array::nom_parse)(&input)
        .map(|(_, arrays)| arrays)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    Ok(arrays.into_iter().map(|array| array.find_mirror()).sum())
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    let input = {
        let mut s = String::new();
        input.read_to_string(&mut s)?;
        s
    };
    let arrays = multi::separated_list1(newline, Array::nom_parse)(&input)
        .map(|(_, arrays)| arrays)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    Ok(arrays
        .into_iter()
        .map(|array| array.find_smudged_mirror())
        .sum())
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2023 Day 13 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2023_13.txt")?))?
        );
    }
    {
        println!("Year 2023 Day 13 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2023_13.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        "#.##..##.\n",
        "..#.##.#.\n",
        "##......#\n",
        "##......#\n",
        "..#.##.#.\n",
        "..##..##.\n",
        "#.#.##.#.\n",
        "\n",
        "#...##..#\n",
        "#....#..#\n",
        "..##..###\n",
        "#####.##.\n",
        "#####.##.\n",
        "..##..###\n",
        "#....#..#\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 405;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 400;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
