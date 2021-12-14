use std::{
    collections::HashSet,
    fmt::{self, Display, Formatter},
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Dots {
    positions: HashSet<(usize, usize)>,
}

impl Dots {
    fn read(input: &mut dyn BufRead) -> io::Result<Self> {
        let mut ret = Self::default();
        let mut buf = String::new();
        loop {
            buf.clear();
            input.read_line(&mut buf)?;
            if buf.trim().is_empty() {
                break;
            }
            let (x, y) = buf.trim().split_once(',').ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Invalid point: {:?}", buf),
                )
            })?;
            ret.positions.insert((
                x.parse().map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Invalid x-coordinate: {:?}: {:?}", x, e),
                    )
                })?,
                y.parse().map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Invalid y-coordinate: {:?}: {:?}", y, e),
                    )
                })?,
            ));
        }
        Ok(ret)
    }
}

impl Dots {
    fn num_dots(&self) -> usize {
        self.positions.len()
    }
}

impl Dots {
    fn fold_up(&mut self, y: usize) {
        // This could be `drain_filter` to avoid just putting `left` right back into
        // `self.positions`, but `drain_filter` is not yet stable:
        // https://github.com/rust-lang/rfcs/issues/2140
        let (left, right) = self
            .positions
            .drain()
            .partition::<Vec<_>, _>(|&(_, dot_y)| dot_y < y);
        self.positions.extend(left);
        self.positions
            .extend(right.into_iter().map(|(x, dot_y)| (x, 2 * y - dot_y)));
    }

    fn fold_left(&mut self, x: usize) {
        // This could be `drain_filter` to avoid just putting `top` right back into
        // `self.positions`, but `drain_filter` is not yet stable:
        // https://github.com/rust-lang/rfcs/issues/2140
        let (top, bottom) = self
            .positions
            .drain()
            .partition::<Vec<_>, _>(|&(dot_x, _)| dot_x < x);
        self.positions.extend(top);
        self.positions
            .extend(bottom.into_iter().map(|(dot_x, y)| (2 * x - dot_x, y)));
    }
}

impl Display for Dots {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let max_x = self.positions.iter().map(|&(x, _)| x).max().unwrap_or(0);
        let max_y = self.positions.iter().map(|&(_, y)| y).max().unwrap_or(0);
        for y in 0..=max_y {
            for x in 0..=max_x {
                if self.positions.contains(&(x, y)) {
                    write!(f, "{}", '\u{2588}')?;
                } else {
                    write!(f, "{}", ' ')?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

enum Axis {
    X,
    Y,
}

fn folds<'input>(
    input: &'input mut dyn BufRead,
) -> impl Iterator<Item = io::Result<(Axis, usize)>> + 'input {
    input.lines().map(|fold| {
        let fold = fold?;
        let line = fold.strip_prefix("fold along ").ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid fold direction: {:?}", fold),
            )
        })?;
        let (axis, value) = line.trim().split_once('=').ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Missing {:?} in fold {:?}", '=', fold),
            )
        })?;
        let value = value.parse().map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid position {:?} in fold{:?}: {:?}", value, fold, e),
            )
        })?;
        match axis {
            "x" => Ok((Axis::X, value)),
            "y" => Ok((Axis::Y, value)),
            axis => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid axis {:?} in fold {:?}", axis, fold),
            )),
        }
    })
}

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    let mut page_1 = Dots::read(&mut *input)?;
    let mut folds = folds(input);
    if let Some(fold) = folds.next() {
        let (axis, value) = fold?;
        match axis {
            Axis::X => page_1.fold_left(value),
            Axis::Y => page_1.fold_up(value),
        }
        Ok(page_1.num_dots())
    } else {
        Err(io::Error::new(io::ErrorKind::InvalidData, "Missing folds"))
    }
}

fn part2(input: &mut dyn BufRead) -> io::Result<String> {
    let mut page_1 = Dots::read(&mut *input)?;
    for fold in folds(input) {
        match fold? {
            (Axis::X, value) => page_1.fold_left(value),
            (Axis::Y, value) => page_1.fold_up(value),
        }
    }
    Ok(format!("{}", page_1))
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2021 Day 13 Part 1");
        println!(
            "There are {} visible dots after the first fold",
            part1(&mut BufReader::new(File::open("2021_13.txt")?))?
        );
    }
    {
        println!("Year 2021 Day 13 Part 2");
        println!("The code is");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2021_13.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &'static str = concat!(
        "6,10\n",
        "0,14\n",
        "9,10\n",
        "0,3\n",
        "10,4\n",
        "4,11\n",
        "6,0\n",
        "6,12\n",
        "4,1\n",
        "0,13\n",
        "10,12\n",
        "3,4\n",
        "3,0\n",
        "8,4\n",
        "1,10\n",
        "2,14\n",
        "8,10\n",
        "9,0\n",
        "\n",
        "fold along y=7\n",
        "fold along x=5\n"
    );

    #[test]
    #[ignore]
    fn test_part1() -> io::Result<()> {
        let expected = 17;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part2() -> io::Result<()> {
        let expected = "█████\n█   █\n█   █\n█   █\n█████\n";
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
