use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    num::ParseIntError,
    ops::RangeInclusive,
};

#[derive(Clone, Debug, Eq, PartialEq)]
struct Target {
    target_x: RangeInclusive<u32>,
    target_y: RangeInclusive<i32>,
}

impl Target {
    fn read(input: &mut dyn BufRead) -> io::Result<Self> {
        let mut buf = String::new();
        let _ = input.read_line(&mut buf)?;
        let coordinates = buf
            .trim()
            .strip_prefix("target area: ")
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing prefix"))?;
        let (x_coord, y_coord) = coordinates.split_once(", ").ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "Missing coordinate separator")
        })?;
        let x_coord = x_coord
            .strip_prefix("x=")
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing x"))?;
        let (min_x, max_x) = x_coord.split_once("..").ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "Format for x range invalid")
        })?;
        let min_x = min_x.parse().map_err(|e: ParseIntError| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Error parsing minimum x: {:?}", e.to_string()),
            )
        })?;
        let max_x = max_x.parse().map_err(|e: ParseIntError| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Error parsing maximum x: {:?}", e.to_string()),
            )
        })?;
        let y_coord = y_coord
            .strip_prefix("y=")
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing y"))?;
        let (min_y, max_y) = y_coord.split_once("..").ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "Format for y range invalid")
        })?;
        let min_y = min_y.parse().map_err(|e: ParseIntError| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Error parsing minimum y: {:?}", e.to_string()),
            )
        })?;
        let max_y = max_y.parse().map_err(|e: ParseIntError| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Error parsing maximum y: {:?}", e.to_string()),
            )
        })?;
        Ok(Self {
            target_x: min_x..=max_x,
            target_y: min_y..=max_y,
        })
    }
}

impl Target {
    fn naive_max_upward_velocity(&self) -> i32 {
        if *self.target_y.end() < 0 {
            -self.target_y.start()
        } else {
            // T_n = (n * (n + 1)) / 2
            // T_n <= k => (n**2 + n) <= 2k
            //
            // n <= floor((-1 + sqrt(1 + 8k))/2)
            let max_up =
                (((1.0 + 8.0 * f64::from(*self.target_x.end())).sqrt() - 1.0) / 2.0).floor() as i32;
            assert!(self.target_y.contains(&((max_up * (max_up + 1)) / 2)));
            max_up
        }
    }
}

fn part1(input: &mut dyn BufRead) -> io::Result<u32> {
    fn vertical_finds(target_y: RangeInclusive<i32>, up: i32) -> bool {
        let mut velocity = -up;
        let mut position = up;
        while position > *target_y.end() {
            position += velocity;
            velocity -= 1;
        }
        position >= *target_y.start()
    }

    let target = Target::read(input)?;
    dbg!(&target);
    // T_n = (n * (n + 1)) / 2
    // T_n >= k => n**2 + n >= 2k
    //
    // n >= ceil((-1 + sqrt(1 + 8k))/2)
    let min_forward =
        (((1.0 + 8.0 * f64::from(*target.target_x.start())).sqrt() - 1.0) / 2.0).ceil() as u32;
    let last_x = (min_forward * (min_forward + 1)) / 2;
    assert!(target.target_x.contains(&last_x));
    let max_up = if *target.target_y.end() < 0 {
        (0..=(-target.target_y.start()))
            .filter(|&i| vertical_finds(target.target_y.clone(), i))
            .last()
            .unwrap()
    } else {
        target.naive_max_upward_velocity()
    } as u32;
    Ok((max_up * (max_up + 1)) / 2)
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    let target = Target::read(input)?;
    // T_n = (n * (n + 1)) / 2
    // T_n >= k => n**2 + n >= 2k
    //
    // n >= ceil((-1 + sqrt(1 + 8k))/2)
    let min_forward =
        (((1.0 + 8.0 * f64::from(*target.target_x.start())).sqrt() - 1.0) / 2.0).ceil() as u32;
    let deltas = (min_forward..=*target.target_x.end()).flat_map(|dx| {
        (*target.target_y.start()..=target.naive_max_upward_velocity()).map(move |dy| (dx, dy))
    });
    Ok(deltas
        .filter(|&(mut dx, mut dy)| {
            let (mut x, mut y) = (0, 0);
            while x < *target.target_x.end() && (dy > 0 || y > *target.target_y.start()) {
                x += dx;
                y += dy;
                if dx > 0 {
                    dx -= 1;
                }
                dy -= 1;
                if target.target_x.contains(&x) && target.target_y.contains(&y) {
                    return true;
                }
            }
            false
        })
        .count())
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2021 Day 17 Part 1");
        println!(
            "The highest y position is {}",
            part1(&mut BufReader::new(File::open("2021_17.txt")?))?
        );
    }
    {
        println!("Year 2021 Day 17 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2021_17.txt")?))?
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
        let s = "target area: x=20..30, y=-10..-5";
        let expected = 45;
        let actual = part1(&mut Cursor::new(s))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part2() -> io::Result<()> {
        let s = "target area: x=20..30, y=-10..-5";
        let expected = 112;
        let actual = part2(&mut Cursor::new(s))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
