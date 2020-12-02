use nom::{character::complete as character, combinator as comb, IResult};
use std::str::FromStr;

fn parse_num<T>(s: &str) -> IResult<&str, T>
where
    T: FromStr,
{
    comb::map_res(character::digit1, str::parse)(s)
}

pub trait NomParse: Sized {
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
    ///         comb::cut(comb::complete(<_>::nom_parse))(s)
    ///             .map(|(_, res)| res)
    ///             .map_err(|e| format!("{:?}", e))
    ///     }
    /// }
    /// ```
    ///
    /// [`FromStr`]: /std/str/trait.FromStr.html
    fn nom_parse(s: &str) -> IResult<&str, Self>;
}

macro_rules! impl_nom_parse_for_num {
    ($($t:ty)*) => ($(
        impl NomParse for $t {
            fn nom_parse(s: &str) -> IResult<&str, $t> {
                parse_num::<$t>(s)
            }
        }
    )*)
}

impl_nom_parse_for_num!(
    u8 u16 u32 u64 u128 usize
    i8 i16 i32 i64 i128 isize
);

#[macro_export]
macro_rules! impl_from_str_for_nom_parse {
    ($($t:ty)*) => ($(
        impl FromStr for $t {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
                comb::cut(comb::complete(Self::nom_parse))(s)
                    .map(|(_, res)| res)
                    .map_err(|e| format!("{:?}", e))
            }
        }
    )*)
}
