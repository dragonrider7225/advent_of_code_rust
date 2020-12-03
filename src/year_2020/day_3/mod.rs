use crate::parse::NomParse;

use std::{fs, io, iter};

use nom::{branch, character::complete as character, combinator as comb, multi, sequence, IResult};

#[derive(Clone, Copy, Debug)]
enum Tile {
    Snow,
    Tree,
}

impl NomParse for Tile {
    fn nom_parse(s: &str) -> IResult<&str, Self> {
        branch::alt((
            comb::value(Self::Snow, character::char('.')),
            comb::value(Self::Tree, character::char('#')),
        ))(s)
    }
}

#[derive(Clone, Debug)]
struct TreeMap(Vec<Vec<Tile>>);

impl TreeMap {
    fn count_trees(&self, delta_x: usize, delta_y: usize) -> usize {
        (0..self.0.len())
            .step_by(delta_y)
            .zip((0..).step_by(delta_x).map(|x| x % self.0[0].len()))
            .fold(0, |acc, (y, x)| match self.0[y][x] {
                Tile::Snow => acc,
                Tile::Tree => acc + 1,
            })
    }
}

impl NomParse for TreeMap {
    fn nom_parse(s: &str) -> IResult<&str, Self> {
        let (_, first_line) = sequence::terminated(
            multi::many1(Tile::nom_parse),
            character::line_ending,
        )(s)?;
        comb::map(
            multi::many1(sequence::terminated(
                multi::many_m_n(first_line.len(), first_line.len(), Tile::nom_parse),
                character::line_ending,
            )),
            Self,
        )(s)
    }
}

#[allow(unreachable_code)]
pub(super) fn run() -> io::Result<()> {
    let (_, tree_map) = TreeMap::nom_parse(&fs::read_to_string("2020_03.txt")?)
        .expect("Couldn't parse tree map");
    let three = {
        println!("Year 2020 Day 3 Part 1");
        let three = tree_map.count_trees(3, 1);
        println!("There are {} trees on the path with slope -1/3", three);
        three
    };
    {
        println!("Year 2020 Day 3 Part 2");
        let total = iter::once(three)
            .chain(
                [(1usize, 1usize), (5, 1), (7, 1), (1, 2)]
                   .iter()
                   .map(|&(delta_x, delta_y)| tree_map.count_trees(delta_x, delta_y)),
            ).product::<usize>();
        println!("The product is {} trees**5", total);
    }
    Ok(())
}
