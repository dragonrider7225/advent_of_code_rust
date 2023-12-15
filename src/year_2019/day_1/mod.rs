use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub(super) fn run() -> io::Result<()> {
    {
        // Part 1
        let total_fuel: u32 = BufReader::new(File::open("2019_1.txt")?)
            .lines()
            .map(|line| {
                line?
                    .parse::<u32>()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            })
            .map(|mass| Ok(mass? / 3 - 2))
            .sum::<io::Result<_>>()?;
        println!("Total fuel requirement is {total_fuel}");
    }
    {
        // Part 2
        let total_fuel: u32 = BufReader::new(File::open("2019_1.txt")?)
            .lines()
            .map(|line| {
                line?
                    .parse::<u32>()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            })
            .map(|mass| {
                let mass = mass?;
                let mut ret = 0;
                let mut next = (mass / 3).saturating_sub(2);
                while next > 0 {
                    ret += next;
                    next = (next / 3).saturating_sub(2);
                }
                Ok(ret)
            })
            .sum::<io::Result<_>>()?;
        println!("Total fuel requirement is {total_fuel}");
    }
    Ok(())
}
