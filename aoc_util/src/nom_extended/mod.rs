use nom::{character::complete as character, combinator, multi, IResult};

/// Recognizes both `\n` and `\r\n`.
#[deprecated = "Use character::line_ending"]
pub fn newline(s: &str) -> IResult<&str, &str> {
    character::line_ending(s)
}

/// Parses a decimal number from `s`.
#[deprecated = "Use character::{complete, streaming}::u8"]
pub fn recognize_u8(s: &str) -> IResult<&str, u8> {
    character::u8(s)
}

/// Parses a decimal number from `s`.
#[deprecated = "Use character::{complete, streaming}::u16"]
pub fn recognize_u16(s: &str) -> IResult<&str, u16> {
    character::u16(s)
}

/// Parses a decimal number from `s`.
#[deprecated = "Use character::{complete, streaming}::u32"]
pub fn recognize_u32(s: &str) -> IResult<&str, u32> {
    character::u32(s)
}

/// Parses a decimal number from `s`.
#[deprecated = "Use character::{complete, streaming}::u64"]
pub fn recognize_u64(s: &str) -> IResult<&str, u64> {
    character::u64(s)
}

/// Parses a decimal number from `s`.
#[deprecated = "Use character::{complete, streaming}::u128"]
pub fn recognize_u128(s: &str) -> IResult<&str, u128> {
    character::u128(s)
}

/// Parses a decimal number from `s`.
#[deprecated = "Use character::{complete, streaming}::i8"]
pub fn recognize_i8(s: &str) -> IResult<&str, i8> {
    character::i8(s)
}

/// Parses a decimal number from `s`.
#[deprecated = "Use character::{complete, streaming}::i16"]
pub fn recognize_i16(s: &str) -> IResult<&str, i16> {
    character::i16(s)
}

/// Parses a decimal number from `s`.
#[deprecated = "Use character::{complete, streaming}::i32"]
pub fn recognize_i32(s: &str) -> IResult<&str, i32> {
    character::i32(s)
}

/// Parses a decimal number from `s`.
#[deprecated = "Use character::{complete, streaming}::i64"]
pub fn recognize_i64(s: &str) -> IResult<&str, i64> {
    character::i64(s)
}

/// Parses a decimal number from `s`.
#[deprecated = "Use character::{complete, streaming}::i128"]
pub fn recognize_i128(s: &str) -> IResult<&str, i128> {
    character::i128(s)
}

/// The one true parser for values of type `Self` from values of type `I` using nom.
pub trait NomParse<I>: Sized {
    /// Parse a `Self` from a prefix of `i`. If Rust's orphan rules are ignored, [`FromStr`] can be
    /// trivially implemented for all types T that implement `NomParse<&str> where <Self as
    /// NomParse<_>>::Error: Debug` as
    ///
    /// ```rust.ignore
    /// use nom::{
    ///     self,
    ///     combinator as comb,
    /// };
    ///
    /// impl<T: NomParse<&str>> FromStr for T {
    ///     type Err = String;
    ///
    ///     fn from_str(s: &str) -> Result<T, <Self as FromStr>::Err> {
    ///         Self::nom_parse(s)
    ///             .finish()
    ///             .map(|(_, res)| res)
    ///             .map_err(|e| e.to_string())
    ///     }
    /// }
    /// ```
    ///
    /// [`FromStr`]: /std/str/trait.FromStr.html
    fn nom_parse(input: I) -> IResult<I, Self>;
}

/// A wrapper around a `Vec` that can be parsed from a sequence of `T`s concatenated without any
/// separator.
#[derive(Clone, Debug)]
pub struct ConcatenatedList<T>(pub Vec<T>);

impl<T> ConcatenatedList<T> {
    /// Converts into the inner `Vec` in a more self-descriptive way than just `parsed.0`.
    pub fn unwrap(self) -> Vec<T> {
        self.0
    }
}

impl<'input, T> NomParse<&'input str> for ConcatenatedList<T>
where
    T: NomParse<&'input str>,
{
    fn nom_parse(s: &'input str) -> IResult<&'input str, Self> {
        combinator::map(multi::many0(T::nom_parse), Self)(s)
    }
}

/// Generate a basic `FromStr` impl using `NomParse<&str>`. The parser fails if the input does not
/// contain exactly one value of type `$t`.
#[macro_export]
macro_rules! impl_from_str_for_nom_parse {
    ($($t:ty)*) => {$(
        /// An automatically generated `FromStr` impl which uses `NomParse<&str>` as a base. This
        /// parser will fail if the input does not contain exactly one value of the target type.
        impl ::std::str::FromStr for $t
        where
            $t: for<'s> NomParse<&'s str>,
        {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                use ::nom::{combinator, Finish};

                combinator::complete(combinator::all_consuming(Self::nom_parse))(s)
                    .finish()
                    .map(|(_, o)| o)
                    .map_err(|e| e.to_string())
            }
        }
    )*};
}

#[cfg(test)]
mod tests {
    use super::*;

    use nom::combinator;

    #[derive(Clone, Copy)]
    struct A;

    impl NomParse<&'_ str> for A {
        fn nom_parse(input: &str) -> IResult<&str, Self> {
            combinator::value(A, branch::alt((bytes::tag("a"), bytes::tag("1"))))(input)
        }
    }

    impl_from_str_for_nom_parse!(A);

    #[test]
    fn test_impl_from_str() {
        assert!("a".parse::<A>().is_ok());
        assert!("1".parse::<A>().is_ok());
        assert!("a1".parse::<A>().is_err());
    }
}
