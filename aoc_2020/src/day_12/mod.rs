use aoc_util::{geometry::Point2D as Point, nom_parse::NomParse};
use nom::{character::complete as character, combinator as comb, sequence, IResult};
use std::{io, str::FromStr};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Facing {
    East,
    North,
    South,
    West,
}

impl Facing {
    fn back(&self) -> Self {
        match *self {
            Self::East => Self::West,
            Self::North => Self::South,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }

    fn left(&self) -> Self {
        match *self {
            Self::East => Self::North,
            Self::North => Self::West,
            Self::South => Self::East,
            Self::West => Self::South,
        }
    }

    fn right(&self) -> Self {
        match *self {
            Self::East => Self::South,
            Self::North => Self::East,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Ship {
    facing: Facing,
    location: Point<i32>,
}

impl Ship {
    fn step(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::East(distance) => self.location += Point::at(distance, 0),
            Instruction::South(distance) => self.location += Point::at(0, -distance),
            Instruction::North(distance) => self.location += Point::at(0, distance),
            Instruction::West(distance) => self.location += Point::at(-distance, 0),
            Instruction::Left(0) | Instruction::Right(0) => {}
            Instruction::Left(90) | Instruction::Right(270) => self.facing = self.facing.left(),
            Instruction::Left(180) | Instruction::Right(180) => self.facing = self.facing.back(),
            Instruction::Left(270) | Instruction::Right(90) => self.facing = self.facing.right(),
            Instruction::Left(degrees) => unreachable!("Invalid left rotation: {}", degrees),
            Instruction::Right(degrees) => unreachable!("Invalid right rotation: {}", degrees),
            Instruction::Forward(distance) => self.step(Instruction::from((self.facing, distance))),
        }
    }

    fn execute(&mut self, instructions: &[Instruction]) {
        instructions
            .iter()
            .for_each(|&instruction| self.step(instruction));
    }
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            facing: Facing::East,
            location: Point::at(0, 0),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Waypoint<'ship> {
    ship: &'ship mut Ship,
    location: Point<i32>,
}

impl<'ship> Waypoint<'ship> {
    fn new(ship: &'ship mut Ship) -> Self {
        Self {
            ship,
            location: Point::at(10, 1),
        }
    }

    fn step(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::North(distance) => self.location += Point::at(0, distance),
            Instruction::South(distance) => self.location += Point::at(0, -distance),
            Instruction::East(distance) => self.location += Point::at(distance, 0),
            Instruction::West(distance) => self.location += Point::at(-distance, 0),
            Instruction::Left(0) | Instruction::Right(0) => {}
            Instruction::Left(90) | Instruction::Right(270) => {
                self.location = Point::at(-self.location.y(), *self.location.x())
            }
            Instruction::Left(180) | Instruction::Right(180) => self.location = -self.location,
            Instruction::Left(270) | Instruction::Right(90) => {
                self.location = Point::at(*self.location.y(), -self.location.x())
            }
            Instruction::Left(degrees) => unreachable!("Invalid left rotation: {}", degrees),
            Instruction::Right(degrees) => unreachable!("Invalid right rotation: {}", degrees),
            Instruction::Forward(count) => (0..count).for_each(|_| {
                self.ship
                    .step(Instruction::from((Facing::East, *self.location.x())));
                self.ship
                    .step(Instruction::from((Facing::North, *self.location.y())));
            }),
        }
    }

    fn execute(&mut self, instructions: &[Instruction]) {
        instructions
            .iter()
            .for_each(|&instruction| self.step(instruction));
    }
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

impl From<(Facing, i32)> for Instruction {
    fn from((facing, distance): (Facing, i32)) -> Self {
        match facing {
            Facing::East => Self::East(distance),
            Facing::North => Self::North(distance),
            Facing::South => Self::South(distance),
            Facing::West => Self::West(distance),
        }
    }
}

// TODO: impl_from_str_for_nom_parse!(Instruction);
impl FromStr for Instruction
where
    Self: for<'s> NomParse<'s, &'s str, Error = nom::error::Error<&'s str>>,
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ::nom::Finish;

        Self::nom_parse(s)
            .finish()
            .map(|(_, res)| res)
            .map_err(|error| format!("{error:?}"))
    }
}

impl<'s> NomParse<'s, &'s str> for Instruction {
    type Error = nom::error::Error<&'s str>;

    fn nom_parse(s: &'s str) -> IResult<&'s str, Self> {
        comb::map(
            sequence::pair(character::one_of("NSEWLRF"), u16::nom_parse),
            |(c, distance)| match c {
                'N' => Self::North(i32::from(distance)),
                'S' => Self::South(i32::from(distance)),
                'E' => Self::East(i32::from(distance)),
                'W' => Self::West(i32::from(distance)),
                'L' => Self::Left(i32::from(distance)),
                'R' => Self::Right(i32::from(distance)),
                'F' => Self::Forward(i32::from(distance)),
                _ => unreachable!(),
            },
        )(s)
    }
}

pub(super) fn run() -> io::Result<()> {
    let directions = aoc_util::parse_lines::<Instruction, _>("2020_12.txt")?.collect::<Vec<_>>();
    {
        println!("Year 2020 Day 12 Part 1");
        let mut ship = Ship::default();
        ship.execute(&directions);
        println!(
            "The manhattan distance that the ship covers is {}",
            ship.location.manhattan_distance(&Point::at(0, 0)),
        );
    }
    {
        println!("Year 2020 Day 12 Part 2");
        let mut ship = Ship::default();
        let mut waypoint = Waypoint::new(&mut ship);
        waypoint.execute(&directions);
        println!(
            "The manhattan distance that the ship covers is {}",
            ship.location.manhattan_distance(&Point::at(0, 0)),
        );
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[ignore]
    #[test]
    fn ship_follows_instructions() {
        let expected = Ship {
            facing: Facing::South,
            location: Point::at(17, -8),
        };
        let mut actual = Ship::default();
        actual.execute(&[
            Instruction::Forward(10),
            Instruction::North(3),
            Instruction::Forward(7),
            Instruction::Right(90),
            Instruction::Forward(11),
        ]);
        assert_eq!(expected, actual);
    }

    #[ignore]
    #[test]
    fn waypoint_follows_instructions() {
        let expected = Ship {
            facing: Facing::East,
            location: Point::at(214, -72),
        };
        let mut actual = Ship::default();
        let mut waypoint = Waypoint::new(&mut actual);
        waypoint.execute(&[
            Instruction::Forward(10),
            Instruction::North(3),
            Instruction::Forward(7),
            Instruction::Right(90),
            Instruction::Forward(11),
        ]);
        std::mem::forget(waypoint);
        assert_eq!(expected, actual);
    }
}
