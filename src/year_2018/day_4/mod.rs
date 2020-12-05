use nom::{branch, bytes::complete as bytes, combinator as comb, sequence, IResult};

use std::{
    collections::HashMap,
    convert::TryFrom,
    fmt::{self, Display, Formatter},
    io,
    ops::Range,
    str::FromStr,
};

use crate::parse::NomParse;

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
struct Date {
    year: u32,
    month: u8,
    day: u8,
}

impl Date {
    fn new(year: u32, month: u8, day: u8) -> Date {
        Date { year, month, day }
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}-{}-{}", self.year, self.month, self.day)
    }
}

impl<'s> NomParse<'s> for Date {
    fn nom_parse(s: &str) -> IResult<&str, Date> {
        comb::map(
            sequence::separated_pair(
                u32::nom_parse, // Parse year ("{u32}")
                bytes::tag("-"),
                sequence::separated_pair(
                    u8::nom_parse, // Parse month ("{u8}")
                    bytes::tag("-"),
                    u8::nom_parse, // Parse day ("{u8}")
                ), // Parse (month, day) ("{u8}-{u8}")
            ), // Parse (year, (month, day)) ("{u32}-{u8}-{u8}")
            |(year, (month, day))| Date::new(year, month, day),
        )(s)
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
struct Time {
    hour: u8,
    minute: u8,
}

impl Time {
    fn new(hour: u8, minute: u8) -> Time {
        Time { hour, minute }
    }

    fn hour(&self) -> u32 {
        self.hour as u32
    }

    fn minute(&self) -> u32 {
        self.minute as u32
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}:{}", self.hour, self.minute)
    }
}

macro_rules! impl_into_integer_for_time {
    ($($t:ty)*) => ($(
        impl Into<$t> for Time {
            fn into(self) -> $t {
                60 * self.hour() as $t + self.minute() as $t
            }
        }

        impl Into<$t> for &'_ Time {
            fn into(self) -> $t {
                (*self).into()
            }
        }
    )*)
}

impl_into_integer_for_time!(u16 u32 u64 u128 usize i16 i32 i64 i128 isize);

macro_rules! impl_try_from_integer_for_time {
    ($($t:ty)*) => ($(
        impl TryFrom<$t> for Time {
            type Error = ();

            fn try_from(n: $t) -> Result<Time, ()> {
                if n >= 24 * 60 {
                    Err(())
                } else {
                    Ok(Time::new((n / 60) as u8, (n % 60) as u8))
                }
            }
        }
    )*)
}

impl_try_from_integer_for_time!(u16 u32 u64 u128 usize i16 i32 i64 i128 isize);

impl<'s> NomParse<'s> for Time {
    fn nom_parse(s: &str) -> IResult<&str, Time> {
        comb::map(
            sequence::separated_pair(
                u8::nom_parse, // Parse hour ("{u8}")
                bytes::tag(":"),
                u8::nom_parse, // Parse minute ("{u8}")
            ), // Parse (hour, minute) ("{u8}:{u8}")
            |(hour, minute)| Time::new(hour, minute),
        )(s)
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
struct Datetime {
    date: Date,
    time: Time,
}

impl Datetime {
    /// `month` is assumed to be in the range `1..13`.
    /// `day` is assumed to be a valid day in the specified month.
    /// `hour` is assumed to be in the range `0..24`.
    /// `minute` is assumed to be in the range `0..59`.
    fn new(date: Date, time: Time) -> Datetime {
        Datetime { date, time }
    }

    fn time(&self) -> Time {
        self.time
    }
}

impl Display for Datetime {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{} {}", self.date, self.time)
    }
}

impl<'s> NomParse<'s> for Datetime {
    fn nom_parse(s: &str) -> IResult<&str, Datetime> {
        comb::map(
            // Parse (date, time) ("{u32}-{u8}-{u8} {u8}:{u8}")
            sequence::separated_pair(Date::nom_parse, bytes::tag(" "), Time::nom_parse),
            |(date, time)| Datetime::new(date, time),
        )(s)
    }
}

#[derive(Clone, Copy)]
enum Day4Event {
    WakesUp,
    FallsAsleep,
    BeginsShift(u32),
}

impl Display for Day4Event {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Day4Event::WakesUp => write!(f, "wakes up"),
            Day4Event::FallsAsleep => write!(f, "falls asleep"),
            Day4Event::BeginsShift(g) => write!(f, "Guard #{} begins shift", g),
        }
    }
}

impl<'s> NomParse<'s> for Day4Event {
    fn nom_parse(s: &str) -> IResult<&str, Day4Event> {
        branch::alt((
            comb::value(Day4Event::WakesUp, bytes::tag("wakes up")),
            comb::value(Day4Event::FallsAsleep, bytes::tag("falls asleep")),
            comb::map(
                sequence::delimited(
                    bytes::tag("Guard #"),
                    u32::nom_parse, // Parse guard_num ("{u32}")
                    bytes::tag(" begins shift"),
                ), // Parse guard_num ("Guard #{u32} begins shift")
                |n| Day4Event::BeginsShift(n),
            ),
        ))(s)
    }
}

struct Day4Entry {
    datetime: Datetime,
    event: Day4Event,
}

impl Day4Entry {
    fn new(datetime: Datetime, event: Day4Event) -> Day4Entry {
        Day4Entry { datetime, event }
    }

    fn datetime(&self) -> Datetime {
        self.datetime
    }

    fn event(&self) -> Day4Event {
        self.event
    }
}

impl Display for Day4Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "[{}] {}", self.datetime, self.event)
    }
}

impl<'s> NomParse<'s> for Day4Entry {
    fn nom_parse(s: &str) -> IResult<&str, Day4Entry> {
        comb::map(
            sequence::pair(
                sequence::delimited(bytes::tag("["), Datetime::nom_parse, bytes::tag("] ")),
                Day4Event::nom_parse,
            ),
            |(datetime, event)| Day4Entry::new(datetime, event),
        )(s)
    }
}

impl FromStr for Day4Entry {
    type Err = ();

    fn from_str(s: &str) -> Result<Day4Entry, ()> {
        comb::all_consuming(Day4Entry::nom_parse)(s)
            .map(|(_, entry)| entry)
            .map_err(|_| ())
    }
}

struct ReposeRecord {
    sleep_times: HashMap<u32, Vec<Range<u16>>>,
}

impl ReposeRecord {
    fn new() -> ReposeRecord {
        ReposeRecord {
            sleep_times: HashMap::new(),
        }
    }

    fn add_sleep_range(&mut self, guard: u32, sleeping: Range<u16>) {
        self.sleep_times.entry(guard).or_default().push(sleeping)
    }
}

impl IntoIterator for ReposeRecord {
    type Item = <HashMap<u32, Vec<Range<u16>>> as IntoIterator>::Item;
    type IntoIter = <HashMap<u32, Vec<Range<u16>>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.sleep_times.into_iter()
    }
}

fn get_entries() -> io::Result<Vec<Day4Entry>> {
    let mut ret: Vec<Day4Entry> = super::super::parse_lines("4.txt")?.collect();
    (&mut ret[..]).sort_by_key(|entry| entry.datetime());
    Ok(ret)
}

fn build_repose_record() -> io::Result<ReposeRecord> {
    let entries = get_entries()?;
    let mut repose_record = ReposeRecord::new();
    let mut guard: Option<u32> = None;
    let mut sleep_time: Option<Time> = None;
    for entry in entries {
        match entry.event() {
            Day4Event::WakesUp if sleep_time.is_some() && guard.is_some() => {
                let wake_time = entry.datetime().time().into();
                repose_record
                    .add_sleep_range(guard.unwrap(), sleep_time.unwrap().into()..wake_time);
                sleep_time = None;
            }
            Day4Event::WakesUp => {
                panic!("Nonexistent or awake guard {:?} can't wake up", guard);
            }
            Day4Event::FallsAsleep if sleep_time.is_none() && guard.is_some() => {
                sleep_time = Some(entry.datetime().time());
            }
            Day4Event::FallsAsleep => {
                panic!(
                    "Nonexistent or sleeping guard {:?} can't fall asleep",
                    guard
                );
            }
            Day4Event::BeginsShift(g) if sleep_time.is_none() => {
                guard = Some(g);
            }
            Day4Event::BeginsShift(g) => {
                panic!(
                    "New guard {} can't start shift while old guard {:?} is asleep",
                    g, guard,
                );
            }
        }
    }
    Ok(repose_record)
}

fn build_counts() -> io::Result<HashMap<u32, HashMap<u16, u32>>> {
    let repose_record = build_repose_record()?;
    let mut counts: HashMap<_, HashMap<_, _>> = HashMap::new();
    for (guard, sleep_ranges) in repose_record {
        let freqs = counts.entry(guard).or_default();
        for range in sleep_ranges {
            for minute in range {
                *freqs.entry(minute).or_default() += 1;
            }
        }
    }
    Ok(counts)
}

pub fn run() -> io::Result<()> {
    {
        // Part 1
        let (guard, guard_counts) = build_counts()?
            .into_iter()
            .max_by_key(|(_, guard_counts)| {
                guard_counts
                    .into_iter()
                    .map(|(_, &count)| count as u64)
                    .sum::<u64>()
            })
            .unwrap();
        let (minute, count) = guard_counts
            .into_iter()
            .max_by_key(|(_, count)| count.clone())
            .unwrap();
        println!(
            "Guard #{} slept the most with {} minutes at minute {}",
            guard, count, minute
        );
        println!("Key is {}", guard * minute as u32);
    }
    {
        // Part 2
        let (guard, minute, count) = build_counts()?
            .into_iter()
            .map(|(guard, counts)| {
                let (minute, count) = counts
                    .into_iter()
                    .max_by_key(|(_, count)| count.clone())
                    .unwrap();
                (guard, minute, count)
            })
            .max_by_key(|(_, _, count)| count.clone())
            .unwrap();
        println!(
            "Guard #{} slept the most consistently with {} minutes at minute {}",
            guard, count, minute
        );
        println!("Key is {}", guard * minute as u32);
    }
    Ok(())
}
