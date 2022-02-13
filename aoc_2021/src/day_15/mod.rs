use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader},
};

struct PathNode {
    total_risk: u32,
    position: (usize, usize),
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Grid {
    risk: Vec<u32>,
    width: usize,
    height: usize,
    expanded: bool,
}

impl Grid {
    fn read(input: &mut dyn BufRead) -> io::Result<Self> {
        input.lines().try_fold(Self::default(), |mut acc, line| {
            let line = line?;
            if acc.is_empty() {
                acc.width = line.len();
            } else if acc.width != line.len() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Line {:?} incorrect length, expected {}", line, acc.width),
                ));
            }
            acc.reserve();
            line.chars()
                .map(|c| {
                    c.to_digit(10).ok_or_else(|| {
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("Invalid risk level {c:?} in line {line:?}"),
                        )
                    })
                })
                .try_for_each(|risk| {
                    acc.risk.push(risk?);
                    io::Result::Ok(())
                })?;
            acc.height += 1;
            Ok(acc)
        })
    }
}

impl Grid {
    fn neighbors(&self, (x, y): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        [
            x.checked_sub(1).map(|x| (x, y)),
            y.checked_sub(1).map(|y| (x, y)),
            Some(x + 1).filter(|&x| x < self.width()).map(|x| (x, y)),
            Some(y + 1).filter(|&y| y < self.height()).map(|y| (x, y)),
        ]
        .into_iter()
        .flatten()
    }

    fn is_empty(&self) -> bool {
        self.risk.is_empty()
    }

    fn risk_at(&self, (x, y): (usize, usize)) -> Option<u32> {
        if self.expanded {
            let mega_x = (x / self.width) as u32;
            let mega_y = (y / self.height) as u32;
            let x = x % self.width;
            let y = y % self.height;
            let risk = self.risk[y * self.width + x];
            let risk = risk + mega_x + mega_y;
            if risk > 9 {
                Some(risk - 9)
            } else {
                Some(risk)
            }
        } else if x < self.width && y < self.height {
            Some(self.risk[y * self.width + x])
        } else {
            None
        }
    }

    fn width(&self) -> usize {
        if self.expanded {
            5 * self.width
        } else {
            self.width
        }
    }

    fn height(&self) -> usize {
        if self.expanded {
            5 * self.height
        } else {
            self.height
        }
    }

    fn lowest_risk(&self) -> u32 {
        let mut seen = HashSet::new();
        let mut frontier = vec![PathNode {
            total_risk: 0,
            position: (0, 0),
        }];
        while !frontier.is_empty() {
            if seen.len() % 1000 == 0 {
                println!("Visited {} cells", seen.len());
            }
            let frontier_len = frontier.len();
            frontier.select_nth_unstable_by(frontier_len - 1, |left: &PathNode, right| {
                left.total_risk.cmp(&right.total_risk).reverse()
            });
            let current = frontier.pop().unwrap();
            let pos = current.position;
            if pos.0 > 900 || pos.1 > 900 {
                println!(
                    "Least risky path to {:?} is {} risk",
                    pos, current.total_risk
                );
            }
            if pos == (self.width() - 1, self.height() - 1) {
                println!("Visited {} cells", seen.len());
                return current.total_risk;
            }
            seen.insert(pos);
            let mut neighbors = self
                .neighbors(pos)
                .filter(|pos| !seen.contains(pos))
                .collect::<HashSet<_>>();
            for node in frontier.iter_mut() {
                if neighbors.contains(&node.position) {
                    let new_risk = current.total_risk + self.risk_at(node.position).unwrap();
                    if node.total_risk > new_risk {
                        node.total_risk = new_risk;
                    }
                    neighbors.remove(&node.position);
                }
            }
            neighbors.into_iter().for_each(|neighbor| {
                frontier.push(PathNode {
                    total_risk: current.total_risk + self.risk_at(neighbor).unwrap(),
                    position: neighbor,
                })
            });
        }
        panic!("Saw {} positions without reaching the end", seen.len())
    }
}

impl Grid {
    fn reserve(&mut self) {
        self.risk.reserve(self.width)
    }

    fn expand_map(&mut self) {
        self.expanded = true;
    }
}

fn part1(input: &mut dyn BufRead) -> io::Result<u32> {
    let grid = Grid::read(input)?;
    Ok(grid.lowest_risk())
}

fn part2(input: &mut dyn BufRead) -> io::Result<u32> {
    let mut grid = Grid::read(input)?;
    grid.expand_map();
    Ok(grid.lowest_risk())
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2021 Day 15 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2021_15.txt")?))?
        );
    }
    {
        println!("Year 2021 Day 15 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2021_15.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        "1163751742\n",
        "1381373672\n",
        "2136511328\n",
        "3694931569\n",
        "7463417111\n",
        "1319128137\n",
        "1359912421\n",
        "3125421639\n",
        "1293138521\n",
        "2311944581\n"
    );

    #[test]
    #[ignore]
    fn test_part1() -> io::Result<()> {
        let expected = 40;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part2() -> io::Result<()> {
        let expected = 315;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
