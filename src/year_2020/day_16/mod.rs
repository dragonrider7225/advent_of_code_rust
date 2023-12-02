use crate::parse::NomParse;
use nom::{
    bytes::complete as bytes, character::complete as character, combinator as comb, multi,
    sequence, Finish, IResult,
};
use std::{
    collections::{HashMap, HashSet},
    fs, io,
    ops::RangeInclusive,
};

#[derive(Clone, Debug, Eq, PartialEq)]
struct FieldRule(RangeInclusive<u64>, RangeInclusive<u64>);

impl FieldRule {
    fn is_satisfied_by(&self, value: u64) -> bool {
        self.0.contains(&value) || self.1.contains(&value)
    }
}

impl<'s> NomParse<'s> for FieldRule {
    fn nom_parse(s: &str) -> IResult<&str, Self> {
        fn range(s: &str) -> IResult<&str, RangeInclusive<u64>> {
            comb::map(
                sequence::separated_pair(u64::nom_parse, bytes::tag("-"), u64::nom_parse),
                |(start, end)| start..=end,
            )(s)
        }

        comb::map(
            sequence::separated_pair(range, bytes::tag(" or "), range),
            |(first, second)| Self(first, second),
        )(s)
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct TicketRules<'field> {
    rules: HashMap<&'field str, FieldRule>,
}

impl<'field> TicketRules<'field> {
    fn error(&self, ticket: &Ticket) -> u64 {
        ticket
            .fields
            .iter()
            .copied()
            .filter(|&field| self.rules.values().all(|rule| !rule.is_satisfied_by(field)))
            .sum()
    }

    fn find_fields(&self, tickets: &[Ticket]) -> HashMap<&'field str, usize> {
        let mut intermediate = HashMap::<&'field str, HashSet<usize>>::new();
        let mut result = HashMap::new();
        let full_range = (0..tickets[0].num_fields()).collect::<HashSet<_>>();
        for &field in self.rules.keys() {
            intermediate.insert(field, full_range.clone());
        }
        for ticket in tickets {
            intermediate.iter_mut().for_each(|(&field, indices)| {
                indices.retain(|&idx| {
                    let ret = self.rules[&field].is_satisfied_by(ticket.fields[idx]);
                    if !ret {
                        #[cfg(test)]
                        println!(
                            "Removing index {idx} for field {field:?} because it is not satisfied by {ticket:?}",
                        );
                    }
                    ret
                });
            });
        }
        while !intermediate.is_empty() {
            let singletons = intermediate
                .extract_if(|_, indices| indices.len() == 1)
                .map(|(field, indices)| (field, indices.into_iter().next().unwrap()))
                .collect::<Vec<_>>();
            assert_ne!(
                0,
                singletons.len(),
                "Ran out of singleton fields with {} fields left: {:?}",
                intermediate.len(),
                intermediate,
            );
            for (field, index) in singletons {
                intermediate.iter_mut().for_each(|(&_edit_field, indices)| {
                    if indices.remove(&index) {
                        #[cfg(test)]
                        println!(
                            "Removing index {index} for field {_edit_field:?} because it has been assigned to {field:?}",
                        );
                    }
                });
                result.insert(field, index);
            }
        }
        result
    }
}

impl<'field> NomParse<'field> for TicketRules<'field> {
    fn nom_parse(s: &'field str) -> IResult<&'field str, Self> {
        comb::map(
            multi::many0(sequence::pair(
                sequence::terminated(
                    comb::recognize(multi::many1(character::none_of(":"))),
                    bytes::tag(": "),
                ),
                sequence::terminated(FieldRule::nom_parse, character::line_ending),
            )),
            |rules| Self {
                rules: rules.into_iter().collect(),
            },
        )(s)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Ticket {
    fields: Vec<u64>,
}

impl Ticket {
    fn num_fields(&self) -> usize {
        self.fields.len()
    }
}

impl<'s> NomParse<'s> for Ticket {
    fn nom_parse(s: &str) -> IResult<&str, Self> {
        comb::map(
            sequence::terminated(
                multi::separated_list0(bytes::tag(","), u64::nom_parse),
                character::line_ending,
            ),
            |fields| Self { fields },
        )(s)
    }
}

fn parse_rules_and_tickets(s: &str) -> IResult<&str, (TicketRules<'_>, (Ticket, Vec<Ticket>))> {
    sequence::separated_pair(
        TicketRules::<'_>::nom_parse,
        character::line_ending,
        sequence::separated_pair(
            sequence::preceded(
                sequence::pair(bytes::tag("your ticket:"), character::line_ending),
                Ticket::nom_parse,
            ),
            character::line_ending,
            sequence::preceded(
                sequence::pair(bytes::tag("nearby tickets:"), character::line_ending),
                multi::many0(Ticket::nom_parse),
            ),
        ),
    )(s)
}

fn error_rate(tickets: &[Ticket], rules: &TicketRules<'_>) -> u64 {
    tickets
        .iter()
        .map(|ticket| rules.error(ticket))
        .sum::<u64>()
}

pub(super) fn run() -> io::Result<()> {
    let file_contents = fs::read_to_string("2020_16.txt")?;
    let (rules, (my_ticket, nearby_tickets)) = parse_rules_and_tickets(&file_contents)
        .finish()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("{e}")))?
        .1;
    {
        println!("Year 2020 Day 16 Part 1");
        println!(
            "The ticket-scanning error rate is {}",
            error_rate(&nearby_tickets, &rules)
        );
    }
    {
        println!("Year 2020 Day 16 Part 2");
        let nearby_tickets = nearby_tickets
            .into_iter()
            .filter(|ticket| rules.error(ticket) == 0)
            .collect::<Vec<_>>();
        let named_fields = rules.find_fields(&nearby_tickets);
        let result = named_fields
            .into_iter()
            .filter(|&(field, _)| field.starts_with("departure"))
            .map(|(_, idx)| my_ticket.fields[idx])
            .product::<u64>();
        println!("The product of the six departure fields is {result}");
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[ignore]
    #[test]
    fn ticket_rules_parses() {
        let expected = Ok(TicketRules {
            rules: [
                ("class", FieldRule(1..=3, 5..=7)),
                ("row", FieldRule(6..=11, 33..=44)),
                ("seat", FieldRule(13..=40, 45..=50)),
            ]
            .iter()
            .cloned()
            .collect(),
        });
        let actual = TicketRules::nom_parse(concat!(
            "class: 1-3 or 5-7\n",
            "row: 6-11 or 33-44\n",
            "seat: 13-40 or 45-50\n",
        ))
        .map(|(_, res)| res);
        assert_eq!(expected, actual);
    }

    #[ignore]
    #[test]
    fn parses_ticket_rules_and_tickets() {
        let notes = concat!(
            "class: 1-3 or 5-7\n",
            "row: 6-11 or 33-44\n",
            "seat: 13-40 or 45-50\n",
            "\n",
            "your ticket:\n",
            "7,1,14\n",
            "\n",
            "nearby tickets:\n",
            "7,3,47\n",
            "40,4,50\n",
            "55,2,20\n",
            "38,6,12\n",
        );
        let expected = Ok((
            TicketRules {
                rules: [
                    ("class", FieldRule(1..=3, 5..=7)),
                    ("row", FieldRule(6..=11, 33..=44)),
                    ("seat", FieldRule(13..=40, 45..=50)),
                ]
                .iter()
                .cloned()
                .collect(),
            },
            (
                Ticket {
                    fields: vec![7, 1, 14],
                },
                vec![
                    Ticket {
                        fields: vec![7, 3, 47],
                    },
                    Ticket {
                        fields: vec![40, 4, 50],
                    },
                    Ticket {
                        fields: vec![55, 2, 20],
                    },
                    Ticket {
                        fields: vec![38, 6, 12],
                    },
                ],
            ),
        ));
        let actual = parse_rules_and_tickets(notes).map(|(_, res)| res);
        assert_eq!(expected, actual);
    }

    #[ignore]
    #[test]
    fn calculates_correct_error_rate() {
        let rules = TicketRules {
            rules: [
                ("class", FieldRule(1..=3, 5..=7)),
                ("row", FieldRule(6..=11, 33..=44)),
                ("seat", FieldRule(13..=40, 45..=50)),
            ]
            .iter()
            .cloned()
            .collect(),
        };
        let tickets = [
            Ticket {
                fields: vec![7, 3, 47],
            },
            Ticket {
                fields: vec![40, 4, 50],
            },
            Ticket {
                fields: vec![55, 2, 20],
            },
            Ticket {
                fields: vec![38, 6, 12],
            },
        ];
        let expected = 71;
        let actual = error_rate(&tickets, &rules);
        assert_eq!(expected, actual);
    }

    #[ignore]
    #[test]
    fn assigns_fields_correctly() {
        let rules = TicketRules {
            rules: [
                ("class", FieldRule(1..=1, 4..=19)),
                ("row", FieldRule(0..=5, 8..=19)),
                ("seat", FieldRule(0..=13, 16..=19)),
            ]
            .iter()
            .cloned()
            .collect(),
        };
        let tickets = [
            Ticket {
                fields: vec![3, 9, 18],
            },
            Ticket {
                fields: vec![15, 1, 5],
            },
            Ticket {
                fields: vec![5, 14, 9],
            },
        ];
        let expected = [("class", 1), ("row", 0), ("seat", 2)]
            .iter()
            .copied()
            .collect::<HashMap<_, _>>();
        let actual = rules.find_fields(&tickets);
        assert_eq!(expected, actual);
    }
}
