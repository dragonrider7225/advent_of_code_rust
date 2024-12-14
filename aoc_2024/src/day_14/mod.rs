use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader, Seek, Write},
    ops::Deref,
    str::FromStr,
};

use aoc_util::{
    geometry::Point2D,
    nom::{bytes::complete as bytes, character::complete as character, IResult, Parser},
    nom_extended::NomParse,
    nom_supreme::ParserExt,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct TorusVelocity<const UPPER_BOUND: usize>(usize);

impl<const UPPER_BOUND: usize> Deref for TorusVelocity<UPPER_BOUND> {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const UPPER_BOUND: usize> NomParse<&str> for TorusVelocity<UPPER_BOUND> {
    fn nom_parse(input: &str) -> IResult<&str, Self> {
        bytes::tag("-")
            .opt()
            .and(character::u32)
            .map(|(negative, n)| {
                if negative.is_some() {
                    UPPER_BOUND - (n as usize)
                } else {
                    n as usize
                }
            })
            .map(Self)
            .parse(input)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct TorusStart<const WIDTH: usize, const HEIGHT: usize> {
    position: Point2D<usize>,
    velocity: Point2D<usize>,
}

impl<const WIDTH: usize, const HEIGHT: usize> NomParse<&str> for TorusStart<WIDTH, HEIGHT> {
    fn nom_parse(input: &str) -> IResult<&str, Self> {
        character::u32
            .and(character::u32.preceded_by(bytes::tag(",")))
            .map(|(x, y)| Point2D::at(x as usize, y as usize))
            .preceded_by(bytes::tag("p="))
            .terminated(bytes::tag(" v="))
            .and(
                TorusVelocity::<WIDTH>::nom_parse
                    .and(TorusVelocity::<HEIGHT>::nom_parse.preceded_by(bytes::tag(",")))
                    .map(|(x, y)| Point2D::at(*x, *y)),
            )
            .map(|(position, velocity)| Self { position, velocity })
            .parse(input)
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> FromStr for TorusStart<WIDTH, HEIGHT> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use aoc_util::{nom::error::Error, nom_supreme::final_parser};

        final_parser::final_parser::<_, _, _, Error<&'_ str>>(Self::nom_parse)(s)
            .map_err(|e| format!("{e:?}"))
    }
}

const REAL_FLOOR_WIDTH: usize = 101;
const REAL_FLOOR_HEIGHT: usize = 103;

fn part1<const WIDTH: usize, const HEIGHT: usize>(input: &mut dyn BufRead) -> io::Result<usize> {
    let starts = input
        .lines()
        .map(|line| {
            let line = line?;
            line.parse::<TorusStart<WIDTH, HEIGHT>>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        })
        .collect::<io::Result<Vec<_>>>()?;
    let ends = starts
        .into_iter()
        .map(|start| {
            let unwrapped = start.position + start.velocity * 100;
            Point2D::at(unwrapped.x() % WIDTH, unwrapped.y() % HEIGHT)
        })
        .collect::<Vec<_>>();
    let quadrant_width = WIDTH / 2;
    let quadrant_height = HEIGHT / 2;
    let num_upper_left = ends
        .iter()
        .filter(|position| *position.x() < quadrant_width && *position.y() < quadrant_height)
        .count();
    let num_upper_right = ends
        .iter()
        .filter(|position| *position.x() > quadrant_width && *position.y() < quadrant_height)
        .count();
    let num_lower_left = ends
        .iter()
        .filter(|position| *position.x() < quadrant_width && *position.y() > quadrant_height)
        .count();
    let num_lower_right = ends
        .iter()
        .filter(|position| *position.x() > quadrant_width && *position.y() > quadrant_height)
        .count();
    Ok(num_upper_left * num_upper_right * num_lower_left * num_lower_right)
}

fn write_floor(f: &mut File, positions: &[Point2D<usize>]) -> io::Result<()> {
    f.seek(io::SeekFrom::Start(0))?;
    f.set_len(0)?;
    writeln!(f, "P3")?;
    writeln!(f, "{REAL_FLOOR_WIDTH} {REAL_FLOOR_HEIGHT}")?;
    writeln!(f, "255")?;
    let mut data = [[0; REAL_FLOOR_WIDTH]; REAL_FLOOR_HEIGHT];
    for position in positions {
        data[*position.y()][*position.x()] = 1;
    }
    for row in data {
        for cell in row {
            if cell == 1 {
                writeln!(f, "255 255 255")?;
            } else {
                writeln!(f, "0 0 0")?;
            }
        }
    }
    Ok(())
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    const MIN_ROBOTS_IN_ROW: usize = 13;
    const DIR_NAME: &str = "2024_14";

    fs::create_dir_all(DIR_NAME)?;
    let starts = input
        .lines()
        .map(|line| {
            let line = line?;
            line.parse::<TorusStart<REAL_FLOOR_WIDTH, REAL_FLOOR_HEIGHT>>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        })
        .collect::<io::Result<Vec<_>>>()?;
    let mut positions = starts
        .iter()
        .map(|start| start.position)
        .collect::<Vec<_>>();
    for current_frame in 0..(REAL_FLOOR_HEIGHT.max(REAL_FLOOR_WIDTH)) {
        for row in 0..REAL_FLOOR_HEIGHT {
            if positions
                .iter()
                .filter(|position| *position.y() == row)
                .count()
                > MIN_ROBOTS_IN_ROW
            {
                let mut image = File::create(format!("{DIR_NAME}/{current_frame}.ppm"))?;
                write_floor(&mut image, &positions)?;
            }
        }
        for column in 0..REAL_FLOOR_WIDTH {
            if positions
                .iter()
                .filter(|position| *position.x() == column)
                .count()
                > MIN_ROBOTS_IN_ROW
            {
                let mut image = File::create(format!("{DIR_NAME}/{current_frame}.ppm"))?;
                write_floor(&mut image, &positions)?;
            }
        }
        positions = positions
            .into_iter()
            .enumerate()
            .map(|(idx, position)| {
                let unwrapped = position + starts[idx].velocity;
                Point2D::at(
                    unwrapped.x() % REAL_FLOOR_WIDTH,
                    unwrapped.y() % REAL_FLOOR_HEIGHT,
                )
            })
            .collect();
    }
    println!("Frames with more than {MIN_ROBOTS_IN_ROW} robots in a single row or column have been written to {DIR_NAME}/");
    eprint!("Enter the index of the first frame that appears to have a horizontal streak: ");
    io::stderr().flush()?;
    let horizontal = {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;
        buf.trim()
            .parse::<usize>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
    };
    eprint!("Enter the index of the first frame that appears to have a vertical streak: ");
    io::stderr().flush()?;
    let vertical = {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;
        buf.trim()
            .parse::<usize>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
    };
    let christmas_tree_frame = (0..REAL_FLOOR_WIDTH)
        .find(|i| (horizontal + i * REAL_FLOOR_HEIGHT) % REAL_FLOOR_WIDTH == vertical)
        .map(|i| horizontal + i * REAL_FLOOR_HEIGHT)
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Couldn't find christmas tree"))?;
    let positions = starts
        .into_iter()
        .map(|start| start.position + start.velocity * christmas_tree_frame)
        .map(|unwrapped| {
            Point2D::at(
                unwrapped.x() % REAL_FLOOR_WIDTH,
                unwrapped.y() % REAL_FLOOR_HEIGHT,
            )
        })
        .collect::<Vec<_>>();
    let christmas_tree_file = format!("{DIR_NAME}/{christmas_tree_frame}.ppm");
    let mut image = File::create(christmas_tree_file)?;
    write_floor(&mut image, &positions)?;
    Ok(christmas_tree_frame)
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2024 Day 14 Part 1");
        println!(
            "{}",
            part1::<REAL_FLOOR_WIDTH, REAL_FLOOR_HEIGHT>(&mut BufReader::new(File::open(
                "2024_14.txt"
            )?))?
        );
    }
    {
        println!("Year 2024 Day 14 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2024_14.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        "p=0,4 v=3,-3\n",
        "p=6,3 v=-1,-3\n",
        "p=10,3 v=-1,2\n",
        "p=2,0 v=2,-1\n",
        "p=0,0 v=1,3\n",
        "p=3,0 v=-2,-2\n",
        "p=7,6 v=-1,-3\n",
        "p=3,0 v=-1,-2\n",
        "p=9,3 v=2,3\n",
        "p=7,3 v=-1,2\n",
        "p=2,4 v=2,-3\n",
        "p=9,5 v=-3,-3\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 12;
        let actual = part1::<11, 7>(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
