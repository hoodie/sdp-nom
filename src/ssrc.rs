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
#[test]
#[rustfmt::skip]
#[ignore]
fn test_unsupported_ssrc_line() {
    assert_line!(ssrc_line, "a=ssrc:3570614608 cname:4TOk42mSjXCkVIa6");
    assert_line!(ssrc_line, "a=ssrc:3570614608 msid:lgsCFqt9kN2fVKw5wg3NKqGdATQoltEwOdMS 35429d94-5637-4686-9ecd-7d0622261ce8");
    assert_line!(ssrc_line, "a=ssrc:3570614608 mslabel:lgsCFqt9kN2fVKw5wg3NKqGdATQoltEwOdMS");
    assert_line!(ssrc_line, "a=ssrc:3570614608 label:35429d94-5637-4686-9ecd-7d0622261ce8");
    assert_line!(ssrc_line, "a=ssrc-group:FID 2231627014 632943048");
    assert_line!(ssrc_line, "a=ssrc:2231627014 cname:4TOk42mSjXCkVIa6");
    assert_line!(ssrc_line, "a=ssrc:2231627014 msid:lgsCFqt9kN2fVKw5wg3NKqGdATQoltEwOdMS daed9400-d0dd-4db3-b949-422499e96e2d");
    assert_line!(ssrc_line, "a=ssrc:2231627014 mslabel:lgsCFqt9kN2fVKw5wg3NKqGdATQoltEwOdMS");
    assert_line!(ssrc_line, "a=ssrc:2231627014 label:daed9400-d0dd-4db3-b949-422499e96e2d");
    assert_line!(ssrc_line, "a=ssrc:632943048 cname:4TOk42mSjXCkVIa6");
    assert_line!(ssrc_line, "a=ssrc:632943048 msid:lgsCFqt9kN2fVKw5wg3NKqGdATQoltEwOdMS daed9400-d0dd-4db3-b949-422499e96e2d");
}

