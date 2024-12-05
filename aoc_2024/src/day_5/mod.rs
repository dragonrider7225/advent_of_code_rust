use std::{
    cmp::Ordering,
    fs::File,
    io::{self, read_to_string, BufRead, BufReader},
    ops::Deref,
};

use aoc_util::{
    nom::{bytes::complete as bytes, character::complete as character, multi, IResult, Parser},
    nom_extended::NomParse,
    nom_supreme::ParserExt,
};

type PageNumber = u32;

fn parse_page_number(s: &str) -> IResult<&str, PageNumber> {
    character::u32.parse(s)
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PageOrderingRule {
    first: PageNumber,
    second: PageNumber,
}

impl PageOrderingRule {
    fn parse_many(s: &str) -> IResult<&str, Vec<Self>> {
        multi::many1(Self::nom_parse.terminated(character::line_ending))(s)
    }
}

impl NomParse<&str> for PageOrderingRule {
    fn nom_parse(input: &str) -> IResult<&str, Self> {
        parse_page_number
            .and(parse_page_number.preceded_by(bytes::tag("|")))
            .map(|(first, second)| Self { first, second })
            .parse(input)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PagesToProduce {
    pages: Vec<PageNumber>,
}

impl PagesToProduce {
    fn parse_many(s: &str) -> IResult<&str, Vec<Self>> {
        multi::many1(Self::nom_parse.terminated(character::line_ending))(s)
    }

    fn is_valid_order(&self, rules: &[PageOrderingRule]) -> bool {
        (0..self.len()).all(|i| {
            let page = self[i];
            self[..i].iter().copied().all(|predecessor| {
                !rules.contains(&PageOrderingRule {
                    first: page,
                    second: predecessor,
                })
            })
        })
    }

    fn reorder(&mut self, rules: &[PageOrderingRule]) {
        self.pages.sort_unstable_by(|&left, &right| {
            if rules.contains(&PageOrderingRule {
                first: right,
                second: left,
            }) {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });
    }

    fn center_page(&self) -> u32 {
        self[(self.len() - 1) / 2]
    }
}

impl Deref for PagesToProduce {
    type Target = [PageNumber];

    fn deref(&self) -> &Self::Target {
        &self.pages
    }
}

impl NomParse<&str> for PagesToProduce {
    fn nom_parse(input: &str) -> IResult<&str, Self> {
        multi::separated_list1(bytes::tag(","), parse_page_number)
            .map(|pages| Self { pages })
            .parse(input)
    }
}

fn parse_input(s: &str) -> Result<(Vec<PageOrderingRule>, Vec<PagesToProduce>), String> {
    PageOrderingRule::parse_many
        .and(PagesToProduce::parse_many.preceded_by(character::line_ending))
        .complete()
        .all_consuming()
        .parse(s)
        .map(|(_, ret)| ret)
        .map_err(|e| e.to_string())
}

fn part1(input: &mut dyn BufRead) -> io::Result<u32> {
    let input = read_to_string(input)?;
    let (rules, steps) =
        parse_input(&input).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(steps
        .into_iter()
        .filter(|step| step.is_valid_order(&rules))
        .map(|step| step.center_page())
        .sum())
}

fn part2(input: &mut dyn BufRead) -> io::Result<u32> {
    let input = read_to_string(input)?;
    let (rules, steps) =
        parse_input(&input).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(steps
        .into_iter()
        .filter(|step| !step.is_valid_order(&rules))
        .map(|mut step| {
            step.reorder(&rules);
            step.center_page()
        })
        .sum())
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2024 Day 5 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2024_05.txt")?))?
        );
    }
    {
        println!("Year 2024 Day 5 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2024_05.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        "47|53\n",
        "97|13\n",
        "97|61\n",
        "97|47\n",
        "75|29\n",
        "61|13\n",
        "75|53\n",
        "29|13\n",
        "97|29\n",
        "53|29\n",
        "61|53\n",
        "97|53\n",
        "61|29\n",
        "47|13\n",
        "75|47\n",
        "97|75\n",
        "47|61\n",
        "75|61\n",
        "47|29\n",
        "75|13\n",
        "53|13\n",
        "\n",
        "75,47,61,53,29\n",
        "97,61,53,29,13\n",
        "75,29,13\n",
        "75,97,47,61,53\n",
        "61,13,29\n",
        "97,13,75,29,47\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 143;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 123;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
