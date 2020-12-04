use crate::parse::NomParse;

use std::{collections::{HashMap, HashSet}, io, mem, str::FromStr};

use nom::{
    IResult,
    branch,
    bytes::complete as bytes,
    combinator as comb,
    multi,
    sequence,
};

#[derive(Clone, Copy, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl<'s> NomParse<'s> for Direction {
    fn nom_parse(s: &str) -> IResult<&str, Self> {
        branch::alt((
            comb::value(Direction::Up, bytes::tag("U")),
            comb::value(Direction::Down, bytes::tag("D")),
            comb::value(Direction::Left, bytes::tag("L")),
            comb::value(Direction::Right, bytes::tag("R")),
        ))(s)
    }
}

struct Movement(Direction, u32);

impl<'s> NomParse<'s> for Movement {
    fn nom_parse(s: &str) -> IResult<&str, Self> {
        comb::map(
            sequence::pair(Direction::nom_parse, u32::nom_parse),
            |(direction, distance)| Movement(direction, distance),
        )(s)
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Point(i32, i32);

impl Point {
    fn manhattan_distance(&self, other: &Self) -> u64 {
        // -2**31 <= self.0 <= 2**31 - 1
        // -2**31 <= other.0 <= 2**31 - 1
        // -2**32 + 1 <= self.0 - other.0 <= 2**32 - 1
        // abs(self.0 - other.0) <= 2**32 - 1
        let x_dist = (self.0 as i64 - other.0 as i64).abs() as u64;
        // -2**31 <= self.1 <= 2**31 - 1
        // -2**31 <= other.1 <= 2**31 - 1
        // -2**32 + 1 <= self.1 - other.1 <= 2**32 - 1
        // abs(self.1 - other.1) <= 2**32 - 1
        let y_dist = (self.1 as i64 - other.1 as i64).abs() as u64;
        // abs(self.0 - other.0) + abs(self.1 - other.1) <= 2**33 - 2
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
        self.points.get(p).map(|s| *s)
    }

    fn intersections(&self, other: &Self) -> HashSet<(Point, u32)> {
        self.points.keys().collect::<HashSet<_>>()
            .intersection(&other.points.keys().collect())
            .map(|p| {
                ((*p).clone(), self.steps(p).unwrap() + other.steps(p).unwrap())
            })
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

impl<'s> NomParse<'s> for Wire {
    fn nom_parse(s: &str) -> IResult<&str, Self> {
        comb::map(
            multi::separated_list1(
                bytes::tag(","),
                Movement::nom_parse,
            ),
            |ms| Wire::from_movements(&ms[..]),
        )(s)
    }
}

impl FromStr for Wire {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::nom_parse(s).map_err(|_| ()).map(|(_, x)| x)
    }
}

pub(super) fn run() -> io::Result<()> {
    {
        // Part 1
        let mut wires = crate::parse_lines::<Wire, _>("2019_3.txt")?;
        let wire1 = wires.next().expect("Missing first wire");
        let wire2 = wires.next().expect("Missing second wire");
        let mut intersections = wire1.intersections(&wire2).into_iter()
            .map(|(p, _)| p.manhattan_distance_o())
            .collect::<Vec<_>>();
        intersections.sort();
        println!("Minimum intersection distance is {}", intersections[0]);
    }
    {
        // Part 2
        let mut wires = crate::parse_lines::<Wire, _>("2019_3.txt")?;
        let wire1 = wires.next().expect("Missing first wire");
        let wire2 = wires.next().expect("Missing second wire");
        let mut intersections = wire1.intersections(&wire2).into_iter()
            .map(|x| x.1)
            .collect::<Vec<_>>();
        intersections.sort();
        println!("Minimum combined steps is {}", intersections[0]);
    }
    Ok(())
}
