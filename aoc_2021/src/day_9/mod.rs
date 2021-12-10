use std::{
    cmp::Ordering,
    collections::HashSet,
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    fs::File,
    io::{self, BufRead, BufReader},
    ops::{Index, IndexMut},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum HeightmapParseError {
    NarrowRow { expected: usize, actual: usize },
    WideRow { expected: usize, actual: usize },
}

impl Display for HeightmapParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::NarrowRow { expected, actual } => write!(
                f,
                "Row too narrow: expected {} cells but got {}",
                expected, actual
            ),
            Self::WideRow { expected, actual } => write!(
                f,
                "Row too wide: expected {} cells but got {}",
                expected, actual
            ),
        }
    }
}

impl Error for HeightmapParseError {}

impl From<HeightmapParseError> for io::Error {
    fn from(this: HeightmapParseError) -> Self {
        Self::new(io::ErrorKind::InvalidData, this)
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Heightmap {
    // `values.len() == width * height` *must* hold.
    values: Vec<u32>,
    width: usize,
    height: usize,
}

impl Heightmap {
    fn read<I, II>(values: I) -> io::Result<Self>
    where
        I: IntoIterator<Item = io::Result<II>>,
        II: IntoIterator<Item = u32>,
    {
        let this = values.into_iter().try_fold(
            Self {
                values: vec![],
                width: 0,
                height: 0,
            },
            |mut this, row| {
                let old_len = this.values.len();
                this.values.extend(row?);
                this.height += 1;
                if this.width == 0 {
                    this.width = this.values.len();
                }
                let actual = this.values.len() - old_len;
                match actual.cmp(&this.width) {
                    Ordering::Less => Err(HeightmapParseError::NarrowRow {
                        expected: this.width,
                        actual,
                    })?,
                    Ordering::Greater => Err(HeightmapParseError::WideRow {
                        expected: this.width,
                        actual,
                    })?,
                    Ordering::Equal => io::Result::Ok(this),
                }
            },
        )?;
        assert_eq!(this.height * this.width, this.values.len());
        Ok(this)
    }
}

impl Heightmap {
    fn local_minima<'this>(&'this self) -> impl Iterator<Item = (usize, usize)> + 'this {
        (0..self.height)
            .flat_map(|y| (0..self.width).map(move |x| (x, y)))
            .filter_map(|(x, y)| {
                let point_height = self[(x, y)];
                let neighbors = [
                    y.checked_sub(1).map(|y| (y, x)),
                    x.checked_sub(1).map(|x| (y, x)),
                    Some(x + 1).filter(|&x| x < self.width).map(|x| (y, x)),
                    Some(y + 1).filter(|&y| y < self.height).map(|y| (y, x)),
                ];
                let neighbor_heights = neighbors
                    .into_iter()
                    .filter_map(|x| x)
                    .map(|(y, x)| self[(x, y)])
                    .collect::<Vec<_>>();
                let is_lowpoint = neighbor_heights
                    .into_iter()
                    .all(|neighbor_height| point_height < neighbor_height);
                if is_lowpoint {
                    Some((x, y))
                } else {
                    None
                }
            })
    }

    fn neighbors(&self, (x, y): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        [
            y.checked_sub(1).map(|y| (x, y)),
            x.checked_sub(1).map(|x| (x, y)),
            Some(x + 1).filter(|&x| x < self.width).map(|x| (x, y)),
            Some(y + 1).filter(|&y| y < self.height).map(|y| (x, y)),
        ]
        .into_iter()
        .filter_map(|x| x)
    }

    fn basin_size_at(&self, pos: (usize, usize)) -> usize {
        let mut points = HashSet::new();
        // Rust complains about unknown `S` with `HashSet::from_iter([pos])` in spite of the type
        // default.
        let mut new_points = HashSet::<_>::from_iter([pos]);
        let mut tmp_points = HashSet::new();
        while !new_points.is_empty() {
            for point in new_points.drain() {
                if self[point] < 9 {
                    points.insert(point);
                    // We don't need to filter `neighbors` here because we are already doing it all
                    // at once at the end of the while loop.
                    tmp_points.extend(self.neighbors(point));
                }
            }
            new_points.extend(tmp_points.drain().filter(|point| !points.contains(point)));
        }
        points.len()
    }
}

impl Debug for Heightmap {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut dstruct = f.debug_struct("Heightmap");
        if self.values.len() == self.width * self.height {
            dstruct.field(
                "values",
                &self.values.chunks_exact(self.width).collect::<Vec<_>>(),
            );
        } else {
            dstruct.field("values", &self.values);
        }
        dstruct
            .field("width", &self.width)
            .field("height", &self.height)
            .finish()
    }
}

impl Index<(usize, usize)> for Heightmap {
    type Output = u32;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.values[y * self.width + x]
    }
}

impl IndexMut<(usize, usize)> for Heightmap {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.values[y * self.width + x]
    }
}

fn read_heightmap(input: &mut dyn BufRead) -> io::Result<Heightmap> {
    Heightmap::read(input.lines().map(|line| {
        let line = line?;
        line.chars()
            .map(|c| {
                c.to_digit(10).ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Got non-digit {:?} in line {:?}", c, line),
                    )
                })
            })
            .collect::<io::Result<Vec<_>>>()
    }))
}

fn part1(input: &mut dyn BufRead) -> io::Result<u32> {
    let heightmap = read_heightmap(input)?;
    Ok(heightmap.local_minima().map(|pos| 1 + heightmap[pos]).sum())
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    let heightmap = read_heightmap(input)?;
    let mut basin_sizes = heightmap
        .local_minima()
        .map(|point| heightmap.basin_size_at(point))
        .collect::<Vec<_>>();
    basin_sizes.sort_by(|left, right| left.cmp(right).reverse());
    Ok(basin_sizes[..3].into_iter().product())
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2021 Day 9 Part 1");
        println!(
            "The total risk level is {}",
            part1(&mut BufReader::new(File::open("2021_09.txt")?))?
        );
    }
    {
        println!("Year 2021 Day 9 Part 2");
        println!(
            "The product of the sizes of the three largest basins is {}",
            part2(&mut BufReader::new(File::open("2021_09.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_part1() -> io::Result<()> {
        let s = "2199943210\n3987894921\n9856789892\n8767896789\n9899965678";
        let expected = 15;
        let actual = part1(&mut Cursor::new(s))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let s = "2199943210\n3987894921\n9856789892\n8767896789\n9899965678";
        let expected = 1134;
        let actual = part2(&mut Cursor::new(s))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
