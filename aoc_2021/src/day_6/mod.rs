use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    mem,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct LanternfishTimers {
    zero_remaining: u64,
    one_remaining: u64,
    two_remaining: u64,
    three_remaining: u64,
    four_remaining: u64,
    five_remaining: u64,
    six_remaining: u64,
    seven_remaining: u64,
    eight_remaining: u64,
}

impl LanternfishTimers {
    pub fn total_fish(&self) -> u64 {
        self.zero_remaining
            + self.one_remaining
            + self.two_remaining
            + self.three_remaining
            + self.four_remaining
            + self.five_remaining
            + self.six_remaining
            + self.seven_remaining
            + self.eight_remaining
    }

    pub fn tick(&mut self) {
        mem::swap(&mut self.zero_remaining, &mut self.one_remaining);
        mem::swap(&mut self.one_remaining, &mut self.two_remaining);
        mem::swap(&mut self.two_remaining, &mut self.three_remaining);
        mem::swap(&mut self.three_remaining, &mut self.four_remaining);
        mem::swap(&mut self.four_remaining, &mut self.five_remaining);
        mem::swap(&mut self.five_remaining, &mut self.six_remaining);
        mem::swap(&mut self.six_remaining, &mut self.seven_remaining);
        mem::swap(&mut self.seven_remaining, &mut self.eight_remaining);
        self.six_remaining += self.eight_remaining;
    }
}

impl LanternfishTimers {
    fn read(input: &mut dyn BufRead) -> io::Result<Self> {
        let mut timers = LanternfishTimers::default();
        for i in read_line(&mut *input)?.split(',') {
            match i.trim().parse() {
                Ok(0) => timers.zero_remaining += 1,
                Ok(1) => timers.one_remaining += 1,
                Ok(2) => timers.two_remaining += 1,
                Ok(3) => timers.three_remaining += 1,
                Ok(4) => timers.four_remaining += 1,
                Ok(5) => timers.five_remaining += 1,
                Ok(6) => timers.six_remaining += 1,
                Ok(7) => timers.seven_remaining += 1,
                Ok(8) => timers.eight_remaining += 1,
                Ok(i) => Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Invalid timer: {i}"),
                ))?,
                Err(e) => Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Invalid timer {i:?}: {e:?}"),
                ))?,
            }
        }
        Ok(timers)
    }
}

fn read_line(input: &mut dyn BufRead) -> io::Result<String> {
    let mut buf = String::new();
    input.read_line(&mut buf)?;
    Ok(buf)
}

fn part1(input: &mut dyn BufRead) -> io::Result<u64> {
    let mut timers = LanternfishTimers::read(input)?;
    for _ in 0..80 {
        timers.tick();
    }
    Ok(timers.total_fish())
}

fn part2(input: &mut dyn BufRead) -> io::Result<u64> {
    let mut timers = LanternfishTimers::read(input)?;
    for _ in 0..256 {
        timers.tick();
    }
    Ok(timers.total_fish())
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2021 Day 6 Part 1");
        println!(
            "After 80 days, the number of lanternfish would be {}",
            part1(&mut BufReader::new(File::open("2021_06.txt")?))?,
        );
    }
    {
        println!("Year 2021 Day 6 Part 2");
        println!(
            "{:?}",
            part2(&mut BufReader::new(File::open("2021_06.txt")?))?,
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
        let s = "3,4,3,1,2";
        let expected = 5934;
        let actual = part1(&mut Cursor::new(s))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part2() -> io::Result<()> {
        let s = "3,4,3,1,2";
        let expected = 26_984_457_539;
        let actual = part2(&mut Cursor::new(s))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
