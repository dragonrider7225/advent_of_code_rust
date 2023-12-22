use std::{
    cmp::Reverse,
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader},
    ops::RangeInclusive,
    sync::atomic::AtomicUsize,
};

use aoc_util::{geometry::Point3D, nom_extended::NomParse};
use nom::{
    bytes::complete as bytes, character::complete as character, combinator, sequence, IResult,
};

type Point = Point3D<u32>;

fn nom_parse_point(s: &str) -> IResult<&str, Point> {
    combinator::map(
        sequence::tuple((
            sequence::terminated(character::u32, bytes::tag(",")),
            sequence::terminated(character::u32, bytes::tag(",")),
            character::u32,
        )),
        |(x, y, z)| Point3D::at(x, y, z),
    )(s)
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Brick {
    id: usize,
    ends: (Point, Point),
}

impl Brick {
    fn x_range(&self) -> RangeInclusive<u32> {
        let x1 = *self.ends.0.x();
        let x2 = *self.ends.1.x();
        x1.min(x2)..=x2.max(x1)
    }

    fn y_range(&self) -> RangeInclusive<u32> {
        let y1 = *self.ends.0.y();
        let y2 = *self.ends.1.y();
        y1.min(y2)..=y2.max(y1)
    }

    fn z_range(&self) -> RangeInclusive<u32> {
        (*self.ends.0.z())..=(*self.ends.1.z())
    }

    fn min_z(&self) -> u32 {
        *self.z_range().start()
    }

    fn max_z(&self) -> u32 {
        *self.z_range().end()
    }
}

impl<'s> NomParse<&'s str> for Brick {
    fn nom_parse(input: &'s str) -> nom::IResult<&'s str, Self> {
        static BRICK_ID: AtomicUsize = AtomicUsize::new(0);

        combinator::map(
            sequence::separated_pair(nom_parse_point, bytes::tag("~"), nom_parse_point),
            |ends| Self {
                id: BRICK_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
                ends,
            },
        )(input)
    }
}

/// Tests whether `settled_brick` would support `falling_brick` if they were the only two bricks
/// and were allowed to fall.
fn under(settled_brick: &Brick, falling_brick: &Brick) -> bool {
    falling_brick
        .x_range()
        .any(|x| settled_brick.x_range().contains(&x))
        && falling_brick
            .y_range()
            .any(|y| settled_brick.y_range().contains(&y))
        && settled_brick.max_z() < falling_brick.min_z()
}

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    let mut bricks = input
        .lines()
        .map(|line| {
            let line = line?;
            Brick::nom_parse(&line)
                .map(|(_, x)| x)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
        })
        .collect::<io::Result<Vec<_>>>()?;
    bricks.sort_unstable_by_key(|brick| Reverse(brick.min_z()));
    let mut indestructible_bricks = HashSet::new();
    let mut settled_bricks: Vec<Brick> = vec![];
    while let Some(mut brick) = bricks.pop() {
        let mut bricks_in_shadow = settled_bricks
            .iter()
            .copied()
            .filter(|settled_brick| under(settled_brick, &brick))
            .collect::<Vec<_>>();
        bricks_in_shadow.sort_unstable_by_key(|&brick| brick.max_z());
        let upper_surface_height = bricks_in_shadow
            .last()
            .map(|brick| brick.max_z())
            .unwrap_or(0);
        match bricks_in_shadow[..] {
            [] => {}
            [b] => {
                indestructible_bricks.insert(b);
            }
            [.., a, b] => {
                if a.max_z() != upper_surface_height {
                    indestructible_bricks.insert(b);
                }
            }
        }
        let height_difference = brick.min_z() - upper_surface_height;
        if height_difference > 1 {
            brick.ends.0 -= Point::at(0, 0, height_difference - 1);
            brick.ends.1 -= Point::at(0, 0, height_difference - 1);
        }
        settled_bricks.push(brick);
    }
    Ok(settled_bricks.len() - indestructible_bricks.len())
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    let mut bricks = input
        .lines()
        .map(|line| {
            let line = line?;
            Brick::nom_parse(&line)
                .map(|(_, x)| x)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
        })
        .collect::<io::Result<Vec<_>>>()?;
    bricks.sort_unstable_by_key(|brick| Reverse(brick.min_z()));
    let mut indispensible_bricks = HashSet::new();
    let mut settled_bricks: Vec<Brick> = vec![];
    while let Some(mut brick) = bricks.pop() {
        let mut bricks_in_shadow = settled_bricks
            .iter()
            .copied()
            .filter(|settled_brick| under(settled_brick, &brick))
            .collect::<Vec<_>>();
        bricks_in_shadow.sort_unstable_by_key(|&brick| brick.max_z());
        let upper_surface_height = bricks_in_shadow
            .last()
            .map(|brick| brick.max_z())
            .unwrap_or(0);
        match bricks_in_shadow[..] {
            [] => {}
            [b] => {
                indispensible_bricks.insert(b);
            }
            [.., a, b] => {
                if a.max_z() != upper_surface_height {
                    indispensible_bricks.insert(b);
                }
            }
        }
        let height_difference = brick.min_z() - upper_surface_height;
        if height_difference > 1 {
            brick.ends.0 -= Point::at(0, 0, height_difference - 1);
            brick.ends.1 -= Point::at(0, 0, height_difference - 1);
        }
        settled_bricks.push(brick);
    }
    settled_bricks.sort_unstable_by_key(|brick| brick.min_z());
    let _num_indispensible_bricks = indispensible_bricks.len();
    Ok(indispensible_bricks
        .into_iter()
        .enumerate()
        .map(|(_idx, indispensible_brick)| {
            // println!("Checking brick {_idx} out of {_num_indispensible_bricks}");
            settled_bricks
                .iter()
                .copied()
                .skip_while(|settled_brick| settled_brick.min_z() <= indispensible_brick.max_z())
                .fold(
                    HashSet::<Brick>::from_iter([indispensible_brick]),
                    |mut dependent, settled_brick| {
                        if settled_bricks
                            .iter()
                            .filter(|brick| {
                                under(brick, &settled_brick)
                                    && brick.max_z() + 1 == settled_brick.min_z()
                            })
                            .all(|supporter| dependent.contains(supporter))
                        {
                            dependent.insert(settled_brick);
                        }
                        dependent
                    },
                )
                .len()
                // The indispensible brick that we're hypothetically vaporizing is in the folded set
                // but would not fall (because it's been vaporized) so we need to uncount it.
                - 1
        })
        .sum())
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2023 Day 22 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2023_22.txt")?))?
        );
    }
    {
        println!("Year 2023 Day 22 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2023_22.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        "1,0,1~1,2,1\n",
        "0,0,2~2,0,2\n",
        "0,2,3~2,2,3\n",
        "0,0,4~0,2,4\n",
        "2,0,5~2,2,5\n",
        "0,1,6~2,1,6\n",
        "1,1,8~1,1,9\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 5;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 7;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
