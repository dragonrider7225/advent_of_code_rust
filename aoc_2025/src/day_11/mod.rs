use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
    iter::Sum,
    ops::Add,
    str::FromStr,
};

#[derive(Clone, Debug)]
struct Device {
    name: String,
    outputs: Vec<String>,
}

impl Device {
    pub fn paths_to_out<'d, F>(&self, devices: F) -> usize
    where
        F: FnMut(&str) -> Option<&'d Self>,
        F: Clone,
    {
        fn go<'d, F>(this: &Device, mut devices: F, cache: &mut HashMap<String, usize>) -> usize
        where
            F: FnMut(&str) -> Option<&'d Device>,
            F: Clone,
        {
            match cache.get(&this.name) {
                Some(&n) => n,
                None => {
                    let total = this
                        .outputs
                        .iter()
                        .map(|output| match devices(output) {
                            Some(output) => go(output, devices.clone(), cache),
                            None => {
                                debug_assert_eq!(output, "out");
                                1
                            }
                        })
                        .sum();
                    cache.insert(this.name.clone(), total);
                    total
                }
            }
        }

        go(
            self,
            devices,
            &mut [("out".to_string(), 1)].into_iter().collect(),
        )
    }

    pub fn paths_to_out_through_dac_fft<'d, F>(&self, devices: F) -> usize
    where
        F: FnMut(&str) -> Option<&'d Self>,
        F: Clone,
    {
        #[derive(Clone, Copy, Debug, Default)]
        struct SubResult {
            with_neither: usize,
            with_dac: usize,
            with_fft: usize,
            with_both: usize,
        }

        impl SubResult {
            pub fn add_fft(&mut self) {
                self.with_both += std::mem::take(&mut self.with_dac);
                self.with_fft += std::mem::take(&mut self.with_neither);
            }

            pub fn add_dac(&mut self) {
                self.with_both += std::mem::take(&mut self.with_fft);
                self.with_dac += std::mem::take(&mut self.with_neither);
            }
        }

        impl Add for SubResult {
            type Output = Self;

            fn add(mut self, rhs: Self) -> Self::Output {
                self.with_neither += rhs.with_neither;
                self.with_dac += rhs.with_dac;
                self.with_fft += rhs.with_fft;
                self.with_both += rhs.with_both;
                self
            }
        }

        impl Sum<Self> for SubResult {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Self::default(), |acc, elem| acc + elem)
            }
        }

        fn go<'d, F>(
            this: &Device,
            mut devices: F,
            cache: &mut HashMap<String, SubResult>,
        ) -> SubResult
        where
            F: FnMut(&str) -> Option<&'d Device>,
            F: Clone,
        {
            #[cfg(test)]
            eprintln!("Counting paths from {}", this.name);
            match cache.get(&this.name) {
                Some(&total) => total,
                None => {
                    let mut total = this
                        .outputs
                        .iter()
                        .map(|output| match devices(output) {
                            Some(output) => go(output, devices.clone(), cache),
                            None => {
                                debug_assert_eq!(output, "out");
                                cache["out"]
                            }
                        })
                        .sum::<SubResult>();
                    match &*this.name {
                        "fft" => total.add_fft(),
                        "dac" => total.add_dac(),
                        _ => {}
                    }
                    #[cfg(test)]
                    eprintln!("Total is {total:?}");
                    cache.insert(this.name.clone(), total);
                    total
                }
            }
        }

        go(
            self,
            devices,
            &mut [(
                "out".to_string(),
                SubResult {
                    with_neither: 1,
                    ..Default::default()
                },
            )]
            .into_iter()
            .collect(),
        )
        .with_both
    }
}

impl FromStr for Device {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((name, outputs)) = s.split_once(": ") else {
            return Err(format!("Missing name-connections separator in {s:?}"));
        };
        Ok(Self {
            name: name.into(),
            outputs: outputs.split_whitespace().map(str::to_string).collect(),
        })
    }
}

fn part1(input: &mut dyn BufRead) -> io::Result<usize> {
    let devices = input
        .lines()
        .map(|line| {
            line?
                .parse::<Device>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        })
        .try_fold(HashMap::new(), |mut acc, device| -> io::Result<_> {
            let device = device?;
            if let Some(entry) = acc.insert(device.name.clone(), device) {
                eprintln!("Duplicate device name: {entry:?}");
            }
            Ok(acc)
        })?;
    Ok(devices["you"].paths_to_out(|name| devices.get(name)))
}

fn part2(input: &mut dyn BufRead) -> io::Result<usize> {
    let devices = input
        .lines()
        .map(|line| {
            line?
                .parse::<Device>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        })
        .try_fold(HashMap::new(), |mut acc, device| -> io::Result<_> {
            let device = device?;
            if let Some(entry) = acc.insert(device.name.clone(), device) {
                eprintln!("Duplicate device name: {entry:?}");
            }
            Ok(acc)
        })?;
    Ok(devices["svr"].paths_to_out_through_dac_fft(|name| devices.get(name)))
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2025 Day 11 Part 1");
        println!(
            "{:?}",
            part1(&mut BufReader::new(File::open("2025_11.txt")?))?
        );
    }
    {
        println!("Year 2025 Day 11 Part 2");
        println!(
            "{:?}",
            part2(&mut BufReader::new(File::open("2025_11.txt")?))?
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
        const TEST_DATA: &str = concat!(
            "aaa: you hhh\n",
            "you: bbb ccc\n",
            "bbb: ddd eee\n",
            "ccc: ddd eee fff\n",
            "ddd: ggg\n",
            "eee: out\n",
            "fff: out\n",
            "ggg: out\n",
            "hhh: ccc fff iii\n",
            "iii: out\n",
        );
        let expected = 5;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        const TEST_DATA: &str = concat!(
            "svr: aaa bbb\n",
            "aaa: fft\n",
            "fft: ccc\n",
            "bbb: tty\n",
            "tty: ccc\n",
            "ccc: ddd eee\n",
            "ddd: hub\n",
            "hub: fff\n",
            "eee: dac\n",
            "dac: fff\n",
            "fff: ggg hhh\n",
            "ggg: out\n",
            "hhh: out\n",
        );
        let expected = 2;
        let actual = part2(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
