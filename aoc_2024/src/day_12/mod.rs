use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader},
    iter,
};

use aoc_util::geometry::Direction;

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    let map = input
        .lines()
        .map(|line| Ok(line?.chars().map(|c| c as u8 - b'A').collect::<Vec<_>>()))
        .collect::<io::Result<Vec<_>>>()?;
    let mut visited = map
        .iter()
        .map(|row| row.iter().map(|_| false).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut total = 0;
    for row_idx in 0..map.len() {
        let row = &map[row_idx];
        for col_idx in 0..row.len() {
            if visited[row_idx][col_idx] {
                continue;
            }
            let mut num_edges = 0;
            let mut current = HashSet::<_>::from_iter([(col_idx, row_idx)]);
            let mut next = current.clone();
            while !next.is_empty() {
                next = next
                    .into_iter()
                    .flat_map(|(col_idx, row_idx)| {
                        [
                            (col_idx.saturating_sub(1), row_idx),
                            (col_idx, row_idx.saturating_sub(1)),
                            (col_idx, (row_idx + 1).min(map.len() - 1)),
                            ((col_idx + 1).min(row.len() - 1), row_idx),
                        ]
                    })
                    .filter(|pos| {
                        if map[pos.1][pos.0] != map[row_idx][col_idx] {
                            num_edges += 1;
                            false
                        } else {
                            true
                        }
                    })
                    .filter(|&pos| current.insert(pos))
                    .collect()
            }
            num_edges += current
                .iter()
                .map(|pos| {
                    (pos.0 == 0 || pos.0 + 1 == row.len()) as usize
                        + (pos.1 == 0 || pos.1 + 1 == map.len()) as usize
                })
                .sum::<usize>();
            total += current.len() * num_edges;
            current
                .into_iter()
                .for_each(|(col_idx, row_idx)| visited[row_idx][col_idx] = true);
        }
    }
    Ok(total)
}

type Position = (usize, usize);

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    fn fence_len(
        map: &[Vec<u8>],
        current_group: u8,
        neighbors: &mut HashSet<Position>,
        first_corner: Position,
        first_facing: Direction,
    ) -> usize {
        iter::successors(Some((first_corner, first_facing)), |(corner, facing)| {
            match facing {
                Direction::Right => {
                    if corner.1 == 0 {
                        let next_out = (corner.0..map[0].len())
                            .find(|&col_idx| map[0][col_idx] != current_group)
                            .unwrap_or(map[0].len());
                        Some(((next_out, 0), Direction::Down))
                    } else {
                        let (next_corner, next_facing) = (corner.0..map[corner.1].len())
                            .find(|&col_idx| {
                                map[corner.1][col_idx] != current_group
                                    || map[corner.1 - 1][col_idx] == current_group
                            })
                            .map(|col_idx| {
                                if map[corner.1][col_idx] != current_group {
                                    ((col_idx, corner.1), Direction::Down)
                                } else {
                                    ((col_idx, corner.1), Direction::Up)
                                }
                            })
                            .unwrap_or(((map[corner.1].len(), corner.1), Direction::Down));
                        for col_idx in corner.0..next_corner.0 {
                            neighbors.remove(&(col_idx, corner.1 - 1));
                        }
                        Some((next_corner, next_facing))
                    }
                }
                Direction::Down => {
                    if corner.0 == map[corner.1].len() {
                        let next_out = (corner.1..map.len())
                            .find(|&row_idx| *map[row_idx].last().unwrap() != current_group)
                            .unwrap_or(map.len());
                        Some(((corner.0, next_out), Direction::Left))
                    } else {
                        let (next_corner, next_facing) = (corner.1..map.len())
                            .find(|&row_idx| {
                                map[row_idx][corner.0 - 1] != current_group
                                    || map[row_idx][corner.0] == current_group
                            })
                            .map(|row_idx| {
                                if map[row_idx][corner.0 - 1] != current_group {
                                    ((corner.0, row_idx), Direction::Left)
                                } else {
                                    ((corner.0, row_idx), Direction::Right)
                                }
                            })
                            .unwrap_or(((corner.0, map.len()), Direction::Left));
                        for row_idx in corner.1..next_corner.1 {
                            neighbors.remove(&(corner.0, row_idx));
                        }
                        Some((next_corner, next_facing))
                    }
                }
                Direction::Left => {
                    if corner.1 == map.len() {
                        let next_out = (0..corner.0)
                            .rev()
                            .find(|&col_idx| map.last().unwrap()[col_idx] != current_group)
                            .map(|n| n + 1)
                            .unwrap_or(0);
                        Some(((next_out, corner.1), Direction::Up))
                    } else {
                        let (next_corner, next_facing) = (0..corner.0)
                            .rev()
                            .find(|&col_idx| {
                                map[corner.1 - 1][col_idx] != current_group
                                    || map[corner.1][col_idx] == current_group
                            })
                            .map(|col_idx| {
                                if map[corner.1 - 1][col_idx] != current_group {
                                    ((col_idx + 1, corner.1), Direction::Up)
                                } else {
                                    ((col_idx + 1, corner.1), Direction::Down)
                                }
                            })
                            .unwrap_or(((0, corner.1), Direction::Up));
                        for col_idx in next_corner.0..corner.0 {
                            neighbors.remove(&(col_idx, corner.1));
                        }
                        Some((next_corner, next_facing))
                    }
                }
                Direction::Up => {
                    if corner.0 == 0 {
                        let next_out = (0..corner.1)
                            .rev()
                            .find(|&row_idx| map[row_idx][0] != current_group)
                            .map(|n| n + 1)
                            .unwrap_or(0);
                        Some(((corner.0, next_out), Direction::Right))
                    } else {
                        let (next_corner, next_facing) = (0..corner.1)
                            .rev()
                            .find(|&row_idx| {
                                map[row_idx][corner.0] != current_group
                                    || map[row_idx][corner.0 - 1] == current_group
                            })
                            .map(|row_idx| {
                                if map[row_idx][corner.0] != current_group {
                                    ((corner.0, row_idx + 1), Direction::Right)
                                } else {
                                    ((corner.0, row_idx + 1), Direction::Left)
                                }
                            })
                            .unwrap_or(((corner.0, 0), Direction::Right));
                        for row_idx in next_corner.1..corner.1 {
                            neighbors.remove(&(corner.0 - 1, row_idx));
                        }
                        Some((next_corner, next_facing))
                    }
                }
            }
            .filter(|&(corner, _)| corner != first_corner)
        })
        .count()
    }

    let map = input
        .lines()
        .map(|line| Ok(line?.chars().map(|c| c as u8 - b'A').collect::<Vec<_>>()))
        .collect::<io::Result<Vec<_>>>()?;
    let mut visited = map
        .iter()
        .map(|row| row.iter().map(|_| false).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut total = 0;
    for row_idx in 0..map.len() {
        let row = &map[row_idx];
        for col_idx in 0..row.len() {
            if visited[row_idx][col_idx] {
                continue;
            }
            let current_group = map[row_idx][col_idx];
            let mut neighbors = HashSet::new();
            let mut current = HashSet::<_>::from_iter([(col_idx, row_idx)]);
            let mut next = current.clone();
            while !next.is_empty() {
                next = next
                    .into_iter()
                    .flat_map(|(col_idx, row_idx)| {
                        [
                            (col_idx.saturating_sub(1), row_idx),
                            (col_idx, row_idx.saturating_sub(1)),
                            (col_idx, (row_idx + 1).min(map.len() - 1)),
                            ((col_idx + 1).min(row.len() - 1), row_idx),
                        ]
                    })
                    .filter(|&pos| {
                        if map[pos.1][pos.0] == map[row_idx][col_idx] {
                            current.insert(pos)
                        } else {
                            neighbors.insert(pos);
                            false
                        }
                    })
                    .collect()
            }
            let current = current;
            let &first_corner = current
                .iter()
                .min_by_key(|&(col_idx, row_idx)| (row_idx, col_idx))
                .unwrap();
            let mut num_edges = fence_len(
                &map,
                current_group,
                &mut neighbors,
                first_corner,
                Direction::Right,
            );
            while let Some(&first_corner) = neighbors
                .iter()
                .min_by_key(|&(col_idx, row_idx)| (row_idx, col_idx))
            {
                num_edges += fence_len(
                    &map,
                    current_group,
                    &mut neighbors,
                    first_corner,
                    Direction::Down,
                );
            }
            total += current.len() * num_edges;
            current
                .into_iter()
                .for_each(|(col_idx, row_idx)| visited[row_idx][col_idx] = true);
        }
    }
    Ok(total)
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2024 Day 12 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2024_12.txt")?))?
        );
    }
    {
        println!("Year 2024 Day 12 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2024_12.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use io::Cursor;

    use super::*;

    const TEST_DATA_1: &str = "AAAA\nBBCD\nBBCC\nEEEC\n";

    const TEST_DATA_2: &str = "OOOOO\nOXOXO\nOOOOO\nOXOXO\nOOOOO\n";

    const TEST_DATA_3: &str = concat!(
        "RRRRIICCFF\n",
        "RRRRIICCCF\n",
        "VVRRRCCFFF\n",
        "VVRCCCJFFF\n",
        "VVVVCJJCFE\n",
        "VVIVCCJJEE\n",
        "VVIIICJJEE\n",
        "MIIIIIJJEE\n",
        "MIIISIJEEE\n",
        "MMMISSJEEE\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 140;
        let actual = part1(&mut Cursor::new(TEST_DATA_1))?;
        assert_eq!(expected, actual);
        let expected = 772;
        let actual = part1(&mut Cursor::new(TEST_DATA_2))?;
        assert_eq!(expected, actual);
        let expected = 1930;
        let actual = part1(&mut Cursor::new(TEST_DATA_3))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2_a() -> io::Result<()> {
        let expected = 80;
        let actual = part2(&mut Cursor::new(TEST_DATA_1))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2_b() -> io::Result<()> {
        let expected = 436;
        let actual = part2(&mut Cursor::new(TEST_DATA_2))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2_c() -> io::Result<()> {
        let expected = 1206;
        let actual = part2(&mut Cursor::new(TEST_DATA_3))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    const TEST_DATA_4: &str = "EEEEE\nEXXXX\nEEEEE\nEXXXX\nEEEEE\n";

    #[test]
    fn test_part2_d() -> io::Result<()> {
        let expected = 236;
        let actual = part2(&mut Cursor::new(TEST_DATA_4))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    const TEST_DATA_5: &str = "AAAAAA\nAAABBA\nAAABBA\nABBAAA\nABBAAA\nAAAAAA\n";

    #[test]
    fn test_part2_e() -> io::Result<()> {
        let expected = 368;
        let actual = part2(&mut Cursor::new(TEST_DATA_5))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
