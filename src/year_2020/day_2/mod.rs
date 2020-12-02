use crate::parse::NomParse;
use nom::{
    bytes::complete as bytes,
    character::complete as character,
    combinator as comb,
    sequence,
    IResult,
};
use std::{io, iter, str::FromStr};

enum PasswordPolicy {
    SingleLetterCount {
        min_count: usize,
        max_count: usize,
        c: char,
    },
    MultiLetterCheck {
        first_position: usize,
        second_position: usize,
        c: char,
    },
}

impl PasswordPolicy {
    fn is_satisfied_by(&self, s: &str) -> bool {
        match *self {
            Self::SingleLetterCount { min_count, max_count, c } => {
                let count = s.chars().filter(|&ch| ch == c).count();
                (min_count..=max_count).contains(&count)
            }
            Self::MultiLetterCheck { first_position, second_position, c } => {
                let mut chars = iter::once('\0').chain(s.chars());
                let char_1 = chars.clone().nth(first_position).unwrap();
                let char_2 = chars.nth(second_position).unwrap();
                ((char_1 == c) && (char_2 != c)) || ((char_1 != c) && (char_2 == c))
            }
        }
    }
}

impl NomParse for PasswordPolicy {
    fn nom_parse(s: &str) -> IResult<&str, Self> {
        comb::map(
            sequence::separated_pair(
                sequence::separated_pair(usize::nom_parse, bytes::tag("-"), usize::nom_parse),
                bytes::tag(" "),
                character::one_of(&*('a'..='z').collect::<String>()),
            ),
            |((min_count, max_count), c)| PasswordPolicy::SingleLetterCount {
                min_count,
                max_count,
                c,
            },
        )(s)
    }
}

struct PasswordDatabaseEntry {
    policy: PasswordPolicy,
    password: String,
}

impl PasswordDatabaseEntry {
    fn is_valid(&self) -> bool {
        self.policy.is_satisfied_by(&self.password)
    }
}

impl NomParse for PasswordDatabaseEntry {
    fn nom_parse(s: &str) -> IResult<&str, Self> {
        comb::map(
            sequence::separated_pair(
                PasswordPolicy::nom_parse,
                bytes::tag(": "),
                character::alpha0,
            ),
            |(policy, password)| Self { policy, password: password.to_owned() },
        )(s)
    }
}

impl_from_str_for_nom_parse!(PasswordDatabaseEntry);

struct PasswordDatabase(Vec<PasswordDatabaseEntry>);

impl PasswordDatabase {
    fn count_valid(&self) -> usize {
        self.0.iter().filter(|&entry| entry.is_valid()).count()
    }
}

#[allow(unreachable_code)]
pub(super) fn run() -> io::Result<()> {
    let mut password_database = PasswordDatabase(crate::parse_lines("2020_02.txt")?.collect());
    {
        println!("Year 2020 Day 2 Part 1");
        println!("There are {} valid passwords in the database", password_database.count_valid());
    }
    {
        println!("Year 2020 Day 2 Part 2");
        password_database.0.iter_mut()
            .for_each(|entry| if let PasswordPolicy::SingleLetterCount { min_count, max_count, c } = entry.policy {
            entry.policy = PasswordPolicy::MultiLetterCheck {
                first_position: min_count,
                second_position: max_count,
                c,
            };
        });
        println!("There are {} valid passwords in the database", password_database.count_valid());
    }
    Ok(())
}
