//! [`elucidate`][elucidate-repo] provides an intuitive API for efficiently and
//! correctly parsing arbitrary JSON data.
//!
//! # NOTE: This software is very unstable. Use at your own risk.
//! __This warning will be removed once the public API is considered stable
//! enough to be useful.__
//!
//! The primary goals of this project are:
//! 1. Performant at runtime — high speed and efficient with resources
//! 2. Memory-safe with limited to no usages of `unsafe`
//! 3. Provide intuitive APIs for parsing JSON into useful, structured data.
//!
//! [elucidate-repo]: https://github.com/dark-fusion/elucidate
use nom::branch::alt;
use nom::bytes::streaming::tag;
use nom::combinator::value;
use nom::IResult;

use Value::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Boolean(bool),
    Null,
}

/// Parses a JSON value with the `boolean` type.
pub fn boolean(input: &str) -> IResult<&str, Value> {
    alt((
        value(Boolean(true), tag("true")),
        value(Boolean(false), tag("false")),
    ))(input)
}

/// Parses a null value into a unit struct
pub fn null(input: &str) -> IResult<&str, Value> {
    value(Null, tag("null"))(input)
}

#[cfg(test)]
mod tests {
    use nom::error::{Error, ErrorKind};
    use nom::{Err, Needed};

    use super::*;

    #[test]
    fn parser_recognizes_null() {
        assert_eq!(null("nullabc"), Ok(("abc", Null)));
        assert_eq!(
            null("()"),
            Err(Err::Error(Error::new("()", ErrorKind::Tag)))
        );
        assert_eq!(null("nul"), Err(Err::Incomplete(Needed::new(1))))
    }

    #[test]
    fn parser_recognizes_booleans() {
        assert_eq!(boolean("true\"more"), Ok(("\"more", Boolean(true))));
        assert_eq!(boolean("falseXYZ"), Ok(("XYZ", Boolean(false))));
        assert_eq!(
            boolean("1234567890"),
            Err(Err::Error(Error::new("1234567890", ErrorKind::Tag)))
        );
        assert_eq!(boolean("tr"), Err(Err::Incomplete(Needed::new(2))));
        assert_eq!(boolean("fals"), Err(Err::Incomplete(Needed::new(1))));
    }
}
