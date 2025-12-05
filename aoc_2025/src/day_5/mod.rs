use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    let ingredient_id_ranges = input.lines()
        .take_while(|line| !line.as_ref().is_ok_and(String::is_empty))
        .map(|range| {
            range.and_then(|range| {
                let Some((start, end)) = range.split_once('-') else {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, format!("{range:?} is not a valid range: Ranges must consist of digits and a single '-'")));
                };
                let start = start.parse::<usize>().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("{e:?}: {start:?} is not a number")))?;
                let end = end.parse::<usize>().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("{e:?}: {end:?} is not a number")))?;
                Ok(start..=end)
            })
        })
        .collect::<io::Result<Vec<_>>>()?;
    let available_ingredient_ids = input
        .lines()
        .map(|line| {
            line.and_then(|line| {
                line.parse::<usize>().map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("{e:?}: {line:?} is not a number"),
                    )
                })
            })
        })
        .collect::<io::Result<Vec<_>>>()?;
    Ok(available_ingredient_ids
        .into_iter()
        .filter(|id| ingredient_id_ranges.iter().any(|range| range.contains(id)))
        .count())
}

fn slice_swap_remove<T>(slice: &mut [T], idx: usize) -> &mut [T] {
    let last_idx = slice.len() - 1;
    slice.swap(idx, last_idx);
    &mut slice[..last_idx]
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    let mut ingredient_id_ranges = input.lines()
        .take_while(|line| !line.as_ref().is_ok_and(String::is_empty))
        .map(|range| {
            range.and_then(|range| {
                let Some((start, end)) = range.split_once('-') else {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, format!("{range:?} is not a valid range: Ranges must consist of digits and a single '-'")));
                };
                let start = start.parse::<usize>().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("{e:?}: {start:?} is not a number")))?;
                let end = end.parse::<usize>().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("{e:?}: {end:?} is not a number")))?;
                Ok(start..=end)
            })
        })
        .collect::<io::Result<Vec<_>>>()?;
    ingredient_id_ranges.sort_unstable_by_key(|range| *range.start());
    let mut i = 0;
    while i < ingredient_id_ranges.len() {
        let (before, mut after) = ingredient_id_ranges.split_at_mut(i + 1);
        let current = before.last_mut().unwrap();
        while let Some(j) = after
            .iter()
            .position(|range| current.contains(range.start()))
        {
            *current = (*current.start())..=((*current.end()).max(*after[j].end()));
            after = slice_swap_remove(after, j);
        }
        after.sort_unstable_by_key(|range| *range.start());
        let num_kept = before.len() + after.len();
        ingredient_id_ranges.truncate(num_kept);
        i += 1;
    }
    Ok(ingredient_id_ranges.into_iter().map(Iterator::count).sum())
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2025 Day 5 Part 1");
        println!(
            "{:?}",
            part1(&mut BufReader::new(File::open("2025_05.txt")?))?
        );
    }
    {
        println!("Year 2025 Day 5 Part 2");
        println!(
            "{:?}",
            part2(&mut BufReader::new(File::open("2025_05.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32\n";

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 3;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 14;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
