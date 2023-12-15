use aoc_util::nom_extended::NomParse;

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead, BufReader},
    mem,
};

use nom::{
    branch, bytes::complete as bytes, character::complete as character, combinator as comb, multi,
    sequence, IResult,
};

#[derive(Clone, Copy, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl<'s> NomParse<&'s str> for Direction {
    fn nom_parse(s: &'s str) -> IResult<&'s str, Self> {
        branch::alt((
            comb::value(Direction::Up, bytes::tag("U")),
            comb::value(Direction::Down, bytes::tag("D")),
            comb::value(Direction::Left, bytes::tag("L")),
            comb::value(Direction::Right, bytes::tag("R")),
        ))(s)
    }
}

struct Movement(Direction, u32);

impl<'s> NomParse<&'s str> for Movement {
    fn nom_parse(s: &'s str) -> IResult<&'s str, Self> {
        comb::map(
            sequence::pair(Direction::nom_parse, character::u32),
            |(direction, distance)| Movement(direction, distance),
        )(s)
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Point(i32, i32);

impl Point {
    fn manhattan_distance(&self, other: &Self) -> u64 {
        let x_dist = self.0.abs_diff(other.0) as u64;
        let y_dist = self.1.abs_diff(other.1) as u64;
        x_dist + y_dist
    }

    fn manhattan_distance_o(&self) -> u64 {
        self.manhattan_distance(&Point(0, 0))
    }
}

struct Wire {
    end: Point,
    len: u32,
    /// Maps the points that the wire passes through to the number of steps it
    /// takes to get there the first time.
    points: HashMap<Point, u32>,
}

impl Wire {
    fn new() -> Wire {
        Wire {
            end: Point(0, 0),
            len: 0,
            points: HashMap::new(),
        }
    }

    fn from_movements(movements: &[Movement]) -> Wire {
        let mut ret = Wire::new();
        for movement in movements {
            ret.add_segment(movement);
        }
        ret
    }

    fn steps(&self, p: &Point) -> Option<u32> {
        self.points.get(p).copied()
    }

    fn intersections(&self, other: &Self) -> HashSet<(Point, u32)> {
        self.points
            .keys()
            .collect::<HashSet<_>>()
            .intersection(&other.points.keys().collect())
            .map(|&p| (*p, self.steps(p).unwrap() + other.steps(p).unwrap()))
            .collect()
    }

    fn add_segment(&mut self, movement: &Movement) {
        let mut new_end = mem::replace(&mut self.end, Point(0, 0));
        match movement.0 {
            Direction::Up => {
                for _ in 0..movement.1 {
                    new_end.1 += 1;
                    self.len += 1;
                    self.points.entry(new_end).or_insert(self.len);
                }
            }
            Direction::Down => {
                for _ in 0..movement.1 {
                    new_end.1 -= 1;
                    self.len += 1;
                    self.points.entry(new_end).or_insert(self.len);
                }
            }
            Direction::Left => {
                for _ in 0..movement.1 {
                    new_end.0 -= 1;
                    self.len += 1;
                    self.points.entry(new_end).or_insert(self.len);
                }
            }
            Direction::Right => {
                for _ in 0..movement.1 {
                    new_end.0 += 1;
                    self.len += 1;
                    self.points.entry(new_end).or_insert(self.len);
                }
            }
        }
        self.end = new_end;
    }
}

impl<'s> NomParse<&'s str> for Wire {
    fn nom_parse(s: &'s str) -> IResult<&'s str, Self> {
        comb::map(
            multi::separated_list1(bytes::tag(","), Movement::nom_parse),
            |ms| Wire::from_movements(&ms[..]),
        )(s)
    }
}

aoc_util::impl_from_str_for_nom_parse!(Wire);

pub(super) fn run() -> io::Result<()> {
    {
        // Part 1
        let mut wires = BufReader::new(File::open("2019_3.txt")?)
            .lines()
            .map(|line| {
                line?
                    .parse::<Wire>()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            });
        let wire1 = wires.next().expect("Missing first wire")?;
        let wire2 = wires.next().expect("Missing second wire")?;
        let mut intersections = wire1
            .intersections(&wire2)
            .into_iter()
            .map(|(p, _)| p.manhattan_distance_o())
            .collect::<Vec<_>>();
        intersections.sort_unstable();
        println!("Minimum intersection distance is {}", intersections[0]);
    }
    {
        // Part 2
        let mut wires = BufReader::new(File::open("2019_3.txt")?)
            .lines()
            .map(|line| {
                line?
                    .parse::<Wire>()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            });
        let wire1 = wires.next().expect("Missing first wire")?;
        let wire2 = wires.next().expect("Missing second wire")?;
        let mut intersections = wire1
            .intersections(&wire2)
            .into_iter()
            .map(|x| x.1)
            .collect::<Vec<_>>();
        intersections.sort_unstable();
        println!("Minimum combined steps is {}", intersections[0]);
    }
    Ok(())
}
