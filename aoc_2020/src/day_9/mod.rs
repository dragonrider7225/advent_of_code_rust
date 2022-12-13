use std::{cmp::Ordering, fmt::Display, io};

enum SumResult {
    Incomplete,
    Weakness(u64),
    Overflow,
}

impl SumResult {
    fn expect<Message>(self, message: Message) -> u64
    where
        Message: Display,
    {
        match self {
            Self::Weakness(res) => res,
            Self::Incomplete | Self::Overflow => panic!("{}", message),
        }
    }
}

use SumResult::{Incomplete, Overflow, Weakness};

pub(super) fn run() -> io::Result<()> {
    const PREAMBLE_LENGTH: usize = 25;
    let xmas_stream = crate::parse_lines("2020_09.txt")?.collect::<Vec<u64>>();
    let invalid_follower = {
        println!("Year 2020 Day 9 Part 1");
        let invalid_follower = xmas_stream
            .windows(PREAMBLE_LENGTH + 1)
            .find_map(|window| {
                for (idx, &first_value) in window[..PREAMBLE_LENGTH].iter().enumerate() {
                    if window[(idx + 1)..PREAMBLE_LENGTH]
                        .iter()
                        .any(|&second_value| first_value + second_value == window[PREAMBLE_LENGTH])
                    {
                        return None;
                    }
                }
                Some(window[PREAMBLE_LENGTH])
            })
            .expect("All values in XMAS stream are valid");
        println!("The first invalid number in the XMAS stream is {invalid_follower}");
        invalid_follower
    };
    {
        println!("Year 2020 Day 9 Part 2");
        let encryption_weakness = (0..xmas_stream.len())
            .fold(Incomplete, |acc, start| match acc {
                Weakness(_) => acc,
                Incomplete | Overflow => {
                    ((start + 2)..=xmas_stream.len()).fold(Incomplete, |acc, end| match acc {
                        Weakness(_) | Overflow => acc,
                        Incomplete => {
                            let window = &xmas_stream[start..end];
                            match window.iter().sum::<u64>().cmp(&invalid_follower) {
                                Ordering::Less => Incomplete,
                                Ordering::Equal => {
                                    let least = window.iter().min().unwrap();
                                    let most = window.iter().max().unwrap();
                                    Weakness(least + most)
                                }
                                Ordering::Greater => Overflow,
                            }
                        }
                    })
                }
            })
            .expect("Couldn't find weakness");
        println!("Weakness is {encryption_weakness}");
    }
    Ok(())
}
