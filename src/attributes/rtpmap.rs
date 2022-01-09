use std::borrow::Cow;

use derive_into_owned::IntoOwned;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace1,
    combinator::{map, opt},
    sequence::{preceded, tuple},
    IResult,
};

use crate::parsers::*;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[non_exhaustive]
pub enum PTime {
    MaxPTime(u32),
    MinPTime(u32),
    PTime(u32),
}

pub fn read_p_time(input: &str) -> IResult<&str, PTime> {
    alt((
        attribute("ptime", map(read_number, PTime::PTime)),
        attribute("minptime", map(read_number, PTime::MinPTime)),
        attribute("maxptime", map(read_number, PTime::MaxPTime)),
    ))(input)
}

/// RtpMap
/// `a=rtpmap:<payload type> <encoding name>/<clock rate> [/<encoding` parameters>]
///<https://tools.ietf.org/html/rfc4566#section-6>
#[derive(Clone, Debug, IntoOwned, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct RtpMap<'a> {
    pub payload: u32,
    pub encoding_name: Cow<'a, str>,
    pub clock_rate: Option<u32>,
    pub encoding: Option<u32>,
}

pub fn rtpmap_line(input: &str) -> IResult<&str, RtpMap> {
    attribute(
        "rtpmap",
        map(
            tuple((
                read_number,                                          // payload_typ
                preceded(multispace1, cowify(read_non_slash_string)), // encoding_name
                opt(preceded(tag("/"), read_number)),                 // clock_rate
                opt(preceded(
                    tag("/"),
                    read_number, // encoding
                )),
            )),
            |(payload, encoding_name, clock_rate, encoding)| RtpMap {
                payload,
                encoding_name,
                clock_rate,
                encoding,
            },
        ),
    )(input)
}
