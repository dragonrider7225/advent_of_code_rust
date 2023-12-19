use std::{
    cmp::Ordering,
    collections::HashMap,
    fmt::{self, Display, Formatter},
    fs::File,
    io::{self, BufRead, BufReader},
    mem,
    ops::{ControlFlow, Range},
};

use aoc_util::nom_extended::NomParse;
use nom::{
    branch, bytes::complete as bytes, character::complete as character, combinator, multi,
    sequence, IResult,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Terminal {
    Accept,
    Reject,
}

impl<'s> NomParse<&'s str> for Terminal {
    fn nom_parse(input: &'s str) -> IResult<&'s str, Self> {
        branch::alt((
            combinator::value(Self::Accept, bytes::tag("A")),
            combinator::value(Self::Reject, bytes::tag("R")),
        ))(input)
    }
}

type RuleTarget = ControlFlow<Terminal, String>;
fn parse_rule_target(s: &str) -> IResult<&str, RuleTarget> {
    branch::alt((
        combinator::map(Terminal::nom_parse, ControlFlow::Break),
        combinator::map(
            combinator::recognize(multi::many1(character::one_of(
                "abcdefghijklmnopqrstuvwxyz",
            ))),
            |name: &str| ControlFlow::Continue(name.to_string()),
        ),
    ))(s)
}

type Attribute = u64;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Field {
    X,
    M,
    A,
    S,
}

impl<'s> NomParse<&'s str> for Field {
    fn nom_parse(input: &'s str) -> IResult<&'s str, Self> {
        branch::alt((
            combinator::value(Self::X, bytes::tag("x")),
            combinator::value(Self::M, bytes::tag("m")),
            combinator::value(Self::A, bytes::tag("a")),
            combinator::value(Self::S, bytes::tag("s")),
        ))(input)
    }
}

struct Rule {
    field: Field,
    order: Ordering,
    value: Attribute,
    target: RuleTarget,
}

impl<'s> NomParse<&'s str> for Rule {
    fn nom_parse(input: &'s str) -> IResult<&'s str, Self> {
        combinator::map(
            sequence::tuple((
                Field::nom_parse,
                character::one_of("<>"),
                character::u64,
                sequence::preceded(bytes::tag(":"), parse_rule_target),
            )),
            |(field, comparison, value, target)| match comparison {
                '<' => Self {
                    field,
                    order: Ordering::Less,
                    value,
                    target,
                },
                '>' => Self {
                    field,
                    order: Ordering::Greater,
                    value,
                    target,
                },
                _ => unreachable!(),
            },
        )(input)
    }
}

impl FnOnce<(&Part,)> for Rule {
    type Output = Option<RuleTarget>;

    extern "rust-call" fn call_once(self, args: (&Part,)) -> Self::Output {
        self.call(args)
    }
}

impl FnMut<(&Part,)> for Rule {
    extern "rust-call" fn call_mut(&mut self, args: (&Part,)) -> Self::Output {
        self.call(args)
    }
}

impl Fn<(&Part,)> for Rule {
    extern "rust-call" fn call(&self, (part,): (&Part,)) -> Self::Output {
        if part.get(self.field).cmp(&self.value) == self.order {
            Some(self.target.clone())
        } else {
            None
        }
    }
}

impl FnOnce<(PartRange,)> for Rule {
    type Output = Vec<(PartRange, Option<RuleTarget>)>;

    extern "rust-call" fn call_once(self, args: (PartRange,)) -> Self::Output {
        self.call(args)
    }
}

impl FnMut<(PartRange,)> for Rule {
    extern "rust-call" fn call_mut(&mut self, args: (PartRange,)) -> Self::Output {
        self.call(args)
    }
}

impl Fn<(PartRange,)> for Rule {
    extern "rust-call" fn call(&self, (part_range,): (PartRange,)) -> Self::Output {
        part_range
            .split_at(self.field, self.value, self.order)
            .into_iter()
            .map(|range| {
                if range.get(self.field).start.cmp(&self.value) == self.order {
                    (range, Some(self.target.clone()))
                } else {
                    (range, None)
                }
            })
            .collect()
    }
}

#[derive(Clone, Debug)]
struct PartRange {
    x: Range<Attribute>,
    m: Range<Attribute>,
    a: Range<Attribute>,
    s: Range<Attribute>,
}

impl PartRange {
    fn get(&self, field: Field) -> &Range<Attribute> {
        match field {
            Field::X => &self.x,
            Field::M => &self.m,
            Field::A => &self.a,
            Field::S => &self.s,
        }
    }

    fn size(self) -> usize {
        self.x.count() * self.m.count() * self.a.count() * self.s.count()
    }

    /// Splits the part range into two: one containing all the values that compare `ordering` with
    /// `value` and one containing the rest. The range that starts at the same place as `self` will
    /// always be first. If either of these ranges is empty, the range is omitted.
    fn split_at(self, field: Field, mut value: Attribute, ordering: Ordering) -> Vec<Self> {
        match field {
            Field::X => {
                if value < self.x.start || self.x.end <= value {
                    vec![self]
                } else {
                    let mut low = self.clone();
                    let mut high = self;
                    if ordering == Ordering::Greater {
                        value += 1;
                    }
                    low.x.end = value;
                    high.x.start = value;
                    vec![low, high]
                }
            }
            Field::M => {
                if value < self.m.start || self.m.end <= value {
                    vec![self]
                } else {
                    let mut low = self.clone();
                    let mut high = self;
                    if ordering == Ordering::Greater {
                        value += 1;
                    }
                    low.m.end = value;
                    high.m.start = value;
                    vec![low, high]
                }
            }
            Field::A => {
                if value < self.a.start || self.a.end <= value {
                    vec![self]
                } else {
                    let mut low = self.clone();
                    let mut high = self;
                    if ordering == Ordering::Greater {
                        value += 1;
                    }
                    low.a.end = value;
                    high.a.start = value;
                    vec![low, high]
                }
            }
            Field::S => {
                if value < self.s.start || self.s.end <= value {
                    vec![self]
                } else {
                    let mut low = self.clone();
                    let mut high = self;
                    if ordering == Ordering::Greater {
                        value += 1;
                    }
                    low.s.end = value;
                    high.s.start = value;
                    vec![low, high]
                }
            }
        }
    }
}

impl Display for PartRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{x={:?},m={:?},a={:?},s={:?}}}",
            self.x, self.m, self.a, self.s
        )
    }
}

struct Workflow {
    name: String,
    rules: Vec<Rule>,
    default: ControlFlow<Terminal, String>,
}

impl FnOnce<(&Part,)> for Workflow {
    type Output = RuleTarget;

    extern "rust-call" fn call_once(self, args: (&Part,)) -> Self::Output {
        self.call(args)
    }
}

impl FnMut<(&Part,)> for Workflow {
    extern "rust-call" fn call_mut(&mut self, args: (&Part,)) -> Self::Output {
        self.call(args)
    }
}

impl Fn<(&Part,)> for Workflow {
    extern "rust-call" fn call(&self, args: (&Part,)) -> Self::Output {
        for rule in &self.rules {
            if let Some(target) = rule.call(args) {
                return target;
            }
        }
        self.default.clone()
    }
}

impl FnOnce<(PartRange,)> for Workflow {
    type Output = Vec<(PartRange, RuleTarget)>;

    extern "rust-call" fn call_once(self, args: (PartRange,)) -> Self::Output {
        self.call(args)
    }
}
impl FnMut<(PartRange,)> for Workflow {
    extern "rust-call" fn call_mut(&mut self, args: (PartRange,)) -> Self::Output {
        self.call(args)
    }
}

impl Fn<(PartRange,)> for Workflow {
    extern "rust-call" fn call(&self, (range,): (PartRange,)) -> Self::Output {
        let mut remaining_ranges = vec![range];
        let mut completed_ranges = vec![];
        for rule in &self.rules {
            for range in mem::take(&mut remaining_ranges) {
                let mut new_targets = rule(range);
                completed_ranges.extend(
                    new_targets
                        .extract_if(|(_, target)| target.is_some())
                        .map(|(range, target)| (range, target.unwrap())),
                );
                remaining_ranges.extend(new_targets.into_iter().map(|(range, _)| range));
            }
        }
        completed_ranges.extend(
            remaining_ranges
                .into_iter()
                .map(|range| (range, self.default.clone())),
        );
        completed_ranges
    }
}

impl<'s> NomParse<&'s str> for Workflow {
    fn nom_parse(input: &'s str) -> IResult<&'s str, Self> {
        combinator::map(
            sequence::pair(
                character::alpha1,
                sequence::delimited(
                    bytes::tag("{"),
                    sequence::pair(
                        multi::many0(sequence::terminated(Rule::nom_parse, bytes::tag(","))),
                        parse_rule_target,
                    ),
                    bytes::tag("}"),
                ),
            ),
            |(name, (rules, default))| {
                let name = name.to_string();
                Self {
                    name,
                    rules,
                    default,
                }
            },
        )(input)
    }
}

struct Part {
    x: Attribute,
    m: Attribute,
    a: Attribute,
    s: Attribute,
}

impl Part {
    fn get(&self, field: Field) -> &Attribute {
        match field {
            Field::X => &self.x,
            Field::M => &self.m,
            Field::A => &self.a,
            Field::S => &self.s,
        }
    }
}

impl Display for Part {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{{x={},m={},a={},s={}}}", self.x, self.m, self.a, self.s)
    }
}

impl<'s> NomParse<&'s str> for Part {
    fn nom_parse(input: &'s str) -> IResult<&'s str, Self> {
        combinator::map(
            sequence::delimited(
                bytes::tag("{"),
                sequence::tuple((
                    sequence::delimited(bytes::tag("x="), character::u64, bytes::tag(",")),
                    sequence::delimited(bytes::tag("m="), character::u64, bytes::tag(",")),
                    sequence::delimited(bytes::tag("a="), character::u64, bytes::tag(",")),
                    sequence::preceded(bytes::tag("s="), character::u64),
                )),
                bytes::tag("}"),
            ),
            |(x, m, a, s)| Self { x, m, a, s },
        )(input)
    }
}

fn parse_workflows_and_parts(
    input: &mut dyn BufRead,
) -> io::Result<(HashMap<String, Workflow>, Vec<Part>)> {
    input.lines().try_fold(
        (HashMap::new(), vec![]),
        |(mut workflows, mut parts), line| {
            let line = line?;
            if !line.is_empty() {
                if line.as_bytes()[0] == b'{' {
                    let part = Part::nom_parse(&line)
                        .map(|(_, x)| x)
                        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
                    parts.push(part);
                } else {
                    let workflow = Workflow::nom_parse(&line)
                        .map(|(_, x)| x)
                        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
                    debug_assert!(
                        workflows.insert(workflow.name.clone(), workflow).is_none(),
                        "Workflows should have unique names"
                    );
                }
            }
            Ok((workflows, parts))
        },
    )
}

fn part1(input: &mut dyn BufRead) -> io::Result<Attribute> {
    let (workflows, parts) = parse_workflows_and_parts(input)?;
    Ok(parts
        .into_iter()
        .filter_map(|part| {
            let mut next_workflow = ControlFlow::Continue("in".to_string());
            loop {
                match next_workflow {
                    ControlFlow::Break(Terminal::Accept) => {
                        break Some(part.x + part.m + part.a + part.s)
                    }
                    ControlFlow::Break(Terminal::Reject) => break None,
                    ControlFlow::Continue(name) => next_workflow = workflows[&name](&part),
                }
            }
        })
        .sum())
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    let (workflows, _) = parse_workflows_and_parts(input)?;
    let mut remaining_ranges = vec![(
        PartRange {
            x: 1..4001,
            m: 1..4001,
            a: 1..4001,
            s: 1..4001,
        },
        "in".to_string(),
    )];
    let mut finished_ranges = vec![];
    while let Some((range, workflow)) = remaining_ranges.pop() {
        let mut new_ranges = workflows[&workflow](range);
        finished_ranges.extend(
            new_ranges
                .extract_if(|(_, target)| target.is_break())
                .map(|(range, target)| (range, target.break_value().unwrap())),
        );
        remaining_ranges.extend(
            new_ranges
                .into_iter()
                .map(|(range, target)| (range, target.continue_value().unwrap())),
        );
    }
    let ((ranges_rejected, ranges_accepted), total_accepted) = finished_ranges.into_iter().fold(
        ((0, 0), 0),
        |((ranges_rejected, ranges_accepted), total_accepted), (range, terminal)| match terminal {
            Terminal::Accept => (
                (ranges_rejected, ranges_accepted + 1),
                total_accepted + range.size(),
            ),
            Terminal::Reject => ((ranges_rejected + 1, ranges_accepted), total_accepted),
        },
    );
    println!("There were {ranges_rejected} ranges rejected and {ranges_accepted} ranges accepted");
    Ok(total_accepted)
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2023 Day 19 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2023_19.txt")?))?
        );
    }
    {
        println!("Year 2023 Day 19 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2023_19.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        "px{a<2006:qkq,m>2090:A,rfg}\n",
        "pv{a>1716:R,A}\n",
        "lnx{m>1548:A,A}\n",
        "rfg{s<537:gd,x>2440:R,A}\n",
        "qs{s>3448:A,lnx}\n",
        "qkq{x<1416:A,crn}\n",
        "crn{x>2662:A,R}\n",
        "in{s<1351:px,qqz}\n",
        "qqz{s>2770:qs,m<1801:hdj,R}\n",
        "gd{a>3333:R,R}\n",
        "hdj{m>838:A,pv}\n",
        "\n",
        "{x=787,m=2655,a=1222,s=2876}\n",
        "{x=1679,m=44,a=2067,s=496}\n",
        "{x=2036,m=264,a=79,s=2244}\n",
        "{x=2461,m=1339,a=466,s=291}\n",
        "{x=2127,m=1623,a=2188,s=1013}\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 19114;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 167_409_079_868_000;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
