use nom::{
    character::complete as character,
    combinator as comb,
    IResult,
};

pub trait NomParse: Sized {
    /// Parse a `Self` from the prefix of `s`. [`FromStr`] can be trivially
    /// implemented for all types T that implement `NomParse` as
    ///
    /// ```
    /// use nom::{
    ///     self,
    ///     combinator as comb,
    /// };
    ///
    /// impl<T: NomParse> FromStr for T {
    ///     type Err = nom::Err<<Self as NomParse>::Err>;
    ///
    ///     fn from_str(s: &str) -> Result<T, <Self as FromStr>::Err> {
    ///         comb::cut(comb::complete(<_>::nom_parse))(s).map(|(_, res)| res)
    ///     }
    /// }
    /// ```
    ///
    /// [`FromStr`]: /std/str/trait.FromStr.html
    fn nom_parse(s: &str) -> IResult<&str, Self>;
}

macro_rules! impl_nom_parse_for_from_str {
    ($($t:ty)*) => ($(
        impl NomParse for $t {
            fn nom_parse(s: &str) -> IResult<&str, $t> {
                comb::map_opt(character::digit1, |s: &str| s.parse().ok())(s)
            }
        }
    )*)
}

impl_nom_parse_for_from_str!(
    u8 u16 u32 u64 u128 usize
    i8 i16 i32 i64 i128 isize
);
