use std::{hint::unreachable_unchecked, io, iter};

// O(1)
fn calc_fft(i: usize, j: usize) -> Option<i32> {
    match j / i {
        e if e % 2 == 0 => None,
        e if e % 4 == 1 => Some(1),
        e if e % 4 == 3 => Some(-1),
        _ => unsafe { unreachable_unchecked() },
    }
}

// O(j - i)
fn calc_2_fft(i: usize, j: usize) -> i32 {
    (i..=j)
        .into_iter()
        .filter_map(|k| Some(calc_fft(i, k)? * calc_fft(k, j)?))
        .map(|factor| factor % 10)
        .map(|factor| factor - 10)
        .sum()
}

// O((j - i)**2)
fn calc_3_fft(i: usize, j: usize) -> i32 {
    (i..=j)
        .into_iter()
        .filter_map(|k| Some(calc_2_fft(i, k) * calc_fft(k, j)?))
        .map(|factor| factor % 10)
        .map(|factor| factor - 10)
        .sum()
}

// O((j - i)**3)
fn calc_6_fft(i: usize, j: usize) -> i32 {
    (i..=j)
        .into_iter()
        .map(|k| calc_3_fft(i, k) * calc_3_fft(k, j))
        .map(|k| k % 10)
        .map(|k| k - 10)
        .sum()
}

// O((j - i)**4)
fn calc_12_fft(i: usize, j: usize) -> i32 {
    (i..=j)
        .into_iter()
        .map(|k| calc_6_fft(i, k) * calc_6_fft(k, j))
        .map(|k| k % 10)
        .map(|k| k - 10)
        .sum()
}

// O((j - i)**5)
fn calc_24_fft(i: usize, j: usize) -> i32 {
    (i..=j)
        .into_iter()
        .map(|k| calc_12_fft(i, k) * calc_12_fft(k, j))
        .map(|k| k % 10)
        .map(|k| k - 10)
        .sum()
}

// O((j - i)**6)
fn calc_25_fft(i: usize, j: usize) -> i32 {
    (i..=j)
        .into_iter()
        .filter_map(|k| Some(calc_24_fft(i, k) * calc_fft(k, j)?))
        .map(|k| k % 10)
        .map(|k| k - 10)
        .sum()
}

// O((j - i)**7)
fn calc_50_fft(i: usize, j: usize) -> i32 {
    (i..=j)
        .into_iter()
        .map(|k| calc_25_fft(i, k) * calc_25_fft(k, j))
        .map(|factor| factor % 10)
        .map(|factor| factor - 10)
        .sum()
}

// O((j - i)**8)
fn calc_100_fft(i: usize, j: usize) -> i32 {
    (i..=j)
        .into_iter()
        .map(|k| (calc_50_fft(i, k) % 10) * (calc_50_fft(k, j) % 10))
        .map(|factor| factor % 10)
        .map(|factor| factor - 10)
        .sum()
}

// O(n**2)
fn run_fft(digits: &[i32]) -> Vec<i32> {
    (1..=digits.len())
        .into_iter()
        .map(|i| {
            // if (i - 1) % 50 == 0 {
            //     println!("Producing digit {}", i);
            // }
            // O(n - i)
            (i..=digits.len())
                .into_iter()
                .filter_map(|j| Some(digits[j - 1] * calc_fft(i, j)?))
                .sum::<i32>()
                .abs()
                % 10
        })
        .collect()
}

pub(super) fn run() -> io::Result<()> {
    let digits = crate::get_lines("2019_16.txt")?
        .next()
        .unwrap()
        .chars()
        .map(|c| iter::once(c).collect::<String>())
        .map(|s| s.parse().expect("Invalid digit"))
        .collect::<Vec<i32>>();
    {
        println!("Year 2019 Day 16 Part 1");
        let digits = (0..100).fold(digits.clone(), |digits, _| run_fft(&digits));
        let message = digits[..8].iter().copied().fold(0, |acc, x| acc * 10 + x);
        // let message = (1..)
        //     .into_iter()
        //     .take(8)
        //     .map(|i| {
        //         println!("Calculating digit {}", i);
        //         (i..=digits.len()).map(|j| calc_100_fft(i, j) * digits[j - 1]).sum::<i32>() % 10
        //     })
        //     .fold(0, |acc, x| acc * 10 + x);
        println!("The first 8 digits after 100 iterations are {message}");
    }
    {
        println!("Year 2019 Day 16 Part 2");
        let offset = digits[..7]
            .iter()
            .copied()
            .fold(0usize, |acc, x| acc * 10 + x as usize);
        println!("Offset is {} out of {}", offset, digits.len() * 10_000);
        let message = if offset > 10_000 * digits.len() / 2 {
            println!("Offset is sufficiently large to use the simplified algorithm");
            // This method by "paul2718" on Reddit: <https://old.reddit.com/r/adventofcode/comments/ebf5cy/2019_day_16_part_2_understanding_how_to_come_up/fb4bvw4/>
            (0..100)
                .into_iter()
                .fold::<Box<dyn DoubleEndedIterator<Item = i32>>, _>(
                    Box::new(aoc_iter::cycle_bounded(10_000, digits.into_iter()).skip(offset)),
                    |acc_digits, _| {
                        acc_digits
                            .rev()
                            .fold::<(Box<dyn DoubleEndedIterator<Item = i32>>, i32), _>(
                                (Box::new(iter::empty()), 0),
                                |(acc, sum), digit| {
                                    let sum = (sum + digit) % 10;
                                    (Box::new(iter::once(sum).chain(acc)), sum)
                                },
                            )
                            .0
                    },
                )
                .take(8)
                .fold(0, |acc, x| acc * 10 + x)
        } else {
            println!("Offset is too small to use the simplified algorithm");
            let digits = aoc_iter::cycle_bounded(10_000, digits.iter().copied())
                .skip(offset)
                .collect::<Vec<_>>();
            (0..8)
                .into_iter()
                // O(n**9)
                .map(|i| {
                    (i..digits.len())
                        .map(|j| calc_100_fft(offset + i + 1, offset + j + 1) * digits[j])
                        .sum::<i32>()
                        % 10
                })
                .fold(0, |acc, x| acc * 10 + x)
        };
        // let digits = (0..100).fold(
        //     replicate(10_000, digits).flatten().collect::<Vec<_>>(),
        //     |digits, iteration| {
        //         println!("Iteration {}", iteration + 1);
        //         run_fft(&digits)
        //     },
        // );
        // let message = digits.iter().skip(offset).take(7).fold(0, |acc, x| acc * 10 + x);
        println!("The message is {message}");
    }
    Ok(())
}
