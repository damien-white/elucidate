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
use nom::bytes::complete::tag;
use nom::character::complete::{digit0, digit1, one_of};
use nom::combinator::{map_res, opt, recognize, value};
use nom::sequence::{pair, tuple};
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

/// Parses a JSON **number** as an `integer` value.
pub fn integer(input: &str) -> IResult<&str, Value> {
    map_res(
        recognize(pair(opt(tag("-")), unsigned_integer)),
        |val: &str| val.parse::<i64>().map(Integer),
    )(input)
}

// Parses a JSON `number` as a **float** value.
pub fn float(input: &str) -> IResult<&str, Value> {
    map_res(
        recognize(tuple((
            opt(tag("-")),
            unsigned_integer,
            opt(recognize(pair(tag("."), digit1))),
            opt(recognize(tuple((tag("e"), opt(alt((tag("-"), tag("+")))))))),
        ))),
        |val: &str| val.parse().map(Float),
    )(input)
}

/// Parses an unsigned integer value.
pub(crate) fn unsigned_integer(input: &str) -> IResult<&str, &str> {
    alt((tag("0"), recognize(pair(nonzero_digit, digit0))))(input)
}

/// Parses a non-zero digit from 0-9, returning a `char`.
pub(crate) fn nonzero_digit(input: &str) -> IResult<&str, char> {
    one_of("123456789")(input)
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use nom::error::{Error, ErrorKind};
    use nom::Err;

    use super::*;

    // Convenience function for error cases
    fn make_nom_error(input: &str, kind: ErrorKind) -> Err<Error<&str>> {
        Err::Error(Error::new(input, kind))
    }

    #[test]
    fn parses_null_values() {
        assert_eq!(null("nullabc"), Ok(("abc", Null)));
        assert_eq!(null("()"), Err(make_nom_error("()", ErrorKind::Tag)));
        assert_eq!(null("nul"), Err(make_nom_error("nul", ErrorKind::Tag)));
    }

    #[test]
    fn parses_boolean_values() {
        assert_eq!(boolean("true\"more"), Ok(("\"more", Boolean(true))));
        assert_eq!(boolean("falseXYZ"), Ok(("XYZ", Boolean(false))));
        assert_eq!(
            boolean("1234567890"),
            Err(make_nom_error("1234567890", ErrorKind::Tag))
        );
    }

    #[test]
    fn recognizes_nonzero_digit() {
        assert_eq!(nonzero_digit("4567"), Ok(("567", '4')));
        assert_eq!(
            nonzero_digit("0123456789"),
            Err(make_nom_error("0123456789", ErrorKind::OneOf))
        );
    }

    #[test]
    fn recognizes_unsigned_integer() {
        assert_eq!(unsigned_integer("4567xyz"), Ok(("xyz", "4567")));
        assert_eq!(unsigned_integer("00000XXX"), Ok(("0000XXX", "0")));
        assert_eq!(unsigned_integer("0123456789xyz"), Ok(("123456789xyz", "0")));
        assert_eq!(
            nonzero_digit("0123456789"),
            Err(make_nom_error("0123456789", ErrorKind::OneOf))
        );
    }

    #[test]
    fn parses_integer_values() {
        assert_eq!(integer("4567xyz"), Ok(("xyz", Integer(4567))));
        assert_eq!(integer("00000XXX"), Ok(("0000XXX", Integer(0))));
        assert_eq!(integer("0123456789xyz"), Ok(("123456789xyz", Integer(0))));
        assert_eq!(integer("-500"), Ok(("", Integer(-500))));
        assert_eq!(integer("-127."), Ok((".", Integer(-127))));
        assert_eq!(integer("abc"), Err(make_nom_error("abc", ErrorKind::OneOf)));
        assert_eq!(
            integer("9223372036854775808"),
            Err(make_nom_error("9223372036854775808", ErrorKind::MapRes))
        );
    }

    #[test]
    fn parses_float_values() {
        assert_eq!(float("456.7xyz"), Ok(("xyz", Float(456.7))));
        assert_eq!(float("00000XXX"), Ok(("0000XXX", Float(0.0000))));
        assert_eq!(float("0123456789xyz"), Ok(("123456789xyz", Float(0.0))));
        assert_eq!(float("-500.98"), Ok(("", Float(-500.98))));
        assert_eq!(float("-127.."), Ok(("..", Float(-127.0))));
        assert_eq!(float("abc"), Err(make_nom_error("abc", ErrorKind::OneOf)));
        //assert_eq!(
        //    float("9223372036854775808"),
        //    Err(make_nom_error("9223372036854775808", ErrorKind::MapRes))
        //);
    }
}
