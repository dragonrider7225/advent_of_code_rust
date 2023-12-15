use std::{
    fmt::{self, Display, Formatter},
    fs::File,
    io::{self, BufRead, BufReader},
    mem,
    ops::Sub,
};

use aoc_util::a_star::{self, AStarState};
use nom::{branch, bytes::complete as bytes, combinator as comb, multi, sequence, Finish, IResult};

fn abs_sub<T>(x: T, y: T) -> T
where
    T: Ord + Sub<Output = T>,
{
    if x < y {
        y - x
    } else {
        x - y
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Amphipod {
    A,
    B,
    C,
    D,
}

impl Amphipod {
    fn nom_parse(s: &str) -> IResult<&str, Self> {
        branch::alt((
            comb::value(Self::A, bytes::tag("A")),
            comb::value(Self::B, bytes::tag("B")),
            comb::value(Self::C, bytes::tag("C")),
            comb::value(Self::D, bytes::tag("D")),
        ))(s)
    }
}

impl Amphipod {
    fn energy_per_step(&self) -> u64 {
        match self {
            Self::A => 1,
            Self::B => 10,
            Self::C => 100,
            Self::D => 1000,
        }
    }
}

impl Display for Amphipod {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::A => write!(f, "A"),
            Self::B => write!(f, "B"),
            Self::C => write!(f, "C"),
            Self::D => write!(f, "D"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum RoomContents {
    Empty,
    Single(Amphipod),
    Double { front: Amphipod, back: Amphipod },
}

impl RoomContents {
    /// Try to move the frontmost amphipod out of the room and return that amphipod.
    #[must_use]
    fn move_out(&mut self) -> Option<Amphipod> {
        match mem::replace(self, Self::Empty) {
            Self::Empty => None,
            Self::Single(amphipod) => Some(amphipod),
            Self::Double { front, back } => {
                *self = Self::Single(back);
                Some(front)
            }
        }
    }

    /// Try to move `amphipod` into the room. If the operation could not be completed, returns
    /// `Some(amphipod)`.
    #[must_use]
    fn move_in(&mut self, amphipod: Amphipod) -> Option<Amphipod> {
        match *self {
            Self::Empty => {
                *self = Self::Single(amphipod);
                None
            }
            Self::Single(back) if back == amphipod => {
                *self = Self::Double {
                    front: amphipod,
                    back,
                };
                None
            }
            Self::Single(_) | Self::Double { .. } => Some(amphipod),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Room {
    desired: Amphipod,
    contents: RoomContents,
}

impl Room {
    /// Try to move the frontmost amphipod out of the room and return that amphipod.
    fn move_out(&mut self) -> Option<Amphipod> {
        match &mut self.contents {
            RoomContents::Empty => None,
            RoomContents::Single(amphipod) if *amphipod == self.desired => None,
            RoomContents::Double { front, back }
                if *front == self.desired && *back == self.desired =>
            {
                None
            }
            contents => contents.move_out(),
        }
    }

    /// Try to move `amphipod` into the room. If the operation could not be completed, returns
    /// `Some(amphipod)`.
    fn move_in(&mut self, amphipod: Amphipod) -> Option<Amphipod> {
        if amphipod != self.desired {
            Some(amphipod)
        } else {
            self.contents.move_in(amphipod)
        }
    }
}

const NUM_ROOMS: usize = 4;
const ENTRANCES: [usize; NUM_ROOMS] = [2, 4, 6, 8];

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct State {
    rooms: [Room; NUM_ROOMS],
    hallway: [Option<Amphipod>; 11],
}

impl State {
    fn read(input: &mut dyn BufRead) -> io::Result<Self> {
        fn read_line(input: &mut dyn BufRead, buf: &mut String) -> io::Result<usize> {
            buf.clear();
            input.read_line(buf)
        }

        let mut buf = String::new();
        let _ = read_line(input, &mut buf)?;
        let _ = read_line(input, &mut buf)?;
        let _ = read_line(input, &mut buf)?;
        let upper = comb::all_consuming(sequence::delimited(
            bytes::tag("###"),
            multi::separated_list1(bytes::tag("#"), Amphipod::nom_parse),
            bytes::tag("###"),
        ))(buf.trim_end())
        .finish()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?
        .1;
        assert_eq!(upper.len(), 4);
        let _ = read_line(input, &mut buf)?;
        let lower = comb::all_consuming(sequence::delimited(
            bytes::tag("  #"),
            multi::separated_list1(bytes::tag("#"), Amphipod::nom_parse),
            bytes::tag("#"),
        ))(buf.trim_end())
        .finish()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?
        .1;
        assert_eq!(lower.len(), 4);
        let _ = read_line(input, &mut buf)?;
        let mut state = State {
            rooms: [
                Room {
                    desired: Amphipod::A,
                    contents: RoomContents::Empty,
                },
                Room {
                    desired: Amphipod::B,
                    contents: RoomContents::Empty,
                },
                Room {
                    desired: Amphipod::C,
                    contents: RoomContents::Empty,
                },
                Room {
                    desired: Amphipod::D,
                    contents: RoomContents::Empty,
                },
            ],
            hallway: [None; 11],
        };
        for i in 0..4 {
            state.rooms[i].contents = RoomContents::Double {
                front: upper[i],
                back: lower[i],
            };
        }
        Ok(state)
    }
}

impl AStarState for State {
    type Distance = u64;

    fn neighbors(&self) -> Vec<(Self::Distance, Self)> {
        let mut neighbors = vec![];
        // For each amphipod in the hallway, try to move it into each room.
        for (i, amphipod) in self
            .hallway
            .iter()
            .enumerate()
            .filter_map(|(i, cell)| cell.as_ref().map(|amphipod| (i, amphipod)))
        {
            #[allow(clippy::needless_range_loop)]
            for room_number in 0..self.rooms.len() {
                let entrance = ENTRANCES[room_number];
                // Can't move through other amphipods.
                if (entrance..i)
                    .chain(i..entrance)
                    .any(|i| self.hallway[i].is_some())
                {
                    continue;
                }
                let mut neighbor = self.clone();
                if neighbor.rooms[room_number].move_in(*amphipod).is_none() {
                    neighbor.hallway[i] = None;
                    let steps_in_hallway = abs_sub(i, entrance) as u64;
                    let steps_in_room = match self.rooms[room_number].contents {
                        RoomContents::Empty => 2,
                        RoomContents::Single(_) => 1,
                        RoomContents::Double { .. } => {
                            unreachable!("Amphipod successfully moved into fully-occupied room")
                        }
                    };
                    let total_steps = steps_in_hallway + steps_in_room;
                    neighbors.push((total_steps * amphipod.energy_per_step(), neighbor));
                }
            }
        }
        // For each room, try to move an amphipod from it into each spot in the hallway.
        let nonempty_rooms = (0..self.rooms.len())
            .filter(|&room_number| self.rooms[room_number].contents != RoomContents::Empty);
        for room_number in nonempty_rooms.clone() {
            assert_ne!(self.rooms[room_number].contents, RoomContents::Empty);
            let entrance = ENTRANCES[room_number];
            for i in (0..entrance)
                .rev()
                .take_while(|&i| self.hallway[i].is_none())
                .filter(|i| !ENTRANCES.contains(i))
                .chain(
                    (entrance..self.hallway.len())
                        .take_while(|&i| self.hallway[i].is_none())
                        .filter(|i| !ENTRANCES.contains(i)),
                )
            {
                let mut neighbor = self.clone();
                if let Some(amphipod) = neighbor.rooms[room_number].move_out() {
                    neighbor.hallway[i] = Some(amphipod);
                    let steps_in_hallway = abs_sub(i, entrance) as u64;
                    let steps_in_room = match self.rooms[room_number].contents {
                        RoomContents::Empty => unreachable!("Filtered out empty rooms"),
                        RoomContents::Single(_) => 2,
                        RoomContents::Double { .. } => 1,
                    };
                    let total_steps = steps_in_hallway + steps_in_room;
                    neighbors.push((
                        total_steps * neighbor.hallway[i].unwrap().energy_per_step(),
                        neighbor,
                    ));
                }
            }
        }
        // For each room, try to move an amphipod from it into each other room.
        for room_number1 in nonempty_rooms.clone() {
            let entrance1 = ENTRANCES[room_number1];
            #[allow(clippy::needless_range_loop)]
            for room_number2 in 0..self.rooms.len() {
                let entrance2 = ENTRANCES[room_number2];
                if entrance1 == entrance2 {
                    continue;
                }
                if (entrance1..entrance2)
                    .chain(entrance2..entrance1)
                    .any(|i| self.hallway[i].is_some())
                {
                    continue;
                }
                let mut neighbor = self.clone();
                let amphipod = match neighbor.rooms[room_number1].move_out() {
                    None => continue,
                    Some(amphipod) => amphipod,
                };
                if neighbor.rooms[room_number2].move_in(amphipod).is_none() {
                    let steps_in_room1 = match self.rooms[room_number1].contents {
                        RoomContents::Empty => unreachable!("Filtered out empty rooms"),
                        RoomContents::Single(_) => 2,
                        RoomContents::Double { .. } => 1,
                    };
                    let steps_in_hallway = abs_sub(entrance1, entrance2) as u64;
                    let steps_in_room2 = match self.rooms[room_number2].contents {
                        RoomContents::Empty => 2,
                        RoomContents::Single(_) => 1,
                        RoomContents::Double { .. } => {
                            unreachable!("Successfully moved into fully-occupied room")
                        }
                    };
                    let total_steps = steps_in_room1 + steps_in_hallway + steps_in_room2;
                    neighbors.push((total_steps * amphipod.energy_per_step(), neighbor));
                }
            }
        }
        // for (_, neighbor) in neighbors.iter() {
        //     let num_amphipods_in_hallway = neighbor.hallway.iter().filter(|o| o.is_some()).count();
        //     let num_amphipods_in_rooms = neighbor
        //         .rooms
        //         .iter()
        //         .map(|room| match room.contents {
        //             RoomContents::Empty => 0,
        //             RoomContents::Single(_) => 1,
        //             RoomContents::Double { .. } => 2,
        //         })
        //         .sum::<usize>();
        //     let num_amphipods = num_amphipods_in_hallway + num_amphipods_in_rooms;
        //     if num_amphipods != 8 {
        //         println!("Have {} amphipods instead of 8", num_amphipods);
        //         println!("Stepped from {:?} to {:?}", self, neighbor);
        //     }
        // }
        neighbors
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        writeln!(f, "#############")?;
        write!(f, "#")?;
        for cell in self.hallway.iter() {
            match cell {
                None => write!(f, ".")?,
                Some(amphipod) => write!(f, "{amphipod}")?,
            }
        }
        writeln!(f, "#")?;
        write!(f, "###")?;
        for room in self.rooms.iter() {
            match room.contents {
                RoomContents::Double { front, .. } => write!(f, "{front}#")?,
                _ => write!(f, ".#")?,
            }
        }
        writeln!(f, "##")?;
        write!(f, "  #")?;
        for room in self.rooms.iter() {
            match room.contents {
                RoomContents::Single(back) | RoomContents::Double { back, .. } => {
                    write!(f, "{back}#")?
                }
                _ => write!(f, ".#")?,
            }
        }
        writeln!(f)?;
        writeln!(f, "  #########")
    }
}

fn amphipod_heuristic(s: &State) -> u64 {
    // Count the energy necessary to move the amphipods into the final state if two amphipods
    // can simultaneously occupy the same space.

    // Count the energy necessary to move the amphipods in the hallway into the backs of their
    // rooms.
    let energy_from_hallway = s
        .hallway
        .iter()
        .enumerate()
        .filter_map(|(i, cell)| cell.as_ref().map(|amphipod| (i, amphipod)))
        .map(|(i, amphipod)| {
            let steps_in_hallway = match amphipod {
                Amphipod::A => abs_sub(i, ENTRANCES[0]),
                Amphipod::B => abs_sub(i, ENTRANCES[1]),
                Amphipod::C => abs_sub(i, ENTRANCES[2]),
                Amphipod::D => abs_sub(i, ENTRANCES[3]),
            } as u64;
            let steps_in_room = 2;
            let total_steps = steps_in_hallway + steps_in_room;
            total_steps * amphipod.energy_per_step()
        })
        .sum::<u64>();
    // Count the energy necessary to move the amphipods in rooms into the backs of their rooms.
    let energy_from_rooms = s
        .rooms
        .iter()
        .enumerate()
        .map(|(room_number1, room)| match room.contents {
            RoomContents::Empty => 0,
            RoomContents::Single(amphipod) => {
                let entrance1 = ENTRANCES[room_number1];
                let entrance2 = match amphipod {
                    Amphipod::A => ENTRANCES[0],
                    Amphipod::B => ENTRANCES[1],
                    Amphipod::C => ENTRANCES[2],
                    Amphipod::D => ENTRANCES[3],
                };
                if entrance1 == entrance2 {
                    0
                } else {
                    let steps_in_room1 = 2;
                    let steps_in_hallway = abs_sub(entrance1, entrance2) as u64;
                    let steps_in_room2 = 2;
                    let total_steps = steps_in_room1 + steps_in_hallway + steps_in_room2;
                    total_steps * amphipod.energy_per_step()
                }
            }
            RoomContents::Double { front, back } => {
                let entrance1 = ENTRANCES[room_number1];
                // front
                let energy1 = {
                    let entrance2 = match front {
                        Amphipod::A => ENTRANCES[0],
                        Amphipod::B => ENTRANCES[1],
                        Amphipod::C => ENTRANCES[2],
                        Amphipod::D => ENTRANCES[3],
                    };
                    let total_steps = if entrance1 == entrance2 {
                        1
                    } else {
                        let steps_in_room1 = 1;
                        let steps_in_hallway = abs_sub(entrance1, entrance2) as u64;
                        let steps_in_room2 = 2;
                        steps_in_room1 + steps_in_hallway + steps_in_room2
                    };
                    total_steps * front.energy_per_step()
                };
                // back
                let energy2 = {
                    let entrance2 = match back {
                        Amphipod::A => ENTRANCES[0],
                        Amphipod::B => ENTRANCES[1],
                        Amphipod::C => ENTRANCES[2],
                        Amphipod::D => ENTRANCES[3],
                    };
                    let total_steps = if entrance1 == entrance2 {
                        0
                    } else {
                        let steps_in_room1 = 2;
                        let steps_in_hallway = abs_sub(entrance1, entrance2) as u64;
                        let steps_in_room2 = 2;
                        steps_in_room1 + steps_in_hallway + steps_in_room2
                    };
                    total_steps * back.energy_per_step()
                };
                energy1 + energy2
            }
        })
        .sum::<u64>();
    // Uncount the energy necessary to move exactly one of the each of the doubled amphipods
    // from the back of their rooms to the front.
    if energy_from_hallway + energy_from_rooms < 1111 {
        dbg!(energy_from_hallway, energy_from_rooms, s);
        0
    } else {
        energy_from_hallway + energy_from_rooms - 1111
    }
}

fn part1(input: &mut dyn BufRead) -> io::Result<u64> {
    a_star::run_a_star_for_distance::<_, u64, _, _>(State::read(input)?, amphipod_heuristic)
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Couldn't find path to sorted state"))
}

fn part2(_input: &mut dyn BufRead) -> io::Result<u64> {
    todo!("Year 2021 Day 23 Part 2")
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2021 Day 23 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2021_23.txt")?))?
        );
    }
    {
        println!("Year 2021 Day 23 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2021_23.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, io::Cursor};

    use super::*;

    const TEST_DATA: &str = concat!(
        "#############\n",
        "#...........#\n",
        "###B#C#B#D###\n",
        "  #A#D#C#A#\n",
        "  #########",
    );

    #[test]
    #[ignore]
    fn test_amphipod_heuristic() {
        let s1 = State {
            rooms: [
                Room {
                    desired: Amphipod::A,
                    contents: RoomContents::Double {
                        front: Amphipod::B,
                        back: Amphipod::A,
                    },
                },
                Room {
                    desired: Amphipod::B,
                    contents: RoomContents::Double {
                        front: Amphipod::C,
                        back: Amphipod::D,
                    },
                },
                Room {
                    desired: Amphipod::C,
                    contents: RoomContents::Double {
                        front: Amphipod::B,
                        back: Amphipod::C,
                    },
                },
                Room {
                    desired: Amphipod::D,
                    contents: RoomContents::Double {
                        front: Amphipod::D,
                        back: Amphipod::A,
                    },
                },
            ],
            hallway: [None; 11],
        };
        let h1 = amphipod_heuristic(&s1);
        assert_eq!(9 + 90 + 400 + 8000, h1);
    }

    #[test]
    #[ignore]
    fn test_read_state() -> io::Result<()> {
        let expected = State {
            rooms: [
                Room {
                    desired: Amphipod::A,
                    contents: RoomContents::Double {
                        front: Amphipod::B,
                        back: Amphipod::A,
                    },
                },
                Room {
                    desired: Amphipod::B,
                    contents: RoomContents::Double {
                        front: Amphipod::C,
                        back: Amphipod::D,
                    },
                },
                Room {
                    desired: Amphipod::C,
                    contents: RoomContents::Double {
                        front: Amphipod::B,
                        back: Amphipod::C,
                    },
                },
                Room {
                    desired: Amphipod::D,
                    contents: RoomContents::Double {
                        front: Amphipod::D,
                        back: Amphipod::A,
                    },
                },
            ],
            hallway: [None; 11],
        };
        let actual = State::read(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_neighbors() {
        let expected = [
            (
                State {
                    rooms: [
                        Room {
                            desired: Amphipod::A,
                            contents: RoomContents::Empty,
                        },
                        Room {
                            desired: Amphipod::B,
                            contents: RoomContents::Empty,
                        },
                        Room {
                            desired: Amphipod::C,
                            contents: RoomContents::Empty,
                        },
                        Room {
                            desired: Amphipod::D,
                            contents: RoomContents::Empty,
                        },
                    ],
                    hallway: [
                        Some(Amphipod::A),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    ],
                },
                6,
            ),
            (
                State {
                    rooms: [
                        Room {
                            desired: Amphipod::A,
                            contents: RoomContents::Empty,
                        },
                        Room {
                            desired: Amphipod::B,
                            contents: RoomContents::Empty,
                        },
                        Room {
                            desired: Amphipod::C,
                            contents: RoomContents::Empty,
                        },
                        Room {
                            desired: Amphipod::D,
                            contents: RoomContents::Empty,
                        },
                    ],
                    hallway: [
                        None,
                        Some(Amphipod::A),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    ],
                },
                5,
            ),
            (
                State {
                    rooms: [
                        Room {
                            desired: Amphipod::A,
                            contents: RoomContents::Single(Amphipod::A),
                        },
                        Room {
                            desired: Amphipod::B,
                            contents: RoomContents::Empty,
                        },
                        Room {
                            desired: Amphipod::C,
                            contents: RoomContents::Empty,
                        },
                        Room {
                            desired: Amphipod::D,
                            contents: RoomContents::Empty,
                        },
                    ],
                    hallway: [None; 11],
                },
                6,
            ),
            (
                State {
                    rooms: [
                        Room {
                            desired: Amphipod::A,
                            contents: RoomContents::Empty,
                        },
                        Room {
                            desired: Amphipod::B,
                            contents: RoomContents::Empty,
                        },
                        Room {
                            desired: Amphipod::C,
                            contents: RoomContents::Empty,
                        },
                        Room {
                            desired: Amphipod::D,
                            contents: RoomContents::Empty,
                        },
                    ],
                    hallway: [
                        None,
                        None,
                        None,
                        Some(Amphipod::A),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                    ],
                },
                3,
            ),
            (
                State {
                    rooms: [
                        Room {
                            desired: Amphipod::A,
                            contents: RoomContents::Empty,
                        },
                        Room {
                            desired: Amphipod::B,
                            contents: RoomContents::Empty,
                        },
                        Room {
                            desired: Amphipod::C,
                            contents: RoomContents::Empty,
                        },
                        Room {
                            desired: Amphipod::D,
                            contents: RoomContents::Empty,
                        },
                    ],
                    hallway: [
                        None,
                        None,
                        None,
                        None,
                        None,
                        Some(Amphipod::A),
                        None,
                        None,
                        None,
                        None,
                        None,
                    ],
                },
                3,
            ),
            (
                State {
                    rooms: [
                        Room {
                            desired: Amphipod::A,
                            contents: RoomContents::Empty,
                        },
                        Room {
                            desired: Amphipod::B,
                            contents: RoomContents::Empty,
                        },
                        Room {
                            desired: Amphipod::C,
                            contents: RoomContents::Empty,
                        },
                        Room {
                            desired: Amphipod::D,
                            contents: RoomContents::Empty,
                        },
                    ],
                    hallway: [
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        Some(Amphipod::A),
                        None,
                        None,
                        None,
                    ],
                },
                5,
            ),
            (
                State {
                    rooms: [
                        Room {
                            desired: Amphipod::A,
                            contents: RoomContents::Empty,
                        },
                        Room {
                            desired: Amphipod::B,
                            contents: RoomContents::Empty,
                        },
                        Room {
                            desired: Amphipod::C,
                            contents: RoomContents::Empty,
                        },
                        Room {
                            desired: Amphipod::D,
                            contents: RoomContents::Empty,
                        },
                    ],
                    hallway: [
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        Some(Amphipod::A),
                        None,
                    ],
                },
                7,
            ),
            (
                State {
                    rooms: [
                        Room {
                            desired: Amphipod::A,
                            contents: RoomContents::Empty,
                        },
                        Room {
                            desired: Amphipod::B,
                            contents: RoomContents::Empty,
                        },
                        Room {
                            desired: Amphipod::C,
                            contents: RoomContents::Empty,
                        },
                        Room {
                            desired: Amphipod::D,
                            contents: RoomContents::Empty,
                        },
                    ],
                    hallway: [
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        Some(Amphipod::A),
                    ],
                },
                8,
            ),
        ]
        .into_iter()
        .collect::<HashMap<_, _>>();
        let s = State {
            rooms: [
                Room {
                    desired: Amphipod::A,
                    contents: RoomContents::Empty,
                },
                Room {
                    desired: Amphipod::B,
                    contents: RoomContents::Single(Amphipod::A),
                },
                Room {
                    desired: Amphipod::C,
                    contents: RoomContents::Empty,
                },
                Room {
                    desired: Amphipod::D,
                    contents: RoomContents::Empty,
                },
            ],
            hallway: [None; 11],
        };
        let actual = s
            .neighbors()
            .into_iter()
            .map(|(distance, state)| (state, distance))
            .collect::<HashMap<_, _>>();
        assert_eq!(expected, actual);
    }

    #[test]
    #[ignore = "A* not implemented correctly"]
    fn test_part1() -> io::Result<()> {
        let expected = 12_521;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
