use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
    str::FromStr,
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Point {s:?} missing comma"),
        ))?;
        let x = x.parse().map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid x coordinate {x:?}: {e}"),
            )
        })?;
        let y = y.parse().map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid y coordinate {y:?}: {e}"),
            )
        })?;
        Ok(Self { x, y })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Line {
    from: Point,
    to: Point,
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.from.y == self.to.y
    }

    fn is_vertical(&self) -> bool {
        self.from.x == self.to.x
    }
}

impl FromStr for Line {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, to) = s.split_once(" -> ").ok_or(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Line {s:?} missing arrow"),
        ))?;
        let from = from.parse()?;
        let to = to.parse()?;
        let (from, to) = if to < from { (to, from) } else { (from, to) };
        Ok(Self { from, to })
    }
}

fn read_lines(input: &mut dyn BufRead) -> impl Iterator<Item = io::Result<Line>> + '_ {
    input.lines().map(|line| {
        line?
            .parse::<Line>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    })
}

fn count_points_covered(lines: impl Iterator<Item = io::Result<Line>>) -> io::Result<usize> {
    let mut points_covered = HashMap::<_, usize>::new();
    for line in lines {
        let line = line?;
        if line.is_horizontal() {
            for x in line.from.x..=line.to.x {
                *points_covered
                    .entry(Point { x, y: line.from.y })
                    .or_default() += 1;
            }
        } else if line.is_vertical() {
            for y in line.from.y..=line.to.y {
                *points_covered
                    .entry(Point { y, x: line.from.x })
                    .or_default() += 1;
            }
        } else if line.from.y < line.to.y {
            // line is diagonal with positive slope
            for delta in 0..=(line.to.x - line.from.x) {
                *points_covered
                    .entry(Point {
                        x: line.from.x + delta,
                        y: line.from.y + delta,
                    })
                    .or_default() += 1;
            }
        } else {
            // line is diagonal with negative slope
            for delta in 0..=(line.to.x - line.from.x) {
                *points_covered
                    .entry(Point {
                        x: line.from.x + delta,
                        y: line.from.y - delta,
                    })
                    .or_default() += 1;
            }
        }
    }
    Ok(points_covered
        .into_iter()
        .filter_map(|(_, num_lines)| Some(num_lines).filter(|&x| x > 1))
        .count())
}

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    count_points_covered(read_lines(input).filter_map(|line| {
        let line = match line {
            Err(e) => return Some(Err(e)),
            Ok(line) => line,
        };
        Some(line)
            .filter(|line| line.is_horizontal() || line.is_vertical())
            .map(Ok)
    }))
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    count_points_covered(read_lines(input))
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2021 Day 5 Part 1");
        println!(
            "There are {} points that are part of multiple vertical and horizontal lines",
            part1(&mut BufReader::new(File::open("2021_05.txt")?))?
        );
    }
    {
        println!("Year 2021 Day 5 Part 2");
        println!(
            "There are {} points in multiple lines",
            part2(&mut BufReader::new(File::open("2021_05.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    #[ignore]
    fn test_part1() -> io::Result<()> {
        let s = "0,9 -> 5,9\n8,0 -> 0,8\n9,4 -> 3,4\n2,2 -> 2,1\n7,0 -> 7,4\n6,4 -> 2,0\n0,9 -> 2,9\n3,4 -> 1,4\n0,0 -> 8,8\n5,5 -> 8,2";
        let expected = 5;
        let actual = part1(&mut Cursor::new(s))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part2() -> io::Result<()> {
        let s = "0,9 -> 5,9\n8,0 -> 0,8\n9,4 -> 3,4\n2,2 -> 2,1\n7,0 -> 7,4\n6,4 -> 2,0\n0,9 -> 2,9\n3,4 -> 1,4\n0,0 -> 8,8\n5,5 -> 8,2";
        let expected = 12;
        let actual = part2(&mut Cursor::new(s))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
