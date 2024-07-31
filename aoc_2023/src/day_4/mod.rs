use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use nom::{
    bytes::complete as bytes, character::complete as character, combinator, multi, sequence,
    IResult,
};

#[derive(Clone, Debug)]
struct Scratchcard {
    winning_numbers: Vec<u32>,
    available_numbers: Vec<u32>,
}

impl Scratchcard {
    fn nom_parse(s: &str) -> io::Result<Scratchcard> {
        fn number_list(s: &str) -> IResult<&str, Vec<u32>> {
            multi::many1(sequence::preceded(character::space1, character::u32))(s)
        }

        combinator::map(
            sequence::tuple((
                sequence::preceded(
                    sequence::tuple((
                        bytes::tag("Card"),
                        character::space1,
                        character::u32,
                        bytes::tag(":"),
                    )),
                    number_list,
                ),
                sequence::preceded(bytes::tag(" |"), number_list),
            )),
            |(winning_numbers, available_numbers)| Self {
                winning_numbers,
                available_numbers,
            },
        )(s)
        .map(|(_, card)| card)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
    }

    fn count_matches(&self) -> usize {
        self.available_numbers
            .iter()
            .copied()
            .filter(|number| self.winning_numbers.contains(number))
            .count()
    }
}

fn part1(input: &mut dyn BufRead) -> io::Result<u32> {
    input
        .lines()
        .map::<io::Result<_>, _>(|line| Ok(Scratchcard::nom_parse(&line?)?.count_matches()))
        .filter_map(|num_matches| match num_matches {
            Ok(0) => None,
            Ok(n) => Some(Ok(2u32.pow(n as u32 - 1))),
            Err(e) => Some(Err(e)),
        })
        .try_fold(0, |acc, elem| Ok(acc + elem?))
}

fn part2(input: &mut dyn BufRead) -> io::Result<u32> {
    let mut cards = input
        .lines()
        .map(|line| Scratchcard::nom_parse(&line?).map(|card| (1, card)))
        .collect::<io::Result<Vec<_>>>()?;
    for i in 0..cards.len() {
        let &(num_copies, ref card) = &cards[i];
        let num_matches = card.count_matches();
        for j in 1..=num_matches {
            cards[i + j].0 += num_copies;
        }
    }
    Ok(cards.into_iter().map(|(num_cards, _)| num_cards).sum())
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2023 Day 4 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2023_04.txt")?))?
        );
    }
    {
        println!("Year 2023 Day 4 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2023_04.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n",
        "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n",
        "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n",
        "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n",
        "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n",
        "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 13;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 30;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
