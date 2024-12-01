use aoc_util::{
    nom::{bytes::complete as bytes, character::complete as character, IResult, Parser},
    nom_extended::NomParse,
    nom_supreme::ParserExt,
};
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    iter,
};

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
            Self::SingleLetterCount {
                min_count,
                max_count,
                c,
            } => {
                let count = s.chars().filter(|&ch| ch == c).count();
                (min_count..=max_count).contains(&count)
            }
            Self::MultiLetterCheck {
                first_position,
                second_position,
                c,
            } => {
                let mut chars = iter::once('\0').chain(s.chars());
                (chars.clone().nth(first_position).unwrap() == c)
                    ^ (chars.nth(second_position).unwrap() == c)
            }
        }
    }

    fn switch_to_multi_letter_check(&mut self) {
        match *self {
            Self::SingleLetterCount {
                min_count,
                max_count,
                c,
            } => {
                *self = Self::MultiLetterCheck {
                    first_position: min_count,
                    second_position: max_count,
                    c,
                }
            }
            Self::MultiLetterCheck { .. } => {}
        }
    }
}

impl<'s> NomParse<&'s str> for PasswordPolicy {
    fn nom_parse(s: &'s str) -> IResult<&'s str, Self> {
        fn parse_usize(s: &str) -> IResult<&str, usize> {
            character::u64.map(|n| n as usize).parse(s)
        }

        parse_usize
            .and(parse_usize.preceded_by(bytes::tag("-")))
            .and(character::one_of("abcdefghijklmnopqrstuvwxyz").preceded_by(bytes::tag(" ")))
            .map(
                |((min_count, max_count), c)| PasswordPolicy::SingleLetterCount {
                    min_count,
                    max_count,
                    c,
                },
            )
            .parse(s)
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

impl<'s> NomParse<&'s str> for PasswordDatabaseEntry {
    fn nom_parse(s: &'s str) -> IResult<&'s str, Self> {
        PasswordPolicy::nom_parse
            .and(
                character::alpha0
                    .map(str::to_owned)
                    .preceded_by(bytes::tag(": ")),
            )
            .map(|(policy, password)| Self { policy, password })
            .parse(s)
    }
}

aoc_util::impl_from_str_for_nom_parse!(PasswordDatabaseEntry);

struct PasswordDatabase(Vec<PasswordDatabaseEntry>);

impl PasswordDatabase {
    fn count_valid(&self) -> usize {
        self.0.iter().filter(|&entry| entry.is_valid()).count()
    }
}

impl FromIterator<PasswordDatabaseEntry> for PasswordDatabase {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = PasswordDatabaseEntry>,
    {
        Self(iter.into_iter().collect())
    }
}

#[allow(unreachable_code)]
pub(super) fn run() -> io::Result<()> {
    let mut password_database = BufReader::new(File::open("2020_02.txt")?)
        .lines()
        .map(|line| {
            line?
                .parse()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        })
        .collect::<io::Result<PasswordDatabase>>()?;
    {
        println!("Year 2020 Day 2 Part 1");
        println!(
            "There are {} valid passwords in the database",
            password_database.count_valid()
        );
    }
    {
        println!("Year 2020 Day 2 Part 2");
        password_database
            .0
            .iter_mut()
            .for_each(|entry| entry.policy.switch_to_multi_letter_check());
        println!(
            "There are {} valid passwords in the database",
            password_database.count_valid()
        );
    }
    Ok(())
}
