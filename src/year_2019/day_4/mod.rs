use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    ops::Range,
};

use nom::{
    bytes::complete as bytes, character::complete as character, combinator as comb, sequence,
    IResult,
};

fn parse_range(s: &str) -> IResult<&str, Range<u32>> {
    comb::map(
        sequence::separated_pair(character::u32, bytes::tag("-"), character::u32),
        |(least, most)| least..most,
    )(s)
}

fn possible_pw(pw: u32) -> bool {
    let hundred_thousands = pw / 100_000;
    let ten_thousands = pw / 10_000 % 10;
    let thousands = pw / 1000 % 10;
    let hundreds = pw / 100 % 10;
    let tens = pw / 10 % 10;
    let ones = pw % 10;
    let is_valid_len = 111_110 < pw && pw <= 999_999;
    let has_pair = hundred_thousands == ten_thousands
        || ten_thousands == thousands
        || thousands == hundreds
        || hundreds == tens
        || tens == ones;
    let is_mono_inc = hundred_thousands <= ten_thousands
        && ten_thousands <= thousands
        && thousands <= hundreds
        && hundreds <= tens
        && tens <= ones;
    is_valid_len && has_pair && is_mono_inc
}

fn possible_pw_modified(pw: u32) -> bool {
    let hundred_thousands = pw / 100_000;
    let ten_thousands = pw / 10_000 % 10;
    let thousands = pw / 1000 % 10;
    let hundreds = pw / 100 % 10;
    let tens = pw / 10 % 10;
    let ones = pw % 10;
    let is_valid_old = possible_pw(pw);
    let has_pair = hundred_thousands == ten_thousands && ten_thousands != thousands
        || hundred_thousands != ten_thousands
            && ten_thousands == thousands
            && thousands != hundreds
        || ten_thousands != thousands && thousands == hundreds && hundreds != tens
        || thousands != hundreds && hundreds == tens && tens != ones
        || hundreds != tens && tens == ones;
    is_valid_old && has_pair
}

pub(super) fn run() -> io::Result<()> {
    {
        // Part 1
        let num_pws = BufReader::new(File::open("2019_4.txt")?)
            .lines()
            .map(|s| {
                parse_range(&s?)
                    .map(|(_, range)| range)
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
            })
            .next()
            .unwrap()?
            .filter(|&pw| possible_pw(pw))
            .count();
        println!("The number of potential passwords is {num_pws}");
    }
    {
        // Part 2
        let num_pws = BufReader::new(File::open("2019_4.txt")?)
            .lines()
            .map(|s| {
                parse_range(&s?)
                    .map(|(_, range)| range)
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
            })
            .next()
            .unwrap()?
            .filter(|&pw| possible_pw_modified(pw))
            .count();
        println!("The number of potential passwords is {num_pws}");
    }
    Ok(())
}
