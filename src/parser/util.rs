//! Parser utilities and domain-specific custom combinators.

use nom::error::ParseError;
use nom::sequence::delimited;
use nom::{IResult, InputTakeAtPosition};

/// Discards any surrounding whitespace before running an inner parser function.
pub(crate) fn trim_whitespace<'a, O, P>(parser: P) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    P: FnMut(&'a str) -> IResult<&'a str, O>,
{
    delimited(whitespace0, parser, whitespace0)
}

/// Specialized version of the `multispace0` combinator to work explicitly with `&'a str` types.
pub(crate) fn whitespace0<'a, E>(input: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str>,
{
    input.split_at_position_complete(|c| !(c == ' ' || c == '\t' || c == '\r' || c == '\n'))
}

#[cfg(test)]
mod tests {
    use nom::bytes::complete::tag;
    use nom::combinator::value;
    use nom::number::complete::recognize_float;

    use super::*;

    #[test]
    fn whitespace_trimmer_works() {
        let input = "\n\r\t    123    \r\n\t";
        assert_eq!(trim_whitespace(recognize_float)(input), Ok(("", "123")));
        assert_eq!(trim_whitespace(value(42, tag("123")))(input), Ok(("", 42)));
        assert_eq!(trim_whitespace(value(42, tag("123")))(input), Ok(("", 42)));
    }
}
