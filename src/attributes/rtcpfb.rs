//! This one is fun to parse
use nom::*;
use nom::{
    branch::alt,
    bytes::complete::{tag},
    combinator::{map, opt},
    sequence::{preceded, separated_pair, tuple},
};

use super::{read_direction, Direction};
#[cfg(test)]
use crate::assert_line;
use crate::parsers::*;


/// RtcpFeedback
///
/// https://tools.ietf.org/html/rfc6642
/// https://tools.ietf.org/html/rfc4585#section-4.2
/// https://datatracker.ietf.org/doc/draft-ietf-mmusic-sdp-mux-attributes/16/?include_text=1
/// eg `a=rtcp-fb:98 trr-int 100`
#[derive(Debug, PartialEq)]
pub struct RtcpFb<'a> {
    payload: u32,
    val: RtcpFbVal<'a>,
}

#[derive(Debug, PartialEq)]
pub enum RtcpFbVal<'a> {
    Ack(RtcpFbAckParam<'a>),
    Nack(RtcpFbNackParam<'a>),
    TrrInt(u32),
    RtcpFbId {
        id: &'a str,
        param: Option<RtcpFbParam<'a>>,
    },
}

#[derive(Debug, PartialEq)]
pub enum RtcpFbParam<'a> {
    App(&'a str),
    Single(&'a str),
    Pair(&'a str, &'a str),
}

fn read_param(input: &str) -> IResult<&str, RtcpFbParam> {
    alt((
        map(preceded(tag("app"), wsf(read_string)), RtcpFbParam::App),
        map(
            tuple((wsf(read_string), wsf(read_string))),
            |(token, value)| RtcpFbParam::Pair(token, value),
        ),
        map(wsf(read_string), |value| RtcpFbParam::Single(value)),
    ))(input)
}

#[derive(Debug, PartialEq)]
pub enum RtcpFbAckParam<'a> {
    Rpsi,
    Sli(Option<&'a str>),
    App(&'a str),
    Other(&'a str, Option<&'a str>),
}

fn read_ack_param(input: &str) -> IResult<&str, RtcpFbAckParam> {
    alt((
        map(tag("rpsi"), |_| RtcpFbAckParam::Rpsi),
        map(preceded(tag("app"), wsf(read_string)), RtcpFbAckParam::App),
        map(
            preceded(tag("sli"), opt(wsf(read_string))),
            RtcpFbAckParam::Sli,
        ),
        map(
            tuple((wsf(read_string), wsf(read_string))),
            |(token, value)| RtcpFbAckParam::Other(token, Some(value)),
        ),
    ))(input)
}

#[test]
fn test_rtcpfb_ack_param() {
    assert_line!(read_ack_param, "sli", RtcpFbAckParam::Sli(None));
    assert_line!(
        read_ack_param,
        "sli 5432",
        RtcpFbAckParam::Sli(Some("5432"))
    );
}

#[derive(Debug, PartialEq)]
pub enum RtcpFbNackParam<'a> {
    Pli,
    Sli,
    Rpsi,
    App(&'a str),
    Other(&'a str, &'a str),
}

fn read_nack_param(input: &str) -> IResult<&str, RtcpFbNackParam> {
    alt((
        map(tag("rpsi"), |_| RtcpFbNackParam::Rpsi),
        map(preceded(tag("app"), wsf(read_string)), RtcpFbNackParam::App),
        map(
            tuple((wsf(read_string), wsf(read_string))),
            |(token, value)| RtcpFbNackParam::Other(token, value),
        ),
    ))(input)
}

fn read_val(input: &str) -> IResult<&str, RtcpFbVal> {
    alt((
        map(preceded(tag("ack"), wsf(read_ack_param)), RtcpFbVal::Ack),
        map(preceded(tag("nack"), wsf(read_nack_param)), RtcpFbVal::Nack),
        map(
            preceded(tag("trr-int"), wsf(read_number)),
            RtcpFbVal::TrrInt,
        ),
        map(
            tuple((wsf(read_string), opt(wsf(read_param)))),
            |(id, param)| RtcpFbVal::RtcpFbId { id, param },
        ),
    ))(input)
}

#[test]
#[rustfmt::skip]
fn test_read_val() {
    assert_line!(read_val, "trr-int 100", RtcpFbVal::TrrInt(100));
    assert_line!(read_val, "ack sli", RtcpFbVal::Ack(RtcpFbAckParam::Sli(None)));
    assert_line!(read_val, "ack sli 5432", RtcpFbVal::Ack(RtcpFbAckParam::Sli(Some("5432"))));
    assert_line!(read_val, "nack rpsi", RtcpFbVal::Nack(RtcpFbNackParam::Rpsi));
    assert_line!(read_val, "goog-remb", RtcpFbVal:: RtcpFbId{id: "goog-remb", param: None});
    assert_line!(read_val,  "ccm", RtcpFbVal:: RtcpFbId{id: "ccm", param: None});
    assert_line!(read_val,  "ccm fir", RtcpFbVal:: RtcpFbId{id: "ccm", param: Some(RtcpFbParam::Single("fir"))});
    assert_line!(read_val,  "fb foo bar", RtcpFbVal:: RtcpFbId{id: "fb", param: Some(RtcpFbParam::Pair("foo", "bar"))});
}

pub(crate) fn rtcpfb_attribute_line(input: &str) -> IResult<&str, RtcpFb> {
    preceded(
        tag("a=rtcp-fb:"),
        map(
            tuple((
                read_number, // payload
                // val
                wsf(read_val),
            )),
            |(payload, val)| RtcpFb { payload, val },
        ),
    )(input)
}

#[test]
#[rustfmt::skip]
fn test_rtcpfb_line() {
    assert_line!(rtcpfb_attribute_line, "a=rtcp-fb:98 trr-int 100");
    assert_line!(rtcpfb_attribute_line, "a=rtcp-fb:98 ack sli");
    assert_line!(rtcpfb_attribute_line, "a=rtcp-fb:98 ack sli 5432");
    assert_line!(rtcpfb_attribute_line, "a=rtcp-fb:98 nack rpsi", RtcpFb {payload: 98, val: RtcpFbVal::Nack(RtcpFbNackParam::Rpsi)});

    assert_line!(rtcpfb_attribute_line, "a=rtcp-fb:96 goog-remb", RtcpFb {payload: 96, val: RtcpFbVal::RtcpFbId{id: "goog-remb", param: None}});
    assert_line!(rtcpfb_attribute_line, "a=rtcp-fb:96 transport-cc", RtcpFb {payload: 96, val: RtcpFbVal::RtcpFbId{id: "transport-cc", param: None}});
    assert_line!(
        rtcpfb_attribute_line,
        "a=rtcp-fb:96 ccm fir",
        RtcpFb {
            payload: 96,
            val: RtcpFbVal::RtcpFbId{
                id: "ccm",
                param: Some(RtcpFbParam::Single("fir"))
            }
        }
    );
}
