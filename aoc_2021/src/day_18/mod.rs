use std::{
    fmt::{self, Display, Formatter},
    fs::File,
    io::{self, BufRead, BufReader, Cursor},
    iter::Sum,
    ops::{Add, Index, IndexMut},
};

use nom::{
    branch, character::complete as character, combinator as comb, sequence, Finish, IResult,
};

#[derive(Clone, Debug, Eq, PartialEq)]
enum Number {
    SN(Box<SnailfishNumber>),
    Literal(u32),
}

impl Number {
    fn nom_parse(input: &str) -> IResult<&str, Self> {
        branch::alt((
            comb::map(character::u32, Self::from),
            comb::map(SnailfishNumber::nom_parse, Self::from),
        ))(input)
    }
}

impl Number {
    fn magnitude(&self) -> u32 {
        match self {
            Self::Literal(n) => *n,
            Self::SN(inner) => inner.magnitude(),
        }
    }
}

impl Number {
    fn take(&mut self, path: &[Branch]) -> Option<Self> {
        if path.is_empty() {
            Some(std::mem::replace(self, Self::Literal(0)))
        } else {
            match path[0] {
                Branch::Left => self.snailfish_mut()?.left_mut().take(&path[1..]),
                Branch::Right => self.snailfish_mut()?.right_mut().take(&path[1..]),
            }
        }
    }

    fn split(&mut self) {
        if let &mut Self::Literal(n) = self {
            *self = Self::from(SnailfishNumber(Self::from(n / 2), Self::from((n + 1) / 2)));
        }
    }

    fn snailfish_mut(&mut self) -> Option<&mut SnailfishNumber> {
        match self {
            Self::SN(inner) => Some(&mut **inner),
            _ => None,
        }
    }

    fn unwrap_snailfish_mut(&mut self) -> &mut SnailfishNumber {
        self.snailfish_mut()
            .expect("Can't unwrap non-snailfish number as snailfish number")
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(n) => write!(f, "{n}"),
            Self::SN(inner) => write!(f, "{inner}"),
        }
    }
}

impl From<SnailfishNumber> for Number {
    fn from(value: SnailfishNumber) -> Self {
        Self::SN(Box::new(value))
    }
}

impl From<u32> for Number {
    fn from(value: u32) -> Self {
        Self::Literal(value)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Branch {
    Left,
    Right,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SnailfishNumber(Number, Number);

impl SnailfishNumber {
    fn read(input: &mut dyn BufRead) -> io::Result<Self> {
        let mut buf = String::new();
        input.read_line(&mut buf)?;
        let (_, this) = comb::all_consuming(Self::nom_parse)(buf.trim())
            .finish()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
        Ok(this)
    }

    fn nom_parse(input: &str) -> IResult<&str, Self> {
        comb::map(
            sequence::delimited(
                character::char('['),
                sequence::separated_pair(
                    Number::nom_parse,
                    character::char(','),
                    Number::nom_parse,
                ),
                character::char(']'),
            ),
            Self::from,
        )(input)
    }
}

impl SnailfishNumber {
    fn magnitude(&self) -> u32 {
        3 * self.0.magnitude() + 2 * self.1.magnitude()
    }
}

impl SnailfishNumber {
    /// Add the numbers in the pair pointed to by `path` to the numbers adjacent in the linear
    /// representation and replace the pair with `0`.
    /// Returns whether the number's representation was modified.
    fn explode(&mut self, path: &[Branch]) -> bool {
        if path.len() >= 4 {
            let removed = match self.take(path) {
                None => return false,
                Some(removed) => removed,
            };
            let (removed_left, removed_right) = match removed {
                Self(Number::Literal(left), Number::Literal(right)) => (left, right),
                _ => panic!("Can't explode pair which contains a snailfish number"),
            };
            if let Some(idx) = (0..path.len())
                .rev()
                .find(|&idx| path[idx] == Branch::Right)
            {
                let mut target = &mut *self;
                for &branch in &path[..idx] {
                    target = target[branch].unwrap_snailfish_mut();
                }
                let mut target = &mut target[Branch::Left];
                while let Number::SN(sn) = target {
                    target = &mut sn[Branch::Right];
                }
                match target {
                    Number::Literal(x) => *x += removed_left,
                    Number::SN(_) => unreachable!(),
                }
            }
            if let Some(idx) = (0..path.len()).rev().find(|&idx| path[idx] == Branch::Left) {
                let mut target = self;
                for &branch in &path[..idx] {
                    target = target[branch].unwrap_snailfish_mut();
                }
                let mut target = &mut target[Branch::Right];
                while let Number::SN(sn) = target {
                    target = &mut sn[Branch::Left];
                }
                match target {
                    Number::Literal(x) => *x += removed_right,
                    Number::SN(_) => unreachable!(),
                }
            }
            true
        } else {
            false
        }
    }

    fn left_mut(&mut self) -> &mut Number {
        &mut self.0
    }

    fn right_mut(&mut self) -> &mut Number {
        &mut self.1
    }

    fn take(&mut self, path: &[Branch]) -> Option<Self> {
        if path.is_empty() {
            None
        } else {
            match path[0] {
                Branch::Left => self.left_mut(),
                Branch::Right => self.right_mut(),
            }
            .take(&path[1..])
            .and_then(|removed| match removed {
                Number::SN(inner) => Some(*inner),
                Number::Literal(_) => None,
            })
        }
    }

    fn reduce(&mut self) {
        'outer: loop {
            // If any pair is nested inside four pairs, the leftmost such pair explodes.
            let mut branches = vec![];
            let mut pop = false;
            'explode: loop {
                if pop {
                    'explode_backtrack: loop {
                        match branches.pop() {
                            Some(Branch::Left) => {
                                branches.push(Branch::Right);
                                pop = false;
                                break 'explode_backtrack;
                            }
                            Some(Branch::Right) => {}
                            None => break 'explode,
                        }
                    }
                }
                if branches.len() >= 4 {
                    self.explode(&branches);
                    continue 'outer;
                }
                let mut target = &mut *self;
                for branch in branches.iter().copied() {
                    let side = match branch {
                        Branch::Left => target.left_mut(),
                        Branch::Right => target.right_mut(),
                    };
                    match side {
                        Number::Literal(_) => {
                            pop = true;
                            continue 'explode;
                        }
                        Number::SN(sn) => target = &mut **sn,
                    }
                }
                match target {
                    Self(Number::SN(_), _) => branches.push(Branch::Left),
                    Self(Number::Literal(_), Number::SN(_)) => branches.push(Branch::Right),
                    Self(Number::Literal(_), Number::Literal(_)) => {
                        pop = true;
                        continue 'explode;
                    }
                }
            }
            // Otherwise, if any regular number is 10 or greater, the leftmost such regular number
            // splits
            branches.clear();
            pop = false;
            'split: loop {
                if pop {
                    'split_backtrack: loop {
                        match branches.pop() {
                            Some(Branch::Left) => {
                                branches.push(Branch::Right);
                                pop = false;
                                break 'split_backtrack;
                            }
                            Some(Branch::Right) => {}
                            None => break 'split,
                        }
                        if branches.is_empty() {
                            break 'split;
                        }
                    }
                }
                let mut target = &mut *self;
                for branch in branches.iter().copied() {
                    let side = match branch {
                        Branch::Left => target.left_mut(),
                        Branch::Right => target.right_mut(),
                    };
                    match side {
                        Number::Literal(0..=9) => {
                            pop = true;
                            continue 'split;
                        }
                        &mut Number::Literal(_) => {
                            side.split();
                            continue 'outer;
                        }
                        Number::SN(sn) => target = &mut **sn,
                    }
                }
                branches.push(Branch::Left);
            }
            // Otherwise, break
            break 'outer;
        }
    }
}

impl Add for SnailfishNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut ret = Self(Number::from(self), Number::from(rhs));
        ret.reduce();
        ret
    }
}

impl Display for SnailfishNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{},{}]", self.0, self.1)
    }
}

impl From<(Number, Number)> for SnailfishNumber {
    fn from((left, right): (Number, Number)) -> Self {
        Self(left, right)
    }
}

impl Index<Branch> for SnailfishNumber {
    type Output = Number;

    fn index(&self, index: Branch) -> &Self::Output {
        match index {
            Branch::Left => &self.0,
            Branch::Right => &self.1,
        }
    }
}

impl IndexMut<Branch> for SnailfishNumber {
    fn index_mut(&mut self, index: Branch) -> &mut Self::Output {
        match index {
            Branch::Left => &mut self.0,
            Branch::Right => &mut self.1,
        }
    }
}

impl Sum<SnailfishNumber> for Option<SnailfishNumber> {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = SnailfishNumber>,
    {
        iter.fold(None, |acc, x| match acc {
            None => Some(x),
            Some(acc) => Some(acc + x),
        })
    }
}

fn part1(input: &mut dyn BufRead) -> io::Result<u32> {
    let sum = input
        .lines()
        .map(|line| SnailfishNumber::read(&mut Cursor::new(line?)))
        .sum::<io::Result<Option<SnailfishNumber>>>()?
        .ok_or(io::Error::new(io::ErrorKind::InvalidData, "No input"))?;
    Ok(sum.magnitude())
}

fn part2(input: &mut dyn BufRead) -> io::Result<u32> {
    let numbers = input
        .lines()
        .map(|line| SnailfishNumber::read(&mut Cursor::new(line?)))
        .collect::<io::Result<Vec<_>>>()?;
    (0..numbers.len())
        .flat_map(|i| (0..numbers.len()).map(move |j| (i, j)))
        .filter(|(i, j)| i != j)
        .map(|(i, j)| {
            let sum: SnailfishNumber = numbers[i].clone() + numbers[j].clone();
            sum.magnitude()
        })
        .reduce(u32::max)
        .ok_or(io::Error::new(io::ErrorKind::InvalidData, "Missing input"))
}

pub(super) fn run() -> io::Result<()> {
    {
        println!("Year 2021 Day 18 Part 1");
        println!(
            "{}",
            part1(&mut BufReader::new(File::open("2021_18.txt")?))?
        );
    }
    {
        println!("Year 2021 Day 18 Part 2");
        println!(
            "{}",
            part2(&mut BufReader::new(File::open("2021_18.txt")?))?
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
    fn test_explode_no_left() {
        let expected = SnailfishNumber(
            Number::from(SnailfishNumber(
                Number::from(SnailfishNumber(
                    Number::from(SnailfishNumber(Number::from(0), Number::from(9))),
                    Number::from(2),
                )),
                Number::from(3),
            )),
            Number::from(4),
        );
        let mut actual = SnailfishNumber(
            Number::from(SnailfishNumber(
                Number::from(SnailfishNumber(
                    Number::from(SnailfishNumber(
                        Number::from(SnailfishNumber(Number::from(9), Number::from(8))),
                        Number::from(1),
                    )),
                    Number::from(2),
                )),
                Number::from(3),
            )),
            Number::from(4),
        );
        actual.explode(&[Branch::Left, Branch::Left, Branch::Left, Branch::Left]);
        assert_eq!(expected, actual);
    }

    #[test]
    #[ignore]
    fn test_explode_no_right() {
        let expected = SnailfishNumber(
            Number::from(7),
            Number::from(SnailfishNumber(
                Number::from(6),
                Number::from(SnailfishNumber(
                    Number::from(5),
                    Number::from(SnailfishNumber(Number::from(7), Number::from(0))),
                )),
            )),
        );
        let mut actual = SnailfishNumber(
            Number::from(7),
            Number::from(SnailfishNumber(
                Number::from(6),
                Number::from(SnailfishNumber(
                    Number::from(5),
                    Number::from(SnailfishNumber(
                        Number::from(4),
                        Number::from(SnailfishNumber(Number::from(3), Number::from(2))),
                    )),
                )),
            )),
        );
        actual.explode(&[Branch::Right, Branch::Right, Branch::Right, Branch::Right]);
        assert_eq!(expected, actual);
    }

    #[test]
    #[ignore]
    fn test_explode_left_right() {
        let expected = SnailfishNumber(
            Number::from(SnailfishNumber(
                Number::from(6),
                Number::from(SnailfishNumber(
                    Number::from(5),
                    Number::from(SnailfishNumber(Number::from(7), Number::from(0))),
                )),
            )),
            Number::from(3),
        );
        let mut actual = SnailfishNumber(
            Number::from(SnailfishNumber(
                Number::from(6),
                Number::from(SnailfishNumber(
                    Number::from(5),
                    Number::from(SnailfishNumber(
                        Number::from(4),
                        Number::from(SnailfishNumber(Number::from(3), Number::from(2))),
                    )),
                )),
            )),
            Number::from(1),
        );
        actual.explode(&[Branch::Left, Branch::Right, Branch::Right, Branch::Right]);
        assert_eq!(expected, actual);
    }

    #[test]
    #[ignore]
    fn test_explode_not_alone() {
        let expected = SnailfishNumber(
            Number::from(SnailfishNumber(
                Number::from(3),
                Number::from(SnailfishNumber(
                    Number::from(2),
                    Number::from(SnailfishNumber(Number::from(8), Number::from(0))),
                )),
            )),
            Number::from(SnailfishNumber(
                Number::from(9),
                Number::from(SnailfishNumber(
                    Number::from(5),
                    Number::from(SnailfishNumber(
                        Number::from(4),
                        Number::from(SnailfishNumber(Number::from(3), Number::from(2))),
                    )),
                )),
            )),
        );
        let mut actual = SnailfishNumber(
            Number::from(SnailfishNumber(
                Number::from(3),
                Number::from(SnailfishNumber(
                    Number::from(2),
                    Number::from(SnailfishNumber(
                        Number::from(1),
                        Number::from(SnailfishNumber(Number::from(7), Number::from(3))),
                    )),
                )),
            )),
            Number::from(SnailfishNumber(
                Number::from(6),
                Number::from(SnailfishNumber(
                    Number::from(5),
                    Number::from(SnailfishNumber(
                        Number::from(4),
                        Number::from(SnailfishNumber(Number::from(3), Number::from(2))),
                    )),
                )),
            )),
        );
        actual.explode(&[Branch::Left, Branch::Right, Branch::Right, Branch::Right]);
        assert_eq!(expected, actual);
    }

    #[test]
    #[ignore]
    fn test_explode_end_double_peak() {
        let expected = SnailfishNumber(
            Number::from(SnailfishNumber(
                Number::from(3),
                Number::from(SnailfishNumber(
                    Number::from(2),
                    Number::from(SnailfishNumber(Number::from(8), Number::from(0))),
                )),
            )),
            Number::from(SnailfishNumber(
                Number::from(9),
                Number::from(SnailfishNumber(
                    Number::from(5),
                    Number::from(SnailfishNumber(Number::from(7), Number::from(0))),
                )),
            )),
        );
        let mut actual = SnailfishNumber(
            Number::from(SnailfishNumber(
                Number::from(3),
                Number::from(SnailfishNumber(
                    Number::from(2),
                    Number::from(SnailfishNumber(Number::from(8), Number::from(0))),
                )),
            )),
            Number::from(SnailfishNumber(
                Number::from(9),
                Number::from(SnailfishNumber(
                    Number::from(5),
                    Number::from(SnailfishNumber(
                        Number::from(4),
                        Number::from(SnailfishNumber(Number::from(3), Number::from(2))),
                    )),
                )),
            )),
        );
        actual.explode(&[Branch::Right, Branch::Right, Branch::Right, Branch::Right]);
        assert_eq!(expected, actual);
    }

    #[test]
    #[ignore]
    fn test_parse() -> io::Result<()> {
        let s = "[[[[4,3],4],4],[7,[[8,4],9]]]";
        let expected = SnailfishNumber(
            Number::from(SnailfishNumber(
                Number::from(SnailfishNumber(
                    Number::from(SnailfishNumber(Number::from(4), Number::from(3))),
                    Number::from(4),
                )),
                Number::from(4),
            )),
            Number::from(SnailfishNumber(
                Number::from(7),
                Number::from(SnailfishNumber(
                    Number::from(SnailfishNumber(Number::from(8), Number::from(4))),
                    Number::from(9),
                )),
            )),
        );
        let actual = SnailfishNumber::read(&mut Cursor::new(s))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_addition() -> io::Result<()> {
        let x = "[[[[4,3],4],4],[7,[[8,4],9]]]";
        let x = SnailfishNumber::read(&mut Cursor::new(x))?;
        let y = "[1,1]";
        let y = SnailfishNumber::read(&mut Cursor::new(y))?;
        let expected = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]";
        let expected = SnailfishNumber::read(&mut Cursor::new(expected))?;
        let actual = x + y;
        assert_eq!(expected, actual);
        Ok(())
    }

    const TEST_DATA: &'static str = concat!(
        "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]\n",
        "[[[5,[2,8]],4],[5,[[9,9],0]]]\n",
        "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]\n",
        "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]\n",
        "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]\n",
        "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]\n",
        "[[[[5,4],[7,7]],8],[[8,3],8]]\n",
        "[[9,3],[[9,9],[6,[4,9]]]]\n",
        "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]\n",
        "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]\n"
    );

    #[test]
    #[ignore]
    fn test_part1() -> io::Result<()> {
        let s = TEST_DATA;
        let expected = 4140;
        let actual = part1(&mut Cursor::new(s))?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_part2() -> io::Result<()> {
        let s = TEST_DATA;
        let expected = 3993;
        let actual = part2(&mut Cursor::new(s))?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
