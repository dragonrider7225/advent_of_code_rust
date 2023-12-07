use std::{
    cmp::Reverse,
    fs::File,
    io::{self, BufRead, BufReader},
};

use nom::{
    branch, bytes::complete as bytes, character::complete as character, combinator, sequence,
    IResult,
};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum CardWithoutJokers {
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    T,
    J,
    Q,
    K,
    A,
}

impl CardWithoutJokers {
    fn nom_parse(s: &str) -> IResult<&str, Self> {
        branch::alt((
            combinator::value(Self::_2, bytes::tag("2")),
            combinator::value(Self::_3, bytes::tag("3")),
            combinator::value(Self::_4, bytes::tag("4")),
            combinator::value(Self::_5, bytes::tag("5")),
            combinator::value(Self::_6, bytes::tag("6")),
            combinator::value(Self::_7, bytes::tag("7")),
            combinator::value(Self::_8, bytes::tag("8")),
            combinator::value(Self::_9, bytes::tag("9")),
            combinator::value(Self::T, bytes::tag("T")),
            combinator::value(Self::J, bytes::tag("J")),
            combinator::value(Self::Q, bytes::tag("Q")),
            combinator::value(Self::K, bytes::tag("K")),
            combinator::value(Self::A, bytes::tag("A")),
        ))(s)
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum CardWithJokers {
    J,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    T,
    Q,
    K,
    A,
}

impl CardWithJokers {
    fn nom_parse(s: &str) -> IResult<&str, Self> {
        branch::alt((
            combinator::value(Self::_2, bytes::tag("2")),
            combinator::value(Self::_3, bytes::tag("3")),
            combinator::value(Self::_4, bytes::tag("4")),
            combinator::value(Self::_5, bytes::tag("5")),
            combinator::value(Self::_6, bytes::tag("6")),
            combinator::value(Self::_7, bytes::tag("7")),
            combinator::value(Self::_8, bytes::tag("8")),
            combinator::value(Self::_9, bytes::tag("9")),
            combinator::value(Self::T, bytes::tag("T")),
            combinator::value(Self::J, bytes::tag("J")),
            combinator::value(Self::Q, bytes::tag("Q")),
            combinator::value(Self::K, bytes::tag("K")),
            combinator::value(Self::A, bytes::tag("A")),
        ))(s)
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn group_items<Item>(it: impl IntoIterator<Item = Item>) -> Vec<(Item, usize)>
where
    Item: Eq,
{
    it.into_iter().fold(vec![], |mut acc, card| {
        for (c, ref mut count) in acc.iter_mut() {
            if c == &card {
                *count += 1;
                return acc;
            }
        }
        acc.push((card, 1));
        acc
    })
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Hand<CardType>(HandType, [CardType; 5])
where
    CardType: Ord;

impl Hand<CardWithoutJokers> {
    fn nom_parse(s: &str) -> IResult<&str, Self> {
        combinator::map(
            combinator::map_parser(
                bytes::take(5usize),
                sequence::tuple((
                    CardWithoutJokers::nom_parse,
                    CardWithoutJokers::nom_parse,
                    CardWithoutJokers::nom_parse,
                    CardWithoutJokers::nom_parse,
                    CardWithoutJokers::nom_parse,
                )),
            ),
            |(a, b, c, d, e)| Self::new([a, b, c, d, e]),
        )(s)
    }

    fn new(cards: [CardWithoutJokers; 5]) -> Self {
        let mut groups = group_items(cards);
        groups.sort_by_key(|&(group_card, _)| Reverse(group_card));
        groups.sort_by_key(|&(_, group_size)| Reverse(group_size));
        let hand_type = match groups[..] {
            [(_c, 5)] => HandType::FiveOfAKind,
            [(_same, 4), (_different, 1)] => HandType::FourOfAKind,
            [(_major, 3), (_minor, 2)] => HandType::FullHouse,
            [(_same, 3), (_major, 1), (_minor, 1)] => HandType::ThreeOfAKind,
            [(_major, 2), (_minor, 2), (_other, 1)] => HandType::TwoPair,
            [(_same, 2), (_a, 1), (_b, 1), (_c, 1)] => HandType::OnePair,
            [(_a, 1), (_b, 1), (_c, 1), (_d, 1), (_e, 1)] => HandType::HighCard,
            _ => unreachable!("Invalid groups: {groups:?}"),
        };
        Self(hand_type, cards)
    }
}

impl Hand<CardWithJokers> {
    fn nom_parse(s: &str) -> IResult<&str, Self> {
        combinator::map(
            combinator::map_parser(
                bytes::take(5usize),
                sequence::tuple((
                    CardWithJokers::nom_parse,
                    CardWithJokers::nom_parse,
                    CardWithJokers::nom_parse,
                    CardWithJokers::nom_parse,
                    CardWithJokers::nom_parse,
                )),
            ),
            |(a, b, c, d, e)| Self::new([a, b, c, d, e]),
        )(s)
    }

    fn new(cards: [CardWithJokers; 5]) -> Self {
        let mut groups = group_items(cards);
        groups.sort_by_key(|&(group_card, _)| Reverse(group_card));
        groups.sort_by_key(|&(_, group_size)| Reverse(group_size));
        let hand_type = match groups[..] {
            [(_c, 5)]
            | [(CardWithJokers::J, 4), (_c, 1)]
            | [(CardWithJokers::J, 3), (_c, 2)]
            | [(_c, 3), (CardWithJokers::J, 2)]
            | [(_c, 4), (CardWithJokers::J, 1)] => HandType::FiveOfAKind,
            [(_same, 4), (_different, 1)]
            | [(CardWithJokers::J, 3), (_same, 1), (_different, 1)]
            | [(_same, 2), (CardWithJokers::J, 2), (_different, 1)]
            | [(_same, 3), (_different, 1), (CardWithJokers::J, 1)] => HandType::FourOfAKind,
            [(_major, 3), (_minor, 2)] | [(_major, 2), (_minor, 2), (CardWithJokers::J, 1)] => {
                HandType::FullHouse
            }
            [(_same, 3), (_major, 1), (_minor, 1)]
            | [(_same, 2), (_major, 1), (_minor, 1), (CardWithJokers::J, 1)]
            | [(CardWithJokers::J, 2), (_same, 1), (_major, 1), (_minor, 1)] => {
                HandType::ThreeOfAKind
            }
            [(_major, 2), (_minor, 2), (_other, 1)] => HandType::TwoPair,
            [(_same, 2), (_a, 1), (_b, 1), (_c, 1)]
            | [(_same, 1), (_a, 1), (_b, 1), (_c, 1), (CardWithJokers::J, 1)] => HandType::OnePair,
            [(_a, 1), (_b, 1), (_c, 1), (_d, 1), (_e, 1)] => HandType::HighCard,
            _ => unreachable!("Invalid groups: {groups:?}"),
        };
        Self(hand_type, cards)
    }
}

fn part1(input: &mut dyn BufRead) -> io::Result<u32> {
    let mut hands_and_bids = input
        .lines()
        .map(|line| {
            let line = line?;
            // Need to separate this out here because otherwise the borrow checker claims that
            // `line` doesn't live long enough even though the references in both `Ok` and `Err` are
            // removed in the same expression.
            let res = sequence::separated_pair(
                Hand::<CardWithoutJokers>::nom_parse,
                bytes::tag(" "),
                character::u32,
            )(&line);
            res.map(|(_, hands_and_bids)| hands_and_bids)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
        })
        .collect::<io::Result<Vec<_>>>()?;
    hands_and_bids.sort_by_key(|&(hand, _)| hand);
    Ok(hands_and_bids
        .into_iter()
        .map(|(_, bid)| bid)
        .enumerate()
        .map(|(idx, bid)| (idx + 1) as u32 * bid)
        .sum())
}

fn part2(input: &mut dyn BufRead) -> io::Result<u32> {
    let mut hands_and_bids = input
        .lines()
        .map(|line| {
            let line = line?;
            // Need to separate this out here because otherwise the borrow checker claims that
            // `line` doesn't live long enough even though the references in both `Ok` and `Err` are
            // removed in the same expression.
            let res = sequence::separated_pair(
                Hand::<CardWithJokers>::nom_parse,
                bytes::tag(" "),
                character::u32,
            )(&line);
            res.map(|(_, hands_and_bids)| hands_and_bids)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
        })
        .collect::<io::Result<Vec<_>>>()?;
    hands_and_bids.sort_by_key(|&(hand, _)| hand);
    Ok(hands_and_bids
        .into_iter()
        .map(|(_, bid)| bid)
        .enumerate()
        .map(|(idx, bid)| (idx + 1) as u32 * bid)
        .sum())
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2023 Day 7 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2023_07.txt")?))?
        );
    }
    {
        println!("Year 2023 Day 7 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2023_07.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        "32T3K 765\n",
        "T55J5 684\n",
        "KK677 28\n",
        "KTJJT 220\n",
        "QQQJA 483\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 6440;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 5905;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
