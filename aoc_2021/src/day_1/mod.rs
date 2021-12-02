use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2021 Day 1 Part 1");
        let input = File::open("2021_01.txt")?;
        let mut num_increases = 0;
        let mut last_depth = None;
        for line in BufReader::new(input).lines() {
            let depth = line?
                .parse::<u32>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            match last_depth {
                Some(last_depth) if last_depth < depth => num_increases += 1,
                _ => {}
            }
            last_depth = Some(depth);
        }
        println!("{}", num_increases);
    }
    {
        println!("Year 2021 Day 1 Part 2");
        let input = File::open("2021_01.txt")?;
        let mut num_increases = 0;
        let mut last_depths = [None, None, None];
        for line in BufReader::new(input).lines() {
            let depth = line?
                .parse::<u32>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            match last_depths[0] {
                // `x` only exists if the other two elements of last_depths also exist.
                Some(x) if x < depth => num_increases += 1,
                _ => {}
            }
            let new_depths = [last_depths[1], last_depths[2], Some(depth)];
            last_depths = new_depths;
        }
        println!("{}", num_increases);
    }
    Ok(())
}
