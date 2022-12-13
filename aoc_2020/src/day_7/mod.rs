use aoc_util::nom_parse::NomParse;
use nom::{
    branch, bytes::complete as bytes, character::complete as character, combinator as comb, multi,
    sequence, Finish, IResult,
};
use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Display, Formatter},
    fs, io,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct BagColor<'color>(&'color str);

impl<'color> Display for BagColor<'color> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'s> NomParse<'s, &'s str> for BagColor<'s> {
    type Error = nom::error::Error<&'s str>;

    fn nom_parse(s: &'s str) -> IResult<&'s str, Self> {
        comb::map(
            comb::recognize(multi::separated_list1(
                bytes::tag(" "),
                comb::verify(character::alpha1, |s| !matches!(s, "bag" | "bags")),
            )),
            Self,
        )(s)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct BagRule<'color> {
    contents: HashMap<BagColor<'color>, usize>,
}

impl<'color> BagRule<'color> {
    fn contains(&self, inner: BagColor<'_>) -> bool {
        self.contents.contains_key(&inner)
    }
}

impl<'color, 's> NomParse<'s, &'s str> for BagRule<'color>
where
    's: 'color,
{
    type Error = nom::error::Error<&'s str>;

    fn nom_parse(s: &'s str) -> IResult<&'s str, Self> {
        comb::map(
            branch::alt((
                comb::value(vec![], bytes::tag("no other bags")),
                multi::separated_list1(
                    bytes::tag(", "),
                    branch::alt((
                        comb::map(
                            sequence::delimited(
                                bytes::tag("1 "),
                                BagColor::nom_parse,
                                bytes::tag(" bag"),
                            ),
                            |bag_color| (bag_color, 1),
                        ),
                        comb::map(
                            sequence::separated_pair(
                                usize::nom_parse,
                                bytes::tag(" "),
                                sequence::terminated(BagColor::nom_parse, bytes::tag(" bags")),
                            ),
                            |(count, bag_color)| (bag_color, count),
                        ),
                    )),
                ),
            )),
            |contents| Self {
                contents: contents.into_iter().collect(),
            },
        )(s)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct BagRules<'color>(HashMap<BagColor<'color>, BagRule<'color>>);

impl<'color> BagRules<'color> {
    /// Counts the number of colors of bag that can contain (directly or indirectly) a bag of color
    /// `inner_color` according to the set of rules.
    fn get_wrapper_types<'this>(
        &'this self,
        inner_color: BagColor<'this>,
    ) -> HashSet<BagColor<'color>>
    where
        'color: 'this,
    {
        let mut remaining_colors = self.0.keys().collect::<HashSet<_>>();
        let mut contained = [inner_color].iter().copied().collect::<HashSet<_>>();
        let mut res = HashSet::new();
        while !contained.is_empty() {
            let mut next_contained = HashSet::new();
            for color in contained.into_iter() {
                let (containing, not_containing) = remaining_colors
                    .into_iter()
                    .partition(|remaining_color| self.0[remaining_color].contains(color));
                remaining_colors = not_containing;
                next_contained.extend(containing);
            }
            res.extend(next_contained.iter().copied());
            contained = next_contained;
        }
        res
    }

    fn requires_contained(&self, outer_color: BagColor<'_>) -> usize {
        fn delegate<'color>(
            this: &BagRules<'color>,
            outer_color: BagColor<'color>,
            memoizer: &mut HashMap<BagColor<'color>, usize>,
        ) -> usize {
            if let Some(&count) = memoizer.get(&outer_color) {
                count
            } else {
                let mut total = 0;
                for (&color, &count) in this.0[&outer_color].contents.iter() {
                    total += count * (1 + delegate(this, color, memoizer));
                }
                memoizer.insert(outer_color, total);
                total
            }
        }

        delegate(self, outer_color, &mut HashMap::new())
    }
}

impl<'color, 's> NomParse<'s, &'s str> for BagRules<'color>
where
    's: 'color,
{
    type Error = nom::error::Error<&'s str>;

    fn nom_parse(s: &'s str) -> IResult<&'s str, Self> {
        comb::map(
            multi::many0(sequence::terminated(
                sequence::separated_pair(
                    BagColor::nom_parse,
                    bytes::tag(" bags contain "),
                    sequence::terminated(BagRule::nom_parse, bytes::tag(".")),
                ),
                character::line_ending,
            )),
            |rules| Self(rules.into_iter().collect()),
        )(s)
    }
}

pub(super) fn run() -> io::Result<()> {
    let file_contents = fs::read_to_string("2020_07.txt")?;
    let bag_rules = BagRules::nom_parse(&file_contents)
        .finish()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("{e:?}")))?
        .1;
    {
        println!("Year 2020 Day 7 Part 1");
        let inner = "shiny gold";
        println!(
            "There are {} types of bags that can contain a {} bag at some level of nesting",
            bag_rules.get_wrapper_types(BagColor(inner)).len(),
            inner,
        );
    }
    {
        println!("Year 2020 Day 7 Part 2");
        let outer = "shiny gold";
        println!(
            "A {} bag must contain {} distinct bags at some level of nesting",
            outer,
            bag_rules.requires_contained(BagColor(outer)),
        );
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[ignore]
    #[test]
    fn bag_rule_parses1() {
        let expected = Ok(BagRule {
            contents: [(BagColor("bright white"), 1), (BagColor("muted yellow"), 2)]
                .iter()
                .copied()
                .collect(),
        });
        let actual = BagRule::nom_parse("1 bright white bag, 2 muted yellow bags").map(|res| res.1);
        assert_eq!(expected, actual);
    }

    #[ignore]
    #[test]
    fn bag_rule_parses2() {
        let expected = Ok(BagRule {
            contents: HashMap::new(),
        });
        let actual = BagRule::nom_parse("no other bags").map(|res| res.1);
        assert_eq!(expected, actual);
    }

    #[ignore]
    #[test]
    fn bag_rules_parses() {
        let expected = Ok(BagRules(
            [
                (
                    BagColor("light red"),
                    BagRule {
                        contents: [(BagColor("bright white"), 1), (BagColor("muted yellow"), 2)]
                            .iter()
                            .copied()
                            .collect(),
                    },
                ),
                (
                    BagColor("dark orange"),
                    BagRule {
                        contents: [(BagColor("bright white"), 3), (BagColor("muted yellow"), 4)]
                            .iter()
                            .copied()
                            .collect(),
                    },
                ),
                (
                    BagColor("bright white"),
                    BagRule {
                        contents: [(BagColor("shiny gold"), 1)].iter().copied().collect(),
                    },
                ),
                (
                    BagColor("muted yellow"),
                    BagRule {
                        contents: [(BagColor("shiny gold"), 2), (BagColor("faded blue"), 9)]
                            .iter()
                            .copied()
                            .collect(),
                    },
                ),
                (
                    BagColor("shiny gold"),
                    BagRule {
                        contents: [(BagColor("dark olive"), 1), (BagColor("vibrant plum"), 2)]
                            .iter()
                            .copied()
                            .collect(),
                    },
                ),
                (
                    BagColor("dark olive"),
                    BagRule {
                        contents: [(BagColor("faded blue"), 3), (BagColor("dotted black"), 4)]
                            .iter()
                            .copied()
                            .collect(),
                    },
                ),
                (
                    BagColor("vibrant plum"),
                    BagRule {
                        contents: [(BagColor("faded blue"), 5), (BagColor("dotted black"), 6)]
                            .iter()
                            .copied()
                            .collect(),
                    },
                ),
                (
                    BagColor("faded blue"),
                    BagRule {
                        contents: HashMap::new(),
                    },
                ),
                (
                    BagColor("dotted black"),
                    BagRule {
                        contents: HashMap::new(),
                    },
                ),
            ]
            .iter()
            .cloned()
            .collect(),
        ));
        let actual = BagRules::nom_parse(concat!(
            "light red bags contain 1 bright white bag, 2 muted yellow bags.\n",
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.\n",
            "bright white bags contain 1 shiny gold bag.\n",
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\n",
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\n",
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.\n",
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\n",
            "faded blue bags contain no other bags.\n",
            "dotted black bags contain no other bags.\n",
        ))
        .map(|res| res.1);
        assert_eq!(expected, actual);
    }
}
