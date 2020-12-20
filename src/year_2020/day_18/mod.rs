use crate::parse::NomParse;
use nom::{branch, character::complete as character, combinator as comb, sequence, IResult};
use std::{
    fmt::{self, Display, Formatter},
    fs, io, iter,
    str::FromStr,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ExprToken {
    LeftParen,
    RightParen,
    Add,
    Mul,
    Val(u64),
}

impl Display for ExprToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Self::LeftParen => write!(f, "{}", '('),
            Self::RightParen => write!(f, "{}", ')'),
            Self::Add => write!(f, "{}", '+'),
            Self::Mul => write!(f, "{}", '*'),
            Self::Val(v) => write!(f, "{}", v),
        }
    }
}

impl<'s> NomParse<'s> for ExprToken {
    fn nom_parse(s: &str) -> IResult<&str, Self> {
        sequence::delimited(
            character::space0,
            branch::alt((
                comb::value(Self::LeftParen, character::char('(')),
                comb::value(Self::RightParen, character::char(')')),
                comb::value(Self::Add, character::char('+')),
                comb::value(Self::Mul, character::char('*')),
                comb::map(u64::nom_parse, Self::Val),
            )),
            character::space0,
        )(s)
    }
}

struct ExprTokens<'s> {
    s: &'s str,
}

impl<'s> ExprTokens<'s> {
    fn of(s: &'s str) -> Self {
        Self { s }
    }
}

impl<'s> Iterator for ExprTokens<'s> {
    type Item = ExprToken;

    fn next(&mut self) -> Option<Self::Item> {
        if self.s.len() == 0 {
            None
        } else {
            let (remaining, token) =
                Self::Item::nom_parse(self.s).expect("Invalid expression tail");
            self.s = remaining;
            Some(token)
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Expr {
    Val(u64),
    Add(Box<[Expr; 2]>),
    Mul(Box<[Expr; 2]>),
}

impl Expr {
    /// Convert a sequence of tokens into an expression. Returns an expression only if the entire
    /// sequence of tokens makes up a well-formed expression.
    fn from_tokens(tokens: &[ExprToken]) -> Option<Self> {
        fn delegate(
            tokens: &[ExprToken],
            argument: Option<Expr>,
            parenthesized: usize,
        ) -> Option<(Expr, &[ExprToken], usize)> {
            match (tokens, argument, parenthesized) {
                (&[], Some(argument), 0) => Some((argument, tokens, 0)),
                (&[], _, _) => None,

                (&[ExprToken::Val(v), ..], None, _) => {
                    Some((Expr::Val(v), &tokens[1..], parenthesized))
                }
                (&[ExprToken::Val(_), ..], Some(_), _) => None,

                (&[ExprToken::RightParen, ..], None, _)
                | (&[ExprToken::RightParen, ..], Some(_), 0) => None,
                (&[ExprToken::RightParen, ..], Some(argument), _) => {
                    Some((argument, &tokens[1..], parenthesized - 1))
                }

                (&[ExprToken::LeftParen, ..], None, _) => {
                    let (mut expr, mut unparsed, mut new_parenthesized) =
                        delegate(&tokens[1..], None, parenthesized + 1)?;
                    loop {
                        if new_parenthesized == parenthesized {
                            break Some((expr, unparsed, parenthesized));
                        } else {
                            assert!(new_parenthesized > parenthesized);
                            if unparsed.is_empty() {
                                break None;
                            }
                            let (new_expr, new_unparsed, parenthesized) =
                                delegate(unparsed, Some(expr), new_parenthesized)?;
                            expr = new_expr;
                            unparsed = new_unparsed;
                            new_parenthesized = parenthesized;
                        }
                    }
                }
                (&[ExprToken::LeftParen, ..], Some(_), _) => None,

                (&[ExprToken::Add, ..], None, _) => None,
                (&[ExprToken::Add, ..], Some(argument), _) => {
                    let (right_expr, unparsed, new_parenthesized) =
                        delegate(&tokens[1..], None, parenthesized)?;
                    assert_eq!(parenthesized, new_parenthesized);
                    Some((
                        Expr::Add(box [argument, right_expr]),
                        unparsed,
                        parenthesized,
                    ))
                }

                (&[ExprToken::Mul, ..], None, _) => None,
                (&[ExprToken::Mul, ..], Some(argument), _) => {
                    let (right_expr, unparsed, new_parenthesized) =
                        delegate(&tokens[1..], None, parenthesized)?;
                    assert_eq!(parenthesized, new_parenthesized);
                    Some((
                        Expr::Mul(box [argument, right_expr]),
                        unparsed,
                        parenthesized,
                    ))
                }
            }
        }

        let (mut head, mut unparsed, parenthesized) = delegate(tokens, None, 0)?;
        assert_eq!(0, parenthesized);
        while !unparsed.is_empty() {
            let (new_head, new_unparsed, parenthesized) = delegate(unparsed, Some(head), 0)?;
            assert_eq!(0, parenthesized);
            head = new_head;
            unparsed = new_unparsed;
        }
        Some(head)
    }

    fn eval(&self) -> u64 {
        match self {
            &Self::Val(v) => v,
            Self::Add(box [left, right]) => left.eval() + right.eval(),
            Self::Mul(box [left, right]) => left.eval() * right.eval(),
        }
    }

    fn eval_advanced(mut tokens: Vec<ExprToken>) -> u64 {
        fn find_paren(tokens: &[ExprToken]) -> Option<(usize, usize)> {
            let mut paren = tokens
                .iter()
                .copied()
                .enumerate()
                .find(|&(_idx, token)| match token {
                    ExprToken::LeftParen => true,
                    _ => false,
                })?
                .0;
            for i in paren..tokens.len() {
                match tokens[i] {
                    ExprToken::LeftParen => paren = i,
                    ExprToken::RightParen => return Some((paren, i)),
                    _ => {}
                }
            }
            None
        }

        fn find_add(tokens: &[ExprToken]) -> Option<usize> {
            tokens
                .iter()
                .copied()
                .enumerate()
                .find(|&(_, token)| match token {
                    ExprToken::Add => true,
                    _ => false,
                })
                .map(|(idx, _)| idx)
        }

        fn find_mul(tokens: &[ExprToken]) -> Option<usize> {
            tokens
                .iter()
                .copied()
                .enumerate()
                .find(|&(_, token)| match token {
                    ExprToken::Mul => true,
                    _ => false,
                })
                .map(|(idx, _)| idx)
        }

        while tokens.len() > 1 {
            if let Some(paren_indices) = find_paren(&tokens) {
                let sub_expr = tokens[(paren_indices.0 + 1)..=(paren_indices.1 - 1)]
                    .iter()
                    .copied()
                    .collect();
                let total = Self::eval_advanced(sub_expr);
                tokens.splice(
                    paren_indices.0..=paren_indices.1,
                    iter::once(ExprToken::Val(total)),
                );
            } else if let Some(add_idx) = find_add(&tokens) {
                let left_value = tokens[add_idx - 1];
                let right_value = tokens[add_idx + 1];
                match (left_value, right_value) {
                    (ExprToken::Val(left), ExprToken::Val(right)) => {
                        tokens.splice(
                            (add_idx - 1)..=(add_idx + 1),
                            iter::once(ExprToken::Val(left + right)),
                        );
                    }
                    _ => unreachable!(
                        "Found {:?} and {:?} around plus sign",
                        left_value, right_value
                    ),
                }
            } else if let Some(mul_idx) = find_mul(&tokens) {
                let left_value = tokens[mul_idx - 1];
                let right_value = tokens[mul_idx + 1];
                match (left_value, right_value) {
                    (ExprToken::Val(left), ExprToken::Val(right)) => {
                        tokens.splice(
                            (mul_idx - 1)..=(mul_idx + 1),
                            iter::once(ExprToken::Val(left * right)),
                        );
                    }
                    _ => unreachable!(),
                }
            } else {
                unreachable!("Every compound expression must contain either a pair of parentheses, a plus sign, or a multiplication sign");
            }
        }
        match &*tokens {
            &[] => unreachable!("Need at least token to evaluate"),
            &[ExprToken::Val(v)] => v,
            _ => unreachable!(),
        }
    }
}

impl FromStr for Expr {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_tokens(&ExprTokens::of(s).collect::<Vec<_>>()).ok_or(())
    }
}

pub(super) fn run() -> io::Result<()> {
    let contents = fs::read_to_string("2020_18.txt")?;
    let token_streams = contents
        .lines()
        .map(ExprTokens::of)
        .map(Iterator::collect::<Vec<_>>)
        .collect::<Vec<_>>();
    {
        println!("Year 2020 Day 18 Part 1");
        let total = token_streams
            .iter()
            .filter_map(|line| Expr::from_tokens(&*line))
            .map(|expr| expr.eval())
            .sum::<u64>();
        println!("The total of all expressions is {}", total);
    }
    {
        println!("Year 2020 Day 18 Part 2");
        let total = token_streams
            .into_iter()
            .map(|line| Expr::eval_advanced(line))
            .sum::<u64>();
        println!("The total of all expressions is {}", total);
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[ignore]
    #[test]
    fn parses_tokens_correctly_1() {
        let expected = [
            ExprToken::Val(1),
            ExprToken::Add,
            ExprToken::Val(2),
            ExprToken::Mul,
            ExprToken::Val(3),
            ExprToken::Add,
            ExprToken::Val(4),
            ExprToken::Mul,
            ExprToken::Val(5),
            ExprToken::Add,
            ExprToken::Val(6),
        ];
        let actual = ExprTokens::of("1 + 2 * 3 + 4 * 5 + 6").collect::<Vec<_>>();
        assert_eq!(&expected, &*actual);
    }

    #[ignore]
    #[test]
    fn parses_tokens_correctly_2() {
        let expected = [
            ExprToken::Val(1),
            ExprToken::Add,
            ExprToken::LeftParen,
            ExprToken::Val(2),
            ExprToken::Mul,
            ExprToken::Val(3),
            ExprToken::RightParen,
            ExprToken::Add,
            ExprToken::LeftParen,
            ExprToken::Val(4),
            ExprToken::Mul,
            ExprToken::LeftParen,
            ExprToken::Val(5),
            ExprToken::Add,
            ExprToken::Val(6),
            ExprToken::RightParen,
            ExprToken::RightParen,
        ];
        let actual = ExprTokens::of("1 + (2 * 3) + (4 * (5 + 6))").collect::<Vec<_>>();
        assert_eq!(&expected, &*actual);
    }

    #[ignore]
    #[test]
    fn builds_expr_correctly_1() {
        let tokens = [
            ExprToken::Val(1),
            ExprToken::Add,
            ExprToken::Val(2),
            ExprToken::Mul,
            ExprToken::Val(3),
            ExprToken::Add,
            ExprToken::Val(4),
            ExprToken::Mul,
            ExprToken::Val(5),
            ExprToken::Add,
            ExprToken::Val(6),
        ];
        let expected = Some(Expr::Add(box [
            Expr::Mul(box [
                Expr::Add(box [
                    Expr::Mul(box [Expr::Add(box [Expr::Val(1), Expr::Val(2)]), Expr::Val(3)]),
                    Expr::Val(4),
                ]),
                Expr::Val(5),
            ]),
            Expr::Val(6),
        ]));
        let actual = Expr::from_tokens(&tokens);
        assert_eq!(expected, actual);
    }

    #[ignore]
    #[test]
    fn builds_expr_correctly_2() {
        let tokens = [
            ExprToken::Val(1),
            ExprToken::Add,
            ExprToken::LeftParen,
            ExprToken::Val(2),
            ExprToken::Mul,
            ExprToken::Val(3),
            ExprToken::RightParen,
            ExprToken::Add,
            ExprToken::LeftParen,
            ExprToken::Val(4),
            ExprToken::Mul,
            ExprToken::LeftParen,
            ExprToken::Val(5),
            ExprToken::Add,
            ExprToken::Val(6),
            ExprToken::RightParen,
            ExprToken::RightParen,
        ];
        let expected = Some(Expr::Add(box [
            Expr::Add(box [Expr::Val(1), Expr::Mul(box [Expr::Val(2), Expr::Val(3)])]),
            Expr::Mul(box [Expr::Val(4), Expr::Add(box [Expr::Val(5), Expr::Val(6)])]),
        ]));
        let actual = Expr::from_tokens(&tokens);
        assert_eq!(expected, actual);
    }

    #[ignore]
    #[test]
    fn eval_works_correctly() {
        let expr = Expr::Add(box [
            Expr::Mul(box [
                Expr::Add(box [
                    Expr::Mul(box [Expr::Add(box [Expr::Val(1), Expr::Val(2)]), Expr::Val(3)]),
                    Expr::Val(4),
                ]),
                Expr::Val(5),
            ]),
            Expr::Val(6),
        ]);
        let expected = 71;
        let actual = expr.eval();
        assert_eq!(expected, actual);
    }

    #[ignore]
    #[test]
    fn eval_advanced_works_correctly_1() {
        let tokens = vec![
            ExprToken::Val(1),
            ExprToken::Add,
            ExprToken::Val(2),
            ExprToken::Mul,
            ExprToken::Val(3),
            ExprToken::Add,
            ExprToken::Val(4),
            ExprToken::Mul,
            ExprToken::Val(5),
            ExprToken::Add,
            ExprToken::Val(6),
        ];
        let expected = 231;
        let actual = Expr::eval_advanced(tokens);
        assert_eq!(expected, actual);
    }

    #[ignore]
    #[test]
    fn eval_advanced_works_correctly_2() {
        let tokens = vec![
            ExprToken::Val(1),
            ExprToken::Add,
            ExprToken::LeftParen,
            ExprToken::Val(2),
            ExprToken::Mul,
            ExprToken::Val(3),
            ExprToken::RightParen,
            ExprToken::Add,
            ExprToken::LeftParen,
            ExprToken::Val(4),
            ExprToken::Mul,
            ExprToken::LeftParen,
            ExprToken::Val(5),
            ExprToken::Add,
            ExprToken::Val(6),
            ExprToken::RightParen,
            ExprToken::RightParen,
        ];
        let expected = 51;
        let actual = Expr::eval_advanced(tokens);
        assert_eq!(expected, actual);
    }
}
