use std::{
    collections::HashSet,
    fmt::{self, Display, Formatter},
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum PixelIntensity {
    Dark,
    Light,
}

impl Default for PixelIntensity {
    fn default() -> Self {
        Self::Dark
    }
}

impl From<PixelIntensity> for usize {
    fn from(intensity: PixelIntensity) -> Self {
        match intensity {
            PixelIntensity::Dark => 0,
            PixelIntensity::Light => 1,
        }
    }
}

impl TryFrom<char> for PixelIntensity {
    type Error = io::Error;

    fn try_from(this: char) -> Result<Self, Self::Error> {
        match this {
            '.' => Ok(Self::Dark),
            '#' => Ok(Self::Light),
            c => Err(io::Error::new(io::ErrorKind::InvalidData, c.to_string())),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ImageEnhancementAlgorithm {
    light_indices: HashSet<usize>,
}

impl ImageEnhancementAlgorithm {
    fn read(input: &mut dyn BufRead) -> io::Result<Self> {
        let mut buf = String::new();
        input.read_line(&mut buf)?;
        Ok(Self {
            light_indices: buf
                .trim()
                .chars()
                .map(PixelIntensity::try_from)
                .enumerate()
                .filter_map(|(idx, intensity)| match intensity {
                    Ok(PixelIntensity::Light) => Some(Ok(idx)),
                    Ok(PixelIntensity::Dark) => None,
                    Err(e) => Some(Err(e)),
                })
                .collect::<io::Result<_>>()?,
        })
    }
}

impl ImageEnhancementAlgorithm {
    fn intensity_at(&self, idx: usize) -> PixelIntensity {
        if self.light_indices.contains(&idx) {
            PixelIntensity::Light
        } else {
            PixelIntensity::Dark
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Image {
    light_indices: HashSet<(isize, isize)>,
    background: PixelIntensity,
    min_col: isize,
    max_col: isize,
    min_row: isize,
    max_row: isize,
}

impl Image {
    fn read(input: &mut dyn BufRead) -> io::Result<Self> {
        let mut buf = String::new();
        let mut this = Self::default();
        for row in 0.. {
            buf.clear();
            if 0 == input.read_line(&mut buf)? {
                break;
            }
            let buf = buf.trim();
            let added_pixel = buf
                .chars()
                .zip(0..)
                .filter_map(|(c, col)| match PixelIntensity::try_from(c) {
                    Ok(PixelIntensity::Light) => Some(Ok(col)),
                    Ok(PixelIntensity::Dark) => None,
                    Err(e) => Some(Err(e)),
                })
                .try_fold(false, |_, col| {
                    let col = col?;
                    if col > this.max_col {
                        this.max_col = col;
                    }
                    this.light_indices.insert((col, row));
                    io::Result::Ok(true)
                })?;
            if added_pixel {
                this.max_row = row;
            }
        }
        Ok(this)
    }
}

impl Image {
    fn intensity_at(&self, pos @ (col, row): (isize, isize)) -> PixelIntensity {
        if col < self.min_col || self.max_col < col || row < self.min_row || self.max_row < row {
            self.background
        } else if self.light_indices.contains(&pos) {
            PixelIntensity::Light
        } else {
            PixelIntensity::Dark
        }
    }

    fn apply_filter(&self, mut filter: impl FnMut(usize) -> PixelIntensity) -> Self {
        fn neighbors((x, y): (isize, isize)) -> impl Iterator<Item = (isize, isize)> {
            let min_x = x - 1;
            let mid_x = x;
            let max_x = x + 1;
            let min_y = y - 1;
            let mid_y = y;
            let max_y = y + 1;
            [
                (min_x, min_y),
                (mid_x, min_y),
                (max_x, min_y),
                (min_x, mid_y),
                (mid_x, mid_y),
                (max_x, mid_y),
                (min_x, max_y),
                (mid_x, max_y),
                (max_x, max_y),
            ]
            .into_iter()
        }

        let mut this = ((self.min_row - 1)..=(self.max_row + 1))
            .flat_map(|row| ((self.min_col - 1)..=(self.max_col + 1)).map(move |col| (col, row)))
            .filter(|&pos| {
                let idx = neighbors(pos)
                    .fold(0, |acc, pos| acc * 2 + usize::from(self.intensity_at(pos)));
                PixelIntensity::Light == filter(idx)
            })
            .collect::<Image>();
        match self.background {
            PixelIntensity::Dark => this.background = filter(0),
            PixelIntensity::Light => this.background = filter(511),
        }
        this
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Background: {}",
            match self.background {
                PixelIntensity::Dark => '.',
                PixelIntensity::Light => '#',
            }
        )?;
        for row in self.min_row..=self.max_row {
            for col in self.min_col..=self.max_col {
                if self.light_indices.contains(&(col, row)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl FromIterator<(isize, isize)> for Image {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (isize, isize)>,
    {
        iter.into_iter()
            .fold(Self::default(), |mut acc, pos @ (x, y)| {
                acc.light_indices.insert(pos);
                if x < acc.min_col {
                    acc.min_col = x;
                }
                if acc.max_col < x {
                    acc.max_col = x;
                }
                if y < acc.min_row {
                    acc.min_row = y;
                }
                if acc.max_row < y {
                    acc.max_row = y;
                }
                acc
            })
    }
}

fn read_ieai(input: &mut dyn BufRead) -> io::Result<(ImageEnhancementAlgorithm, Image)> {
    let iea = ImageEnhancementAlgorithm::read(&mut *input)?;
    let mut buf = String::new();
    input.read_line(&mut buf)?;
    if !buf.trim().is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Missing blank line before image: {:?}", buf.trim()),
        ));
    }
    let image = Image::read(input)?;
    Ok((iea, image))
}

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    let (iea, image) = read_ieai(input)?;
    let enhance = |image: Image| image.apply_filter(|value| iea.intensity_at(value));
    let enhanced = enhance(image);
    let double_enhanced = enhance(enhanced);
    Ok(double_enhanced.light_indices.len())
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    let (iea, image) = read_ieai(input)?;
    let enhance = |image: Image| image.apply_filter(|value| iea.intensity_at(value));
    let enhanced = (0..50).fold(image, |image, _| enhance(image));
    Ok(enhanced.light_indices.len())
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2021 Day 20 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2021_20.txt")?))?
        );
    }
    {
        println!("Year 2021 Day 20 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2021_20.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###..",
        ".####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#.",
        ".#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#...",
        "....#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.###",
        "#.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#......",
        ".##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#\n",
        "\n",
        "#..#.\n",
        "#....\n",
        "##..#\n",
        "..#..\n",
        "..###\n",
    );

    #[test]
    #[ignore]
    fn test_part1() -> io::Result<()> {
        let expected = 35;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part2() -> io::Result<()> {
        let expected = 3351;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
