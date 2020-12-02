use crate::{parse::NomParse, util::Point};

use std::{io, str::FromStr};

use nom::{
    branch,
    character::complete as character,
    combinator as comb,
    multi,
    IResult,
};

struct RatioGenerator {
    last_set: Vec<(bool, Point<usize>)>,
    current_set: Vec<(bool, Point<usize>)>,
    max_x: usize,
    max_y: usize,
    has_next: bool,
}

impl RatioGenerator {
    fn new(max_x: usize, max_y: usize) -> Self {
        let last_set = vec![
            (false, Point::at(0, 1)),
            (false, Point::at(1, 0)),
        ];
        Self {
            last_set,
            current_set: vec![],
            max_x,
            max_y,
            has_next: max_x != 0 && max_y != 0,
        }
    }

    fn might_have_next(&mut self) -> bool {
        if !self.has_next {
            return false;
        }
        let current_len = self.current_set.len();
        while self.last_set.len() > 1 {
            if !self.last_set[0].0 && !self.last_set[1].0 {
                return true;
            } else {
                self.current_set.push(self.last_set.remove(0));
            }
        }
        // `self.last_set` has at most one element,
        self.last_set.pop().map(|x| self.current_set.push(x));
        // `self.last_set` is now empty.
        std::mem::swap(&mut self.current_set, &mut self.last_set);
        // `self.current_set` is now empty.
        for _ in 0..current_len {
            if self.last_set.len() > 0 && !self.last_set[0].0
                && !self.last_set[1].0
            {
                return true;
            } else {
                self.current_set.push(self.last_set.remove(0));
            }
        }
        self.current_set.extend(std::mem::replace(&mut self.last_set, vec![]));
        self.has_next = false;
        false
    }

    fn is_too_large(&self, ratio: Point<usize>) -> bool {
        ratio.x() > &self.max_x || ratio.y() > &self.max_y
    }

    fn into_sorted(mut self) -> Vec<Point<usize>> {
        while let Some(_) = self.next() {}
        self.current_set.into_iter().filter_map(|x| Some(x.1).filter(|_| !x.0))
            .collect()
    }
}

impl Iterator for RatioGenerator {
    type Item = Point<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.might_have_next() {
            // `last_head` and `last_next` must both be not too large, since
            // `self.might_have_next()` returned `true`.
            let (_, last_head) = self.last_set.remove(0);
            let (_, last_next) = self.last_set[0];
            self.current_set.push((false, last_head));

            let current_last = last_head + last_next;
            let cl_large = self.is_too_large(current_last);
            self.current_set.push((cl_large, current_last));

            if self.last_set.len() == 1 {
                self.current_set.push(self.last_set.remove(0));
                std::mem::swap(&mut self.last_set, &mut self.current_set);
            }
            if !cl_large {
                return Some(current_last)
            }
        }
        None
    }
}

struct Multiplier {
    base: Point<usize>,
    next: Option<Point<usize>>,
    max_x: usize,
    max_y: usize,
}

impl Multiplier {
    fn new(base: Point<usize>, max_x: usize, max_y: usize) -> Self {
        Self { base, next: Some(base), max_x, max_y }
    }

    fn is_too_large(&self, p: Point<usize>) -> bool {
        p.x() > &self.max_x || p.y() > &self.max_y
    }
}

impl Iterator for Multiplier {
    type Item = Point<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next.take() {
            Some(p) => {
                self.next = Some(p + self.base)
                    .filter(|&next| !self.is_too_large(next));
                Some(p)
            }
            None => None,
        }
    }
}

#[derive(Clone)]
struct AsteroidField {
    asteroids: Vec<Vec<bool>>,
}

impl AsteroidField {
    fn count(&self, row: usize, col: usize) -> Option<usize> {
        if row > self.asteroids.len() {
            return None;
        }
        let asteroid_row = &self.asteroids[row];
        if col > asteroid_row.len() {
            return None;
        }
        let left_space = col;
        let right_space = asteroid_row.len() - col - 1;
        let top_space = row;
        let bottom_space = self.asteroids.len() - row - 1;
        let upper_left_generator = RatioGenerator::new(left_space, top_space)
            .filter_map(|p| {
                Multiplier::new(p, left_space, top_space)
                    .filter_map(|p| {
                        Some(Point::at(col - p.x(), row - p.y()))
                            .filter(|&p| self.has_asteroid_at(p))
                    })
                    .next()
            });
        let left_generator = (1..=left_space)
            .filter_map(|n| {
                Some(Point::at(col - n, row))
                    .filter(|&p| self.has_asteroid_at(p))
            })
            .take(1);
        let lower_left_generator =
            RatioGenerator::new(left_space, bottom_space)
                .filter_map(|p| {
                    Multiplier::new(p, left_space, bottom_space)
                        .filter_map(|p| {
                            Some(Point::at(col - p.x(), row + p.y()))
                                .filter(|&p| self.has_asteroid_at(p))
                        })
                        .next()
                });
        let bottom_generator = (1..=bottom_space)
            .filter_map(|n| {
                Some(Point::at(col, row + n))
                    .filter(|&p| self.has_asteroid_at(p))
            })
            .take(1);
        let lower_right_generator =
            RatioGenerator::new(right_space, bottom_space)
                .filter_map(|p| {
                    Multiplier::new(p, right_space, bottom_space)
                        .filter_map(|p| {
                            Some(Point::at(col + p.x(), row + p.y()))
                                .filter(|&p| self.has_asteroid_at(p))
                        })
                        .next()
                });
        let right_generator = (1..=right_space)
            .filter_map(|n| {
                Some(Point::at(col + n, row))
                    .filter(|&p| self.has_asteroid_at(p))
            })
            .take(1);
        let upper_right_generator =
            RatioGenerator::new(right_space, top_space)
                .filter_map(|p| {
                    Multiplier::new(p, right_space, top_space)
                        .filter_map(|p| {
                            Some(Point::at(col + p.x(), row - p.y()))
                                .filter(|&p| self.has_asteroid_at(p))
                        })
                        .next()
                });
        let upper_generator = (1..=top_space)
            .filter_map(|n| {
                Some(Point::at(col, row - n))
                    .filter(|&p| self.has_asteroid_at(p))
            })
            .take(1);
        let directions = upper_left_generator
            .chain(left_generator)
            .chain(lower_left_generator)
            .chain(bottom_generator)
            .chain(lower_right_generator)
            .chain(right_generator)
            .chain(upper_right_generator)
            .chain(upper_generator);
        Some(directions.count())
    }

    fn has_asteroid_at(&self, p: Point<usize>) -> bool {
        self.asteroids[*p.y()][*p.x()]
    }
}

impl NomParse for AsteroidField {
    fn nom_parse(s: &str) -> IResult<&str, Self> {
        let asteroid = comb::value(true, character::char('#'));
        let blank = comb::value(false, character::char('.'));
        let row_parser = multi::many1(branch::alt((asteroid, blank)));
        let field_parser = multi::separated_list1(
            character::line_ending,
            row_parser,
        );
        comb::map(field_parser, |asteroids| Self { asteroids })(s)
    }
}

impl_from_str_for_nom_parse!(AsteroidField);

pub(super) fn run() -> io::Result<()> {
    let field = std::fs::read_to_string("2019_10.txt")?
        .parse::<AsteroidField>()
        .expect("Invalid asteroid field");
    let p = {
        println!("Year 2019 Day 10 Part 1");
        let mut most = 0;
        let mut most_coords = None;
        for row in 0..field.asteroids.len() {
            for col in 0..field.asteroids[row].len() {
                let count = field.count(col, row).unwrap();
                if count > most {
                    most = count;
                    most_coords = Some(Point::at(col, row));
                }
            }
        }
        println!(
            "{} asteroids is {:?}, which can see {} asteroids",
            "The location that can see the most",
            most_coords.unwrap(),
            most,
        );
        most_coords.unwrap()
    };
    {
        println!("Year 2019 Day 10 Part 2");
        let (&col, &row) = (p.x(), p.y());
        let left_space = col;
        let right_space = field.asteroids[row].len() - col - 1;
        let top_space = row;
        let bottom_space = field.asteroids.len() - row - 1;
        // let upper = (1..=top_space)
        //     .filter_map(|n| {
        //         Some(Point::at(col, row - n))
        //             .filter(|&p| field.has_asteroid_at(p))
        //     })
        //     .take(1)
        //     .collect::<Vec<_>>();
        let upper_right = RatioGenerator::new(right_space, top_space)
            .into_sorted()
            .into_iter()
            .filter_map(|p| {
                Multiplier::new(p, right_space, top_space)
                    .filter_map(|p| {
                        Some(Point::at(col + p.x(), row - p.y()))
                            .filter(|&p| field.has_asteroid_at(p))
                    })
                    .next()
            })
            .collect::<Vec<_>>();
        // let right = (1..=right_space)
        //     .filter_map(|n| {
        //         Some(Point::at(col + n, row))
        //             .filter(|&p| field.has_asteroid_at(p))
        //     })
        //     .take(1)
        //     .collect::<Vec<_>>();
        let lower_right = RatioGenerator::new(right_space, bottom_space)
            .into_sorted()
            .into_iter()
            .filter_map(|p| {
                Multiplier::new(p, right_space, bottom_space)
                    .filter_map(|p| {
                        Some(Point::at(col + p.x(), row + p.y()))
                            .filter(|&p| field.has_asteroid_at(p))
                    })
                    .next()
            })
            .collect::<Vec<_>>();
        // let bottom = (1..=bottom_space)
        //     .filter_map(|n| {
        //         Some(Point::at(col, row + n))
        //             .filter(|&p| field.has_asteroid_at(p))
        //     })
        //     .take(1)
        //     .collect::<Vec<_>>();
        let lower_left = RatioGenerator::new(left_space, bottom_space)
                .into_sorted()
                .into_iter()
                .filter_map(|p| {
                    Multiplier::new(p, left_space, bottom_space)
                        .filter_map(|p| {
                            Some(Point::at(col - p.x(), row + p.y()))
                                .filter(|&p| field.has_asteroid_at(p))
                        })
                        .next()
                })
                .collect::<Vec<_>>();
        // let left = (1..=left_space)
        //     .filter_map(|n| {
        //         Some(Point::at(col - n, row))
        //             .filter(|&p| field.has_asteroid_at(p))
        //     })
        //     .take(1)
        //     .collect::<Vec<_>>();
        let upper_left = RatioGenerator::new(left_space, top_space)
            .into_sorted()
            .into_iter()
            .filter_map(|p| {
                Multiplier::new(p, left_space, top_space)
                    .filter_map(|p| {
                        Some(Point::at(col - p.x(), row - p.y()))
                            .filter(|&p| field.has_asteroid_at(p))
                    })
                    .next()
            })
            .collect::<Vec<_>>();
        // This is missing 7 values, the cardinal direction iterators seem to
        // be doubled by the diagonal iterators, and I can't figure out why.
        // The correct answer on my input was at 184 instead of 200 and I got
        // it by checking only those values between the first two that I
        // checked which were by chance lower and upper bounds on the correct
        // answer.
        let mut asteroids = // upper.into_iter()
            // .chain(
                upper_right.into_iter()
            // )
            // .chain(right.into_iter())
            .chain(lower_right.into_iter())
            // .chain(bottom.into_iter())
            .chain(lower_left.into_iter())
            // .chain(left.into_iter())
            .chain(upper_left.into_iter());
        for i in 1..=282 {
            println!("Asteroid {}: {:?}", i, asteroids.next());
        }
        // println!("The 200th asteroid that will be destroyed is {:?}", p)
    }
    Ok(())
}
