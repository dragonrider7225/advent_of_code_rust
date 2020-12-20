use crate::parse::NomParse;
use nom::{branch, character::complete as character, combinator as comb, multi, sequence, IResult};
use std::{
    collections::HashSet,
    convert::TryFrom,
    fmt::{self, Debug, Formatter},
    fs, io,
};

#[derive(Clone, Default, Eq, PartialEq)]
struct ConwayCubes {
    active: HashSet<(i64, i64, i64, i64)>,
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
    min_z: i64,
    max_z: i64,
    min_w: i64,
    max_w: i64,
    use_w: bool,
}

impl ConwayCubes {
    fn new<V>(values: V) -> Self
    where
        V: IntoIterator<Item = (i64, i64)>,
    {
        let mut res = Self {
            active: values.into_iter().map(|(x, y)| (x, y, 0, 0)).collect(),
            ..Default::default()
        };
        res.update_bounds();
        res
    }

    fn count_active(&self) -> u64 {
        self.active.iter().fold(0, |acc, _| acc + 1)
    }

    fn count_active_neighbors(&self, x: i64, y: i64, z: i64, w: i64) -> usize {
        if x < self.min_x - 1
            || self.max_x + 1 < x
            || y < self.min_y - 1
            || self.max_y + 1 < y
            || z < self.min_z - 1
            || self.max_z + 1 < z
            || (self.use_w && (w < self.min_w - 1 || self.max_w + 1 < w))
            || (!self.use_w && w != self.min_w)
        {
            0
        } else {
            let mut res = 0;
            let w_range = if self.use_w { (w - 1)..=(w + 1) } else { w..=w };
            let pos = (x, y, z, w);
            for l in w_range {
                for i in (x - 1)..=(x + 1) {
                    for j in (y - 1)..=(y + 1) {
                        for k in (z - 1)..=(z + 1) {
                            let adjacent = (i, j, k, l);
                            if pos == adjacent {
                                continue;
                            } else if self.active.contains(&adjacent) {
                                res += 1;
                                // if res > 3 {
                                //     return 4;
                                // }
                            }
                        }
                    }
                }
            }
            res
        }
    }

    fn boot(&mut self) -> &mut Self {
        for _ in 0..6 {
            self.step();
        }
        self
    }

    fn step(&mut self) {
        let mut delta = HashSet::new();
        let w_range = if self.use_w {
            (self.min_w - 1)..=(self.max_w + 1)
        } else {
            self.min_w..=self.min_w
        };
        for w in w_range {
            for x in (self.min_x - 1)..=(self.max_x + 1) {
                for y in (self.min_y - 1)..=(self.max_y + 1) {
                    for z in (self.min_z - 1)..=(self.max_z + 1) {
                        let num_active_neighbors = self.count_active_neighbors(x, y, z, w);
                        let pos = (x, y, z, w);
                        let is_active = self.active.contains(&pos);
                        match (num_active_neighbors, is_active) {
                            (3, false) => delta.insert(pos),
                            (2, true) | (3, true) | (_, false) => false,
                            (_, true) => delta.insert(pos),
                        };
                    }
                }
            }
        }
        for pos in delta {
            if self.active.contains(&pos) {
                self.active.remove(&pos);
            } else {
                self.active.insert(pos);
            }
        }
        self.update_bounds();
    }

    fn update_bounds(&mut self) {
        assert_ne!(self.active.len(), 0);
        let (min_x, max_x, min_y, max_y, min_z, max_z, min_w, max_w) = self.active.iter().fold(
            (None, None, None, None, None, None, None, None),
            |acc, pos| {
                (
                    Some(acc.0.filter(|&min_x| min_x <= pos.0).unwrap_or(pos.0)),
                    Some(acc.1.filter(|&max_x| max_x >= pos.0).unwrap_or(pos.0)),
                    Some(acc.2.filter(|&min_y| min_y <= pos.1).unwrap_or(pos.1)),
                    Some(acc.3.filter(|&max_y| max_y >= pos.1).unwrap_or(pos.1)),
                    Some(acc.4.filter(|&min_z| min_z <= pos.2).unwrap_or(pos.2)),
                    Some(acc.5.filter(|&max_z| max_z >= pos.2).unwrap_or(pos.2)),
                    Some(
                        acc.6
                            .filter(|&min_w| {
                                if self.use_w {
                                    min_w <= pos.3
                                } else {
                                    assert_eq!(
                                        min_w, pos.3,
                                        "All values of w must be the same if w is disabled",
                                    );
                                    true
                                }
                            })
                            .unwrap_or(pos.3),
                    ),
                    Some(
                        acc.7
                            .filter(|&max_w| {
                                if self.use_w {
                                    max_w >= pos.3
                                } else {
                                    assert_eq!(
                                        max_w, pos.3,
                                        "All values of w must be the same if w is disabled",
                                    );
                                    true
                                }
                            })
                            .unwrap_or(pos.3),
                    ),
                )
            },
        );
        self.min_x = min_x.unwrap();
        self.max_x = max_x.unwrap();
        self.min_y = min_y.unwrap();
        self.max_y = max_y.unwrap();
        self.min_z = min_z.unwrap();
        self.max_z = max_z.unwrap();
        self.min_w = min_w.unwrap();
        self.max_w = max_w.unwrap();
    }
}

impl Debug for ConwayCubes {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut active = self.active.iter().copied().collect::<Vec<_>>();
        active.sort();
        let mut debug_struct = f.debug_struct("ConwayCubes");
        if self.use_w {
            debug_struct.field("active", &active);
        } else {
            let active = active
                .into_iter()
                .map(|(x, y, z, _)| (x, y, z))
                .collect::<Vec<_>>();
            debug_struct.field("active", &active);
        }
        debug_struct
            .field("bounds_x", &(self.min_x, self.max_x))
            .field("bounds_y", &(self.min_y, self.max_y))
            .field("bounds_z", &(self.min_z, self.max_z));
        if self.use_w {
            debug_struct.field("bounds_w", &(self.min_w, self.max_w));
        }
        debug_struct.finish()
    }
}

impl_from_str_for_nom_parse!(ConwayCubes);

impl<'s> NomParse<'s> for ConwayCubes {
    fn nom_parse(s: &str) -> IResult<&str, Self> {
        comb::map(
            multi::many0(sequence::terminated(
                multi::many0(branch::alt((
                    comb::value(true, character::char('#')),
                    comb::value(false, character::char('.')),
                ))),
                character::line_ending,
            )),
            |lines| {
                let active = lines
                    .into_iter()
                    .enumerate()
                    .flat_map(|(x, line)| {
                        let x = i64::try_from(x).unwrap();
                        line.into_iter()
                            .enumerate()
                            .filter_map(|(y, active)| Some(y).filter(|_| active))
                            .map(move |y| (x, i64::try_from(y).unwrap()))
                    })
                    .collect::<Vec<_>>();
                Self::new(active)
            },
        )(s)
    }
}

#[allow(unreachable_code)]
pub(super) fn run() -> io::Result<()> {
    let state = fs::read_to_string("2020_17.txt")?
        .parse::<ConwayCubes>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    {
        println!("Year 2020 Day 17 Part 1");
        let mut state = state.clone();
        println!(
            "After the boot sequence, there are {} active cubes",
            state.boot().count_active()
        );
    }
    {
        println!("Year 2020 Day 17 Part 2");
        let mut state = state.clone();
        state.use_w = true;
        println!(
            "After the boot sequence, there are {} active cubes",
            state.boot().count_active()
        );
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[ignore]
    #[test]
    fn parses_layout() {
        let expected = Ok(ConwayCubes {
            active: [
                (0, 1, 0, 0),
                (1, 2, 0, 0),
                (2, 0, 0, 0),
                (2, 1, 0, 0),
                (2, 2, 0, 0),
            ]
            .iter()
            .copied()
            .collect(),
            min_x: 0,
            max_x: 2,
            min_y: 0,
            max_y: 2,
            min_z: 0,
            max_z: 0,
            min_w: 0,
            max_w: 0,
            use_w: false,
        });
        let actual = concat!(".#.\n", "..#\n", "###\n",).parse::<ConwayCubes>();
        assert_eq!(expected, actual);
    }

    #[ignore]
    #[test]
    fn steps_correctly_in_3d() {
        let expected = ConwayCubes {
            active: [
                (1, 0, -1, 0),
                (2, 2, -1, 0),
                (3, 1, -1, 0),
                (1, 0, 0, 0),
                (1, 2, 0, 0),
                (2, 1, 0, 0),
                (2, 2, 0, 0),
                (3, 1, 0, 0),
                (1, 0, 1, 0),
                (2, 2, 1, 0),
                (3, 1, 1, 0),
            ]
            .iter()
            .copied()
            .collect(),
            min_x: 1,
            max_x: 3,
            min_y: 0,
            max_y: 2,
            min_z: -1,
            max_z: 1,
            min_w: 0,
            max_w: 0,
            use_w: false,
        };
        let mut actual = ConwayCubes {
            active: [
                (0, 1, 0, 0),
                (1, 2, 0, 0),
                (2, 0, 0, 0),
                (2, 1, 0, 0),
                (2, 2, 0, 0),
            ]
            .iter()
            .copied()
            .collect(),
            min_x: 0,
            max_x: 2,
            min_y: 0,
            max_y: 2,
            min_z: 0,
            max_z: 0,
            min_w: 0,
            max_w: 0,
            use_w: false,
        };
        actual.step();
        assert_eq!(expected, actual);
    }

    #[ignore]
    #[test]
    fn boots_correctly_in_3d() {
        let expected = 112;
        let actual = ConwayCubes {
            active: [
                (0, 1, 0, 0),
                (1, 2, 0, 0),
                (2, 0, 0, 0),
                (2, 1, 0, 0),
                (2, 2, 0, 0),
            ]
            .iter()
            .copied()
            .collect(),
            min_x: 0,
            max_x: 2,
            min_y: 0,
            max_y: 2,
            min_z: 0,
            max_z: 0,
            min_w: 0,
            max_w: 0,
            use_w: false,
        }
        .boot()
        .count_active();
        assert_eq!(expected, actual);
    }

    #[ignore]
    #[test]
    fn steps_correctly_in_4d() {
        let expected = ConwayCubes {
            active: [
                (1, 0, -1, -1),
                (2, 2, -1, -1),
                (3, 1, -1, -1),
                (1, 0, 0, -1),
                (2, 2, 0, -1),
                (3, 1, 0, -1),
                (1, 0, 1, -1),
                (2, 2, 1, -1),
                (3, 1, 1, -1),
                (1, 0, -1, 0),
                (2, 2, -1, 0),
                (3, 1, -1, 0),
                (1, 0, 0, 0),
                (1, 2, 0, 0),
                (2, 1, 0, 0),
                (2, 2, 0, 0),
                (3, 1, 0, 0),
                (1, 0, 1, 0),
                (2, 2, 1, 0),
                (3, 1, 1, 0),
                (1, 0, -1, 1),
                (2, 2, -1, 1),
                (3, 1, -1, 1),
                (1, 0, 0, 1),
                (2, 2, 0, 1),
                (3, 1, 0, 1),
                (1, 0, 1, 1),
                (2, 2, 1, 1),
                (3, 1, 1, 1),
            ]
            .iter()
            .copied()
            .collect(),
            min_x: 1,
            max_x: 3,
            min_y: 0,
            max_y: 2,
            min_z: -1,
            max_z: 1,
            min_w: -1,
            max_w: 1,
            use_w: true,
        };
        let mut actual = ConwayCubes {
            active: [
                (0, 1, 0, 0),
                (1, 2, 0, 0),
                (2, 0, 0, 0),
                (2, 1, 0, 0),
                (2, 2, 0, 0),
            ]
            .iter()
            .copied()
            .collect(),
            min_x: 0,
            max_x: 2,
            min_y: 0,
            max_y: 2,
            min_z: 0,
            max_z: 0,
            min_w: 0,
            max_w: 0,
            use_w: true,
        };
        actual.step();
        assert_eq!(expected, actual);
    }

    #[ignore]
    #[test]
    fn steps_twice_correctly_in_4d() {
        let expected = ConwayCubes {
            active: [
                (2, 1, -2, -2),
                (0, -1, 0, -2),
                (0, 0, 0, -2),
                (0, 1, 0, -2),
                (1, -1, 0, -2),
                (1, 0, 0, -2),
                (1, 2, 0, -2),
                (1, 3, 0, -2),
                (2, -1, 0, -2),
                (2, 3, 0, -2),
                (3, 0, 0, -2),
                (3, 3, 0, -2),
                (4, 0, 0, -2),
                (4, 1, 0, -2),
                (4, 2, 0, -2),
                (2, 1, 2, -2),
                (0, -1, -2, 0),
                (0, 0, -2, 0),
                (0, 1, -2, 0),
                (1, -1, -2, 0),
                (1, 0, -2, 0),
                (1, 2, -2, 0),
                (1, 3, -2, 0),
                (2, -1, -2, 0),
                (2, 3, -2, 0),
                (3, 0, -2, 0),
                (3, 3, -2, 0),
                (4, 0, -2, 0),
                (4, 1, -2, 0),
                (4, 2, -2, 0),
                (0, -1, 2, 0),
                (0, 0, 2, 0),
                (0, 1, 2, 0),
                (1, -1, 2, 0),
                (1, 0, 2, 0),
                (1, 2, 2, 0),
                (1, 3, 2, 0),
                (2, -1, 2, 0),
                (2, 3, 2, 0),
                (3, 0, 2, 0),
                (3, 3, 2, 0),
                (4, 0, 2, 0),
                (4, 1, 2, 0),
                (4, 2, 2, 0),
                (2, 1, -2, 2),
                (0, -1, 0, 2),
                (0, 0, 0, 2),
                (0, 1, 0, 2),
                (1, -1, 0, 2),
                (1, 0, 0, 2),
                (1, 2, 0, 2),
                (1, 3, 0, 2),
                (2, -1, 0, 2),
                (2, 3, 0, 2),
                (3, 0, 0, 2),
                (3, 3, 0, 2),
                (4, 0, 0, 2),
                (4, 1, 0, 2),
                (4, 2, 0, 2),
                (2, 1, 2, 2),
            ]
            .iter()
            .copied()
            .collect(),
            min_x: 0,
            max_x: 4,
            min_y: -1,
            max_y: 3,
            min_z: -2,
            max_z: 2,
            min_w: -2,
            max_w: 2,
            use_w: true,
        };
        let mut actual = ConwayCubes {
            active: [
                (0, 1, 0, 0),
                (1, 2, 0, 0),
                (2, 0, 0, 0),
                (2, 1, 0, 0),
                (2, 2, 0, 0),
            ]
            .iter()
            .copied()
            .collect(),
            min_x: 0,
            max_x: 2,
            min_y: 0,
            max_y: 2,
            min_z: 0,
            max_z: 0,
            min_w: 0,
            max_w: 0,
            use_w: true,
        };
        actual.step();
        actual.step();
        assert_eq!(expected, actual);
    }

    #[ignore]
    #[test]
    fn boots_correctly_in_4d() {
        let expected = 848;
        let actual = ConwayCubes {
            active: [
                (0, 1, 0, 0),
                (1, 2, 0, 0),
                (2, 0, 0, 0),
                (2, 1, 0, 0),
                (2, 2, 0, 0),
            ]
            .iter()
            .copied()
            .collect(),
            min_x: 0,
            max_x: 2,
            min_y: 0,
            max_y: 2,
            min_z: 0,
            max_z: 0,
            min_w: 0,
            max_w: 0,
            use_w: true,
        }
        .boot()
        .count_active();
        assert_eq!(expected, actual);
    }
}
