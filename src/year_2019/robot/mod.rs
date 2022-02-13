use crate::util::Point;

use std::{
    collections::HashSet,
    convert::{TryFrom, TryInto},
    fmt::{self, Debug, Formatter},
    io,
    ops::{Index, IndexMut},
};

use extended_io::{
    self as eio,
    pipe::{PipeRead, PipeWrite},
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Rotation {
    Left,
    Right,
}

impl TryFrom<i64> for Rotation {
    type Error = String;

    fn try_from(id: i64) -> Result<Self, Self::Error> {
        match id {
            0 => Ok(Self::Left),
            1 => Ok(Self::Right),
            _ => Err(format!("Invalid rotation id {id}")),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Color {
    Black,
    White,
}

impl Default for Color {
    fn default() -> Self {
        Self::Black
    }
}

impl From<Color> for i64 {
    fn from(val: Color) -> Self {
        match val {
            Color::Black => 0,
            Color::White => 1,
        }
    }
}

impl TryFrom<i64> for Color {
    type Error = String;

    fn try_from(id: i64) -> Result<Self, Self::Error> {
        match id {
            0 => Ok(Self::Black),
            1 => Ok(Self::White),
            _ => Err(format!("Invalid color id {id}")),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Field(Vec<Vec<Color>>);

impl Field {
    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn add_first_row(&mut self) {
        let len = self.0[0].len();
        self.0.insert(0, vec![Color::default(); len]);
    }

    fn add_first_col(&mut self) {
        self.0
            .iter_mut()
            .for_each(|v| v.insert(0, Color::default()));
    }

    fn add_last_row(&mut self) {
        let len = self.0[0].len();
        self.0.push(vec![Color::default(); len]);
    }

    fn add_last_col(&mut self) {
        self.0.iter_mut().for_each(|v| v.push(Color::default()));
    }
}

impl Default for Field {
    fn default() -> Self {
        Self(vec![vec![Color::default()]])
    }
}

impl Index<Point<usize>> for Field {
    type Output = Color;

    fn index(&self, p: Point<usize>) -> &Self::Output {
        &self.0[*p.y()][*p.x()]
    }
}

impl IndexMut<Point<usize>> for Field {
    fn index_mut(&mut self, p: Point<usize>) -> &mut Self::Output {
        &mut self.0[*p.y()][*p.x()]
    }
}

impl ToString for Field {
    fn to_string(&self) -> String {
        let mut ret = String::new();
        for row in &self.0 {
            for color in row {
                match color {
                    Color::Black => ret.push('.'),
                    Color::White => ret.push('#'),
                }
            }
            ret.push('\n');
        }
        ret
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Up
    }
}

#[derive(Clone)]
pub struct Robot {
    pos: Point<usize>,
    field: Field,
    input: PipeRead,
    output: PipeWrite,
    facing: Direction,
    painted: HashSet<Point<usize>>,
}

impl Robot {
    pub fn new(input: PipeRead, output: PipeWrite) -> Self {
        Self {
            pos: Default::default(),
            field: Default::default(),
            input,
            output,
            facing: Default::default(),
            painted: Default::default(),
        }
    }

    pub fn num_panels(&self) -> usize {
        self.painted.len()
    }

    pub fn print_field(&self) {
        print!("{}", self.field.to_string());
    }

    fn try_read<T>(&mut self) -> io::Result<T>
    where
        i64: TryInto<T>,
        <i64 as TryInto<T>>::Error: ToString,
    {
        eio::read_i64(&mut self.input)?
            .try_into()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
    }

    fn write(&mut self, value: i64) -> io::Result<()> {
        eio::write_i64(&mut self.output, value)
    }

    fn rotate(&mut self, r: Rotation) {
        self.facing = match (self.facing, r) {
            (Direction::Up, Rotation::Left) => Direction::Left,
            (Direction::Up, Rotation::Right) => Direction::Right,
            (Direction::Left, Rotation::Left) => Direction::Down,
            (Direction::Left, Rotation::Right) => Direction::Up,
            (Direction::Down, Rotation::Left) => Direction::Right,
            (Direction::Down, Rotation::Right) => Direction::Left,
            (Direction::Right, Rotation::Left) => Direction::Up,
            (Direction::Right, Rotation::Right) => Direction::Down,
        };
    }

    fn r#move(&mut self) {
        match self.facing {
            Direction::Up => {
                if *self.pos.y() == self.field.height() - 1 {
                    self.field.add_last_row();
                }
                self.pos += Point::at(0, 1);
            }
            Direction::Left => {
                if *self.pos.x() == 0 {
                    self.field.add_first_col();
                    self.painted = self.painted.iter().map(|p| p + Point::at(1, 0)).collect();
                } else {
                    self.pos -= Point::at(1, 0);
                }
            }
            Direction::Down => {
                if *self.pos.y() == 0 {
                    self.field.add_first_row();
                    self.painted = self.painted.iter().map(|p| p + Point::at(0, 1)).collect();
                } else {
                    self.pos -= Point::at(0, 1);
                }
            }
            Direction::Right => {
                if *self.pos.x() == self.field.width() - 1 {
                    self.field.add_last_col();
                }
                self.pos += Point::at(1, 0);
            }
        }
    }

    pub(super) fn set(&mut self, at: Point<usize>, color: Color) {
        self.field[at] = color;
    }

    fn paint(&mut self, color: Color) {
        self.set(self.pos, color);
        self.painted.insert(self.pos);
    }

    pub fn run(&mut self) {
        loop {
            self.write(self.field[self.pos].into())
                .expect("Failed to write to pipe");
            let color = match self.try_read() {
                Ok(color) => color,
                Err(_) => break,
            };
            self.paint(color);
            let rotation = match self.try_read() {
                Ok(rotation) => rotation,
                Err(e) => panic!("{}", e),
            };
            self.rotate(rotation);
            self.r#move();
        }
    }
}

impl Debug for Robot {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Robot")
            .field("pos", &self.pos)
            .field("field", &self.field)
            .field("facing", &self.facing)
            .field("painted", &self.painted)
            .finish()
    }
}
