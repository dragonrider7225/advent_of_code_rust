use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use nom::{branch, bytes::complete as bytes, combinator, multi, IResult};

#[derive(Clone, Copy, Debug)]
enum Pixel {
    Space,
    Galaxy,
}

impl Pixel {
    fn nom_parse(s: &str) -> IResult<&str, Self> {
        branch::alt((
            combinator::value(Self::Space, bytes::tag(".")),
            combinator::value(Self::Galaxy, bytes::tag("#")),
        ))(s)
    }
}

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    let mut image = input
        .lines()
        .map(|line| {
            let line = line?;
            // TODO: Return this directly when it gets fixed
            #[allow(clippy::let_and_return)]
            let ret = multi::many1(Pixel::nom_parse)(&line)
                .map(|(_, pixels)| pixels)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()));
            ret
        })
        .try_fold::<_, _, io::Result<_>>(vec![], |mut acc, row| {
            let row = row?;
            if row.iter().all(|pixel| matches!(pixel, Pixel::Space)) {
                acc.extend([row.clone(), row]);
            } else {
                acc.push(row);
            }
            Ok(acc)
        })?;
    for column in (0..(image[0].len() - 1)).rev() {
        if image
            .iter()
            .map(|row| row[column])
            .all(|pixel| matches!(pixel, Pixel::Space))
        {
            image
                .iter_mut()
                .for_each(|row| row.insert(column, Pixel::Space));
        }
    }
    let galaxies = image
        .into_iter()
        .enumerate()
        .fold(vec![], |mut acc, (row_idx, row)| {
            acc.extend(row.into_iter().enumerate().filter_map(|(col_idx, pixel)| {
                Some((row_idx, col_idx)).filter(|_| matches!(pixel, Pixel::Galaxy))
            }));
            acc
        });
    Ok(galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, galaxy1)| {
            galaxies[(i + 1)..]
                .iter()
                .map(|galaxy2| galaxy2.0.abs_diff(galaxy1.0) + galaxy2.1.abs_diff(galaxy1.1))
        })
        .sum())
}

fn part2(input: &mut dyn BufRead, scale_factor: usize) -> io::Result<usize> {
    let (image, duplicated_rows) = input
        .lines()
        .map(|line| {
            let line = line?;
            // TODO: Return this directly when it gets fixed
            #[allow(clippy::let_and_return)]
            let ret = multi::many1(Pixel::nom_parse)(&line)
                .map(|(_, pixels)| pixels)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()));
            ret
        })
        .enumerate()
        .try_fold::<_, _, io::Result<_>>((vec![], vec![]), |mut acc, (row_idx, row)| {
            let row = row?;
            if row.iter().all(|pixel| matches!(pixel, Pixel::Space)) {
                acc.1.push(row_idx);
            }
            acc.0.push(row);
            Ok(acc)
        })?;
    let duplicated_columns = (0..(image[0].len() - 1))
        .filter(|&col_idx| {
            image
                .iter()
                .map(|row| row[col_idx])
                .all(|pixel| matches!(pixel, Pixel::Space))
        })
        .collect::<Vec<_>>();
    let galaxies = image
        .into_iter()
        .enumerate()
        .fold(vec![], |mut acc, (row_idx, row)| {
            acc.extend(row.into_iter().enumerate().filter_map(|(col_idx, pixel)| {
                Some((row_idx, col_idx)).filter(|_| matches!(pixel, Pixel::Galaxy))
            }));
            acc
        });
    Ok(galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, galaxy1)| {
            galaxies[(i + 1)..].iter().map(|galaxy2| {
                let row_range = if galaxy1.0 < galaxy2.0 {
                    galaxy1.0..galaxy2.0
                } else {
                    galaxy2.0..galaxy1.0
                };
                let col_range = if galaxy1.1 < galaxy2.1 {
                    galaxy1.1..galaxy2.1
                } else {
                    galaxy2.1..galaxy1.1
                };
                (galaxy2.0.abs_diff(galaxy1.0)
                    + duplicated_rows
                        .iter()
                        .filter(|row_idx| row_range.contains(row_idx))
                        .count()
                        * (scale_factor - 1))
                    + (galaxy2.1.abs_diff(galaxy1.1)
                        + duplicated_columns
                            .iter()
                            .filter(|col_idx| col_range.contains(col_idx))
                            .count()
                            * (scale_factor - 1))
            })
        })
        .sum())
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2023 Day 11 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2023_11.txt")?))?
        );
    }
    {
        println!("Year 2023 Day 11 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2023_11.txt")?), 1_000_000)?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        "...#......\n",
        ".......#..\n",
        "#.........\n",
        "..........\n",
        "......#...\n",
        ".#........\n",
        ".........#\n",
        "..........\n",
        ".......#..\n",
        "#...#.....\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 374;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 1030;
        let actual = part2(&mut Cursor::new(TEST_DATA), 10)?;
        assert_eq!(expected, actual);
        let expected = 8410;
        let actual = part2(&mut Cursor::new(TEST_DATA), 100)?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
