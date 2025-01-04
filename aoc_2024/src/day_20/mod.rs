use std::{
    collections::HashMap,
    fmt::{self, Debug, Display, Formatter},
    fs::File,
    io::{self, BufRead, BufReader},
};

use aoc_util::{
    geometry::{Direction, Point2D},
    impl_from_str_for_nom_parse,
    nom::{bytes::complete as bytes, character::complete as character, multi, IResult, Parser},
    nom_extended::NomParse,
    nom_supreme::ParserExt,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tile {
    Track,
    Start,
    End,
    Wall,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Track => write!(f, "."),
            Self::Start => write!(f, "S"),
            Self::End => write!(f, "E"),
            Self::Wall => write!(f, "#"),
        }
    }
}

impl NomParse<&str> for Tile {
    fn nom_parse(input: &str) -> IResult<&str, Self> {
        bytes::tag(".")
            .map(|_| Self::Track)
            .or(bytes::tag("S").map(|_| Self::Start))
            .or(bytes::tag("E").map(|_| Self::End))
            .or(bytes::tag("#").map(|_| Self::Wall))
            .parse(input)
    }
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    start: Point2D<usize>,
    end: Point2D<usize>,
}

impl Map {
    fn new(tiles: Vec<Vec<Tile>>) -> Result<Self, &'static str> {
        let start = tiles
            .iter()
            .enumerate()
            .find_map(|(row_idx, row)| {
                row.iter()
                    .position(|tile| matches!(tile, Tile::Start))
                    .map(move |col_idx| Point2D::at(col_idx, row_idx))
            })
            .ok_or("Couldn't find start tile")?;
        let end = tiles
            .iter()
            .enumerate()
            .find_map(|(row_idx, row)| {
                row.iter()
                    .position(|tile| matches!(tile, Tile::End))
                    .map(move |col_idx| Point2D::at(col_idx, row_idx))
            })
            .ok_or("Couldn't find end tile")?;
        Ok(Self { tiles, start, end })
    }

    fn shortest_path_no_cheats(&self) -> Vec<Point2D<usize>> {
        let mut ret = vec![self.start];
        while *ret.last().unwrap() != self.end {
            let next = Direction::values()
                .iter()
                .copied()
                .map(|direction| *ret.last().unwrap() + direction)
                .filter(|neighbor| {
                    self.tiles
                        .get(*neighbor.y())
                        .and_then(|row| {
                            row.get(*neighbor.x())
                                .filter(|tile| !matches!(tile, Tile::Wall))
                        })
                        .is_some()
                })
                .find(|neighbor| !ret.contains(neighbor))
                .unwrap();
            ret.push(next);
        }
        ret
    }

    /// Maps number of ticks saved over the no-cheat path to the number of possible ways to cheat
    /// for up to 20 ticks and save exactly that many ticks.
    fn cheat_efficiencies(&self, min_savings: usize, max_cheat: usize) -> HashMap<usize, usize> {
        let path = self.shortest_path_no_cheats();
        path.iter()
            .copied()
            .enumerate()
            .flat_map(|(trigger_idx, cheat_start)| {
                path.iter()
                    .copied()
                    .enumerate()
                    .skip(trigger_idx + min_savings)
                    .filter_map(move |(release_idx, cheat_end)| {
                        let cheat_time = cheat_start.x().abs_diff(*cheat_end.x())
                            + cheat_start.y().abs_diff(*cheat_end.y());
                        let maze_time = release_idx - trigger_idx;
                        if cheat_time <= max_cheat && maze_time >= cheat_time {
                            Some(maze_time - cheat_time).filter(|&saved| saved >= min_savings)
                        } else {
                            None
                        }
                    })
            })
            .fold(HashMap::new(), |mut acc, savings| {
                *acc.entry(savings).or_default() += 1;
                acc
            })
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let label_width = self.tiles.len().ilog10() as usize + 1;
        for (row_idx, row) in self.tiles.iter().enumerate() {
            write!(f, "{row_idx: >label_width$} ")?;
            for tile in row {
                write!(f, "{tile}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl NomParse<&str> for Map {
    fn nom_parse(input: &str) -> IResult<&str, Self> {
        multi::many1(multi::many1(Tile::nom_parse).terminated(character::line_ending))
            .map_res(Self::new)
            .parse(input)
    }
}

impl_from_str_for_nom_parse!(Map);

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    let input = io::read_to_string(input)?;
    let map = input
        .parse::<Map>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(map.cheat_efficiencies(100, 2).values().sum())
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    let input = io::read_to_string(input)?;
    let map = input
        .parse::<Map>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(map.cheat_efficiencies(100, 20).values().sum())
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2024 Day 20 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2024_20.txt")?))?
        );
    }
    {
        println!("Year 2024 Day 20 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2024_20.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = concat!(
        "###############\n",
        "#...#...#.....#\n",
        "#.#.#.#.#.###.#\n",
        "#S#...#.#.#...#\n",
        "#######.#.#.###\n",
        "#######.#.#...#\n",
        "#######.#.###.#\n",
        "###..E#...#...#\n",
        "###.#######.###\n",
        "#...###...#...#\n",
        "#.#####.#.###.#\n",
        "#.#...#.#.#...#\n",
        "#.#.#.#.#.#.###\n",
        "#...#...#...###\n",
        "###############\n",
    );

    #[test]
    fn test_part1() {
        let expected = [
            (2, 14),
            (4, 14),
            (6, 2),
            (8, 4),
            (10, 2),
            (12, 3),
            (20, 1),
            (36, 1),
            (38, 1),
            (40, 1),
            (64, 1),
        ];
        let map = TEST_DATA.parse::<Map>().unwrap();
        eprintln!("{map}");
        let mut actual = map.cheat_efficiencies(1, 2).into_iter().collect::<Vec<_>>();
        actual.sort_unstable();
        assert_eq!(&expected, &actual[..]);
    }

    #[test]
    fn test_part2() {
        let expected = [
            (50, 32),
            (52, 31),
            (54, 29),
            (56, 39),
            (58, 25),
            (60, 23),
            (62, 20),
            (64, 19),
            (66, 12),
            (68, 14),
            (70, 12),
            (72, 22),
            (74, 4),
            (76, 3),
        ];
        let map = TEST_DATA.parse::<Map>().unwrap();
        eprintln!("{map}");
        let mut actual = map
            .cheat_efficiencies(50, 20)
            .into_iter()
            .collect::<Vec<_>>();
        actual.sort_unstable();
        dbg!(&actual);
        assert_eq!(&expected, &actual[..]);
    }
}
