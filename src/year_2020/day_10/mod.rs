use std::{collections::HashMap, io};

fn count_arrangements(adapters: &[u32]) -> u64 {
    fn delegate(adapters: &[u32], memoizer: &mut HashMap<usize, u64>) -> u64 {
        if let Some(&total) = memoizer.get(&adapters.len()) {
            total
        } else {
            let idx = adapters.len() - 1;
            let total = match (
                adapters.get(idx),
                idx.checked_sub(1).and_then(|idx_1| adapters.get(idx_1)),
                idx.checked_sub(2).and_then(|idx_2| adapters.get(idx_2)),
                idx.checked_sub(3).and_then(|idx_3| adapters.get(idx_3)),
            ) {
                (None, ..) | (Some(_), None, ..) => 1,
                (Some(last), Some(first), None, ..) => (1..=3).contains(&(last - first)).into(),
                (last, second_last, third_last, fourth_last) => {
                    let mut res = fourth_last
                        .filter(|&fourth_last| last.unwrap() - fourth_last == 3)
                        .map(|_| delegate(&adapters[..(idx - 2)], memoizer))
                        .unwrap_or(0);
                    res += third_last
                        .filter(|&third_last| last.unwrap() - third_last <= 3)
                        .map(|_| delegate(&adapters[..(idx - 1)], memoizer))
                        .unwrap_or(0);
                    res += second_last
                        .filter(|&second_last| last.unwrap() - second_last <= 3)
                        .map(|_| delegate(&adapters[..idx], memoizer))
                        .unwrap_or(0);
                    res
                }
            };
            memoizer.insert(adapters.len(), total);
            total
        }
    }
    delegate(adapters, &mut HashMap::new())
}

pub(super) fn run() -> io::Result<()> {
    let adapters = {
        let mut res = crate::parse_lines("2020_10.txt")?.collect::<Vec<u32>>();
        res.push(0);
        res.sort();
        res.push(res.last().unwrap() + 3);
        res
    };
    {
        println!("Year 2020 Day 10 Part 1");
        let (num_ones, num_threes) = adapters.windows(2)
            .fold((0, 0), |(num_ones, num_threes), window| {
                match window[1] - window[0] {
                    1 => (num_ones + 1, num_threes),
                    2 => (num_ones, num_threes),
                    3 => (num_ones, num_threes + 1),
                    delta => unreachable!("Can't create a working chain if there is a difference of {} jolts between two consecutive adapters", delta),
                }
            });
        println!(
            "There are {} 1-jolt differences and {} 3-jolt differences. Their product is {}",
            num_ones,
            num_threes,
            num_ones * num_threes,
        );
    }
    {
        println!("Year 2020 Day 10 Part 2");
        let num_sets = count_arrangements(&adapters);
        println!(
            "There are {} sets of adapters which can charge the device",
            num_sets
        );
    }
    Ok(())
}
