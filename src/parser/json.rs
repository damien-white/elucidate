//! JSON parsers for transforming JSON into Rust data types.
use std::collections::HashMap;

use nom::combinator::{map_res, opt, recognize};
use nom::error::context;
use nom::sequence::{pair, tuple};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, value},
    IResult,
};

use super::util::{exponent, fraction, recognize_int, whitespace};

/// Tree-like data structure for building the AST.
///
/// Used to represent loosely-typed JSON values as well-formed Rust types.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    /// Ordered list of zero or more elements
    Array(Vec<Node>),
    /// Value of either true or false
    Boolean(bool),
    /// 64-bit floating point
    Real(f64),
    /// 64-bit signed integer
    Integer(i64),
    /// Collection of key-value pairs where all keys are strings
    Object(HashMap<String, Node>),
    /// UTF-encoded string
    String(String),
    /// Empty value
    Null,
    /// Undefined value due to malformed or unrecognized input
    Invalid,
}

/// Parses a JSON `boolean` value.
fn boolean(input: &str) -> IResult<&str, bool> {
    alt((value(true, tag("true")), value(false, tag("false"))))(input)
}

/// Parses a JSON `null` value.
fn null(input: &str) -> IResult<&str, ()> {
    value((), tag("null"))(input)
}

/// Parses a JSON `number` value as a 64-bit floating point.
fn real(input: &str) -> IResult<&str, f64> {
    context(
        "real",
        map_res(
            recognize(tuple((
                opt(tag("-")),
                recognize_int,
                alt((recognize(pair(fraction, opt(exponent))), exponent)),
                opt(fraction),
                opt(exponent),
            ))),
            str::parse,
        ),
    )(input)
}

/// Parses a JSON `number` value as a 64-bit integer.
fn integer(input: &str) -> IResult<&str, i64> {
    map_res(recognize(pair(opt(tag("-")), recognize_int)), str::parse)(input)
}

/// Tries to assemble a JSON parse tree using the recursive descent algorithm.
pub fn json(input: &str) -> IResult<&str, Node> {
    whitespace(alt((
        map(boolean, Node::Boolean),
        map(real, Node::Real),
        map(integer, Node::Integer),
        map(null, |_| Node::Null),
    )))(input)
}

#[cfg(test)]
mod tests {
    use nom::error::ErrorKind;

    use crate::parser::util::make_nom_error;

    use super::*;

    #[test]
    fn parse_to_ast() {
        assert_eq!(json("    null    "), Ok(("", Node::Null)));
        assert_eq!(json("true"), Ok(("", Node::Boolean(true))));
        assert_eq!(json("false"), Ok(("", Node::Boolean(false))));
        assert_eq!(json("-42.42e7"), Ok(("", Node::Real(-42.42e7))));
        assert_eq!(json("-424200000"), Ok(("", Node::Integer(-424200000))));
    }

    #[test]
    fn parses_null_values() {
        assert_eq!(null("nullabc"), Ok(("abc", ())));
        assert_eq!(null("()"), make_nom_error("()", ErrorKind::Tag));
        assert_eq!(null("nul"), make_nom_error("nul", ErrorKind::Tag));
    }

    #[test]
    fn parses_boolean_values() {
        assert_eq!(boolean("true\"more"), Ok(("\"more", true)));
        assert_eq!(boolean("falseXYZ"), Ok(("XYZ", false)));
        assert_eq!(
            boolean("1234567890"),
            make_nom_error("1234567890", ErrorKind::Tag)
        );
    }

    #[test]
    fn parses_integer_numerical_values() {
        assert_eq!(integer("4567xyz"), Ok(("xyz", 4567)));
        assert_eq!(integer("00000XXX"), Ok(("0000XXX", 0)));
        assert_eq!(integer("123456789xyz"), Ok(("xyz", 123456789)));
        assert_eq!(integer("-500abc"), Ok(("abc", -500)));
        assert_eq!(
            integer("92233e72036854775808"),
            Ok(("e72036854775808", 92233))
        );
        assert_eq!(integer("abc"), make_nom_error("abc", ErrorKind::OneOf));
    }

    #[test]
    fn parse_real_numerical_values() {
        assert_eq!(real("456.7xyz"), Ok(("xyz", 456.7)));
        assert_eq!(real("0.0000XXX"), Ok(("XXX", 0.0)));
        assert_eq!(
            real("0123456789xyz"),
            make_nom_error("123456789xyz", ErrorKind::Tag)
        );
        assert_eq!(real("-500.98"), Ok(("", -500.98)));
        assert_eq!(real("6.89985307179586."), Ok((".", 6.89985307179586)));
        assert_eq!(real("-12.7.e8"), Ok((".e8", -12.7)));
        assert_eq!(real("1e+7qwerty"), Ok(("qwerty", 10_000_000.0)));
        assert_eq!(real("-127."), make_nom_error(".", ErrorKind::Tag));
        assert_eq!(real("abc"), make_nom_error("abc", ErrorKind::OneOf));
    }
}
