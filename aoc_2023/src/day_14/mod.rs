use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
    fs::File,
    io::{self, BufRead, BufReader},
};

use nom::{branch, bytes::complete as bytes, combinator, multi, sequence, IResult};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Tile {
    Empty,
    Round,
    Cube,
}

impl Tile {
    fn nom_parse(s: &str) -> IResult<&str, Self> {
        branch::alt((
            combinator::value(Tile::Empty, bytes::tag(".")),
            combinator::value(Tile::Round, bytes::tag("O")),
            combinator::value(Tile::Cube, bytes::tag("#")),
        ))(s)
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::Round => write!(f, "O"),
            Self::Cube => write!(f, "#"),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Platform(Vec<Vec<Tile>>);

impl Platform {
    fn nom_parse(s: &str) -> IResult<&str, Self> {
        combinator::map(
            multi::many1(sequence::terminated(
                multi::many1(Tile::nom_parse),
                aoc_util::newline,
            )),
            Self,
        )(s)
    }

    fn total_northward_load(&self) -> usize {
        self.0
            .iter()
            .rev()
            .enumerate()
            .map(|(row_idx, row)| {
                let load = row_idx + 1;
                load * row
                    .iter()
                    .filter(|tile| matches!(tile, Tile::Round))
                    .count()
            })
            .sum()
    }

    fn shift_north(&mut self) {
        fn do_shift(this: &mut Platform, gap: &mut Option<(usize, usize)>, coords: (usize, usize)) {
            let (row_idx, column_idx) = coords;
            if let Some((gap_start, num_round)) = gap.take() {
                this.0[gap_start..(gap_start + num_round)]
                    .iter_mut()
                    .for_each(|row| row[column_idx] = Tile::Round);
                this.0[(gap_start + num_round)..row_idx]
                    .iter_mut()
                    .for_each(|row| row[column_idx] = Tile::Empty);
            }
        }

        for column_idx in 0..self.0[0].len() {
            let mut gap = None;
            for row_idx in 0..self.0.len() {
                match self.0[row_idx][column_idx] {
                    Tile::Cube => do_shift(self, &mut gap, (row_idx, column_idx)),
                    Tile::Empty => {
                        gap.get_or_insert((row_idx, 0));
                    }
                    Tile::Round => {
                        if let Some((_, num_round)) = gap.as_mut() {
                            *num_round += 1;
                        }
                    }
                }
            }
            do_shift(self, &mut gap, (self.0.len(), column_idx));
        }
    }

    fn shift_west(&mut self) {
        fn do_shift(this: &mut Platform, gap: &mut Option<(usize, usize)>, coords: (usize, usize)) {
            let (row_idx, column_idx) = coords;
            if let Some((gap_start, num_round)) = gap.take() {
                this.0[row_idx][gap_start..(gap_start + num_round)]
                    .iter_mut()
                    .for_each(|tile| *tile = Tile::Round);
                this.0[row_idx][(gap_start + num_round)..column_idx]
                    .iter_mut()
                    .for_each(|tile| *tile = Tile::Empty);
            }
        }

        for row_idx in 0..self.0.len() {
            let mut gap = None;
            for column_idx in 0..self.0[row_idx].len() {
                match self.0[row_idx][column_idx] {
                    Tile::Cube => do_shift(self, &mut gap, (row_idx, column_idx)),
                    Tile::Empty => {
                        gap.get_or_insert((column_idx, 0));
                    }
                    Tile::Round => {
                        if let Some((_, num_round)) = gap.as_mut() {
                            *num_round += 1;
                        }
                    }
                }
            }
            do_shift(self, &mut gap, (row_idx, self.0[row_idx].len()));
        }
    }

    fn shift_south(&mut self) {
        fn do_shift(this: &mut Platform, gap: &mut Option<(usize, usize)>, coords: (usize, usize)) {
            let (row_idx, column_idx) = coords;
            if let Some((gap_start, num_round)) = gap.take() {
                let last_round = gap_start + 1 - num_round;
                this.0[last_round..=gap_start]
                    .iter_mut()
                    .for_each(|row| row[column_idx] = Tile::Round);
                this.0[row_idx..last_round]
                    .iter_mut()
                    .for_each(|row| row[column_idx] = Tile::Empty);
            }
        }

        for column_idx in 0..self.0[0].len() {
            let mut gap = None;
            for row_idx in (0..self.0.len()).rev() {
                match self.0[row_idx][column_idx] {
                    Tile::Cube => do_shift(self, &mut gap, (row_idx + 1, column_idx)),
                    Tile::Empty => {
                        gap.get_or_insert((row_idx, 0));
                    }
                    Tile::Round => {
                        if let Some((_, num_round)) = gap.as_mut() {
                            *num_round += 1;
                        }
                    }
                }
            }
            do_shift(self, &mut gap, (0, column_idx));
        }
    }

    fn shift_east(&mut self) {
        fn do_shift(this: &mut Platform, gap: &mut Option<(usize, usize)>, coords: (usize, usize)) {
            let (row_idx, column_idx) = coords;
            if let Some((gap_start, num_round)) = gap.take() {
                let last_round = gap_start + 1 - num_round;
                this.0[row_idx][last_round..=gap_start]
                    .iter_mut()
                    .for_each(|tile| *tile = Tile::Round);
                this.0[row_idx][column_idx..last_round]
                    .iter_mut()
                    .for_each(|tile| *tile = Tile::Empty);
            }
        }

        for row_idx in 0..self.0.len() {
            let mut gap = None;
            for column_idx in (0..self.0[row_idx].len()).rev() {
                match self.0[row_idx][column_idx] {
                    Tile::Cube => do_shift(self, &mut gap, (row_idx, column_idx + 1)),
                    Tile::Empty => {
                        gap.get_or_insert((column_idx, 0));
                    }
                    Tile::Round => {
                        if let Some((_, num_round)) = gap.as_mut() {
                            *num_round += 1;
                        }
                    }
                }
            }
            do_shift(self, &mut gap, (row_idx, 0));
        }
    }

    fn spin_cycle(&mut self) {
        self.shift_north();
        self.shift_west();
        self.shift_south();
        self.shift_east();
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in &self.0 {
            for tile in row {
                write!(f, "{tile}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    let input = {
        let mut s = String::new();
        input.read_to_string(&mut s)?;
        s
    };
    let mut platform = Platform::nom_parse(&input)
        .map(|(_, platform)| platform)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    platform.shift_north();
    Ok(platform.total_northward_load())
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    let input = {
        let mut s = String::new();
        input.read_to_string(&mut s)?;
        s
    };
    let mut platform = Platform::nom_parse(&input)
        .map(|(_, platform)| platform)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    const NUM_CYCLES: usize = 1_000_000_000;
    let mut cache = [(platform.clone(), 0)]
        .into_iter()
        .collect::<HashMap<_, _>>();
    for i in 1..=NUM_CYCLES {
        platform.spin_cycle();
        if let Some(cycle_start) = cache.get(&platform) {
            let cycle_len = i - cycle_start;
            let remaining_cycles = NUM_CYCLES - i;
            let final_offset = cycle_start + (remaining_cycles % cycle_len);
            println!("Cycle of length {cycle_len} started after {cycle_start} cycles");
            println!("Final state is equivalent to state after {final_offset} cycles");
            platform = cache
                .into_iter()
                .find_map(|(p, i)| Some(p).filter(|_| i == final_offset))
                .expect("Skipped a cycle");
            break;
        } else {
            cache.insert(platform.clone(), i);
        }
    }
    Ok(platform.total_northward_load())
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2023 Day 14 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2023_14.txt")?))?
        );
    }
    {
        println!("Year 2023 Day 14 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2023_14.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        "O....#....\n",
        "O.OO#....#\n",
        ".....##...\n",
        "OO.#O....O\n",
        ".O.....O#.\n",
        "O.#..O.#.#\n",
        "..O..#O..O\n",
        ".......O..\n",
        "#....###..\n",
        "#OO..#....\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 136;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_spin_cycle() -> io::Result<()> {
        let mut platform = Platform::nom_parse(TEST_DATA)
            .map(|(_, platform)| platform)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        let expected = concat!(
            ".....#....\n",
            "....#...O#\n",
            "...OO##...\n",
            ".OO#......\n",
            ".....OOO#.\n",
            ".O#...O#.#\n",
            "....O#....\n",
            "......OOOO\n",
            "#...O###..\n",
            "#..OO#....\n",
        );
        platform.spin_cycle();
        let actual = platform.to_string();
        assert_eq!(expected, actual);
        let expected = concat!(
            ".....#....\n",
            "....#...O#\n",
            ".....##...\n",
            "..O#......\n",
            ".....OOO#.\n",
            ".O#...O#.#\n",
            "....O#...O\n",
            ".......OOO\n",
            "#..OO###..\n",
            "#.OOO#...O\n",
        );
        platform.spin_cycle();
        let actual = platform.to_string();
        assert_eq!(expected, actual);
        let expected = concat!(
            ".....#....\n",
            "....#...O#\n",
            ".....##...\n",
            "..O#......\n",
            ".....OOO#.\n",
            ".O#...O#.#\n",
            "....O#...O\n",
            ".......OOO\n",
            "#...O###.O\n",
            "#.OOO#...O\n",
        );
        platform.spin_cycle();
        let actual = platform.to_string();
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 64;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
