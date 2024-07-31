use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn part1(input: &mut dyn BufRead) -> io::Result<u32> {
    let (numbers, parts) = input
        .lines()
        .map::<io::Result<_>, _>(|line| {
            let line = line?;
            let mut numbers = vec![];
            let mut parts = vec![];
            let mut idx = 0;
            let bytes = line.as_bytes();
            let mut number_start = None;
            let mut number_in_progress = 0;
            while idx < bytes.len() {
                if bytes[idx].is_ascii_digit() {
                    if number_start.is_none() {
                        number_start = Some(idx);
                    }
                    number_in_progress = number_in_progress * 10 + (bytes[idx] - b'0') as u32;
                } else {
                    if let Some(start) = number_start.take() {
                        numbers.push((number_in_progress, start..idx));
                        number_in_progress = 0;
                    }
                    if bytes[idx] != b'.' {
                        parts.push(idx);
                    }
                }
                idx += 1;
            }
            if let Some(start) = number_start {
                numbers.push((number_in_progress, start..idx));
            }
            Ok((numbers, parts))
        })
        .try_fold::<_, _, io::Result<_>>(
            (vec![], vec![]),
            |(mut acc_numbers, mut acc_parts), elem| {
                let (numbers, parts) = elem?;
                acc_numbers.push(numbers);
                acc_parts.push(parts);
                Ok((acc_numbers, acc_parts))
            },
        )?;
    let mut part_numbers = vec![];
    for row in 0..numbers.len() {
        for &(number, ref number_span) in &numbers[row] {
            if parts[row.saturating_sub(1)..(row + 2).min(parts.len())]
                .iter()
                .flatten()
                .any(|part_column| {
                    (number_span.start.saturating_sub(1)..number_span.end.saturating_add(1))
                        .contains(part_column)
                })
            {
                part_numbers.push(number);
            }
        }
    }
    Ok(part_numbers.into_iter().sum())
}

fn part2(input: &mut dyn BufRead) -> io::Result<u32> {
    let (numbers, parts) = input
        .lines()
        .map::<io::Result<_>, _>(|line| {
            let line = line?;
            let mut numbers = vec![];
            let mut parts = vec![];
            let mut idx = 0;
            let bytes = line.as_bytes();
            let mut number_start = None;
            let mut number_in_progress = 0;
            while idx < bytes.len() {
                if bytes[idx].is_ascii_digit() {
                    if number_start.is_none() {
                        number_start = Some(idx);
                    }
                    number_in_progress = number_in_progress * 10 + (bytes[idx] - b'0') as u32;
                } else {
                    if let Some(start) = number_start.take() {
                        numbers.push((number_in_progress, start..idx));
                        number_in_progress = 0;
                    }
                    if bytes[idx] == b'*' {
                        parts.push(idx);
                    }
                }
                idx += 1;
            }
            if let Some(start) = number_start {
                numbers.push((number_in_progress, start..idx));
            }
            Ok((numbers, parts))
        })
        .try_fold::<_, _, io::Result<_>>(
            (vec![], vec![]),
            |(mut acc_numbers, mut acc_parts), elem| {
                let (numbers, parts) = elem?;
                acc_numbers.push(numbers);
                acc_parts.push(parts);
                Ok((acc_numbers, acc_parts))
            },
        )?;
    let mut gear_numbers = vec![];
    for (row, parts_row) in parts.iter().enumerate() {
        let mut part_numbers = vec![];
        for &part_column in parts_row {
            part_numbers.clear();
            let part_neighborhood = part_column.saturating_sub(1)..=part_column.saturating_add(1);
            (row.saturating_sub(1)..(row.saturating_add(2)).min(numbers.len())).for_each(|row| {
                part_numbers.extend(numbers[row].iter().cloned().filter_map(
                    |(number, number_span)| {
                        Some(number).filter(|_| {
                            number_span.start <= *part_neighborhood.end()
                                && *part_neighborhood.start() < number_span.end
                        })
                    },
                ));
            });
            if let &[number1, number2] = &part_numbers[..] {
                gear_numbers.push([number1, number2]);
            }
        }
    }
    Ok(gear_numbers.into_iter().map(|[a, b]| a * b).sum())
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2023 Day 3 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2023_03.txt")?))?
        );
    }
    {
        println!("Year 2023 Day 3 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2023_03.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!(
        "467..114..\n",
        "...*......\n",
        "..35..633.\n",
        "......#...\n",
        "617*......\n",
        ".....+.58.\n",
        "..592.....\n",
        "......755.\n",
        "...$.*....\n",
        ".664.598..\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 4361;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 467835;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
