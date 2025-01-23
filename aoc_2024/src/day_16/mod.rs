use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead, BufReader},
    ops::Index,
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
    Start,
    End,
    Wall,
    Floor,
}

impl NomParse<&str> for Tile {
    fn nom_parse(input: &str) -> IResult<&str, Self> {
        bytes::tag("S")
            .map(|_| Self::Start)
            .or(bytes::tag("E").map(|_| Self::End))
            .or(bytes::tag("#").map(|_| Self::Wall))
            .or(bytes::tag(".").map(|_| Self::Floor))
            .parse(input)
    }
}

fn raster_direction(direction: Direction) -> Direction {
    match direction {
        Direction::Up | Direction::Down => -direction,
        Direction::Left | Direction::Right => direction,
    }
}

type Position = (Point2D<usize>, Direction);

#[derive(Clone, Debug, Eq, PartialEq)]
struct Map {
    rows: Vec<Vec<Tile>>,
    start: Point2D<usize>,
    end: Point2D<usize>,
}

impl Map {
    fn best_score(&self) -> Option<usize> {
        let mut visited = HashSet::new();
        let mut to_visit = HashMap::<_, _>::from_iter([((self.start, Direction::Right), 0)]);
        while let Some((&(pos, facing), &score)) = to_visit.iter().min_by_key(|&(_, score)| score) {
            if pos == self.end {
                return Some(score);
            }
            visited.insert((pos, facing));
            to_visit.remove(&(pos, facing));
            for (key, value) in self
                .path_step((pos, facing), score)
                .filter(|(key, _)| !visited.contains(key))
            {
                if to_visit.get(&key).filter(|&&score| score < value).is_none() {
                    to_visit.insert(key, value);
                }
            }
        }
        None
    }

    fn contains(&self, pos: &Point2D<usize>) -> bool {
        self.rows.len() > *pos.y() && self.rows[*pos.y()].len() > *pos.x()
    }

    fn path_step(
        &self,
        position: Position,
        score: usize,
    ) -> impl Iterator<Item = (Position, usize)> {
        let saturating_add = |(pos, facing): Position| match facing {
            Direction::Up if *pos.y() == 0 => pos,
            Direction::Left if *pos.x() == 0 => pos,
            _ => pos + raster_direction(facing),
        };
        let (pos, facing) = position;
        [
            ((saturating_add(position), facing), score + 1),
            ((pos, facing.rotate_clockwise()), score + 1000),
            ((pos, facing.rotate_counter_clockwise()), score + 1000),
        ]
        .into_iter()
        .filter(|((pos, _), _)| self.contains(pos) && !matches!(self[*pos], Tile::Wall))
    }

    /// From the given position and orientation, calculate the cells used by paths to `Tile::End` in
    /// the map with score at most `max_score`.
    fn on_paths(&self, max_score: usize) -> HashSet<Point2D<usize>> {
        let _ = max_score;
        // use aoc_util::collections::{PriorityQueue, SharedFilo};
        // use std::cmp::Reverse;
        // #[derive(Clone, Debug, Eq, PartialEq)]
        // enum CacheValue {
        //     /// All paths from the cache key associated with this value to the end of the race cost
        //     /// at least `cost` and the set of fastest paths collectively use the set of tiles
        //     /// `successors`.
        //     Found {
        //         /// The cost of the fastest path(s) from the cache key associated with this value
        //         /// to the end of the race.
        //         cost: usize,
        //         /// The set of all tiles on any path to the end of the race with cost `cost`.
        //         successors: HashSet<Point2D<usize>>,
        //     },
        //     /// There is no path from the cache key associated with this value to the end of the
        //     /// race that costs less than `min_cost`.
        //     NotFound {
        //         /// The cost at which attempts to find a path to the end of the race were given up.
        //         min_cost: usize,
        //     },
        // }

        // fn go(
        //     this: &Map,
        //     start: Position,
        //     max_score: usize,
        //     cache: &mut HashMap<Position, CacheValue>,
        // ) -> HashSet<Point2D<usize>> {
        //     let (position, _) = start;
        //     if position == this.end {
        //         return HashSet::from_iter([position]);
        //     }
        //     let mut ret = HashSet::new();
        //     for (neighbor, cost) in this
        //         .path_step(start, 0)
        //         .filter(|&(_, cost)| cost <= max_score)
        //     {
        //         let cached = cache.get(&neighbor).filter(|&v| match *v {
        //             CacheValue::Found {
        //                 cost: remaining_cost,
        //                 ..
        //             } => remaining_cost <= max_score - cost,
        //             CacheValue::NotFound { min_cost } => min_cost > max_score - cost,
        //         });
        //         match cached {
        //             Some(CacheValue::Found { successors, .. }) => {
        //                 ret.extend(successors.iter().copied());
        //             }
        //             Some(&CacheValue::NotFound { .. }) => {}
        //             None => {
        //                 let max_score = max_score - cost;
        //                 let path_tiles = go(this, neighbor, max_score, cache);
        //                 if !path_tiles.is_empty() {
        //                     ret.extend(path_tiles.iter().copied());
        //                     cache.insert(
        //                         neighbor,
        //                         CacheValue::Found {
        //                             cost: max_score,
        //                             successors: path_tiles,
        //                         },
        //                     );
        //                 } else {
        //                     cache.insert(
        //                         neighbor,
        //                         CacheValue::NotFound {
        //                             min_cost: max_score,
        //                         },
        //                     );
        //                 }
        //             }
        //         }
        //     }
        //     ret
        // }

        // let start = (self.start, Direction::Right);
        // go(self, start, max_score, &mut HashMap::new())

        // fn bfs(this: &Map, start: Position, max_score: usize) -> HashSet<Point2D<usize>> {
        //     let mut frontier = PriorityQueue::from_iter([(start, Reverse(0))]);
        //     let mut visited = frontier
        //         .iter()
        //         .map(|(&position, _)| (position, (0, vec![SharedFilo::new()])))
        //         .collect::<HashMap<_, _>>();
        //     while let Some((position, Reverse(score))) = frontier.pop_with_priority() {
        //         if score > max_score {
        //             break;
        //         }
        //         let Some((_, paths)) = visited.get(&position).cloned() else {
        //             unreachable!()
        //         };
        //         for (neighbor, neighbor_cost) in this.path_step(position, score) {
        //             let &mut (best_neighbor, ref mut neighbor_paths) = visited
        //                 .entry(neighbor)
        //                 .or_insert_with(|| (neighbor_cost, vec![]));
        //             if best_neighbor < neighbor_cost {
        //                 continue;
        //             }
        //             neighbor_paths.extend(paths.iter().map(|path| path.push(neighbor)));
        //             frontier.insert(neighbor, Reverse(neighbor_cost));
        //         }
        //     }
        //     visited
        //         .into_iter()
        //         .filter(|&((position, _), _)| this.end == position)
        //         .map(|(_, (_, paths))| paths)
        //         .fold(HashSet::new(), |mut acc, paths| {
        //             paths
        //                 .into_iter()
        //                 .for_each(|path| acc.extend(path.into_iter().map(|(tile, _)| tile)));
        //             acc
        //         })
        // }

        // bfs(self, start, max_score)
        unimplemented!()
    }
}

impl Index<Point2D<usize>> for Map {
    type Output = Tile;

    fn index(&self, index: Point2D<usize>) -> &Self::Output {
        &self.rows[*index.y()][*index.x()]
    }
}

impl NomParse<&str> for Map {
    fn nom_parse(input: &str) -> IResult<&str, Self> {
        multi::many1(multi::many1(Tile::nom_parse).terminated(character::line_ending))
            .map_res(Self::try_from)
            .parse(input)
    }
}

impl_from_str_for_nom_parse!(Map);

impl TryFrom<Vec<Vec<Tile>>> for Map {
    type Error = &'static str;

    fn try_from(rows: Vec<Vec<Tile>>) -> Result<Self, Self::Error> {
        let start = rows
            .iter()
            .enumerate()
            .find_map(|(row_idx, row)| {
                row.iter().enumerate().find_map(|(col_idx, tile)| {
                    Some(Point2D::at(col_idx, row_idx)).filter(|_| matches!(tile, Tile::Start))
                })
            })
            .ok_or("Missing start tile")?;
        let end = rows
            .iter()
            .enumerate()
            .find_map(|(row_idx, row)| {
                row.iter().enumerate().find_map(|(col_idx, tile)| {
                    Some(Point2D::at(col_idx, row_idx)).filter(|_| matches!(tile, Tile::End))
                })
            })
            .ok_or("Missing end tile")?;
        Ok(Self { rows, start, end })
    }
}

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    let input = io::read_to_string(input)?;
    let map = input
        .parse::<Map>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    map.best_score()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "End tile could not be reached"))
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    let input = io::read_to_string(input)?;
    let map = input
        .parse::<Map>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let max_score = map.best_score().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidData, "End tile could not be reached")
    })?;
    dbg!(max_score);
    // return Err(io::Error::new(
    //     io::ErrorKind::Unsupported,
    //     "The test cases currently cause part 2 to freeze",
    // ));
    Ok(map.on_paths(max_score).len())
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2024 Day 16 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2024_16.txt")?))?
        );
    }
    {
        println!("Year 2024 Day 16 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2024_16.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use io::Cursor;

    use super::*;

    const TEST_DATA_1: &str = concat!(
        "###############\n",
        "#.......#....E#\n",
        "#.#.###.#.###.#\n",
        "#.....#.#...#.#\n",
        "#.###.#####.#.#\n",
        "#.#.#.......#.#\n",
        "#.#.#####.###.#\n",
        "#...........#.#\n",
        "###.#.#####.#.#\n",
        "#...#.....#.#.#\n",
        "#.#.#.###.#.#.#\n",
        "#.....#...#.#.#\n",
        "#.###.#.#.#.#.#\n",
        "#S..#.....#...#\n",
        "###############\n",
    );

    const TEST_DATA_2: &str = concat!(
        "#################\n",
        "#...#...#...#..E#\n",
        "#.#.#.#.#.#.#.#.#\n",
        "#.#.#.#...#...#.#\n",
        "#.#.#.#.###.#.#.#\n",
        "#...#.#.#.....#.#\n",
        "#.#.#.#.#.#####.#\n",
        "#.#...#.#.#.....#\n",
        "#.#.#####.#.###.#\n",
        "#.#.#.......#...#\n",
        "#.#.###.#####.###\n",
        "#.#.#...#.....#.#\n",
        "#.#.#.#####.###.#\n",
        "#.#.#.........#.#\n",
        "#.#.#.#########.#\n",
        "#S#.............#\n",
        "#################\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 7036;
        let actual = part1(&mut Cursor::new(TEST_DATA_1))?;
        assert_eq!(expected, actual);
        let expected = 11048;
        let actual = part1(&mut Cursor::new(TEST_DATA_2))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 45;
        let actual = part2(&mut Cursor::new(TEST_DATA_1))?;
        assert_eq!(expected, actual);
        let expected = 64;
        let actual = part2(&mut Cursor::new(TEST_DATA_2))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
