use crate::parse::NomParse;
use nom::{branch, character::complete as character, combinator as comb, multi, sequence, IResult};
use std::{
    fmt::{self, Debug, Formatter},
    fs, io,
    ops::Deref,
};

#[derive(Clone, Copy, Eq, PartialEq)]
enum Tile {
    Floor,
    EmptyChair,
    OccupiedChair,
}

impl Tile {
    fn is_occupied(&self) -> bool {
        match *self {
            Self::OccupiedChair => true,
            Self::EmptyChair | Self::Floor => false,
        }
    }

    fn is_seat(&self) -> bool {
        match *self {
            Self::EmptyChair | Self::OccupiedChair => true,
            Self::Floor => false,
        }
    }

    fn leave(&mut self) -> bool {
        match *self {
            Self::Floor | Self::EmptyChair => false,
            Self::OccupiedChair => {
                *self = Self::EmptyChair;
                true
            }
        }
    }

    fn occupy(&mut self) -> bool {
        match *self {
            Self::Floor | Self::OccupiedChair => false,
            Self::EmptyChair => {
                *self = Self::OccupiedChair;
                true
            }
        }
    }
}

impl Debug for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Floor => '.',
                Self::EmptyChair => 'L',
                Self::OccupiedChair => '#',
            },
        )
    }
}

impl<'s> NomParse<'s> for Tile {
    fn nom_parse(s: &str) -> IResult<&str, Self> {
        branch::alt((
            comb::value(Self::Floor, character::char('.')),
            comb::value(Self::EmptyChair, character::char('L')),
            comb::value(Self::OccupiedChair, character::char('#')),
        ))(s)
    }
}

trait OccupationBehavior<TileRow>
where
    Self: Debug,
    TileRow: Deref<Target = [Tile]>,
{
    /// Determine whether the tile at `tiles[row][column]` should switch from `Tile::OccupiedChair`
    /// to `Tile::UnoccupiedChair` or vice versa.
    fn update_tile(&self, row: usize, column: usize, tiles: &[TileRow]) -> bool;
}

/// The basic occupation behavior for part 1.
#[derive(Clone, Copy, Debug)]
struct BasicOccupationBehavior;

impl OccupationBehavior<Vec<Tile>> for BasicOccupationBehavior {
    fn update_tile(&self, row: usize, column: usize, tiles: &[Vec<Tile>]) -> bool {
        if !tiles[row][column].is_seat() {
            false
        } else {
            let left_column = column.checked_sub(1);
            let upper_row = row.checked_sub(1);
            let right_column = column
                .checked_add(1)
                .filter(|&column| column < tiles[0].len());
            let lower_row = row.checked_add(1).filter(|&row| row < tiles.len());

            let neighbors = [
                left_column.map(|column| (row, column)),
                upper_row.and_then(|row| Some((row, left_column?))),
                upper_row.map(|row| (row, column)),
                upper_row.and_then(|row| Some((row, right_column?))),
                right_column.map(|column| (row, column)),
                lower_row.and_then(|row| Some((row, right_column?))),
                lower_row.map(|row| (row, column)),
                lower_row.and_then(|row| Some((row, left_column?))),
            ];

            let num_occupied_neighbors = neighbors
                .iter()
                .copied()
                .flatten()
                .map(|(row, column)| tiles[row][column])
                .filter(Tile::is_occupied)
                .count();
            match num_occupied_neighbors {
                0 => !tiles[row][column].is_occupied(),
                1..=3 => false,
                4..=usize::MAX => tiles[row][column].is_occupied(),
                _ => unreachable!(),
            }
        }
    }
}

/// The line-of-sight occupation behavior for part 2.
#[derive(Clone, Copy, Debug)]
struct LosOccupationBehavior;

impl OccupationBehavior<Vec<Tile>> for LosOccupationBehavior {
    fn update_tile(&self, row: usize, column: usize, tiles: &[Vec<Tile>]) -> bool {
        if !tiles[row][column].is_seat() {
            false
        } else {
            let max_left_distance = column;
            let max_up_distance = row;
            let max_right_distance = tiles[0].len() - 1 - column;
            let max_down_distance = tiles.len() - 1 - row;

            let mut left_los = |distance| tiles[row][column - distance];
            let mut upper_left_los = |distance: usize| tiles[row - distance][column - distance];
            let mut upper_los = |distance: usize| tiles[row - distance][column];
            let mut upper_right_los = |distance: usize| tiles[row - distance][column + distance];
            let mut right_los = |distance| tiles[row][column + distance];
            let mut lower_right_los = |distance: usize| tiles[row + distance][column + distance];
            let mut lower_los = |distance: usize| tiles[row + distance][column];
            let mut lower_left_los = |distance: usize| tiles[row + distance][column - distance];

            let mut lines_of_sight = [
                (1..=max_left_distance).map::<_, &mut dyn FnMut(_) -> _>(&mut left_los),
                (1..=Ord::min(max_left_distance, max_up_distance)).map(&mut upper_left_los),
                (1..=max_up_distance).map(&mut upper_los),
                (1..=Ord::min(max_right_distance, max_up_distance)).map(&mut upper_right_los),
                (1..=max_right_distance).map(&mut right_los),
                (1..=Ord::min(max_right_distance, max_down_distance)).map(&mut lower_right_los),
                (1..=max_down_distance).map(&mut lower_los),
                (1..=Ord::min(max_left_distance, max_down_distance)).map(&mut lower_left_los),
            ];

            let num_lines_of_sight_occupied = lines_of_sight
                .iter_mut()
                .flat_map(|iter| iter.find(Tile::is_seat))
                .filter(Tile::is_occupied)
                .count();
            match num_lines_of_sight_occupied {
                0 => !tiles[row][column].is_occupied(),
                1..=4 => false,
                5..=usize::MAX => tiles[row][column].is_occupied(),
                _ => unreachable!(),
            }
        }
    }
}

#[derive(Clone, Debug)]
struct GameOfLife<'behavior> {
    tiles: Vec<Vec<Tile>>,
    occupation_behavior: &'behavior dyn OccupationBehavior<Vec<Tile>>,
}

impl<'behavior> GameOfLife<'behavior> {
    fn num_occupied_seats(&self) -> usize {
        self.tiles
            .iter()
            .flat_map(|iter| iter.iter())
            .copied()
            .filter(Tile::is_occupied)
            .count()
    }

    fn step(&mut self) -> bool {
        let mut new_tiles = self.tiles.clone();
        let mut changed = false;
        for (i, new_tile_row) in new_tiles.iter_mut().enumerate().take(self.tiles.len()) {
            for (j, new_tile) in new_tile_row
                .iter_mut()
                .enumerate()
                .take(self.tiles[i].len())
                .filter(|&(j, _)| self.occupation_behavior.update_tile(i, j, &self.tiles))
            {
                if self.tiles[i][j].is_occupied() {
                    new_tile.leave();
                } else {
                    new_tile.occupy();
                }
                changed = true;
            }
        }
        self.tiles = new_tiles;
        changed
    }

    fn run_to_stasis(&mut self) {
        while self.step() {}
    }
}

impl<'behavior> Eq for GameOfLife<'behavior> {}

impl_from_str_for_nom_parse!(GameOfLife<'static>);

impl<'s> NomParse<'s> for GameOfLife<'static> {
    fn nom_parse(s: &str) -> IResult<&str, Self> {
        let (s, first_line) =
            sequence::terminated(multi::many0(Tile::nom_parse), character::line_ending)(s)?;
        let (s, mut remaining_lines) = sequence::terminated(
            multi::separated_list0(
                character::line_ending,
                multi::many_m_n(first_line.len(), first_line.len(), Tile::nom_parse),
            ),
            comb::opt(character::line_ending),
        )(s)?;
        remaining_lines.insert(0, first_line);
        Ok((
            s,
            Self {
                tiles: remaining_lines,
                occupation_behavior: &BasicOccupationBehavior,
            },
        ))
    }
}

impl<'behavior> PartialEq for GameOfLife<'behavior> {
    fn eq(&self, rhs: &Self) -> bool {
        self.tiles.eq(&rhs.tiles)
    }
}

pub(super) fn run() -> io::Result<()> {
    let seating_area = fs::read_to_string("2020_11.txt")?
        .parse::<GameOfLife<'_>>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    {
        println!("Year 2020 Day 11 Part 1");
        let mut seating_area = seating_area.clone();
        seating_area.run_to_stasis();
        println!(
            "When the seating area reaches equilibrium, there are {} occupied seats",
            seating_area.num_occupied_seats(),
        );
    }
    {
        println!("Year 2020 Day 11 Part 2");
        let mut seating_area = seating_area;
        seating_area.occupation_behavior = &LosOccupationBehavior;
        seating_area.run_to_stasis();
        println!(
            "When the seating area reaches equilibrium this time, there are {} occupied seats",
            seating_area.num_occupied_seats(),
        );
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[ignore]
    #[test]
    fn runs_correctly() {
        let expected = concat!(
            "#.##.##.##\n",
            "#######.##\n",
            "#.#.#..#..\n",
            "####.##.##\n",
            "#.##.##.##\n",
            "#.#####.##\n",
            "..#.#.....\n",
            "##########\n",
            "#.######.#\n",
            "#.#####.##\n",
        )
        .parse::<GameOfLife<'_>>()
        .unwrap();
        let mut actual = concat!(
            "L.LL.LL.LL\n",
            "LLLLLLL.LL\n",
            "L.L.L..L..\n",
            "LLLL.LL.LL\n",
            "L.LL.LL.LL\n",
            "L.LLLLL.LL\n",
            "..L.L.....\n",
            "LLLLLLLLLL\n",
            "L.LLLLLL.L\n",
            "L.LLLLL.LL\n",
        )
        .parse::<GameOfLife<'_>>()
        .unwrap();
        actual.step();
        assert_eq!(expected, actual);
    }

    #[ignore]
    #[test]
    fn terminates_correctly() {
        let expected = concat!(
            "#.#L.L#.##\n",
            "#LLL#LL.L#\n",
            "L.#.L..#..\n",
            "#L##.##.L#\n",
            "#.#L.LL.LL\n",
            "#.#L#L#.##\n",
            "..L.L.....\n",
            "#L#L##L#L#\n",
            "#.LLLLLL.L\n",
            "#.#L#L#.##\n",
        )
        .parse::<GameOfLife<'_>>()
        .unwrap();
        let mut actual = concat!(
            "L.LL.LL.LL\n",
            "LLLLLLL.LL\n",
            "L.L.L..L..\n",
            "LLLL.LL.LL\n",
            "L.LL.LL.LL\n",
            "L.LLLLL.LL\n",
            "..L.L.....\n",
            "LLLLLLLLLL\n",
            "L.LLLLLL.L\n",
            "L.LLLLL.LL\n",
        )
        .parse::<GameOfLife<'_>>()
        .unwrap();
        actual.run_to_stasis();
        assert_eq!(expected, actual);
    }

    #[ignore]
    #[test]
    fn los_runs_correctly() {
        let expected = {
            let mut res = concat!(
                "#.##.##.##\n",
                "#######.##\n",
                "#.#.#..#..\n",
                "####.##.##\n",
                "#.##.##.##\n",
                "#.#####.##\n",
                "..#.#.....\n",
                "##########\n",
                "#.######.#\n",
                "#.#####.##\n",
            )
            .parse::<GameOfLife<'_>>()
            .unwrap();
            res.occupation_behavior = &LosOccupationBehavior;
            res
        };
        let mut actual = concat!(
            "L.LL.LL.LL\n",
            "LLLLLLL.LL\n",
            "L.L.L..L..\n",
            "LLLL.LL.LL\n",
            "L.LL.LL.LL\n",
            "L.LLLLL.LL\n",
            "..L.L.....\n",
            "LLLLLLLLLL\n",
            "L.LLLLLL.L\n",
            "L.LLLLL.LL\n",
        )
        .parse::<GameOfLife<'_>>()
        .unwrap();
        actual.occupation_behavior = &LosOccupationBehavior;
        actual.step();
        assert_eq!(expected, actual);
    }

    #[ignore]
    #[test]
    fn los_terminates_correctly() {
        let expected = {
            let mut res = concat!(
                "#.L#.L#.L#\n",
                "#LLLLLL.LL\n",
                "L.L.L..#..\n",
                "##L#.#L.L#\n",
                "L.L#.LL.L#\n",
                "#.LLLL#.LL\n",
                "..#.L.....\n",
                "LLL###LLL#\n",
                "#.LLLLL#.L\n",
                "#.L#LL#.L#\n",
            )
            .parse::<GameOfLife<'_>>()
            .unwrap();
            res.occupation_behavior = &LosOccupationBehavior;
            res
        };
        let mut actual = concat!(
            "L.LL.LL.LL\n",
            "LLLLLLL.LL\n",
            "L.L.L..L..\n",
            "LLLL.LL.LL\n",
            "L.LL.LL.LL\n",
            "L.LLLLL.LL\n",
            "..L.L.....\n",
            "LLLLLLLLLL\n",
            "L.LLLLLL.L\n",
            "L.LLLLL.LL\n",
        )
        .parse::<GameOfLife<'_>>()
        .unwrap();
        actual.occupation_behavior = &LosOccupationBehavior;
        actual.run_to_stasis();
        assert_eq!(expected, actual);
    }
}
