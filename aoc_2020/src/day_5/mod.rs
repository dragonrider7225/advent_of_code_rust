use aoc_util::nom_extended::NomParse;
use nom::{bytes::complete as bytes, combinator as comb, sequence, IResult};
use std::{
    cmp::Ordering,
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
struct Row(u8);

impl<'s> NomParse<&'s str> for Row {
    fn nom_parse(s: &'s str) -> IResult<&'s str, Self> {
        comb::map(
            comb::map_res(bytes::take(7usize), |s: &str| {
                s.chars().try_fold(0, |acc, c| match (acc, c) {
                    (acc, 'F') => Ok(acc * 2),
                    (acc, 'B') => Ok(acc * 2 + 1),
                    (_, c) => Err(format!("Invalid row character: {c:?}")),
                })
            }),
            Row,
        )(s)
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
struct Column(u8);

impl<'s> NomParse<&'s str> for Column {
    fn nom_parse(s: &'s str) -> IResult<&'s str, Self> {
        comb::map(
            comb::map_res(bytes::take(3usize), |s: &str| {
                s.chars().try_fold(0, |acc, c| match (acc, c) {
                    (acc, 'L') => Ok(acc * 2),
                    (acc, 'R') => Ok(acc * 2 + 1),
                    (_, c) => Err(format!("Invalid column character: {c:?}")),
                })
            }),
            Column,
        )(s)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Seat {
    row: Row,
    column: Column,
}

impl Seat {
    fn seat_id(&self) -> u32 {
        u32::from(self.row.0) * 8 + u32::from(self.column.0)
    }
}

impl PartialOrd for Seat {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for Seat {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.seat_id().cmp(&rhs.seat_id())
    }
}

impl<'s> NomParse<&'s str> for Seat {
    fn nom_parse(s: &'s str) -> IResult<&'s str, Self> {
        comb::map(
            sequence::pair(Row::nom_parse, Column::nom_parse),
            |(row, column)| Seat { row, column },
        )(s)
    }
}

aoc_util::impl_from_str_for_nom_parse!(Row Column Seat);

#[allow(unreachable_code)]
pub(super) fn run() -> io::Result<()> {
    let mut seats = BufReader::new(File::open("2020_05.txt")?)
        .lines()
        .map(|line| {
            line?
                .parse::<Seat>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        })
        .collect::<io::Result<Vec<_>>>()?;
    seats.sort();
    {
        println!("Year 2020 Day 5 Part 1");
        println!(
            "The highest seat ID is {}",
            seats.iter().last().unwrap().seat_id()
        );
    }
    {
        println!("Year 2020 Day 5 Part 2");
        let seat = seats
            .windows(2)
            .map(|window| match window {
                &[left, right] => [left.seat_id(), right.seat_id()],
                _ => unreachable!("Windows are of width 2"),
            })
            .find_map(|[left_seat, right_seat]| {
                Some(left_seat + 1).filter(|&seat| seat == right_seat - 1)
            })
            .expect("No pair of seats with exactly one seat between them");
        println!("The only empty seat is ID {seat}");
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[ignore]
    #[test]
    fn row_parses() {
        let expected = Ok(Row(44));
        let actual = "FBFBBFF".parse::<Row>();
        assert_eq!(expected, actual);
    }

    #[ignore]
    #[test]
    fn column_parses() {
        let expected = Ok(Column(5));
        let actual = "RLR".parse::<Column>();
        assert_eq!(expected, actual);
    }

    #[ignore]
    #[test]
    fn seat_parses() {
        let expected = Ok(Seat {
            row: Row(44),
            column: Column(5),
        });
        let actual = "FBFBBFFRLR".parse::<Seat>();
        assert_eq!(expected, actual);
    }
}
