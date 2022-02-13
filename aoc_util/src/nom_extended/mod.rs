use nom::{branch, bytes::complete as bytes, character::complete as character, IResult};

/// Recognizes both `\n` and `\r\n`.
pub fn newline(s: &str) -> IResult<&str, &str> {
    branch::alt((bytes::tag("\n"), bytes::tag("\r\n")))(s)
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

/// Represents a type which can be parsed using `nom`.
pub trait NomParse<I>: Sized {
    /// Parse `input` into a value of type `Self` using `nom`.
    fn nom_parse(input: I) -> IResult<I, Self>;
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
