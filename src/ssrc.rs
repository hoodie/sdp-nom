#![allow(unused_imports)]
use nom::*;
use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, take_while, take_while1},
    character::{
        complete::{anychar, char, multispace0, none_of, space1},
        is_digit,
    },
    combinator::{map, map_res, opt},
    error::ParseError,
    multi::many0,
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    Parser,
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
pub(crate) fn raw_ssrc_line(input: &str) -> IResult<&str, Ssrc> {
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
fn parse_ssrc_line() {
    assert_line!(
        raw_ssrc_line,
        "a=ssrc:1366781084 cname:EocUG1f0fcg/yvY7",
        Ssrc { id: 1366781084, attribute: "cname", value: "EocUG1f0fcg/yvY7" }
    );
    assert_line!(
        raw_ssrc_line,
        "a=ssrc: 1366781084 cname: EocUG1f0fcg/yvY7",
        Ssrc { id: 1366781084, attribute: "cname", value: "EocUG1f0fcg/yvY7" }
    );
}
