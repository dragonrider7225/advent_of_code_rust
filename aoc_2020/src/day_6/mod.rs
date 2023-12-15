use aoc_util::nom_extended::NomParse;

use std::{
    convert::TryFrom,
    fs, io,
    iter::{FromIterator, Product, Sum},
    ops::{Add, Index, Mul},
};

use nom::{character::complete as character, combinator as comb, multi, IResult};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct QuestionId(u8);

impl<'s> NomParse<&'s str> for QuestionId {
    fn nom_parse(s: &'s str) -> IResult<&'s str, Self> {
        comb::map(character::one_of(&*('a'..='z').collect::<String>()), |c| {
            Self(u8::try_from(u32::from(c) - u32::from('a')).unwrap())
        })(s)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Answers(u32);

impl Answers {
    fn count_answers(&self) -> usize {
        usize::try_from(self.0.count_ones()).expect("That's a lot of ones, for a 32-bit number")
    }

    fn set_question(&mut self, QuestionId(idx): QuestionId) -> &mut Self {
        self.0 |= 1 << idx;
        self
    }
}

impl Add for Answers {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        #[allow(clippy::suspicious_arithmetic_impl)]
        Self(self.0 | rhs.0)
    }
}

impl FromIterator<QuestionId> for Answers {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = QuestionId>,
    {
        *iter.into_iter().fold(&mut Self(0), |acc, question_id| {
            acc.set_question(question_id)
        })
    }
}

aoc_util::impl_from_str_for_nom_parse!(Answers);

impl Index<QuestionId> for Answers {
    type Output = bool;

    fn index(&self, QuestionId(idx): QuestionId) -> &Self::Output {
        if self.0 & (1 << idx) != 0 {
            &true
        } else {
            &false
        }
    }
}

impl Mul for Answers {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        #[allow(clippy::suspicious_arithmetic_impl)]
        Self(self.0 & rhs.0)
    }
}

impl<'s> NomParse<&'s str> for Answers {
    fn nom_parse(s: &'s str) -> IResult<&'s str, Self> {
        comb::map(multi::many1(QuestionId::nom_parse), Answers::from_iter)(s)
    }
}

impl Product for Answers {
    fn product<Iter>(iter: Iter) -> Self
    where
        Iter: Iterator<Item = Self>,
    {
        iter.fold(Self((1 << 26) - 1), Mul::mul)
    }
}

impl Sum for Answers {
    fn sum<Iter>(iter: Iter) -> Self
    where
        Iter: Iterator<Item = Self>,
    {
        iter.fold(Self(0), Add::add)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct GroupAnswers(Vec<Answers>);

impl GroupAnswers {
    fn count_distinct_answers(&self) -> usize {
        self.0.iter().copied().sum::<Answers>().count_answers()
    }

    fn count_shared_answers(&self) -> usize {
        self.0.iter().copied().product::<Answers>().count_answers()
    }
}

impl<'s> NomParse<&'s str> for GroupAnswers {
    fn nom_parse(s: &'s str) -> IResult<&'s str, Self> {
        comb::map(
            multi::separated_list1(character::line_ending, Answers::nom_parse),
            Self,
        )(s)
    }
}

aoc_util::impl_from_str_for_nom_parse!(GroupAnswers);

pub(super) fn run() -> io::Result<()> {
    let group_answers = fs::read_to_string("2020_06.txt")?
        .split("\n\n")
        .map(|s| s.parse::<GroupAnswers>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    {
        println!("Year 2020 Day 6 Part 1");
        let distinct_answers = group_answers
            .iter()
            .map(GroupAnswers::count_distinct_answers)
            .sum::<usize>();
        println!(
            "The total number of answers, counting each answer only once within each group, is {distinct_answers}",
        );
    }
    {
        println!("Year 2020 Day 6 Part 2");
        let shared_answers = group_answers
            .iter()
            .map(GroupAnswers::count_shared_answers)
            .sum::<usize>();
        println!(
            "The total number of answers, counting each answer for a group only if all members of that group answered that question, is {shared_answers}",
        );
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[ignore]
    #[test]
    fn answer_sheet_parses() {
        let expected = Ok(Answers(7));
        let actual = "abc".parse::<Answers>();
        assert_eq!(expected, actual);
    }

    #[ignore]
    #[test]
    fn single_member_group_parses() {
        let expected = Ok(GroupAnswers(vec![Answers(7)]));
        let actual = "abc".parse::<GroupAnswers>();
        assert_eq!(expected, actual);
    }

    #[ignore]
    #[test]
    fn three_member_group_parses() {
        let expected = Ok(GroupAnswers(vec![Answers(1), Answers(2), Answers(4)]));
        let actual = "a\nb\nc".parse::<GroupAnswers>();
        assert_eq!(expected, actual);
    }

    #[ignore]
    #[test]
    fn single_member_group_counts_distinct_answers_correctly() {
        let expected = 3;
        let actual = GroupAnswers(vec![Answers(7)]).count_distinct_answers();
        assert_eq!(expected, actual);
    }

    #[ignore]
    #[test]
    fn three_member_group_counts_distinct_answers_correctly() {
        let expected = 3;
        let actual =
            GroupAnswers(vec![Answers(1), Answers(2), Answers(4)]).count_distinct_answers();
        assert_eq!(expected, actual);
    }

    #[ignore]
    #[test]
    fn group_doesnt_count_repeated_answers_distinctly() {
        let expected = 3;
        let actual = GroupAnswers(vec![Answers(3), Answers(5)]).count_distinct_answers();
        assert_eq!(expected, actual);
    }

    #[ignore]
    #[test]
    fn single_member_group_counts_shared_answers_correctly() {
        let expected = 3;
        let actual = GroupAnswers(vec![Answers(7)]).count_shared_answers();
        assert_eq!(expected, actual);
    }

    #[ignore]
    #[test]
    fn three_member_group_counts_shared_answers_correctly() {
        let expected = 0;
        let actual = GroupAnswers(vec![Answers(1), Answers(2), Answers(4)]).count_shared_answers();
        assert_eq!(expected, actual);
    }

    #[ignore]
    #[test]
    fn group_counts_only_shared_answers_as_shared() {
        let expected = 1;
        let actual = GroupAnswers(vec![Answers(3), Answers(5)]).count_shared_answers();
        assert_eq!(expected, actual);
    }
}
