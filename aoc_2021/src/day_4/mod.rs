use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    iter,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Space<N> {
    number: N,
    marked: bool,
}

impl<N> Space<N> {
    fn marked(&self) -> bool {
        self.marked
    }

    fn unmarked(&self) -> bool {
        !self.marked()
    }

    fn mark(&mut self) {
        self.marked = true;
    }
}

impl<N> PartialEq<N> for Space<N>
where
    N: PartialEq,
{
    fn eq(&self, other: &N) -> bool {
        &self.number == other
    }
}

struct BingoCard {
    numbers: [[Space<u32>; 5]; 5],
}

impl BingoCard {
    /// Marks `number` if it is on the card and returns whether the card now has a bingo involving
    /// `number`.
    fn mark_number(&mut self, number: u32) -> bool {
        let (i, j) = match (0..5)
            .flat_map(|i| iter::repeat(i).zip(0..5))
            .find(|&(i, j)| self.numbers[i][j] == number)
        {
            Some(coords) => coords,
            None => return false,
        };
        self.numbers[i][j].mark();
        (0..5).all(|j| self.numbers[i][j].marked()) || (0..5).all(|i| self.numbers[i][j].marked())
    }

    /// Gets all numbers on the card that are unmarked.
    fn unmarked_numbers(&self) -> Vec<u32> {
        self.numbers
            .iter()
            .flat_map(|row| row.into_iter())
            .filter_map(|space| Some(space.number).filter(|_| space.unmarked()))
            .collect()
    }
}

impl BingoCard {
    fn from_lines(mut lines: impl Iterator<Item = io::Result<String>>) -> io::Result<Self> {
        let mut numbers = [[Space {
            number: 0,
            marked: false,
        }; 5]; 5];
        for i in 0..5 {
            let line = lines.next().ok_or(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Missing card line {}/5", i + 1),
            ))??;
            for (j, number) in (0..).zip(line.split_whitespace()) {
                if j >= 5 {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Got too many numbers on line {}", i),
                    ))?;
                }
                numbers[i][j].number = number.parse().map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!(
                            "Invalid number {:?} at row {} column {}: {}",
                            number, i, j, e
                        ),
                    )
                })?;
            }
        }
        Ok(Self { numbers })
    }

    fn read_cards(mut lines: impl Iterator<Item = io::Result<String>>) -> io::Result<Vec<Self>> {
        let mut cards = vec![];
        while let Some(line) = lines.next() {
            let line = line?;
            if line != "" {
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Card {} is too tall", cards.len() + 1),
                ))?;
            }
            cards.push(BingoCard::from_lines(lines.by_ref().take(5))?);
        }
        Ok(cards)
    }
}

fn part1(input: &mut dyn BufRead) -> io::Result<u32> {
    let mut lines = input.lines();
    let numbers = lines.next().ok_or(io::Error::new(
        io::ErrorKind::InvalidData,
        "Missing number line",
    ))??;
    let mut cards = BingoCard::read_cards(lines)?;
    for number in numbers.split(',') {
        let number = number.parse().map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid drawn number {:?}: {}", number, e),
            )
        })?;
        for card in cards.iter_mut() {
            if card.mark_number(number) {
                let unmarked_numbers = card.unmarked_numbers();
                return Ok(number * unmarked_numbers.into_iter().sum::<u32>());
            }
        }
    }
    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "Ran out of numbers",
    ))
}

fn part2(input: &mut dyn BufRead) -> io::Result<u32> {
    let mut lines = input.lines();
    let numbers = lines.next().ok_or(io::Error::new(
        io::ErrorKind::InvalidData,
        "Missing number line",
    ))??;
    let mut cards = BingoCard::read_cards(lines)?;
    let mut done_cards = vec![];
    for number in numbers.split(',') {
        let number = number.parse().map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid drawn number {:?}: {}", number, e),
            )
        })?;
        for card_idx in 0..cards.len() {
            if cards[card_idx].mark_number(number) {
                if cards.len() == 1 {
                    let unmarked_numbers = cards[card_idx].unmarked_numbers();
                    return Ok(number * unmarked_numbers.into_iter().sum::<u32>());
                } else {
                    done_cards.push(card_idx);
                }
            }
        }
        for done_card in done_cards.drain(..).rev() {
            cards.remove(done_card);
        }
    }
    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        format!("Ran out of numbers with {} cards remaining", cards.len()),
    ))
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2021 Day 4 Part 1");
        println!(
            "The final score is {}",
            part1(&mut BufReader::new(File::open("2021_04.txt")?))?
        );
    }
    {
        println!("Year 2021 Day 4 Part 2");
        println!(
            "The final FINAL score is {}",
            part2(&mut BufReader::new(File::open("2021_04.txt")?))?,
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    #[ignore]
    fn test_part1() -> io::Result<()> {
        let s = r"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11 0
 8 2 23 4 24
21 9 14 16 7
 6 10 3 18 5
 1 12 20 15 19

 3 15 0 2 22
 9 18 13 17 5
19 8 7 25 23
20 11 10 24 4
14 21 16 12 6

14 21 17 24 4
10 16 15 9 19
18 8 23 26 20
22 11 13 6 5
 2 0 12 3 7
";
        let expected = 4512;
        let actual = part1(&mut Cursor::new(s))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part2() -> io::Result<()> {
        let s = r"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11 0
 8 2 23 4 24
21 9 14 16 7
 6 10 3 18 5
 1 12 20 15 19

 3 15 0 2 22
 9 18 13 17 5
19 8 7 25 23
20 11 10 24 4
14 21 16 12 6

14 21 17 24 4
10 16 15 9 19
18 8 23 26 20
22 11 13 6 5
 2 0 12 3 7
";
        let expected = 1924;
        let actual = part2(&mut Cursor::new(s))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
