//! [`elucidate`][elucidate-repo] provides an intuitive API for efficiently and
//! correctly parsing arbitrary JSON data.
//!
//! # NOTE: This software is very unstable. Use at your own risk.
//! __This warning will be removed once the public API is considered stable
//! enough to be useful.__
//!
//! The primary goals of this project are:
//! 1. High-performance and speed at runtime
//! 2. Memory-safe with limited usages of `unsafe`
//!     - Current plans are to not use it at all. If this changes, it will be
//!     documented.
//! 3. Provide useful utilities that help transform the resulting data into
//!    something useful.
//!
//! [elucidate-repo]: https://github.com/dark-fusion/elucidate
use nom::branch::alt;
use nom::bytes::streaming::tag;
use nom::combinator::value;
use nom::IResult;

/// Parses a null value into a unit struct
pub fn null(input: &str) -> IResult<&str, ()> {
    value((), tag("null"))(input)
}

/// Parses a JSON value with the `boolean` type.
pub fn boolean(input: &str) -> IResult<&str, bool> {
    alt((value(true, tag("true")), value(false, tag("false"))))(input)
}

#[cfg(test)]
mod tests {
    use nom::error_position;
    use nom::{error::ErrorKind, Err, Needed};

    use super::*;

    #[test]
    fn null_parser() {
        assert_eq!(null("nullabc"), Ok(("abc", ())));
        assert_eq!(
            null("()"),
            Err(Err::Error(error_position!("()", ErrorKind::Tag)))
        );
    }

    #[test]
    fn boolean_parser() {
        assert_eq!(boolean("true\"more"), Ok(("\"more", true)));
        assert_eq!(
            boolean("1234567890"),
            Err(Err::Error(nom::error::Error::new(
                "1234567890",
                ErrorKind::Tag,
            )))
        );
        assert_eq!(boolean("falseXYZ"), Ok(("XYZ", false)));
        assert_eq!(boolean("tr"), Err(Err::Incomplete(Needed::new(2))));
    }
}
