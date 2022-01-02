use std::borrow::Cow;

use derive_into_owned::IntoOwned;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::multispace0,
    combinator::map,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

use crate::parsers::*;
#[cfg(test)]
use crate::{assert_line, assert_line_print};

#[derive(Clone, Debug, IntoOwned, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct Ssrc<'a> {
    pub id: u64,
    pub attribute: Cow<'a, str>,
    pub value: Cow<'a, str>,
}

/// ssrc
pub fn ssrc_line(input: &str) -> IResult<&str, Ssrc> {
    attribute(
        "ssrc",
        map(
            tuple((
                wsf(read_big_number), // id
                preceded(
                    multispace0,
                    separated_pair(
                        cowify(read_non_colon_string),
                        tag(":"),
                        cowify(wsf(is_not("\n"))),
                    ),
                ),
            )),
            |(id, (attribute, value))| Ssrc {
                id,
                attribute,
                value,
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
        Ssrc { id: 1366781084, attribute: "cname".into(), value: "EocUG1f0fcg/yvY7".into() },
        print
    );
    assert_line!(
        ssrc_line,
        "a=ssrc: 1366781084 cname: EocUG1f0fcg/yvY7",
        Ssrc { id: 1366781084, attribute: "cname".into(), value: "EocUG1f0fcg/yvY7".into() }
    );
    assert_line!(ssrc_line, "a=ssrc:3570614608 cname:4TOk42mSjXCkVIa6");
    assert_line!(ssrc_line, "a=ssrc:3570614608 msid:lgsCFqt9kN2fVKw5wg3NKqGdATQoltEwOdMS 35429d94-5637-4686-9ecd-7d0622261ce8");
    assert_line!(ssrc_line, "a=ssrc:3570614608 mslabel:lgsCFqt9kN2fVKw5wg3NKqGdATQoltEwOdMS");
    assert_line!(ssrc_line, "a=ssrc:3570614608 label:35429d94-5637-4686-9ecd-7d0622261ce8");
    assert_line!(ssrc_line, "a=ssrc:2231627014 cname:4TOk42mSjXCkVIa6");
    assert_line!(ssrc_line, "a=ssrc:2231627014 msid:lgsCFqt9kN2fVKw5wg3NKqGdATQoltEwOdMS daed9400-d0dd-4db3-b949-422499e96e2d");
    assert_line!(ssrc_line, "a=ssrc:2231627014 mslabel:lgsCFqt9kN2fVKw5wg3NKqGdATQoltEwOdMS");
    assert_line!(ssrc_line, "a=ssrc:2231627014 label:daed9400-d0dd-4db3-b949-422499e96e2d");
    assert_line!(ssrc_line, "a=ssrc:632943048 cname:4TOk42mSjXCkVIa6");
    assert_line!(ssrc_line, "a=ssrc:632943048 msid:lgsCFqt9kN2fVKw5wg3NKqGdATQoltEwOdMS daed9400-d0dd-4db3-b949-422499e96e2d");
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[non_exhaustive]
pub enum SsrcSemantic {
    FID,
    FEC,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct SsrcGroup {
    pub semantic: SsrcSemantic,
    pub ids: Vec<u32>,
}

pub fn ssrc_group_line(input: &str) -> IResult<&str, SsrcGroup> {
    attribute("ssrc-group", ssrc_group)(input)
}

pub fn ssrc_group(input: &str) -> IResult<&str, SsrcGroup> {
    map(
        tuple((
            alt((
                // semantic
                map(tag("FID"), |_| SsrcSemantic::FID),
                map(tag("FEC"), |_| SsrcSemantic::FEC),
            )),
            wsf(read_as_numbers), // ids
        )),
        |(semantic, ids)| SsrcGroup { semantic, ids },
    )(input)
}

#[test]
#[rustfmt::skip]
fn test_ssrc_group_line() {
    assert_line_print!(ssrc_group_line, "a=ssrc-group:FID 2231627014 632943048");
}
