use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{self, BufRead, BufReader},
};

type Coord = i32;

fn region_contains(
    min: (Coord, Coord, Coord),
    max: (Coord, Coord, Coord),
    p: (Coord, Coord, Coord),
) -> bool {
    min.0 <= p.0 && min.1 <= p.1 && min.2 <= p.2 && p.0 <= max.0 && p.1 <= max.1 && p.2 <= max.2
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Cube(Coord, Coord, Coord);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Air(Coord, Coord, Coord);

fn read_cubes(input: &mut dyn BufRead) -> io::Result<HashSet<Cube>> {
    input
        .lines()
        .map(|line| {
            let line = line?;
            let (x, line) = line.split_once(',').ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "Line does not contain commas")
            })?;
            let (y, z) = line.split_once(',').ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Line does not contain two commas",
                )
            })?;
            Ok(Cube(
                x.parse::<i32>()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?,
                y.parse::<i32>()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?,
                z.parse::<i32>()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?,
            ))
        })
        .collect::<io::Result<HashSet<_>>>()
}

fn surface_faces(cubes: &HashSet<Cube>) -> impl Iterator<Item = Air> + '_ {
    cubes
        .iter()
        .copied()
        .flat_map(|Cube(x, y, z)| {
            [
                Air(x - 1, y, z),
                Air(x + 1, y, z),
                Air(x, y - 1, z),
                Air(x, y + 1, z),
                Air(x, y, z - 1),
                Air(x, y, z + 1),
            ]
        })
        .filter(|&Air(x, y, z)| !cubes.contains(&Cube(x, y, z)))
}

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    let cubes = read_cubes(input)?;
    Ok(surface_faces(&cubes).count())
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    let cubes = read_cubes(input)?;
    let (min, max) = cubes.iter().fold(
        (
            (Coord::MAX, Coord::MAX, Coord::MAX),
            (Coord::MIN, Coord::MIN, Coord::MIN),
        ),
        |((min_x, min_y, min_z), (max_x, max_y, max_z)), &Cube(x, y, z)| {
            (
                (min_x.min(x), min_y.min(y), min_z.min(z)),
                (max_x.max(x), max_y.max(y), max_z.max(z)),
            )
        },
    );
    println!("Droplet extends from {min:?} to {max:?}");
    let surface_faces = surface_faces(&cubes).collect::<Vec<_>>();
    let mut escaped = HashSet::<Air>::new();
    let mut trapped = HashSet::<Air>::new();
    let mut current = HashSet::<Air>::new();
    let mut remaining = VecDeque::new();
    for &start in &surface_faces {
        current.insert(start);
        remaining.push_back(start);
        while let Some(next @ Air(x, y, z)) = remaining.pop_front() {
            let neighbors = [
                (x - 1, y, z),
                (x + 1, y, z),
                (x, y - 1, z),
                (x, y + 1, z),
                (x, y, z - 1),
                (x, y, z + 1),
            ]
            .into_iter()
            .filter_map(|(x, y, z)| {
                if current.contains(&Air(x, y, z)) || cubes.contains(&Cube(x, y, z)) {
                    None
                } else {
                    Some(Air(x, y, z))
                }
            })
            .collect::<Vec<_>>();
            current.extend(neighbors.iter().copied());
            remaining.extend(neighbors.iter().copied());
            let escaped_bounding_box = !region_contains(min, max, (x, y, z));
            if escaped.contains(&next) || escaped_bounding_box {
                escaped.extend(current.drain());
                remaining.clear();
            } else if trapped.contains(&next) {
                break;
            }
        }
        remaining.clear();
        trapped.extend(current.drain());
    }
    Ok(surface_faces
        .into_iter()
        .filter(|air| escaped.contains(air))
        .count())
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2022 Day 18 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2022_18.txt")?))?
        );
    }
    {
        println!("Year 2022 Day 18 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2022_18.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        "2,2,2\n", "1,2,2\n", "3,2,2\n", "2,1,2\n", "2,3,2\n", "2,2,1\n", "2,2,3\n", "2,2,4\n",
        "2,2,6\n", "1,2,5\n", "3,2,5\n", "2,1,5\n", "2,3,5\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 64;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 58;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
