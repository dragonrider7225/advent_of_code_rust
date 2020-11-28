use nom::{
    bytes::complete as bytes,
    character::complete as character,
    combinator as comb,
    sequence,
    IResult,
};

pub fn parse_u32(s: &str) -> IResult<&str, u32> {
    comb::map(character::digit1, |s: &str| s.parse().expect("Invalid u32"))(s)
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

macro_rules! impl_nom_parse_for_from_str {
    ($($t:ty)*) => ($(
        impl NomParse for $t {
            fn nom_parse(s: &str) -> IResult<&str, $t> {
                comb::map_opt(
                    comb::recognize(
                        sequence::pair(
                            comb::opt(bytes::tag("-")),
                            character::digit1,
                        ),
                    ),
                    |s: &str| s.parse().ok()
                )(s)
            }
        }
    )*)
}

impl_nom_parse_for_from_str!(
    u8 u16 u32 u64 u128 usize
    i8 i16 i32 i64 i128 isize
);

macro_rules! impl_from_str_for_nom_parse {
    ($($t:ty)*) => ($(
        impl FromStr for $t {
            type Err = String;
       
            fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
                comb::cut(comb::complete(<_>::nom_parse))(s)
                    .map(|(_, res)| res)
                    .map_err(|e| format!("{:?}", e))
            }
        }
    )*)
}
