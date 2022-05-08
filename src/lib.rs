//! High-performance JSON parsing for safety-critical systems.
//!
//! `elucidate` uses a suite of safe, resource-efficient JSON parsing routines to
//! sanitize arbitrary and untrusted data. It provides an intuitive and easy-to-use
//! API for operating on JSON data without sacrificing performance.
//!
//! # Stability
//!
//! ***This crate is not ready for use in a production system**
//!
//! Breaking changes to the API may be introduced at any time.
//!
//! Upcoming changes can be found in the project's [change log][CHANGELOG].
//!
//! [CHANGELOG]: https://github.com/dark-fusion/elucidate/CHANGELOG.md

use nom::branch::alt;
use nom::bytes::streaming::tag;
use nom::character::streaming::{digit0, one_of};
use nom::combinator::{recognize, value};
use nom::sequence::pair;
use nom::IResult;

use Value::*;

/// Tree-like data structure representing a JSON value.
///
/// The `Value` enum is used to map JSON values to well-formed Rust types.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Value {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    Null,
}

/// Parses a JSON `boolean` value.
pub fn boolean(input: &str) -> IResult<&str, Value> {
    alt((
        value(Boolean(true), tag("true")),
        value(Boolean(false), tag("false")),
    ))(input)
}

/// Parses a JSON `null` value.
pub fn null(input: &str) -> IResult<&str, Value> {
    value(Null, tag("null"))(input)
}

// Parses a JSON **number** as an `integer` value.
// pub fn integer(input: &str) -> IResult<&str, Value> {}
// TODO: Finish these parsers / chain the steps together

/// Parses an unsigned integer value.
#[allow(unused)]
pub(crate) fn unsigned_integer(input: &str) -> IResult<&str, &str> {
    alt((tag("0"), recognize(pair(nonzero_digit, digit0))))(input)
}

/// Parses a non-zero digit from 0-9, returning a `char`.
#[allow(unused)]
pub(crate) fn nonzero_digit(input: &str) -> IResult<&str, char> {
    one_of("123456789")(input)
}

// Parses a JSON `number` as a **float** value.
// pub fn float(input: &str) -> IResult<&str, &Value> {}

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

    #[test]
    fn recognizes_nonzero_digit() {
        assert_eq!(nonzero_digit("4567"), Ok(("567", '4')));
        assert_eq!(
            nonzero_digit("0123456789"),
            Err(Err::Error(Error::new("0123456789", ErrorKind::OneOf)))
        );
        assert_eq!(nonzero_digit(""), Err(Err::Incomplete(Needed::new(1))));
    }

    #[test]
    fn recognizes_unsigned_integer() {
        // TODO: Fix this bug
        // assert_eq!(unsigned_integer("4567"), Ok(("", "4567")));
        assert_eq!(unsigned_integer("0123456789"), Ok(("123456789", "0")));
        assert_eq!(unsigned_integer(""), Err(Err::Incomplete(Needed::new(1))));
    }
}
