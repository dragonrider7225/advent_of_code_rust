use crate::parse::NomParse;

use std::{
    collections::{HashMap, HashSet},
    fs, io,
};

use nom::{
    branch, bytes::complete as bytes, character::complete as character, combinator as comb,
    sequence, IResult,
};

#[derive(Clone, Copy)]
struct Passport<'s> {
    birth_year: Option<&'s str>,
    issue_year: Option<&'s str>,
    expiration_year: Option<&'s str>,
    height: Option<&'s str>,
    hair_color: Option<&'s str>,
    eye_color: Option<&'s str>,
    passport_id: Option<&'s str>,
    #[allow(unused)]
    country_id: Option<&'s str>,
}

impl<'s> Passport<'s> {
    const fn is_filled(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passport_id.is_some()
    }

    fn is_valid(&self) -> bool {
        self.has_valid_birth_year()
            && self.has_valid_issue_year()
            && self.has_valid_expiration_year()
            && self.has_valid_height()
            && self.has_valid_hair_color()
            && self.has_valid_eye_color()
            && self.has_valid_passport_id()
    }

    fn has_valid_birth_year(&self) -> bool {
        let res = self
            .birth_year
            .and_then(|s| Some((1920..=2002).contains(&s.parse::<u32>().ok()?)))
            .unwrap_or(false);
        if !res {
            #[cfg(test)]
            println!("{:?} is not a valid birth year", self.birth_year);
        } else {
            #[cfg(test)]
            println!("{:?} is a valid birth year", self.birth_year);
        }
        res
    }

    fn has_valid_issue_year(&self) -> bool {
        let res = self
            .issue_year
            .and_then(|s| Some((2010..=2020).contains(&s.parse::<u32>().ok()?)))
            .unwrap_or(false);
        if !res {
            #[cfg(test)]
            println!("{:?} is not a valid issue year", self.issue_year);
        } else {
            #[cfg(test)]
            println!("{:?} is a valid issue year", self.issue_year);
        }
        res
    }

    fn has_valid_expiration_year(&self) -> bool {
        let res = self
            .expiration_year
            .and_then(|s| Some((2020..=2030).contains(&s.parse::<u32>().ok()?)))
            .unwrap_or(false);
        if !res {
            #[cfg(test)]
            println!("{:?} is not a valid expiration year", self.expiration_year);
        } else {
            #[cfg(test)]
            println!("{:?} is a valid expiration year", self.expiration_year);
        }
        res
    }

    fn has_valid_height(&self) -> bool {
        let res = self
            .height
            .and_then(|s| match &*s.chars().rev().take(2).collect::<String>() {
                "ni" => Some((s, false)),
                "mc" => Some((s, true)),
                _ => None,
            })
            .and_then(|(s, metric)| {
                let len = &s[..(s.len() - 2)].parse::<u8>().ok()?;
                match (len, metric) {
                    (150..=193, true) | (59..=76, false) => Some(()),
                    _ => None,
                }
            })
            .is_some();
        if !res {
            #[cfg(test)]
            println!("{:?} is not a valid height", self.height);
        } else {
            #[cfg(test)]
            println!("{:?} is a valid height", self.height);
        }
        res
    }

    fn has_valid_hair_color(&self) -> bool {
        let res = self
            .hair_color
            .and_then(|s| {
                Some(s.chars().next()? == '#' && u32::from_str_radix(&s[1..], 16).is_ok())
            })
            .unwrap_or(false);
        if !res {
            #[cfg(test)]
            println!("{:?} is not a valid hair color", self.hair_color);
        } else {
            #[cfg(test)]
            println!("{:?} is a valid hair color", self.hair_color);
        }
        res
    }

    fn has_valid_eye_color(&self) -> bool {
        let res = self
            .eye_color
            .filter(|s| ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(s))
            .is_some();
        if !res {
            #[cfg(test)]
            println!("{:?} is not a valid eye color", self.eye_color);
        } else {
            #[cfg(test)]
            println!("{:?} is a valid eye color", self.eye_color);
        }
        res
    }

    fn has_valid_passport_id(&self) -> bool {
        let res = self
            .passport_id
            .map(|s| {
                s.chars()
                    .map(|c| if c.is_numeric() { 1 } else { 10 })
                    .sum::<u32>()
            })
            .unwrap_or(0)
            == 9;
        if !res {
            #[cfg(test)]
            println!("{:?} is not a valid passport id", self.passport_id);
        } else {
            #[cfg(test)]
            println!("{:?} is a valid passport id", self.passport_id);
        }
        res
    }
}

impl<'s> NomParse<'s> for Passport<'s> {
    fn nom_parse(s: &'s str) -> IResult<&'s str, Self> {
        fn parse_field(
            field_name: &'static str,
        ) -> impl for<'s> FnMut(&'s str) -> IResult<&'s str, &'s str> {
            move |s| {
                // println!("Parsing field {:?}", field_name);
                let res = sequence::delimited(
                    sequence::pair(bytes::tag(field_name), bytes::tag(":")),
                    bytes::is_not(" \r\n"),
                    branch::alt((character::line_ending, character::multispace1, comb::eof)),
                )(s);
                res.map(|(remaining, value)| {
                    // println!("Found {:?}:{:?}", field_name, value);
                    (remaining, value)
                })
            }
        }

        let mut birth_year = None;
        let mut issue_year = None;
        let mut expiration_year = None;
        let mut height = None;
        let mut hair_color = None;
        let mut eye_color = None;
        let mut passport_id = None;
        let mut country_id = None;
        let mut parsers = HashMap::<&str, _>::new();
        parsers.insert("byr", &mut birth_year);
        parsers.insert("iyr", &mut issue_year);
        parsers.insert("eyr", &mut expiration_year);
        parsers.insert("hgt", &mut height);
        parsers.insert("hcl", &mut hair_color);
        parsers.insert("ecl", &mut eye_color);
        parsers.insert("pid", &mut passport_id);
        parsers.insert("cid", &mut country_id);
        let mut remaining = s;
        loop {
            let mut completed_parsers = HashSet::new();
            for (&name, field) in parsers.iter_mut() {
                let res = parse_field(name)(remaining).map(|(s, value)| {
                    field.get_or_insert(value);
                    s
                });
                if let Ok(s) = res {
                    remaining = s;
                    completed_parsers.insert(name);
                }
            }
            if completed_parsers.is_empty() {
                break;
            } else {
                for name in completed_parsers.into_iter() {
                    parsers.remove(&name);
                }
            }
        }
        Ok((
            remaining,
            Self {
                birth_year,
                issue_year,
                expiration_year,
                height,
                hair_color,
                eye_color,
                passport_id,
                country_id,
            },
        ))
    }
}

pub(super) fn run() -> io::Result<()> {
    let passport_text = fs::read_to_string("2020_04.txt")?;
    let passports = passport_text
        .split("\n\n")
        .map(|s| {
            // println!("Parsing {:?}", s);
            Passport::nom_parse(s)
                .map(|(_, res)| res)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("{e:?}")))
        })
        .collect::<Result<Vec<Passport<'_>>, _>>()?;
    {
        println!("Year 2020 Day 4 Part 1");
        println!(
            "There are {} valid passports",
            passports
                .iter()
                .copied()
                .filter(Passport::is_filled)
                .count(),
        );
    }
    {
        println!("Year 2020 Day 4 Part 2");
        println!(
            "There are {} valid passports",
            passports.iter().copied().filter(Passport::is_valid).count(),
        );
    }
    Ok(())
}
