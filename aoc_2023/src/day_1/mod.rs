use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn part1(input: &mut dyn BufRead) -> io::Result<u32> {
    input
        .lines()
        .map(|line| {
            let line = line?;
            let first_digit = line
                .bytes()
                .find(|b| b.is_ascii_digit())
                .unwrap_or_else(|| panic!("Line {line:?} doesn't contain any digits"))
                - b'0';
            let last_digit = line
                .bytes()
                .rfind(|b| b.is_ascii_digit())
                .unwrap_or_else(|| panic!("Line {line:?} doesn't contain any digits"))
                - b'0';
            let value = first_digit as u32 * 10 + last_digit as u32;
            Ok(value)
        })
        .try_fold(0, |acc, elem: io::Result<_>| Ok(acc + elem?))
}

fn part2(input: &mut dyn BufRead) -> io::Result<u32> {
    input
        .lines()
        .map(|line| {
            #[derive(Clone, Copy)]
            enum State {
                Nothing,
                O,
                On,
                T,
                Tw,
                Th,
                Thr,
                Thre,
                F,
                Fo,
                Fou,
                Fi,
                Fiv,
                S,
                Si,
                Se,
                Sev,
                Seve,
                E,
                Ei,
                Eig,
                Eigh,
                N,
                Ni,
                Nin,
                Done(u32),
            }

            #[derive(Clone, Copy)]
            enum RState {
                Nothing,
                E,
                En,
                O,
                Ow,
                Ee,
                Eer,
                Eerh,
                R,
                Ru,
                Ruo,
                Ev,
                Evi,
                X,
                Xi,
                N,
                Ne,
                Nev,
                Neve,
                T,
                Th,
                Thg,
                Thgi,
                Eni,
                Done(u32),
            }

            let line = line?;
            let first_digit =
                match line
                    .bytes()
                    .fold(State::Nothing, |acc, byte| match (acc, byte) {
                        (State::Done(_), _) => acc,
                        (_, b'0'..=b'9') => State::Done((byte - b'0') as u32),
                        (State::O, b'n') => State::On,
                        (State::T, b'w') => State::Tw,
                        (State::T, b'h') => State::Th,
                        (State::F, b'o') => State::Fo,
                        (State::F, b'i') => State::Fi,
                        (State::S, b'i') => State::Si,
                        (State::S, b'e') => State::Se,
                        (State::E, b'i') => State::Ei,
                        (State::N, b'i') => State::Ni,
                        (State::On, b'e') => State::Done(1),
                        (State::On, b'i') => State::Ni,
                        (State::Tw, b'o') => State::Done(2),
                        (State::Th, b'r') => State::Thr,
                        (State::Fo, b'u') => State::Fou,
                        (State::Fo, b'n') => State::On,
                        (State::Fi, b'v') => State::Fiv,
                        (State::Si, b'x') => State::Done(6),
                        (State::Se, b'v') => State::Sev,
                        (State::Se, b'i') => State::Ei,
                        (State::Ei, b'g') => State::Eig,
                        (State::Ni, b'n') => State::Nin,
                        (State::Thr, b'e') => State::Thre,
                        (State::Fou, b'r') => State::Done(4),
                        (State::Fiv, b'e') => State::Done(5),
                        (State::Sev, b'e') => State::Seve,
                        (State::Eig, b'h') => State::Eigh,
                        (State::Nin, b'e') => State::Done(9),
                        (State::Nin, b'i') => State::Ni,
                        (State::Thre, b'e') => State::Done(3),
                        (State::Thre, b'i') => State::Ei,
                        (State::Seve, b'n') => State::Done(7),
                        (State::Seve, b'i') => State::Ei,
                        (State::Eigh, b't') => State::Done(8),
                        (_, b'o') => State::O,
                        (_, b't') => State::T,
                        (_, b'f') => State::F,
                        (_, b's') => State::S,
                        (_, b'e') => State::E,
                        (_, b'n') => State::N,
                        (_, _) => State::Nothing,
                    }) {
                    State::Done(d) => d,
                    _ => panic!("Coludn't find first digit in {line:?}"),
                };
            let last_digit =
                match line
                    .bytes()
                    .rev()
                    .fold(RState::Nothing, |acc, byte| match (acc, byte) {
                        (RState::Done(_), _) => acc,
                        (_, b'0'..=b'9') => RState::Done((byte - b'0') as u32),
                        (RState::E, b'n') => RState::En,
                        (RState::E, b'e') => RState::Ee,
                        (RState::E, b'v') => RState::Ev,
                        (RState::O, b'w') => RState::Ow,
                        (RState::R, b'u') => RState::Ru,
                        (RState::X, b'i') => RState::Xi,
                        (RState::N, b'e') => RState::Ne,
                        (RState::T, b'h') => RState::Th,
                        (RState::En, b'o') => RState::Done(1),
                        (RState::En, b'i') => RState::Eni,
                        (RState::En, b'e') => RState::Ne,
                        (RState::Ee, b'r') => RState::Eer,
                        (RState::Ee, b'e') => RState::Ee,
                        (RState::Ee, b'n') => RState::En,
                        (RState::Ee, b'v') => RState::Ev,
                        (RState::Ev, b'i') => RState::Evi,
                        (RState::Ow, b't') => RState::Done(2),
                        (RState::Ru, b'o') => RState::Ruo,
                        (RState::Xi, b's') => RState::Done(6),
                        (RState::Ne, b'v') => RState::Nev,
                        (RState::Ne, b'e') => RState::Ee,
                        (RState::Ne, b'n') => RState::En,
                        (RState::Th, b'g') => RState::Thg,
                        (RState::Eni, b'n') => RState::Done(9),
                        (RState::Eer, b'h') => RState::Eerh,
                        (RState::Evi, b'f') => RState::Done(5),
                        (RState::Ruo, b'f') => RState::Done(4),
                        (RState::Ruo, b'w') => RState::Ow,
                        (RState::Nev, b'e') => RState::Neve,
                        (RState::Nev, b'i') => RState::Evi,
                        (RState::Thg, b'i') => RState::Thgi,
                        (RState::Eerh, b't') => RState::Done(3),
                        (RState::Neve, b's') => RState::Done(7),
                        (RState::Neve, b'e') => RState::Ee,
                        (RState::Neve, b'n') => RState::En,
                        (RState::Neve, b'v') => RState::Ev,
                        (RState::Thgi, b'e') => RState::Done(8),
                        (_, b'e') => RState::E,
                        (_, b'o') => RState::O,
                        (_, b'r') => RState::R,
                        (_, b'x') => RState::X,
                        (_, b'n') => RState::N,
                        (_, b't') => RState::T,
                        (_, _) => RState::Nothing,
                    }) {
                    RState::Done(d) => d,
                    _ => panic!("Couldn't find last digit in {line:?}"),
                };
            Ok(10 * first_digit + last_digit)
        })
        .try_fold(0, |acc, elem: io::Result<u32>| Ok(acc + elem?))
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2023 Day 1 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2023_01.txt")?))?
        );
    }
    {
        println!("Year 2023 Day 1 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2023_01.txt")?))?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const TEST_DATA: &str = concat!("1abc2\n", "pqr3stu8vwx\n", "a1b2c3d4e5f\n", "treb7uchet\n");
    const TEST_DATA_2: &str = concat!(
        "two1nine\n",
        "eightwothree\n",
        "abcone2threexyz\n",
        "xtwone3four\n",
        "4nineeightseven2\n",
        "zoneight234\n",
        "7pqrstsixteen\n",
    );

    #[test]
    fn test_part1() -> io::Result<()> {
        let expected = 142;
        let actual = part1(&mut Cursor::new(TEST_DATA))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part2() -> io::Result<()> {
        let expected = 281;
        let actual = part2(&mut Cursor::new(TEST_DATA_2))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
