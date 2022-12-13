use aoc_util::{impl_from_str_for_nom_parse, nom_parse::NomParse};
use nom::{
    branch, bytes::complete as bytes, character::complete as character, combinator as comb,
    error::ParseError, multi, sequence, AsChar, Finish, IResult, InputIter, InputLength, Offset,
    Slice,
};
use std::{
    collections::{HashMap, HashSet},
    fs, io, iter,
    ops::{RangeFrom, RangeTo},
};

#[derive(Clone, Debug, Eq, PartialEq)]
enum UnnamedRule {
    Literal(String),
    Branch(Box<[Self; 2]>),
    Sequence(Box<[Self]>),
    Proxy(RuleId),
}

impl UnnamedRule {
    fn length(
        &self,
        rules: &HashMap<RuleId, Rule>,
        lengths: &mut HashMap<RuleId, HashSet<usize>>,
        max_length: usize,
    ) -> HashSet<usize> {
        if max_length == 0 {
            HashSet::new()
        } else {
            match self {
                Self::Literal(s) => iter::once(s.len())
                    .filter(|&len| len <= max_length)
                    .collect::<HashSet<_>>(),
                Self::Branch(box [left, right]) => left
                    .length(rules, lengths, max_length)
                    .into_iter()
                    .chain(right.length(rules, lengths, max_length))
                    .collect(),
                Self::Sequence(box parts) => {
                    let mut res = [0].iter().copied().collect::<HashSet<_>>();
                    let mut min_consumed = 0;
                    for part in parts {
                        let part_length =
                            part.length(rules, lengths, max_length.saturating_sub(min_consumed));
                        if part_length.is_empty() {
                            return HashSet::new();
                        } else {
                            min_consumed += part_length.iter().min().unwrap();
                            res = res
                                .into_iter()
                                .flat_map(|old_length: usize| {
                                    part_length
                                        .iter()
                                        .filter_map(move |&length| old_length.checked_add(length))
                                        .filter(move |&length| length <= max_length)
                                })
                                .collect();
                        }
                    }
                    res
                }
                Self::Proxy(target) => rules[target]
                    .length(rules, lengths, max_length)
                    .into_iter()
                    .filter(|&length| length <= max_length)
                    .collect(),
            }
        }
    }

    fn matches(
        &self,
        s: &str,
        rules: &HashMap<RuleId, Rule>,
        lengths: &mut HashMap<RuleId, HashSet<usize>>,
    ) -> bool {
        match self {
            Self::Literal(literal) => s == literal,
            Self::Branch(box [left, right]) => {
                left.matches(s, rules, lengths) || right.matches(s, rules, lengths)
            }
            Self::Sequence(box parts) => {
                fn slice_matches(
                    parts: &[UnnamedRule],
                    s: &str,
                    rules: &HashMap<RuleId, Rule>,
                    lengths: &mut HashMap<RuleId, HashSet<usize>>,
                ) -> bool {
                    let max_length = s.len();
                    match parts {
                        [] => max_length == 0,
                        [first, ..] => {
                            for length in first.length(rules, lengths, max_length) {
                                if first.matches(&s[..length], rules, lengths)
                                    && slice_matches(&parts[1..], &s[length..], rules, lengths)
                                {
                                    return true;
                                }
                            }
                            false
                        }
                    }
                }

                slice_matches(parts, s, rules, lengths)
            }
            Self::Proxy(id) => rules[id].matches(s, rules, lengths),
        }
    }
}

impl_from_str_for_nom_parse!(UnnamedRule);

impl<'s> NomParse<'s, &'s str> for UnnamedRule {
    type Error = nom::error::Error<&'s str>;

    fn nom_parse(s: &'s str) -> IResult<&'s str, Self> {
        fn satisfy_many1<I, E>(f: impl Fn(char) -> bool) -> impl FnMut(I) -> IResult<I, I, E>
        where
            E: ParseError<I>,
            I: Slice<RangeFrom<usize>> + InputIter,
            <I as InputIter>::Item: AsChar,
            I: Clone + InputLength,
            I: Clone + Offset + Slice<RangeTo<usize>>,
        {
            comb::recognize(multi::many1(character::satisfy(f)))
        }

        comb::map(
            sequence::pair(
                comb::map(
                    multi::separated_list1(
                        bytes::tag(" "),
                        branch::alt((
                            comb::map(
                                sequence::delimited(
                                    bytes::tag(r#"""#),
                                    satisfy_many1(|c| !c.is_whitespace() && c != '"'),
                                    bytes::tag(r#"""#),
                                ),
                                |s: &str| Self::Literal(s.into()),
                            ),
                            comb::map(RuleId::nom_parse, Self::Proxy),
                        )),
                    ),
                    |rules| {
                        if rules.len() == 1 {
                            rules.into_iter().next().unwrap()
                        } else {
                            Self::Sequence(rules.into_boxed_slice())
                        }
                    },
                ),
                comb::opt(sequence::preceded(bytes::tag(" | "), Self::nom_parse)),
            ),
            |(first, second)| match second {
                None => first,
                Some(second) => Self::Branch(Box::new([first, second])),
            },
        )(s)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct RuleId(u32);

impl<'s> NomParse<'s, &'s str> for RuleId {
    type Error = nom::error::Error<&'s str>;

    fn nom_parse(s: &'s str) -> IResult<&'s str, Self> {
        comb::map(u32::nom_parse, Self)(s)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Rule {
    id: RuleId,
    inner: UnnamedRule,
}

impl Rule {
    fn length(
        &self,
        rules: &HashMap<RuleId, Rule>,
        lengths: &mut HashMap<RuleId, HashSet<usize>>,
        max_length: usize,
    ) -> HashSet<usize> {
        if let Some(lengths) = lengths.get(&self.id) {
            lengths.clone()
        } else {
            let res = self.inner.length(rules, lengths, max_length);
            lengths.insert(self.id, res.clone());
            res
        }
    }

    fn matches(
        &self,
        s: &str,
        rules: &HashMap<RuleId, Rule>,
        lengths: &mut HashMap<RuleId, HashSet<usize>>,
    ) -> bool {
        self.length(rules, lengths, s.len()).contains(&s.len())
            && self.inner.matches(s, rules, lengths)
    }
}

impl_from_str_for_nom_parse!(Rule);

impl<'s> NomParse<'s, &'s str> for Rule {
    type Error = nom::error::Error<&'s str>;

    fn nom_parse(s: &'s str) -> IResult<&'s str, Self> {
        comb::map(
            sequence::terminated(
                sequence::separated_pair(
                    RuleId::nom_parse,
                    bytes::tag(": "),
                    UnnamedRule::nom_parse,
                ),
                character::line_ending,
            ),
            |(id, inner)| Rule { id, inner },
        )(s)
    }
}

struct Rules(HashMap<RuleId, Rule>);

impl<'s> NomParse<'s, &'s str> for Rules {
    type Error = nom::error::Error<&'s str>;

    fn nom_parse(s: &'s str) -> IResult<&'s str, Self> {
        comb::map(multi::many1(Rule::nom_parse), |rules| {
            Self(rules.into_iter().map(|rule| (rule.id, rule)).collect())
        })(s)
    }
}

struct RulesAndStrings {
    rules: HashMap<RuleId, Rule>,
    strings: Vec<String>,
}

impl<'s> NomParse<'s, &'s str> for RulesAndStrings {
    type Error = nom::error::Error<&'s str>;

    fn nom_parse(s: &'s str) -> IResult<&'s str, Self> {
        comb::map(
            sequence::separated_pair(
                Rules::nom_parse,
                character::line_ending,
                comb::map(
                    multi::many1(sequence::terminated(
                        character::not_line_ending,
                        character::line_ending,
                    )),
                    |strings| strings.into_iter().map(String::from).collect(),
                ),
            ),
            |(rules, strings): (Rules, _)| Self {
                rules: rules.0,
                strings,
            },
        )(s)
    }
}

#[allow(unreachable_code)]
pub(super) fn run() -> io::Result<()> {
    fn build_lengths(
        rule_0: &Rule,
        rules: &HashMap<RuleId, Rule>,
        max_length: usize,
    ) -> HashMap<RuleId, HashSet<usize>> {
        let mut res = HashMap::new();
        rule_0.length(rules, &mut res, max_length);
        res
    }
    let RulesAndStrings { rules, strings } =
        <RulesAndStrings as NomParse<'_, _>>::nom_parse(&fs::read_to_string("2020_19.txt")?)
            .finish()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("{e:?}")))?
            .1;
    {
        println!("Year 2020 Day 19 Part 1");
        let rule_0 = &rules[&RuleId(0)];
        let mut lengths = build_lengths(
            rule_0,
            &rules,
            strings.iter().map(|s| s.len()).max().unwrap(),
        );
        let num_matches = strings
            .iter()
            .filter(|s| rule_0.matches(s, &rules, &mut lengths))
            .count();
        println!("There are {num_matches} strings that match rule 0");
    }
    {
        println!("Year 2020 Day 19 Part 2");
        let mut rules = rules;
        assert_eq!(
            rules.insert(
                RuleId(8),
                Rule {
                    id: RuleId(8),
                    inner: UnnamedRule::Branch(Box::new([
                        UnnamedRule::Proxy(RuleId(42)),
                        UnnamedRule::Sequence(Box::new([
                            UnnamedRule::Proxy(RuleId(42)),
                            UnnamedRule::Proxy(RuleId(8)),
                        ])),
                    ])),
                },
            ),
            Some(Rule {
                id: RuleId(8),
                inner: UnnamedRule::Proxy(RuleId(42)),
            }),
        );
        assert_eq!(
            rules.insert(
                RuleId(11),
                Rule {
                    id: RuleId(11),
                    inner: UnnamedRule::Branch(Box::new([
                        UnnamedRule::Sequence(Box::new([
                            UnnamedRule::Proxy(RuleId(42)),
                            UnnamedRule::Proxy(RuleId(31)),
                        ])),
                        UnnamedRule::Sequence(Box::new([
                            UnnamedRule::Proxy(RuleId(42)),
                            UnnamedRule::Proxy(RuleId(11)),
                            UnnamedRule::Proxy(RuleId(31)),
                        ])),
                    ])),
                },
            ),
            Some(Rule {
                id: RuleId(11),
                inner: UnnamedRule::Sequence(Box::new([
                    UnnamedRule::Proxy(RuleId(42)),
                    UnnamedRule::Proxy(RuleId(31)),
                ])),
            })
        );
        let rule_0 = &rules[&RuleId(0)];
        let mut lengths = build_lengths(
            rule_0,
            &rules,
            strings.iter().map(|s| s.len()).max().unwrap(),
        );
        let num_matches = strings
            .iter()
            .filter(|s| rule_0.matches(s, &rules, &mut lengths))
            .count();
        println!("There are {num_matches} strings that match rule 0");
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_advanced() -> (HashMap<RuleId, Rule>, Vec<String>) {
        let rules_str = concat!(
            "42: 9 14 | 10 1\n",
            "9: 14 27 | 1 26\n",
            "10: 23 14 | 28 1\n",
            "1: \"a\"\n",
            "11: 42 31\n",
            "5: 1 14 | 15 1\n",
            "19: 14 1 | 14 14\n",
            "12: 24 14 | 19 1\n",
            "16: 15 1 | 14 14\n",
            "31: 14 17 | 1 13\n",
            "6: 14 14 | 1 14\n",
            "2: 1 24 | 14 4\n",
            "0: 8 11\n",
            "13: 14 3 | 1 12\n",
            "15: 1 | 14\n",
            "17: 14 2 | 1 7\n",
            "23: 25 1 | 22 14\n",
            "28: 16 1\n",
            "4: 1 1\n",
            "20: 14 14 | 1 15\n",
            "3: 5 14 | 16 1\n",
            "27: 1 6 | 14 18\n",
            "14: \"b\"\n",
            "21: 14 1 | 1 14\n",
            "25: 1 1 | 1 14\n",
            "22: 14 14\n",
            "8: 42\n",
            "26: 14 22 | 1 20\n",
            "18: 15 15\n",
            "7: 14 5 | 1 21\n",
            "24: 14 1\n",
        );
        let strings = [
            "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa",
            "bbabbbbaabaabba",
            "babbbbaabbbbbabbbbbbaabaaabaaa",
            "aaabbbbbbaaaabaababaabababbabaaabbababababaaa",
            "bbbbbbbaaaabbbbaaabbabaaa",
            "bbbababbbbaaaaaaaabbababaaababaabab",
            "ababaaaaaabaaab",
            "ababaaaaabbbaba",
            "baabbaaaabbaaaababbaababb",
            "abbbbabbbbaaaababbbbbbaaaababb",
            "aaaaabbaabaaaaababaa",
            "aaaabbaaaabbaaa",
            "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa",
            "babaaabbbaaabaababbaabababaaab",
            "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba",
        ];
        (
            Rules::nom_parse(rules_str).unwrap().1 .0,
            strings.iter().copied().map(String::from).collect(),
        )
    }

    #[test]
    #[ignore]
    fn parses_branch() {
        let rule_str = "2 3 | 3 2";
        let expected = Ok(UnnamedRule::Branch(Box::new([
            UnnamedRule::Sequence(Box::new([
                UnnamedRule::Proxy(RuleId(2)),
                UnnamedRule::Proxy(RuleId(3)),
            ])),
            UnnamedRule::Sequence(Box::new([
                UnnamedRule::Proxy(RuleId(3)),
                UnnamedRule::Proxy(RuleId(2)),
            ])),
        ])));
        let actual = rule_str.parse::<UnnamedRule>();
        assert_eq!(expected, actual);
    }

    #[test]
    #[ignore]
    fn parses_rules() {
        let rule_str = concat!(
            "0: 4 1 5\n",
            "1: 2 3 | 3 2\n",
            "2: 4 4 | 5 5\n",
            "3: 4 5 | 5 4\n",
            "4: \"a\"\n",
            "5: \"b\"\n",
        );
        let expected = Ok([
            Rule {
                id: RuleId(0),
                inner: UnnamedRule::Sequence(Box::new([
                    UnnamedRule::Proxy(RuleId(4)),
                    UnnamedRule::Proxy(RuleId(1)),
                    UnnamedRule::Proxy(RuleId(5)),
                ])),
            },
            Rule {
                id: RuleId(1),
                inner: UnnamedRule::Branch(Box::new([
                    UnnamedRule::Sequence(Box::new([
                        UnnamedRule::Proxy(RuleId(2)),
                        UnnamedRule::Proxy(RuleId(3)),
                    ])),
                    UnnamedRule::Sequence(Box::new([
                        UnnamedRule::Proxy(RuleId(3)),
                        UnnamedRule::Proxy(RuleId(2)),
                    ])),
                ])),
            },
            Rule {
                id: RuleId(2),
                inner: UnnamedRule::Branch(Box::new([
                    UnnamedRule::Sequence(Box::new([
                        UnnamedRule::Proxy(RuleId(4)),
                        UnnamedRule::Proxy(RuleId(4)),
                    ])),
                    UnnamedRule::Sequence(Box::new([
                        UnnamedRule::Proxy(RuleId(5)),
                        UnnamedRule::Proxy(RuleId(5)),
                    ])),
                ])),
            },
            Rule {
                id: RuleId(3),
                inner: UnnamedRule::Branch(Box::new([
                    UnnamedRule::Sequence(Box::new([
                        UnnamedRule::Proxy(RuleId(4)),
                        UnnamedRule::Proxy(RuleId(5)),
                    ])),
                    UnnamedRule::Sequence(Box::new([
                        UnnamedRule::Proxy(RuleId(5)),
                        UnnamedRule::Proxy(RuleId(4)),
                    ])),
                ])),
            },
            Rule {
                id: RuleId(4),
                inner: UnnamedRule::Literal(String::from("a")),
            },
            Rule {
                id: RuleId(5),
                inner: UnnamedRule::Literal(String::from("b")),
            },
        ]
        .iter()
        .map(|rule| (rule.id, rule.clone()))
        .collect::<HashMap<_, _>>());
        let actual = Rules::nom_parse(rule_str).map(|(_, actual)| actual.0);
        assert_eq!(expected, actual);
    }

    #[test]
    #[ignore]
    fn finds_correct_matches_1() {
        let rules = [
            Rule {
                id: RuleId(0),
                inner: UnnamedRule::Sequence(Box::new([
                    UnnamedRule::Proxy(RuleId(4)),
                    UnnamedRule::Proxy(RuleId(1)),
                    UnnamedRule::Proxy(RuleId(5)),
                ])),
            },
            Rule {
                id: RuleId(1),
                inner: UnnamedRule::Branch(Box::new([
                    UnnamedRule::Sequence(Box::new([
                        UnnamedRule::Proxy(RuleId(2)),
                        UnnamedRule::Proxy(RuleId(3)),
                    ])),
                    UnnamedRule::Sequence(Box::new([
                        UnnamedRule::Proxy(RuleId(3)),
                        UnnamedRule::Proxy(RuleId(2)),
                    ])),
                ])),
            },
            Rule {
                id: RuleId(2),
                inner: UnnamedRule::Branch(Box::new([
                    UnnamedRule::Sequence(Box::new([
                        UnnamedRule::Proxy(RuleId(4)),
                        UnnamedRule::Proxy(RuleId(4)),
                    ])),
                    UnnamedRule::Sequence(Box::new([
                        UnnamedRule::Proxy(RuleId(5)),
                        UnnamedRule::Proxy(RuleId(5)),
                    ])),
                ])),
            },
            Rule {
                id: RuleId(3),
                inner: UnnamedRule::Branch(Box::new([
                    UnnamedRule::Sequence(Box::new([
                        UnnamedRule::Proxy(RuleId(4)),
                        UnnamedRule::Proxy(RuleId(5)),
                    ])),
                    UnnamedRule::Sequence(Box::new([
                        UnnamedRule::Proxy(RuleId(5)),
                        UnnamedRule::Proxy(RuleId(4)),
                    ])),
                ])),
            },
            Rule {
                id: RuleId(4),
                inner: UnnamedRule::Literal(String::from("a")),
            },
            Rule {
                id: RuleId(5),
                inner: UnnamedRule::Literal(String::from("b")),
            },
        ]
        .iter()
        .map(|rule| (rule.id, rule.clone()))
        .collect::<HashMap<_, _>>();
        let rule_0 = &rules[&RuleId(0)];
        let mut lengths = HashMap::new();
        let expected = vec!["ababbb", "abbbab"];
        let actual = ["ababbb", "bababa", "abbbab", "aaabbb", "aaaabbb"]
            .iter()
            .copied()
            .filter(|s| rule_0.matches(s, &rules, &mut lengths))
            .collect::<Vec<_>>();
        assert_eq!(expected, actual);
    }

    #[test]
    #[ignore]
    fn finds_correct_matches_2() {
        let (rules, strings) = get_advanced();
        let expected = ["bbabbbbaabaabba", "ababaaaaaabaaab", "ababaaaaabbbaba"]
            .iter()
            .copied()
            .map(String::from)
            .collect::<HashSet<_>>();
        let actual = strings
            .into_iter()
            .filter(|s| rules[&RuleId(0)].matches(s, &rules, &mut HashMap::new()))
            .collect::<HashSet<_>>();
        assert_eq!(expected, actual);
    }

    #[test]
    #[ignore]
    fn finds_correct_matches_with_loop() {
        let (mut rules, strings) = get_advanced();
        rules.insert(
            RuleId(8),
            Rule {
                id: RuleId(8),
                inner: UnnamedRule::Branch(Box::new([
                    UnnamedRule::Proxy(RuleId(42)),
                    UnnamedRule::Sequence(Box::new([
                        UnnamedRule::Proxy(RuleId(42)),
                        UnnamedRule::Proxy(RuleId(8)),
                    ])),
                ])),
            },
        );
        rules.insert(
            RuleId(11),
            Rule {
                id: RuleId(11),
                inner: UnnamedRule::Branch(Box::new([
                    UnnamedRule::Sequence(Box::new([
                        UnnamedRule::Proxy(RuleId(42)),
                        UnnamedRule::Proxy(RuleId(31)),
                    ])),
                    UnnamedRule::Sequence(Box::new([
                        UnnamedRule::Proxy(RuleId(42)),
                        UnnamedRule::Proxy(RuleId(11)),
                        UnnamedRule::Proxy(RuleId(31)),
                    ])),
                ])),
            },
        );
        let expected = [
            "bbabbbbaabaabba",
            "babbbbaabbbbbabbbbbbaabaaabaaa",
            "aaabbbbbbaaaabaababaabababbabaaabbababababaaa",
            "bbbbbbbaaaabbbbaaabbabaaa",
            "bbbababbbbaaaaaaaabbababaaababaabab",
            "ababaaaaaabaaab",
            "ababaaaaabbbaba",
            "baabbaaaabbaaaababbaababb",
            "abbbbabbbbaaaababbbbbbaaaababb",
            "aaaaabbaabaaaaababaa",
            "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa",
            "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba",
        ]
        .iter()
        .copied()
        .map(String::from)
        .collect::<HashSet<_>>();
        let actual = strings
            .into_iter()
            .filter(|s| rules[&RuleId(0)].matches(s, &rules, &mut HashMap::new()))
            .collect::<HashSet<_>>();
        assert_eq!(expected, actual);
    }
}
