use aoc_util::{
    geometry::Point3D,
    nom::{character::complete as character, combinator, sequence, IResult, Parser},
    nom_extended::NomParse,
    nom_supreme::ParserExt,
};
use std::{
    fmt::{self, Display, Formatter},
    fs::File,
    io::{self, BufRead, BufReader},
    num::NonZeroI128,
    ops::RangeInclusive,
};

mod numbers;
use numbers::Fraction;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Path {
    Vertical {
        x_intercept: Fraction,
    },
    Other {
        slope: Fraction,
        y_intercept: Fraction,
    },
}

impl Path {
    fn intersect(&self, rhs: &Self) -> IntersectionResult {
        if self == rhs {
            return IntersectionResult::Identical;
        }
        match (self, rhs) {
            // We know that the intercepts are different because we already checked that
            // `self != rhs` above the match.
            (Path::Vertical { .. }, Path::Vertical { .. }) => IntersectionResult::Parallel,
            (Path::Other { slope: m1, .. }, Path::Other { slope: m2, .. }) if m1 == m2 => {
                IntersectionResult::Parallel
            }
            (Path::Vertical { x_intercept }, Path::Other { slope, y_intercept })
            | (Path::Other { slope, y_intercept }, Path::Vertical { x_intercept }) => {
                IntersectionResult::At {
                    x: *x_intercept,
                    y: slope * x_intercept + y_intercept,
                }
            }
            (
                Path::Other {
                    slope: m1,
                    y_intercept: b1,
                },
                Path::Other {
                    slope: m2,
                    y_intercept: b2,
                },
            ) => {
                // m1 * x + b1 = m2 * x + b2
                // -> (m1 - m2) * x = b2 - b1
                // -> x = (b2 - b1) / (m1 - m2)
                let x = (b2 - b1) / (m1 - m2);
                let y = m1 * x + b1;
                IntersectionResult::At { x, y }
            }
        }
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Vertical { x_intercept } => write!(f, "x = {x_intercept}"),
            Self::Other { slope, y_intercept } if *slope == 1 => {
                write!(f, "y = x + {y_intercept}")
            }
            Self::Other { slope, y_intercept } => {
                write!(f, "y = {slope} * x + {y_intercept}")
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum IntersectionResult {
    Identical,
    At { x: Fraction, y: Fraction },
    Parallel,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Hailstone {
    position: Point3D<i128>,
    velocity: Point3D<i128>,
}

impl Hailstone {
    fn x(&self) -> i128 {
        *self.position.x()
    }

    fn y(&self) -> i128 {
        *self.position.y()
    }

    fn z(&self) -> i128 {
        *self.position.z()
    }

    fn dx(&self) -> i128 {
        *self.velocity.x()
    }

    fn dy(&self) -> i128 {
        *self.velocity.y()
    }

    fn dz(&self) -> i128 {
        *self.velocity.z()
    }

    fn path(&self) -> Path {
        // self.x = self.position.x
        // self.y = self.position.y
        // self.dx = self.velocity.x
        // self.dy = self.velocity.y
        //
        // path equation:
        //      slope = self.dy / self.dx
        //      y_intercept = self.y - slope * self.x
        if self.dx() == 0 {
            Path::Vertical {
                x_intercept: Fraction::from(self.x()),
            }
        } else {
            let slope = Fraction::new(self.dy(), NonZeroI128::new(self.dx()).unwrap());
            let y_intercept = Fraction::from(self.y()) - slope * Fraction::from(self.x());
            Path::Other { slope, y_intercept }
        }
    }

    fn intersects_2d(&self, rhs: &Self, test_area: RangeInclusive<i128>) -> bool {
        let self_path = self.path();
        let rhs_path = rhs.path();
        #[cfg(test)]
        eprintln!("Testing {self} ({self_path}) and {rhs} ({rhs_path})");
        let result = self_path.intersect(&rhs_path);
        match result {
            IntersectionResult::Identical => todo!("Check intersection with borders"),
            IntersectionResult::At { x, y } => {
                let intersects_in_test_area = test_area.contains(&x) && test_area.contains(&y);
                let intersects_in_self_future = (x + -self.x()).signum() == self.dx().signum();
                let intersects_in_rhs_future = (x + -rhs.x()).signum() == rhs.dx().signum();
                intersects_in_test_area && intersects_in_self_future && intersects_in_rhs_future
            }
            IntersectionResult::Parallel => false,
        }
    }
}

impl Display for Hailstone {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<{}, {}, {}> @ <{}, {}, {}>",
            self.x(),
            self.y(),
            self.z(),
            self.dx(),
            self.dy(),
            self.dz()
        )
    }
}

impl<'a> NomParse<&'a str> for Hailstone {
    fn nom_parse(input: &'a str) -> IResult<&'a str, Self> {
        fn number(s: &str) -> IResult<&str, i128> {
            character::space0
                .precedes(
                    character::char('-')
                        .opt()
                        .precedes(character::digit1)
                        .recognize(),
                )
                .map_res(str::parse)
                .parse(s)
        }

        fn point3d(s: &str) -> IResult<&str, Point3D<i128>> {
            combinator::map(
                sequence::tuple((
                    number,
                    character::char(',').precedes(number),
                    character::char(',').precedes(number),
                )),
                |(x, y, z)| Point3D::at(x, y, z),
            )(s)
        }

        combinator::map(
            sequence::separated_pair(
                point3d,
                sequence::tuple((character::space1, character::char('@'), character::space1)),
                point3d,
            ),
            |(position, velocity)| Self { position, velocity },
        )(input)
    }
}

fn parse_hailstones(input: &mut dyn BufRead) -> io::Result<Vec<Hailstone>> {
    input
        .lines()
        .map(|line| {
            Hailstone::nom_parse(&line?)
                .map(|(_, hailstone)| hailstone)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
        })
        .collect()
}

fn part1(input: &mut dyn BufRead, test_area: RangeInclusive<i128>) -> io::Result<usize> {
    let hailstones = parse_hailstones(input)?;
    Ok(hailstones
        .iter()
        .copied()
        .enumerate()
        .flat_map(|(i, hailstone1)| {
            hailstones
                .iter()
                .copied()
                .skip(i + 1)
                .map(move |hailstone2| (hailstone1, hailstone2))
        })
        .filter(|(hailstone1, hailstone2)| {
            let intersection = hailstone1.intersects_2d(hailstone2, test_area.clone());
            if intersection {
                #[cfg(test)]
                eprintln!("{hailstone1} and {hailstone2} intersected");
            }
            intersection
        })
        .count())
}

fn collapse_ranges(ranges: impl IntoIterator<Item = (i128, i128)>) -> Vec<(i128, i128)> {
    let mut ranges = ranges.into_iter().collect::<Vec<_>>();
    ranges.sort();
    let num_ranges = ranges.len();
    ranges
        .into_iter()
        .fold(Vec::with_capacity(num_ranges), |mut acc, range| {
            let Some(last_range) = acc.last_mut() else {
                acc.push(range);
                return acc;
            };
            if last_range.1 + 1 >= range.0 {
                last_range.1 = last_range.1.max(range.1);
            } else {
                acc.push(range);
            }
            acc
        })
}

fn part2_brute_force(hailstones: &[Hailstone]) -> i128 {
    // Given `h1` and `h2` such that `h1` starts left of `h2` and moves right slower than `h2`.
    // If the rock has a rightward velocity greater than that of `h2` then it must start left of
    // `h1`. If the rock has a rightward velocity less than that of `h1` then it must start
    // right of `h2`. If the rock has a rightward velocity greater than that of `h1` but less
    // than that of `h2` then it either must start left of `h2` and thus never intersect it or
    // it must start right of `h1` and never intersect *it*.
    let excluded_x_velocity = collapse_ranges(
        hailstones
            .iter()
            .enumerate()
            .flat_map(|(idx, h1)| hailstones.iter().skip(idx + 1).map(move |h2| (h1, h2)))
            .filter_map(|(h1, h2)| {
                if h1.x() < h2.x() {
                    Some((h1, h2))
                } else {
                    Some((h2, h1))
                }
                .filter(|(h1, h2)| h1.dx() < h2.dx())
                .map(|(h1, h2)| (h1.dx(), h2.dx()))
            }),
    );
    let excluded_y_velocity = collapse_ranges(
        hailstones
            .iter()
            .enumerate()
            .flat_map(|(idx, h1)| hailstones.iter().skip(idx + 1).map(move |h2| (h1, h2)))
            .filter_map(|(h1, h2)| {
                if h1.y() < h2.y() {
                    Some((h1, h2))
                } else {
                    Some((h2, h1))
                }
                .filter(|(h1, h2)| h1.dy() < h2.dy())
                .map(|(h1, h2)| (h1.dy(), h2.dy()))
            }),
    );
    let mut range_idx = 0;
    let mut dx = -1001;
    while dx < 1000 {
        dx += 1;
        if excluded_x_velocity
            .get(range_idx)
            .filter(|&&(low, high)| (low..=high).contains(&dx))
            .is_some()
        {
            dx = excluded_x_velocity[range_idx].1 + 1;
            range_idx += 1;
        }
        let mut range_idx = 0;
        let mut dy = -1001;
        while dy < 1000 {
            dy += 1;
            if excluded_y_velocity
                .get(range_idx)
                .filter(|&&(low, high)| (low..=high).contains(&dy))
                .is_some()
            {
                dy = excluded_y_velocity[range_idx].1 + 1;
                range_idx += 1;
            }
            let dv = Point3D::at(dx, dy, 0);
            let mut intersection_point = None;
            let h1 = hailstones[0];
            let h1 = Hailstone {
                velocity: h1.velocity - dv,
                ..h1
            };
            for h2 in hailstones[1..].iter() {
                let h2 = Hailstone {
                    position: h2.position,
                    velocity: h2.velocity - dv,
                };
                match h1.path().intersect(&h2.path()) {
                    IntersectionResult::Parallel => continue,
                    IntersectionResult::Identical => continue,
                    IntersectionResult::At { x, y } => {
                        if !x.is_integral() || !y.is_integral() {
                            break;
                        }
                        if let Some((ix, iy)) = intersection_point {
                            if x == ix && y == iy {
                                continue;
                            } else {
                                intersection_point = None;
                                break;
                            }
                        } else {
                            intersection_point = Some((x.trunc(), y.trunc()));
                        }
                    }
                }
            }
            let Some((ix, iy)) = intersection_point else {
                continue;
            };
            let (t1, z1) = {
                let dx = ix - h1.x();
                let dt = dx / h1.dx();
                (dt, h1.z() + dt * h1.dz())
            };
            let (t2, z2) = {
                let h2 = hailstones[1];
                let h2 = Hailstone {
                    velocity: h2.velocity - dv,
                    ..h2
                };
                let dx = ix - h2.x();
                let dt = dx / h2.dx();
                (dt, h2.z() + dt * h2.dz())
            };
            if t1 == t2 {
                continue;
            }
            let dz = Fraction::new(z1 - z2, NonZeroI128::new(t1 - t2).unwrap());
            if !dz.is_integral() {
                continue;
            }
            let dz = dz.trunc();
            let rock_velocity = dv + Point3D::at(0, 0, dz);
            let intersection_point = Point3D::at(ix, iy, z1 - t1 * dz);
            let found_intersection_point = hailstones.iter().all(|h| {
                let h = Hailstone {
                    velocity: h.velocity - rock_velocity,
                    ..*h
                };
                let distance = intersection_point - h.position;
                let dt = if h.dx() != 0 {
                    if distance.x() % h.dx() != 0 {
                        return false;
                    }
                    distance.x() / h.dx()
                } else if h.dy() != 0 {
                    if distance.y() % h.dy() != 0 {
                        return false;
                    }
                    distance.y() / h.dy()
                } else if h.dz() != 0 {
                    if distance.z() % h.dz() != 0 {
                        return false;
                    }
                    distance.z() / h.dz()
                } else {
                    return distance == Point3D::at(0, 0, 0);
                };
                h.velocity * dt == distance
            });
            if found_intersection_point {
                return intersection_point.dot(&Point3D::at(1, 1, 1));
            }
        }
    }
    todo!("Couldn't find intersection point")
}

fn part2(input: &mut dyn BufRead) -> io::Result<i128> {
    let hailstones = parse_hailstones(input)?;
    let Some((h1, h2)) = hailstones
        .iter()
        .enumerate()
        .flat_map(|(i, h1)| hailstones.iter().skip(i + 1).map(move |h2| (h1, h2)))
        .find(|(h1, h2)| {
            let ratio_x = Fraction::new(
                *h1.velocity.x(),
                NonZeroI128::new(*h2.velocity.x()).unwrap(),
            );
            let ratio_y = Fraction::new(
                *h1.velocity.y(),
                NonZeroI128::new(*h2.velocity.y()).unwrap(),
            );
            let ratio_z = Fraction::new(
                *h1.velocity.z(),
                NonZeroI128::new(*h2.velocity.z()).unwrap(),
            );
            ratio_x == ratio_y && ratio_x == ratio_z
        })
    else {
        // We don't have any coplanar lines, so we can't cheat by constructing a plane that must
        // contain the rock's entire path.
        return Ok(part2_brute_force(&hailstones));
    };
    let v1 = h1.velocity;
    let v2 = h2.position - h1.position;
    let normal = v1.cross(&v2);
    // The plane containing the paths of both `h1` and `h2` has equation `normal.dot(x - h1.position) = 0`.
    // The path of a hailstone `h` is `x(t) = h.position + t * h.velocity`.
    // These intersect when `normal.dot(h.position - h1.position + h.velocity * t) = 0`.
    // Rearranging, we get `t = -normal.dot(h.position - h1.position) / normal.dot(h.velocity)`.
    let times = hailstones
        .iter()
        .filter(|hailstone| normal.dot(&hailstone.velocity) != 0)
        .map(|hailstone| {
            let denominator = normal.dot(&hailstone.velocity);
            let numerator = -normal.dot(&(hailstone.position - h1.position));
            assert_eq!(
                numerator % denominator,
                0,
                "Interception must happen at integer times"
            );
            (hailstone, numerator / denominator)
        })
        .take(2)
        .collect::<Vec<_>>();
    let (h1, t1) = times[0];
    let (h2, t2) = times[1];
    // Once we have two hailstone-interception time pairs `(h1, t1)` and `(h2, t2)`, the path of the
    // rock must be described by `h1.position + t1 * h1.velocity = r.position + t1 * r.velocity`
    // and `h2.position + t2 * h2.velocity = r.position + t2 * r.velocity`.

    let intercept1 = h1.position + h1.velocity * t1;
    let intercept2 = h2.position + h2.velocity * t2;
    let rock_velocity = (intercept1 - intercept2) / (t1 - t2);
    let rock_position = intercept1 - rock_velocity * t1;
    Ok(rock_position.dot(&Point3D::at(1, 1, 1)))
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2023 Day 24 Part 1");
        println!(
            "{}",
            part1(
                &mut BufReader::new(File::open("2023_24.txt")?),
                200_000_000_000_000..=400_000_000_000_000
            )?
        );
    }
    {
        println!("Year 2023 Day 24 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2023_24.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        "19, 13, 30 @ -2,  1, -2\n",
        "18, 19, 22 @ -1, -1, -2\n",
        "20, 25, 34 @ -2, -2, -4\n",
        "12, 31, 28 @ -1, -2, -1\n",
        "20, 19, 15 @  1, -5, -3\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 2;
        let actual = part1(&mut Cursor::new(TEST_DATA), 7..=27)?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 47;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2_brute_force() -> io::Result<()> {
        let expected = 47;
        let hailstones = parse_hailstones(&mut Cursor::new(TEST_DATA))?;
        let actual = part2_brute_force(&hailstones);
        assert_eq!(expected, actual);
        Ok(())
    }
}
