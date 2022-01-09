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
