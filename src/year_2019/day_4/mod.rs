use crate::parse::NomParse;

use std::{io, ops::Range};

use nom::{bytes::complete as bytes, combinator as comb, sequence, IResult};

fn parse_range(s: &str) -> IResult<&str, Range<u32>> {
    comb::map(
        sequence::separated_pair(u32::nom_parse, bytes::tag("-"), u32::nom_parse),
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
        let pw_range = crate::get_lines("2019_4.txt")?
            .map(|s| parse_range(&s).unwrap().1)
            .next()
            .unwrap();
        let mut num_pws = 0;
        for pw in pw_range {
            if possible_pw(pw) {
                num_pws += 1;
            }
        }
        println!("The number of potential passwords is {num_pws}");
    }
    {
        // Part 2
        let pw_range = crate::get_lines("2019_4.txt")?
            .map(|s| parse_range(&s).unwrap().1)
            .next()
            .unwrap();
        let mut num_pws = 0;
        for pw in pw_range {
            if possible_pw_modified(pw) {
                num_pws += 1;
            }
        }
        println!("The number of potential passwords is {num_pws}");
    }
    Ok(())
}
