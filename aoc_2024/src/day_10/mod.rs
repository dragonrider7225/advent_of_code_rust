use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead, BufReader},
    ops::Deref,
};

use aoc_util::{
    geometry::Point2D,
    impl_from_str_for_nom_parse,
    nom::{character::complete as character, multi, IResult, Parser},
    nom_extended::NomParse,
    nom_supreme::ParserExt,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Height(u32);

impl Deref for Height {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl NomParse<&str> for Height {
    fn nom_parse(input: &str) -> IResult<&str, Self> {
        character::one_of("0123456789")
            .map(|c| Self((c as u8 - b'0') as _))
            .parse(input)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Map {
    heights: Vec<Vec<Height>>,
}

impl NomParse<&str> for Map {
    fn nom_parse(input: &str) -> IResult<&str, Self> {
        multi::many1(multi::many1(Height::nom_parse).terminated(character::line_ending))
            .map(|heights| Self { heights })
            .parse(input)
    }
}

impl_from_str_for_nom_parse!(Map);

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    let map = io::read_to_string(input)?
        .parse::<Map>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok((0..map.heights.len())
        .map(move |row_idx| {
            let heights = &map.heights;
            let num_cols = heights[row_idx].len();
            (0..num_cols)
                .filter(move |&col_idx| *heights[row_idx][col_idx] == 0)
                .map(move |col_idx| {
                    (1..10)
                        .fold(
                            HashSet::<_>::from_iter([Point2D::at(col_idx, row_idx)]),
                            |acc, height| {
                                acc.into_iter()
                                    .flat_map(|position| {
                                        [
                                            position.checked_sub(Point2D::at(1, 0)),
                                            position.checked_sub(Point2D::at(0, 1)),
                                            position
                                                .checked_add(Point2D::at(1, 0))
                                                .filter(|p| *p.x() < num_cols),
                                            position
                                                .checked_add(Point2D::at(0, 1))
                                                .filter(|p| *p.y() < heights.len()),
                                        ]
                                    })
                                    .flatten()
                                    .filter(|neighbor| {
                                        *heights[*neighbor.y()][*neighbor.x()] == height
                                    })
                                    .collect()
                            },
                        )
                        .len()
                })
                .sum::<usize>()
        })
        .sum())
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    let map = io::read_to_string(input)?
        .parse::<Map>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok((0..map.heights.len())
        .map(move |row_idx| {
            let heights = &map.heights;
            let num_cols = heights[row_idx].len();
            (0..num_cols)
                .filter(move |&col_idx| *heights[row_idx][col_idx] == 0)
                .map(move |col_idx| {
                    (1..10)
                        .fold(
                            HashMap::<_, _>::from_iter([(Point2D::at(col_idx, row_idx), 1usize)]),
                            |acc, height| {
                                acc.into_iter()
                                    .flat_map(|(position, count)| {
                                        [
                                            position.checked_sub(Point2D::at(1, 0)),
                                            position.checked_sub(Point2D::at(0, 1)),
                                            position
                                                .checked_add(Point2D::at(1, 0))
                                                .filter(|p| *p.x() < num_cols),
                                            position
                                                .checked_add(Point2D::at(0, 1))
                                                .filter(|p| *p.y() < heights.len()),
                                        ]
                                        .into_iter()
                                        .flat_map(
                                            move |position| {
                                                position.map(|position| (position, count))
                                            },
                                        )
                                    })
                                    .filter(|(neighbor, _)| {
                                        *heights[*neighbor.y()][*neighbor.x()] == height
                                    })
                                    .fold(HashMap::new(), |mut acc, (position, count)| {
                                        *acc.entry(position).or_default() += count;
                                        acc
                                    })
                            },
                        )
                        .values()
                        .sum::<usize>()
                })
                .sum::<usize>()
        })
        .sum())
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2024 Day 10 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2024_10.txt")?))?
        );
    }
    {
        println!("Year 2024 Day 10 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2024_10.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        "89010123\n",
        "78121874\n",
        "87430965\n",
        "96549874\n",
        "45678903\n",
        "32019012\n",
        "01329801\n",
        "10456732\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 36;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 81;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
