use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use aoc_util::{
    geometry::Point2D,
    nom::{bytes::complete as bytes, character::complete as character, multi, IResult, Parser},
    nom_extended::NomParse,
    nom_supreme::ParserExt,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Machine {
    button_a: Point2D<i128>,
    button_b: Point2D<i128>,
    prize: Point2D<i128>,
}

impl Machine {
    fn min_cost(&self) -> Option<i128> {
        // minimize f(m, n) = 3*m + n where m*button_a + n*button_b = prize
        //  n == (prize.x() - m * button_a.x()) / button_b.x()
        //  n == (prize.y() - m * button_a.y()) / button_b.y()
        //
        // Single solution when
        //  (prize.x() - m * button_a.x()) / button_b.x() = (prize.y() - m * button_a.y()) /
        //  button_b.y()
        //  => (prize.x() - m * button_a.x()) * button_b.y() = (prize.y() - m * button_a.y()) *
        //  button_b.x()
        //  => m * (button_a.y() * button_b.x() - button_a.x() * button_b.y()) = prize.y() *
        //  button_b.x() - prize.x() * button_b.y()
        // doesn't reduce to 0 = 0
        let left = self.button_a.y() * self.button_b.x() - self.button_a.x() * self.button_b.y();
        let right = self.prize.y() * self.button_b.x() - self.prize.x() * self.button_b.y();
        if left == 0 && right == 0 {
            if 3 * self.button_b.x() <= *self.button_a.x() {
                Some(self.prize.x() / self.button_a.x())
            } else {
                Some(self.prize.x() / self.button_b.x())
            }
        } else if right == 0 {
            Some(self.prize.x() / self.button_b.x())
        } else if left == 0 || right % left != 0 {
            None
        } else if left.signum() != right.signum() {
            eprintln!("Machine requires negative button presses");
            None
        } else {
            let m = right / left;
            let n = (self.prize.x() - m * self.button_a.x()) / self.button_b.x();
            Some(3 * m + n)
        }
    }
}

impl NomParse<&str> for Machine {
    fn nom_parse(input: &str) -> IResult<&str, Self> {
        fn delta(input: &str) -> IResult<&str, Point2D<i128>> {
            character::i128
                .and(character::i128.preceded_by(bytes::tag(", Y+")))
                .preceded_by(bytes::tag("X+"))
                .map(|(x, y)| Point2D::at(x, y))
                .parse(input)
        }

        fn absolute(input: &str) -> IResult<&str, Point2D<i128>> {
            character::i128
                .and(character::i128.preceded_by(bytes::tag(", Y=")))
                .preceded_by(bytes::tag("X="))
                .map(|(x, y)| Point2D::at(x, y))
                .parse(input)
        }

        let button_a = delta
            .preceded_by(bytes::tag("Button A: "))
            .terminated(character::line_ending);
        let button_b = delta
            .preceded_by(bytes::tag("Button B: "))
            .terminated(character::line_ending);
        let prize = absolute
            .preceded_by(bytes::tag("Prize: "))
            .terminated(character::line_ending);
        button_a
            .and(button_b)
            .and(prize)
            .map(|((button_a, button_b), prize)| Self {
                button_a,
                button_b,
                prize,
            })
            .parse(input)
    }
}

fn part1(input: &mut dyn BufRead) -> io::Result<i128> {
    let input = io::read_to_string(input)?;
    let machines = multi::separated_list1(character::line_ending, Machine::nom_parse)
        .all_consuming()
        .parse(&input)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?
        .1;
    Ok(machines
        .into_iter()
        .flat_map(|machine| machine.min_cost())
        .sum())
}

fn part2(input: &mut dyn BufRead) -> io::Result<i128> {
    let input = io::read_to_string(input)?;
    let machines = multi::separated_list1(character::line_ending, Machine::nom_parse)
        .all_consuming()
        .parse(&input)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?
        .1;
    Ok(machines
        .into_iter()
        .flat_map(|mut machine| {
            machine.prize += Point2D::at(10_000_000_000_000, 10_000_000_000_000);
            machine.min_cost()
        })
        .sum())
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2024 Day 13 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2024_13.txt")?))?
        );
    }
    {
        println!("Year 2024 Day 13 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2024_13.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        "Button A: X+94, Y+34\n",
        "Button B: X+22, Y+67\n",
        "Prize: X=8400, Y=5400\n",
        "\n",
        "Button A: X+26, Y+66\n",
        "Button B: X+67, Y+21\n",
        "Prize: X=12748, Y=12176\n",
        "\n",
        "Button A: X+17, Y+86\n",
        "Button B: X+84, Y+37\n",
        "Prize: X=7870, Y=6450\n",
        "\n",
        "Button A: X+69, Y+23\n",
        "Button B: X+27, Y+71\n",
        "Prize: X=18641, Y=10279\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 480;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 875_318_608_908;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
