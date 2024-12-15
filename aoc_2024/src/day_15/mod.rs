use std::{
    fmt::{self, Display, Formatter},
    fs::File,
    io::{self, BufRead, BufReader},
    ops::{Index, IndexMut},
};

use aoc_util::{
    geometry::{Direction, Point2D},
    nom::{bytes::complete as bytes, character::complete as character, multi, IResult, Parser},
    nom_extended::NomParse,
    nom_supreme::ParserExt,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tile {
    Wall,
    Floor,
    Box,
    Robot,
    CrateLeft,
    CrateRight,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Wall => write!(f, "#"),
            Self::Floor => write!(f, "."),
            Self::Box => write!(f, "O"),
            Self::Robot => write!(f, "@"),
            Self::CrateLeft => write!(f, "["),
            Self::CrateRight => write!(f, "]"),
        }
    }
}

impl NomParse<&str> for Tile {
    fn nom_parse(input: &str) -> IResult<&str, Self> {
        bytes::tag("#")
            .map(|_| Self::Wall)
            .or(bytes::tag(".").map(|_| Self::Floor))
            .or(bytes::tag("O").map(|_| Self::Box))
            .or(bytes::tag("@").map(|_| Self::Robot))
            .or(bytes::tag("[").map(|_| Self::CrateLeft))
            .or(bytes::tag("]").map(|_| Self::CrateRight))
            .parse(input)
    }
}

struct Warehouse {
    map: Vec<Vec<Tile>>,
    robot: Point2D<usize>,
}

impl Warehouse {
    fn new(map: Vec<Vec<Tile>>) -> Result<Self, &'static str> {
        let robot = map
            .iter()
            .enumerate()
            .find_map(|(row_idx, row)| {
                row.iter().enumerate().find_map(|(col_idx, tile)| {
                    Some(Point2D::at(col_idx, row_idx)).filter(|_| matches!(tile, Tile::Robot))
                })
            })
            .ok_or("Warehouse does not contain robot")?;
        Ok(Self { map, robot })
    }

    fn step(&mut self, step: Direction) {
        match step {
            Direction::Up => {
                let after_boxes = (0..*self.robot.y())
                    .rev()
                    .map(|row_idx| Point2D::at(*self.robot.x(), row_idx))
                    .take_while(|&pos| !matches!(self[pos], Tile::Wall))
                    .find(|&pos| matches!(self[pos], Tile::Floor));
                if let Some(after_boxes) = after_boxes {
                    self[after_boxes] = Tile::Box;
                    let robot = self.robot;
                    self[robot] = Tile::Floor;
                    self.robot -= Point2D::at(0, 1);
                    let robot = self.robot;
                    self[robot] = Tile::Robot;
                }
            }
            Direction::Right => {
                let after_boxes = (*self.robot.x()..self.map[*self.robot.y()].len())
                    .map(|col_idx| Point2D::at(col_idx, *self.robot.y()))
                    .take_while(|&pos| !matches!(self[pos], Tile::Wall))
                    .find(|&pos| matches!(self[pos], Tile::Floor));
                if let Some(after_boxes) = after_boxes {
                    self[after_boxes] = Tile::Box;
                    let robot = self.robot;
                    self[robot] = Tile::Floor;
                    self.robot += Point2D::at(1, 0);
                    let robot = self.robot;
                    self[robot] = Tile::Robot;
                }
            }
            Direction::Down => {
                let after_boxes = (*self.robot.y()..self.map.len())
                    .map(|row_idx| Point2D::at(*self.robot.x(), row_idx))
                    .take_while(|&pos| !matches!(self[pos], Tile::Wall))
                    .find(|&pos| matches!(self[pos], Tile::Floor));
                if let Some(after_boxes) = after_boxes {
                    self[after_boxes] = Tile::Box;
                    let robot = self.robot;
                    self[robot] = Tile::Floor;
                    self.robot += Point2D::at(0, 1);
                    let robot = self.robot;
                    self[robot] = Tile::Robot;
                }
            }
            Direction::Left => {
                let after_boxes = (0..*self.robot.x())
                    .rev()
                    .map(|col_idx| Point2D::at(col_idx, *self.robot.y()))
                    .take_while(|&pos| !matches!(self[pos], Tile::Wall))
                    .find(|&pos| matches!(self[pos], Tile::Floor));
                if let Some(after_boxes) = after_boxes {
                    self[after_boxes] = Tile::Box;
                    let robot = self.robot;
                    self[robot] = Tile::Floor;
                    self.robot -= Point2D::at(1, 0);
                    let robot = self.robot;
                    self[robot] = Tile::Robot;
                }
            }
        }
    }

    fn gps_coordinates(&self) -> impl Iterator<Item = usize> {
        self.map.iter().enumerate().flat_map(|(row_idx, row)| {
            row.iter().enumerate().filter_map(move |(col_idx, tile)| {
                Some(100 * row_idx + col_idx).filter(|_| matches!(tile, Tile::Box))
            })
        })
    }
}

impl Display for Warehouse {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in &self.map {
            for tile in row {
                write!(f, "{tile}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Index<Point2D<usize>> for Warehouse {
    type Output = Tile;

    fn index(&self, index: Point2D<usize>) -> &Self::Output {
        &self.map[*index.y()][*index.x()]
    }
}

impl IndexMut<Point2D<usize>> for Warehouse {
    fn index_mut(&mut self, index: Point2D<usize>) -> &mut Self::Output {
        &mut self.map[*index.y()][*index.x()]
    }
}

impl NomParse<&str> for Warehouse {
    fn nom_parse(input: &str) -> IResult<&str, Self> {
        multi::many1(multi::many1(Tile::nom_parse).terminated(character::line_ending))
            .map_res(Self::new)
            .parse(input)
    }
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    let left = bytes::tag("<").map(|_| Direction::Left);
    let right = bytes::tag(">").map(|_| Direction::Right);
    let up = bytes::tag("^").map(|_| Direction::Up);
    let down = bytes::tag("v").map(|_| Direction::Down);
    left.or(right).or(up).or(down).parse(input)
}

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    let input = io::read_to_string(input)?;
    let (mut warehouse, instructions) = Warehouse::nom_parse
        .and(
            multi::fold_many1(
                multi::many1(parse_direction).terminated(character::line_ending),
                Vec::default,
                |mut acc, tail| {
                    acc.extend(tail);
                    acc
                },
            )
            .preceded_by(character::line_ending),
        )
        .all_consuming()
        .parse(&input)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?
        .1;
    instructions
        .into_iter()
        .for_each(|step| warehouse.step(step));
    Ok(warehouse.gps_coordinates().sum())
}

struct WideWarehouse {
    map: Vec<Vec<Tile>>,
    robot: Point2D<usize>,
}

impl WideWarehouse {
    fn can_step(&mut self, from: Point2D<usize>, step: Direction, max_depth: usize) -> bool {
        if max_depth == 0 {
            eprintln!("Stack overflow detected in can_step, unwinding");
            return false;
        }
        if matches!(self[from], Tile::Wall) {
            return false;
        }
        let next = match step {
            Direction::Up => from - Point2D::at(0, 1),
            Direction::Down => from + Point2D::at(0, 1),
            Direction::Left => from - Point2D::at(1, 0),
            Direction::Right => from + Point2D::at(1, 0),
        };
        match self[from] {
            Tile::Floor => true,
            Tile::Wall => unreachable!("Checked before calculation of next"),
            Tile::Box => unreachable!("Boxes not allowed in WideWarehouse"),
            Tile::CrateLeft => {
                let this_can_step = self.can_step(next, step, max_depth - 1);
                let right_can_step = if matches!(step, Direction::Left | Direction::Right) {
                    true
                } else {
                    self.can_step(next + Point2D::at(1, 0), step, max_depth - 1)
                };
                this_can_step && right_can_step
            }
            Tile::CrateRight => {
                let this_can_step = self.can_step(next, step, max_depth - 1);
                let left_can_step = if matches!(step, Direction::Left | Direction::Right) {
                    true
                } else {
                    self.can_step(next - Point2D::at(1, 0), step, max_depth - 1)
                };
                this_can_step && left_can_step
            }
            Tile::Robot => self.can_step(next, step, max_depth - 1),
        }
    }

    fn step(&mut self, step: Direction) {
        fn go(this: &mut WideWarehouse, from: Point2D<usize>, step: Direction, max_depth: usize) {
            if max_depth == 0 {
                eprintln!("Stack overflow detected in step, unwinding");
                return;
            }
            let next = from + step;
            match this[from] {
                Tile::Floor => {}
                Tile::Robot => {
                    go(this, next, step, max_depth - 1);
                    this[next] = Tile::Robot;
                    this[from] = Tile::Floor;
                    this.robot += step;
                }
                Tile::CrateLeft => {
                    if !matches!(step, Direction::Left | Direction::Right) {
                        go(this, next + Direction::Right, step, max_depth - 1);
                        this[next + Direction::Right] = Tile::CrateRight;
                        this[from + Direction::Right] = Tile::Floor;
                    }
                    go(this, next, step, max_depth - 1);
                    this[next] = Tile::CrateLeft;
                    this[from] = Tile::Floor;
                }
                Tile::CrateRight => {
                    if !matches!(step, Direction::Left | Direction::Right) {
                        go(this, next + Direction::Left, step, max_depth - 1);
                        this[next + Direction::Left] = Tile::CrateLeft;
                        this[from + Direction::Left] = Tile::Floor;
                    }
                    go(this, from + step, step, max_depth - 1);
                    this[next] = Tile::CrateRight;
                    this[from] = Tile::Floor;
                }
                Tile::Box => unreachable!("Boxes not allowed in WideWarehouse"),
                Tile::Wall => unreachable!("Walls can't step"),
            }
        }

        if !self.can_step(self.robot, step, 100) {
            return;
        }
        go(
            self,
            self.robot,
            match step {
                Direction::Up | Direction::Down => -step,
                step => step,
            },
            100,
        );
    }

    fn gps_coordinates(&self) -> impl Iterator<Item = usize> {
        self.map.iter().enumerate().flat_map(|(row_idx, row)| {
            row.iter().enumerate().filter_map(move |(col_idx, tile)| {
                Some(100 * row_idx + col_idx).filter(|_| matches!(tile, Tile::CrateLeft))
            })
        })
    }
}

impl Display for WideWarehouse {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in &self.map {
            for tile in row {
                write!(f, "{tile}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl From<Warehouse> for WideWarehouse {
    fn from(value: Warehouse) -> Self {
        let map = value
            .map
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .flat_map(move |tile| match tile {
                        Tile::Wall => [Tile::Wall, Tile::Wall],
                        Tile::Box => [Tile::CrateLeft, Tile::CrateRight],
                        Tile::Floor => [Tile::Floor, Tile::Floor],
                        Tile::Robot => [Tile::Robot, Tile::Floor],
                        Tile::CrateLeft | Tile::CrateRight => {
                            unreachable!("Crates not allowed in normal warehouse")
                        }
                    })
                    .collect()
            })
            .collect();
        Self {
            map,
            robot: value.robot + Point2D::at(*value.robot.x(), 0),
        }
    }
}

impl Index<Point2D<usize>> for WideWarehouse {
    type Output = Tile;

    fn index(&self, index: Point2D<usize>) -> &Self::Output {
        &self.map[*index.y()][*index.x()]
    }
}

impl IndexMut<Point2D<usize>> for WideWarehouse {
    fn index_mut(&mut self, index: Point2D<usize>) -> &mut Self::Output {
        &mut self.map[*index.y()][*index.x()]
    }
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    let input = io::read_to_string(input)?;
    let (warehouse, instructions) = Warehouse::nom_parse
        .and(
            multi::fold_many1(
                multi::many1(parse_direction).terminated(character::line_ending),
                Vec::default,
                |mut acc, tail| {
                    acc.extend(tail);
                    acc
                },
            )
            .preceded_by(character::line_ending),
        )
        .all_consuming()
        .parse(&input)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?
        .1;
    let mut warehouse = WideWarehouse::from(warehouse);
    instructions
        .into_iter()
        .for_each(|step| warehouse.step(step));
    Ok(warehouse.gps_coordinates().sum())
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2024 Day 15 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2024_15.txt")?))?
        );
    }
    {
        println!("Year 2024 Day 15 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2024_15.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use io::Cursor;

    use super::*;

    const TEST_DATA_1: &str = concat!(
        "########\n",
        "#..O.O.#\n",
        "##@.O..#\n",
        "#...O..#\n",
        "#.#.O..#\n",
        "#...O..#\n",
        "#......#\n",
        "########\n",
        "\n",
        "<^^>>>vv<v>>v<<\n",
    );

    const TEST_DATA_2: &str = concat!(
        "##########\n",
        "#..O..O.O#\n",
        "#......O.#\n",
        "#.OO..O.O#\n",
        "#..O@..O.#\n",
        "#O#..O...#\n",
        "#O..O..O.#\n",
        "#.OO.O.OO#\n",
        "#....O...#\n",
        "##########\n",
        "\n",
        "<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^\n",
        "vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v\n",
        "><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<\n",
        "<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^\n",
        "^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><\n",
        "^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^\n",
        ">^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^\n",
        "<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>\n",
        "^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>\n",
        "v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 2028;
        let actual = part1(&mut Cursor::new(TEST_DATA_1))?;
        assert_eq!(expected, actual);
        let expected = 10_092;
        let actual = part1(&mut Cursor::new(TEST_DATA_2))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    const TEST_DATA_3: &str = concat!(
        "#######\n",
        "#...#.#\n",
        "#.....#\n",
        "#..OO@#\n",
        "#..O..#\n",
        "#.....#\n",
        "#######\n",
        "\n",
        "<vv<<^^<<^^\n",
    );

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 618;
        let actual = part2(&mut Cursor::new(TEST_DATA_3))?;
        assert_eq!(expected, actual);
        let expected = 9021;
        let actual = part2(&mut Cursor::new(TEST_DATA_2))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
