use nom::*;
use nom::{
    bytes::complete::tag,
    character::complete::multispace0,
    combinator::map,
    sequence::{preceded, tuple},
};

#[cfg(test)]
use crate::assert_line;
use crate::parsers::*;

#[derive(Debug, PartialEq)]
pub struct Ssrc<'a> {
    id: u64,
    attribute: &'a str,
    value: &'a str,
}

/// ssrc
pub(crate) fn ssrc_line(input: &str) -> IResult<&str, Ssrc> {
    preceded(
        tag("a=ssrc:"),
        map(
            tuple((
                wsf(read_big_number), // id
                multispace0,
                read_non_colon_string, //attribute
                tag(":"),
                wsf(read_string), // value
            )),
            |(id, _, attribute, _, value)| Ssrc {
                id,
                attribute: &attribute,
                value: &value,
            },
        ),
    )(input)
}

#[test]
#[rustfmt::skip]
fn test_ssrc_line() {
    assert_line!(
        ssrc_line,
        "a=ssrc:1366781084 cname:EocUG1f0fcg/yvY7",
        Ssrc { id: 1366781084, attribute: "cname", value: "EocUG1f0fcg/yvY7" }
    );
    assert_line!(
        ssrc_line,
        "a=ssrc: 1366781084 cname: EocUG1f0fcg/yvY7",
        Ssrc { id: 1366781084, attribute: "cname", value: "EocUG1f0fcg/yvY7" }
    );
}
