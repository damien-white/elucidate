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
use nom::combinator::{map_res, value};
use nom::number::complete::recognize_float;
use nom::IResult;

use Value::*;

/// Tree-like data structure representing a JSON value.
///
/// The `Value` enum is used to map JSON values to well-formed Rust types.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Value {
    Boolean(bool),
    Number(f64),
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

/// Parses a JSON `number` value.
pub fn float(input: &str) -> IResult<&str, Value> {
    map_res(recognize_float, |val: &str| val.parse::<f64>().map(Number))(input)
}

#[cfg(test)]
mod tests {
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
    fn parses_numbers_without_decimal_point() {
        assert_eq!(float("4567xyz"), Ok(("xyz", Number(4567.0))));
        assert_eq!(float("00000XXX"), Ok(("XXX", Number(0.0))));
        assert_eq!(float("123456789xyz"), Ok(("xyz", Number(123456789.0))));
        assert_eq!(float("-500abc"), Ok(("abc", Number(-500.0))));
        assert_eq!(float("abc"), Err(make_nom_error("abc", ErrorKind::Char)));
        assert_eq!(
            float("92233e72036854775808"),
            Ok(("", Number(f64::INFINITY)))
        );
    }

    #[test]
    fn parse_numbers_with_decimal() {
        assert_eq!(float("456.7xyz"), Ok(("xyz", Number(456.7))));
        assert_eq!(float("0.0000XXX"), Ok(("XXX", Number(0.0))));
        assert_eq!(float("0123456789xyz"), Ok(("xyz", Number(123456789.0))));
        assert_eq!(float("-500.98"), Ok(("", Number(-500.98))));
        assert_eq!(float("-127."), Ok(("", Number(-127.0))));
        assert_eq!(float("-12.7.e8"), Ok((".e8", Number(-12.7))));
        assert_eq!(float("1e+7qwerty"), Ok(("qwerty", Number(10_000_000.0))));
        assert_eq!(float("abc"), Err(make_nom_error("abc", ErrorKind::Char)));
        assert_eq!(
            float("9223372036854775808"),
            Ok(("", Number(9.223372036854776e18)))
        );
    }
}
