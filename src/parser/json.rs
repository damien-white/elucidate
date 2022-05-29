use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, map_res, value},
    number::complete::recognize_float,
    IResult,
};

use crate::parser::util::trim_whitespace;

/// Tree-like data structure representing a JSON value.
///
/// The `Value` enum is used to map JSON values to well-formed Rust types.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Value {
    Boolean(bool),
    Number(f64),
    Null(()),
}

/// Parses a JSON value, using branching to coerce into the correct type.
pub fn json_value(input: &str) -> IResult<&str, Value> {
    trim_whitespace(alt((
        map(boolean, Value::Boolean),
        map(number, Value::Number),
        map(null, Value::Null),
    )))(input)
}

/// Parses a JSON `boolean` value.
pub fn boolean(input: &str) -> IResult<&str, bool> {
    alt((value(true, tag("true")), value(false, tag("false"))))(input)
}

/// Parses a JSON `null` value.
pub fn null(input: &str) -> IResult<&str, ()> {
    value((), tag("null"))(input)
}

/// Parses a JSON `number` value.
pub fn number(input: &str) -> IResult<&str, f64> {
    map_res(recognize_float, str::parse::<f64>)(input)
}

// /// Parses a JSON `string` value.
// pub fn string(input: &str) -> IResult<&str, &str> {
//
// }

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
    fn parses_json_value() {
        assert_eq!(json_value("    null    "), Ok(("", Value::Null(()))));
        assert_eq!(json_value("true"), Ok(("", Value::Boolean(true))));
        assert_eq!(json_value("false"), Ok(("", Value::Boolean(false))));
        assert_eq!(json_value("-42.42e7"), Ok(("", Value::Number(-42.42e7))));
        assert_eq!(json_value("-424200000"), Ok(("", Value::Number(-42.42e7))));
    }

    #[test]
    fn parses_null_values() {
        assert_eq!(null("nullabc"), Ok(("abc", ())));
        assert_eq!(null("()"), Err(make_nom_error("()", ErrorKind::Tag)));
        assert_eq!(null("nul"), Err(make_nom_error("nul", ErrorKind::Tag)));
    }

    #[test]
    fn parses_boolean_values() {
        assert_eq!(boolean("true\"more"), Ok(("\"more", true)));
        assert_eq!(boolean("falseXYZ"), Ok(("XYZ", false)));
        assert_eq!(
            boolean("1234567890"),
            Err(make_nom_error("1234567890", ErrorKind::Tag))
        );
    }

    #[test]
    fn parses_integer_values() {
        assert_eq!(number("4567xyz"), Ok(("xyz", 4567.0)));
        assert_eq!(number("00000XXX"), Ok(("XXX", 0.0)));
        assert_eq!(number("123456789xyz"), Ok(("xyz", 123456789.0)));
        assert_eq!(number("-500abc"), Ok(("abc", -500.0)));
        assert_eq!(number("abc"), Err(make_nom_error("abc", ErrorKind::Char)));
        assert_eq!(number("92233e72036854775808"), Ok(("", f64::INFINITY)));
    }

    #[test]
    fn parse_floating_point_values() {
        assert_eq!(number("456.7xyz"), Ok(("xyz", 456.7)));
        assert_eq!(number("0.0000XXX"), Ok(("XXX", 0.0)));
        assert_eq!(number("0123456789xyz"), Ok(("xyz", 123456789.0)));
        assert_eq!(number("-500.98"), Ok(("", -500.98)));
        assert_eq!(number("-127."), Ok(("", -127.0)));
        assert_eq!(number("-12.7.e8"), Ok((".e8", -12.7)));
        assert_eq!(number("1e+7qwerty"), Ok(("qwerty", 10_000_000.0)));
        assert_eq!(number("abc"), Err(make_nom_error("abc", ErrorKind::Char)));
        assert_eq!(
            number("9223372036854775808"),
            Ok(("", 9.223372036854776e18))
        );
    }
}
