use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Delimiter {
    Parenthesis(bool),
    Bracket(bool),
    Brace(bool),
    Angle(bool),
}

impl Delimiter {
    fn syntax_value(&self) -> u32 {
        match self {
            Self::Parenthesis(_) => 3,
            Self::Bracket(_) => 57,
            Self::Brace(_) => 1197,
            Self::Angle(_) => 25137,
        }
    }

    fn autocomplete_value(&self) -> u64 {
        match self {
            Self::Parenthesis(_) => 1,
            Self::Bracket(_) => 2,
            Self::Brace(_) => 3,
            Self::Angle(_) => 4,
        }
    }

    fn matches(&self, rhs: &Self) -> bool {
        matches!(
            (self, rhs),
            (Self::Parenthesis(false), Self::Parenthesis(true))
                | (Self::Bracket(false), Self::Bracket(true))
                | (Self::Brace(false), Self::Brace(true))
                | (Self::Angle(false), Self::Angle(true))
        )
    }
}

impl TryFrom<char> for Delimiter {
    type Error = io::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '(' => Ok(Self::Parenthesis(false)),
            ')' => Ok(Self::Parenthesis(true)),
            '[' => Ok(Self::Bracket(false)),
            ']' => Ok(Self::Bracket(true)),
            '{' => Ok(Self::Brace(false)),
            '}' => Ok(Self::Brace(true)),
            '<' => Ok(Self::Angle(false)),
            '>' => Ok(Self::Angle(true)),
            c => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid chunk delimiter {c:?}"),
            )),
        }
    }
}

fn part1(input: &mut dyn BufRead) -> io::Result<u32> {
    use Delimiter::{Angle, Brace, Bracket, Parenthesis};
    input
        .lines()
        .map(|line| {
            let line = line?;
            let mut stack = vec![];
            for c in line.chars().map(Delimiter::try_from) {
                let c = c?;
                match c {
                    Parenthesis(false) | Bracket(false) | Brace(false) | Angle(false) => {
                        stack.push(c)
                    }
                    c if stack.pop().filter(|r#match| r#match.matches(&c)).is_some() => {}
                    c => return Ok(c.syntax_value()),
                }
            }
            Ok(0)
        })
        .sum()
}

fn part2(input: &mut dyn BufRead) -> io::Result<u64> {
    use Delimiter::{Angle, Brace, Bracket, Parenthesis};
    let mut scores = input
        .lines()
        .map(|line| {
            let line = line?;
            let mut stack = vec![];
            for c in line.chars().map(Delimiter::try_from) {
                let c = c?;
                match c {
                    Parenthesis(false) | Bracket(false) | Brace(false) | Angle(false) => {
                        stack.push(c)
                    }
                    c if stack.pop().filter(|r#match| r#match.matches(&c)).is_some() => {}
                    _ => return Ok(0),
                }
            }
            stack
                .into_iter()
                .rev()
                .map(|d| d.autocomplete_value())
                .try_fold(0, |acc, points| Ok(acc * 5 + points))
        })
        .filter(|score| !matches!(score, Ok(0)))
        .collect::<io::Result<Vec<_>>>()?;
    scores.sort();
    Ok(scores[scores.len() / 2])
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2021 Day 10 Part 1");
        println!(
            "The total syntax error score is {}",
            part1(&mut BufReader::new(File::open("2021_10.txt")?))?
        );
    }
    {
        println!("Year 2021 Day 10 Part 2");
        println!(
            "The middle autocomplete score is {}",
            part2(&mut BufReader::new(File::open("2021_10.txt")?))?
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
        let s = r"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        let expected = 26_397;
        let actual = part1(&mut Cursor::new(s))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part2() -> io::Result<()> {
        let s = r"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        let expected = 288_957;
        let actual = part2(&mut Cursor::new(s))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
