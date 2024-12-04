use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use aoc_util::{
    impl_from_str_for_nom_parse,
    nom::{
        branch, bytes::complete as bytes, character::complete as character, multi, IResult, Parser,
    },
    nom_extended::NomParse,
    nom_supreme::ParserExt,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Letter {
    X,
    M,
    A,
    S,
}

impl NomParse<&str> for Letter {
    fn nom_parse(input: &str) -> IResult<&str, Self> {
        branch::alt((
            bytes::tag("X").map(|_| Self::X),
            bytes::tag("M").map(|_| Self::M),
            bytes::tag("A").map(|_| Self::A),
            bytes::tag("S").map(|_| Self::S),
        ))(input)
    }
}

struct WordSearch {
    lines: Vec<Vec<Letter>>,
}

impl NomParse<&str> for WordSearch {
    fn nom_parse(input: &str) -> IResult<&str, Self> {
        multi::many1(multi::many1(Letter::nom_parse).terminated(character::line_ending))
            .map(|lines| Self { lines })
            .parse(input)
    }
}

impl_from_str_for_nom_parse!(WordSearch);

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    let input = {
        let mut buf = String::new();
        input.read_to_string(&mut buf)?;
        buf
    };
    let word_search = input
        .parse::<WordSearch>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    const WORD: [Letter; 4] = [Letter::X, Letter::M, Letter::A, Letter::S];
    let mut count = 0;
    for row_idx in 0..word_search.lines.len() {
        let row = &word_search.lines[row_idx];
        for col_idx in (0..row.len()).filter(|&col_idx| matches!(row[col_idx], Letter::X)) {
            let space_above = row_idx >= 3;
            let space_left = col_idx >= 3;
            let space_below = row_idx + 3 < word_search.lines.len();
            let space_right = col_idx + 3 < row.len();
            if space_above {
                if space_left {
                    count +=
                        WORD.iter().enumerate().all(|(n, &letter)| {
                            word_search.lines[row_idx - n][col_idx - n] == letter
                        }) as usize;
                }
                count += WORD
                    .iter()
                    .enumerate()
                    .all(|(n, &letter)| word_search.lines[row_idx - n][col_idx] == letter)
                    as usize;
                if space_right {
                    count +=
                        WORD.iter().enumerate().all(|(n, &letter)| {
                            word_search.lines[row_idx - n][col_idx + n] == letter
                        }) as usize;
                }
            }
            if space_left {
                count += WORD
                    .iter()
                    .enumerate()
                    .all(|(n, &letter)| word_search.lines[row_idx][col_idx - n] == letter)
                    as usize;
            }
            if space_right {
                count += WORD
                    .iter()
                    .enumerate()
                    .all(|(n, &letter)| word_search.lines[row_idx][col_idx + n] == letter)
                    as usize;
            }
            if space_below {
                if space_left {
                    count +=
                        WORD.iter().enumerate().all(|(n, &letter)| {
                            word_search.lines[row_idx + n][col_idx - n] == letter
                        }) as usize;
                }
                count += WORD
                    .iter()
                    .enumerate()
                    .all(|(n, &letter)| word_search.lines[row_idx + n][col_idx] == letter)
                    as usize;
                if space_right {
                    count +=
                        WORD.iter().enumerate().all(|(n, &letter)| {
                            word_search.lines[row_idx + n][col_idx + n] == letter
                        }) as usize;
                }
            }
        }
    }
    Ok(count)
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    let input = {
        let mut buf = String::new();
        input.read_to_string(&mut buf)?;
        buf
    };
    let word_search = input
        .parse::<WordSearch>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    const VALID_DIAGONALS: [[Letter; 2]; 2] = [[Letter::M, Letter::S], [Letter::S, Letter::M]];
    let mut count = 0;
    for row_idx in 1..(word_search.lines.len() - 1) {
        let row = &word_search.lines[row_idx];
        #[expect(
            clippy::needless_range_loop,
            reason = "The suggested form is needlessly complex"
        )]
        for col_idx in 1..(row.len() - 1) {
            if matches!(row[col_idx], Letter::A) {
                count += [
                    [
                        word_search.lines[row_idx + 1][col_idx - 1],
                        word_search.lines[row_idx - 1][col_idx + 1],
                    ],
                    [
                        word_search.lines[row_idx - 1][col_idx - 1],
                        word_search.lines[row_idx + 1][col_idx + 1],
                    ],
                ]
                .into_iter()
                .all(|diagonal| VALID_DIAGONALS.contains(&diagonal))
                    as usize;
            }
        }
    }
    Ok(count)
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2024 Day 4 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2024_04.txt")?))?
        );
    }
    {
        println!("Year 2024 Day 4 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2024_04.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        "MMMSXXMASM\n",
        "MSAMXMSMSA\n",
        "AMXSXMAAMM\n",
        "MSAMASMSMX\n",
        "XMASAMXAMM\n",
        "XXAMMXXAMA\n",
        "SMSMSASXSS\n",
        "SAXAMASAAA\n",
        "MAMMMXMMMM\n",
        "MXMXAXMASX\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 18;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 9;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
