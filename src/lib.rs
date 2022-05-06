//! `elucidate` is a crate for parsing arbitrary JSON data.
//!
//! # NOTE: This software is very unstable. Use at your own risk.
//! __This warning will be removed once the public API is considered stable
//! enough to be useful.__
//!
//! The primary goals of [`elucidate`] are:
//! 1. High-performance / high speed at runtime
//! 2. Memory-safety with limited (or no) usage of `unsafe`
//! 3. Provide useful utilities that help transform the resulting data into
//!    something useful.
use nom::branch::alt;
use nom::bytes::streaming::tag;
use nom::combinator::value;
use nom::IResult;

/// Parses a null value into a unit struct
pub fn parse_null(input: &str) -> IResult<&str, ()> {
    value((), tag("null"))(input)
}

/// Parses a JSON value with the `boolean` type.
pub fn boolean(input: &str) -> IResult<&str, bool> {
    alt((value(true, tag("true")), value(false, tag("false"))))(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
