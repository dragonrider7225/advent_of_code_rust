use std::{cmp::Ordering, hint::unreachable_unchecked, io, path::Path};

struct Expenses {
    ends: Vec<Vec<u32>>,
}

impl Expenses {
    fn read_from_file(filename: impl AsRef<Path>) -> io::Result<Self> {
        let mut ends = vec![vec![]; 10];
        let values = aoc_util::parse_lines(filename)?;
        for value in values {
            match value % 10 {
                digit @ 0..=9 => &mut ends[digit as usize],
                // SAFETY: This call to `unreachable_unchecked` is safe because the only possible
                //         results of `u32::rem(_, 10)` are in the range `(0..=9)`.
                _ => unsafe { unreachable_unchecked() },
            }
            .push(value);
        }
        for end in ends.iter_mut() {
            end.sort_unstable();
        }
        Ok(Self { ends })
    }

    fn find_pair_sum(&self, total: u32) -> Option<(u32, u32)> {
        {
            let mut back_iter = self.ends[0].iter().rev();
            let mut back_value = back_iter.next().copied();
            for &value in self.ends[0].iter() {
                while back_value.is_some() && back_value.unwrap() + value > total {
                    back_value = back_iter.next().copied();
                }
                if let Some(back_value) = back_value {
                    if back_value < value {
                        break;
                    } else if back_value + value == total {
                        return Some((value, back_value));
                    }
                } else {
                    break;
                }
            }
        }
        for i in 1..5 {
            let mut back_iter = self.ends[i].iter().rev();
            let mut back_value = back_iter.next().copied();
            for &value in self.ends[10 - i].iter() {
                while back_value.is_some() && back_value.unwrap() + value > total {
                    back_value = back_iter.next().copied();
                }
                if let Some(back_value) = back_value {
                    if back_value + value == total {
                        return Some((value, back_value));
                    }
                } else {
                    break;
                }
            }
        }
        {
            let mut back_iter = self.ends[5].iter().rev();
            let mut back_value = back_iter.next().copied();
            for &value in self.ends[5].iter() {
                while back_value.is_some() && back_value.unwrap() + value > total {
                    back_value = back_iter.next().copied();
                }
                if let Some(back_value) = back_value {
                    if back_value < value {
                        break;
                    } else if back_value + value == total {
                        return Some((value, back_value));
                    }
                } else {
                    break;
                }
            }
        }
        None
    }

    fn find_triple_sum(&self, total: u32) -> Option<(u32, u32, u32)> {
        let mut values = Vec::with_capacity(self.ends.iter().map(|end| end.len()).sum());
        {
            let mut iters = self
                .ends
                .iter()
                .map(|end| end.iter().copied().peekable())
                .collect::<Vec<_>>();
            loop {
                let mut least_index = 0;
                for index in 0..iters.len() {
                    let least = iters[least_index].peek().copied();
                    let current = iters[index].peek().copied();
                    match (least, current) {
                        (Some(least), Some(current)) => {
                            if current < least {
                                least_index = index;
                            }
                        }
                        (None, Some(_)) => least_index = index,
                        _ => {}
                    }
                }
                if let Some(next) = iters[least_index].next() {
                    values.push(next);
                } else {
                    break;
                }
            }
        }
        let mut max_j = values.len();
        let mut max_k = values.len();
        for i in 0..values.len() {
            if values[i] > total {
                break;
            }
            for j in (i + 1)..max_j {
                if values[i] + values[j] > total {
                    max_j = j;
                    break;
                }
                for k in (j + 1)..max_k {
                    match total.cmp(&(values[i] + values[j] + values[k])) {
                        Ordering::Less => {
                            max_k = k;
                            break;
                        }
                        Ordering::Equal => return Some((values[i], values[j], values[k])),
                        Ordering::Greater => {}
                    }
                }
            }
        }
        None
    }
}

pub(super) fn run() -> io::Result<()> {
    let expenses = Expenses::read_from_file("2020_01.txt")?;
    {
        println!("2020 Day 1 Part 1");
        if let Some((v1, v2)) = expenses.find_pair_sum(2020) {
            println!("Values are {} and {}. Their product is {}", v1, v2, v1 * v2);
        }
    }
    {
        println!("2020 Day 1 Part 2");
        if let Some((v1, v2, v3)) = expenses.find_triple_sum(2020) {
            println!(
                "Values are {}, {}, and {}. Their product is {}",
                v1,
                v2,
                v3,
                v1 * v2 * v3
            );
        }
    }
    Ok(())
}
