use std::{
    cmp::Ordering,
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader},
    iter,
};

fn part1(input: &mut dyn BufRead) -> io::Result<u64> {
    let ranges = input
        .lines()
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing notes"))?
        .and_then(|line| {
            line.split(',')
                .map(|range| {
                    let Some((start, end)) = range.split_once('-') else {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Range {range:?} missing '-'",
                        ));
                    };
                    Ok((start.to_string(), end.to_string()))
                })
                .collect::<io::Result<Vec<_>>>()
        })?;
    ranges
        .into_iter()
        .map(|(range_start, range_end)| {
            let min_width = range_start.len().div_ceil(2);
            let max_width = range_end.len() / 2;
            let half_start = if min_width * 2 == range_start.len() {
                let start_high = range_start[..min_width].parse::<u64>().map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("{e:?}: Start of range {range_start:?} must be a number"),
                    )
                })?;
                let start_low = range_start[min_width..].parse::<u64>().map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("{e:?}: Start of range {range_start:?} must be a number"),
                    )
                })?;
                start_high + u64::from(start_low > start_high)
            } else {
                10u64.pow((min_width - 1) as _)
            };
            let half_end = if max_width * 2 == range_end.len() {
                let end_high = range_end[..min_width].parse::<u64>().map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("{e:?}: End of range {range_end:?} must be a number"),
                    )
                })?;
                let end_low = range_end[min_width..].parse::<u64>().map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("{e:?}: End of range {range_end:?} must be a number"),
                    )
                })?;
                end_high - u64::from(end_low < end_high)
            } else {
                10u64.pow(max_width as _) - 1
            };
            Ok((half_start..=half_end)
                .map(|n| {
                    let width = n.ilog10() + 1;
                    n * 10u64.pow(width) + n
                })
                .sum::<u64>())
        })
        .try_fold(0, |acc, total| total.map(|n| acc + n))
}

fn split(n: u64, pieces: u32) -> impl Iterator<Item = u64> {
    let width = n.ilog10() + 1;
    let piece_width = width / pieces;
    let div_rem_denominator = 10u64.pow(piece_width);
    let (remainder, mut pieces) = (0..pieces).fold((n, vec![]), |(remainder, mut acc), _| {
        acc.push(remainder % div_rem_denominator);
        (remainder / div_rem_denominator, acc)
    });
    pieces.reverse();
    assert_eq!(remainder, 0, "{remainder} {pieces:?}");
    pieces.into_iter()
}

fn part2(input: &mut dyn BufRead) -> io::Result<u64> {
    let ranges = input
        .lines()
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing notes"))?
        .and_then(|line| {
            line.split(',')
                .map(|range| {
                    let Some((start, end)) = range.split_once('-') else {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Range {range:?} missing '-'",
                        ));
                    };
                    Ok((
                        start.parse::<u64>().map_err(|e| {
                            io::Error::new(
                                io::ErrorKind::InvalidData,
                                format!("{e:?}: Range start {start:?} must be a number"),
                            )
                        })?,
                        end.parse::<u64>().map_err(|e| {
                            io::Error::new(
                                io::ErrorKind::InvalidData,
                                format!("{e:?}: Range end {end:?} must be a number"),
                            )
                        })?,
                    ))
                })
                .collect::<io::Result<Vec<_>>>()
        })?;
    Ok(ranges
        .into_iter()
        .flat_map(|(range_start, range_end)| {
            let min_width = range_start.ilog10() + 1;
            let max_width = range_end.ilog10() + 1;
            (2..=max_width).flat_map(move |reps| {
                (min_width..=max_width)
                    .filter(move |width| width.is_multiple_of(reps))
                    .flat_map(move |width| {
                        let sub_width = width / reps;
                        let start = if width == min_width {
                            let (Some(head), ord) = split(range_start, reps).fold(
                                (None, Ordering::Equal),
                                |(head, acc), piece| match head {
                                    None => (Some(piece), acc),
                                    Some(head) => (Some(head), acc.then_with(|| head.cmp(&piece))),
                                },
                            ) else {
                                unreachable!("reps cannot be 0");
                            };
                            head + u64::from(matches!(ord, Ordering::Less))
                        } else {
                            10u64.pow(sub_width - 1)
                        };
                        let end = if width == max_width {
                            let (Some(head), ord) = split(range_end, reps).fold(
                                (None, Ordering::Equal),
                                |(head, acc), piece| match head {
                                    None => (Some(piece), acc),
                                    Some(head) => (Some(head), acc.then_with(|| head.cmp(&piece))),
                                },
                            ) else {
                                unreachable!("reps cannot be 0");
                            };
                            head - u64::from(matches!(ord, Ordering::Greater))
                        } else {
                            10u64.pow(sub_width) - 1
                        };
                        (start..=end).map(move |piece| {
                            iter::repeat_n(piece, reps as _)
                                .fold(0, |acc, piece| acc * 10u64.pow(sub_width) + piece)
                        })
                    })
            })
        })
        .collect::<HashSet<_>>()
        .into_iter()
        .sum())
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2025 Day 2 Part 1");
        println!(
            "{:?}",
            part1(&mut BufReader::new(File::open("2025_02.txt")?))?
        );
    }
    {
        println!("Year 2025 Day 2 Part 2");
        println!(
            "{:?}",
            part2(&mut BufReader::new(File::open("2025_02.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124\n";

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 1227775554;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 4_174_379_265;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
