use std::io;

pub(super) fn run() -> io::Result<()> {
    {
        // Part 1
        let total_fuel: u32 = crate::parse_lines::<u32, _>("2019_1.txt")?
            .into_iter()
            .map(|mass| mass / 3 - 2)
            .sum();
        println!("Total fuel requirement is {}", total_fuel);
    }
    {
        // Part 2
        let total_fuel: u32 = crate::parse_lines::<u32, _>("2019_1.txt")?
            .into_iter()
            .map(|mass| {
                let mut ret = 0;
                let mut next = (mass / 3).saturating_sub(2);
                while next > 0 {
                    ret += next;
                    next = (next / 3).saturating_sub(2);
                }
                ret
            })
            .sum();
        println!("Total fuel requirement is {}", total_fuel);
    }
    Ok(())
}
