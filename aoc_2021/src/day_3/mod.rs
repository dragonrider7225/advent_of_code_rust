use std::{
    cmp::Ordering,
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader},
};

fn part1(input: &mut dyn BufRead) -> io::Result<u32> {
    let bit_rates = input
        .lines()
        .fold(Ok(None), |acc, line| match acc? {
            None => Ok(Some({
                line?
                    .chars()
                    .map(|bit| match bit {
                        '0' => Ok((1, 0)),
                        '1' => Ok((0, 1)),
                        bit => Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            String::from(bit),
                        )),
                    })
                    .collect::<io::Result<Vec<_>>>()?
            })),
            Some(mut acc) => {
                for (bit, counts) in line?.chars().zip(acc.iter_mut()) {
                    match bit {
                        '0' => counts.0 += 1,
                        '1' => counts.1 += 1,
                        bit => Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            String::from(bit),
                        ))?,
                    }
                }
                Ok(Some(acc))
            }
        })
        .and_then(|v| match v {
            Some(v) => Ok(v),
            None => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Input was empty",
            )),
        })?;
    let width = bit_rates.len();
    let (gamma_rate, epsilon_rate) = bit_rates.into_iter().enumerate().fold(
        (0u32, 0u32),
        |(mut gamma_rate, mut epsilon_rate), (i, bit)| {
            gamma_rate *= 2;
            epsilon_rate *= 2;
            match bit.0.cmp(&bit.1) {
                Ordering::Less => gamma_rate += 1,
                Ordering::Greater => epsilon_rate += 1,
                Ordering::Equal => println!("1 and 0 are equally common in bit {}", width - 1 - i),
            }
            (gamma_rate, epsilon_rate)
        },
    );
    Ok(gamma_rate * epsilon_rate)
}

fn part2(input: &mut dyn BufRead) -> io::Result<u32> {
    fn collapse_ratings(mut ratings: HashSet<Vec<u32>>, criterion: Ordering) -> u32 {
        let mut i = 0;
        while ratings.len() > 1 {
            let bit_rates = ratings.iter().fold((0, 0), |acc, bits| match bits[i] {
                0 => (acc.0 + 1, acc.1),
                1 => (acc.0, acc.1 + 1),
                _ => unreachable!(),
            });
            let order = (f64::from(bit_rates.0) - 0.5).partial_cmp(&f64::from(bit_rates.1));
            if order == Some(criterion) {
                ratings.retain(|bits| bits[i] == 1);
            } else {
                ratings.retain(|bits| bits[i] == 0);
            }
            i += 1;
        }
        ratings
            .into_iter()
            .next()
            .unwrap()
            .into_iter()
            .fold(0, |acc, bit| acc * 2 + bit)
    }

    let diagnostics = input
        .lines()
        .map(|line| {
            let line = line?;
            line.chars()
                .map(|bit| match bit {
                    '0' => Ok(0),
                    '1' => Ok(1),
                    bit => Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        String::from(bit),
                    ))?,
                })
                .collect::<io::Result<Vec<_>>>()
        })
        .collect::<io::Result<HashSet<_>>>()?;
    let oxygen_generator_rating = collapse_ratings(diagnostics.clone(), Ordering::Greater);
    let co2_scrubber_rating = collapse_ratings(diagnostics, Ordering::Less);
    Ok(oxygen_generator_rating * co2_scrubber_rating)
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2021 Day 3 Part 1");
        println!(
            "The power consumption is {}",
            part1(&mut BufReader::new(File::open("2021_03.txt")?))?
        );
    }
    {
        println!("Year 2021 Day 3 Part 2");
        println!(
            "The life support rating is {}",
            part2(&mut BufReader::new(File::open("2021_03.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_part1() -> io::Result<()> {
        let s =
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";
        let expected = 198;
        let actual = part1(&mut Cursor::new(s))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let s =
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";
        let expected = 230;
        let actual = part2(&mut Cursor::new(s))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
