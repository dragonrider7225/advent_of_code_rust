use nom::{multi, IResult};

pub trait NomParse<'input>: Sized {
    /// Parse a `Self` from the prefix of `s`. If Rust's orphan rules are
    /// ignored, [`FromStr`] can be trivially implemented for all types T that
    /// implement `NomParse` as
    ///
    /// ```rust.ignore
    /// use nom::{
    ///     self,
    ///     combinator as comb,
    /// };
    ///
    /// impl<T: NomParse> FromStr for T {
    ///     type Err = String;
    ///
    ///     fn from_str(s: &str) -> Result<T, <Self as FromStr>::Err> {
    ///         Self::nom_parse(s)
    ///             .finish()
    ///             .map(|(_, res)| res)
    ///             .map_err(|error| format!("{:?}", error))
    ///     }
    /// }
    /// ```
    ///
    /// [`FromStr`]: /std/str/trait.FromStr.html
    fn nom_parse(s: &'input str) -> IResult<&'input str, Self>;
}

impl<'input, T> NomParse<'input> for Vec<T>
where
    T: NomParse<'input>,
{
    fn nom_parse(s: &'input str) -> IResult<&'input str, Self> {
        multi::many0(T::nom_parse)(s)
    }
}

macro_rules! impl_nom_parse_for_unsigned {
    ($($t:ty)*) => ($(
        impl<'input> NomParse<'input> for $t {
            fn nom_parse(s: &'input str) -> ::nom::IResult<&'input str, $t> {
                use ::nom::{character::complete as character, combinator as comb};

                comb::map_res(character::digit1, str::parse)(s)
            }
        }
    )*)
}

macro_rules! impl_nom_parse_for_signed {
    ($($t:ty)*) => ($(
        impl<'input> NomParse<'input> for $t {
            fn nom_parse(s: &str) -> ::nom::IResult<&str, $t> {
                use ::nom::{
                    bytes::complete as bytes,
                    character::complete as character,
                    combinator as comb,
                    sequence,
                };

                comb::map_res(
                    comb::recognize(sequence::pair(comb::opt(bytes::tag("-")), character::digit1)),
                    str::parse,
                )(s)
            }
        }
    )*)
}

impl_nom_parse_for_unsigned!(u8 u16 u32 u64 u128 usize);
impl_nom_parse_for_signed!(i8 i16 i32 i64 i128 isize);

#[macro_export]
macro_rules! impl_from_str_for_nom_parse {
    ($($t:ty)*) => ($(
        impl FromStr for $t
        where
            Self: for<'s> NomParse<'s>,
        {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
                use ::nom::Finish;

                Self::nom_parse(s)
                    .finish()
                    .map(|(_, res)| res)
                    .map_err(|error| format!("{:?}", error))
            }
        }
    )*)
}
