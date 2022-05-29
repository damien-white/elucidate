//! Parser utilities and domain-specific custom combinators.

use nom::{sequence::delimited, IResult, InputTakeAtPosition};

/// Discards any surrounding whitespace before running an inner parser function.
pub(crate) fn trim_whitespace<'a, O, P>(parser: P) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    P: FnMut(&'a str) -> IResult<&'a str, O>,
{
    delimited(whitespace0, parser, whitespace0)
}

/// Specialized version of the `multispace0` combinator to work explicitly with `&'a str` types.
fn whitespace0(input: &str) -> IResult<&str, &str> {
    input.split_at_position_complete(|c| !(c == ' ' || c == '\t' || c == '\r' || c == '\n'))
}

// Convenience function for error cases
#[cfg(test)]
pub(crate) fn make_nom_error<I, O>(
    input: I,
    kind: nom::error::ErrorKind,
) -> Result<(I, O), nom::Err<nom::error::Error<I>>> {
    Err(nom::Err::Error(nom::error::Error::new(input, kind)))
}

#[cfg(test)]
mod tests {
    use nom::{bytes::complete::tag, combinator::value, error::ErrorKind};

    use super::*;

    #[test]
    fn whitespace_trimmer_works() {
        let mut parser = trim_whitespace(value("123", tag("123")));
        assert_eq!(parser("\n\r\t    123    \r\n\t"), Ok(("", "123")));
        assert_eq!(parser("\n\r\t    123"), Ok(("", "123")));
        assert_eq!(parser("123"), Ok(("", "123")));
    }

    #[test]
    fn whitespace0_parser_works() {
        let input = "\n\r  abc  \r\n";
        assert_eq!(whitespace0(input), Ok(("abc  \r\n", "\n\r  ")));
        assert_eq!(
            whitespace0("\r\n\t{\"message\":\"test\""),
            Ok(("{\"message\":\"test\"", "\r\n\t"))
        );
    }

    #[test]
    fn creates_nom_error() {
        assert!(make_nom_error::<&str, ErrorKind>("hello", ErrorKind::Tag).is_err());
        assert!(make_nom_error::<&str, ErrorKind>("1234567890", ErrorKind::Digit).is_err());
    }
}
