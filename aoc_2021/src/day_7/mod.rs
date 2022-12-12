use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn read_positions(input: &mut dyn BufRead) -> io::Result<Vec<usize>> {
    let line = {
        let mut buf = String::new();
        input.read_line(&mut buf)?;
        buf
    };
    line.split(',')
        .map(|s| {
            s.trim()
                .parse()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        })
        .collect()
}

fn count_fuel(positions: &[usize], position: usize) -> usize {
    positions
        .iter()
        .map(|&crab| {
            if crab < position {
                position - crab
            } else {
                crab - position
            }
        })
        .sum()
}

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    let mut positions = read_positions(input)?;
    let num_positions = positions.len();
    positions.sort();
    Ok(count_fuel(&positions, positions[num_positions / 2]))
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    fn calculate_fuel(positions: &[usize], position: usize) -> usize {
        positions
            .iter()
            .map(|&crab| {
                let distance = if crab < position {
                    position - crab
                } else {
                    crab - position
                };
                (distance * (distance + 1)) / 2
            })
            .sum()
    }

    let positions = read_positions(input)?;
    let mean = positions.iter().copied().sum::<usize>() as f64 / positions.len() as f64;
    let initial_guess = mean.round() as usize;
    let initial_guess_fuel = calculate_fuel(&positions, initial_guess);
    let less_than_mean_fuel = calculate_fuel(&positions, initial_guess - 1);
    if less_than_mean_fuel < initial_guess_fuel {
        let result = (2..=initial_guess)
            .map(|delta| initial_guess - delta)
            .try_fold(less_than_mean_fuel, |acc, guess| {
                let new_fuel = calculate_fuel(&positions, guess);
                if new_fuel < acc {
                    Ok(new_fuel)
                } else {
                    Err(acc)
                }
            });
        Ok(result.unwrap_or_else(|e| e))
    } else {
        let greater_than_mean_fuel = calculate_fuel(&positions, initial_guess + 1);
        if greater_than_mean_fuel < initial_guess_fuel {
            let result = (2..=(positions.len() - initial_guess))
                .map(|delta| initial_guess + delta)
                .try_fold(greater_than_mean_fuel, |acc, guess| {
                    let new_fuel = calculate_fuel(&positions, guess);
                    if new_fuel < acc {
                        Ok(new_fuel)
                    } else {
                        Err(acc)
                    }
                });
            Ok(result.unwrap_or_else(|e| e))
        } else {
            Ok(initial_guess_fuel)
        }
    }
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2021 Day 7 Part 1");
        println!(
            "Total fuel is {}",
            part1(&mut BufReader::new(File::open("2021_07.txt")?))?
        );
    }
    {
        println!("Year 2021 Day 7 Part 2");
        println!(
            "{:?}",
            part2(&mut BufReader::new(File::open("2021_07.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    #[ignore]
    fn test_part1() -> io::Result<()> {
        let s = "16,1,2,0,4,2,7,1,2,14";
        let expected = 37;
        let actual = part1(&mut Cursor::new(s))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part2() -> io::Result<()> {
        let s = "16,1,2,0,4,2,7,1,2,14";
        let expected = 168;
        let actual = part2(&mut Cursor::new(s))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
