//! Rtcp<https://tools.ietf.org/html/rfc3605>
// ///////////////////////
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::{preceded, tuple},
    IResult,
};

use std::net::IpAddr;

use crate::parsers::*;
#[cfg(test)]
use crate::{assert_line, assert_line_print};

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum NetType {
    IN,
}

pub fn read_net_type(input: &str) -> IResult<&str, NetType> {
    map(tag("IN"), |_| NetType::IN)(input)
}

/// Rtcp
///
///<https://tools.ietf.org/html/rfc3605>
/// `a=rtcp:65179 IN IP4 10.23.34.567`
#[derive(Debug, PartialEq)]
pub struct Rtcp {
    pub port: u32,
    pub net_type: NetType,
    pub ip_ver: IpVer,
    pub addr: IpAddr,
}

pub fn rtcp_attribute_line(input: &str) -> IResult<&str, Rtcp> {
    attribute("rtcp", rtcp_attribute)(input)
}

fn rtcp_attribute(input: &str) -> IResult<&str, Rtcp> {
    map(
        tuple((
            wsf(read_number),   // port
            wsf(read_net_type), // net_type
            wsf(read_ipver),    // ip_ver
            wsf(read_addr),     // addr
        )),
        |(port, net_type, ip_ver, addr)| Rtcp {
            port,
            net_type,
            ip_ver,
            addr,
        },
    )(input)
}

#[test]
fn test_rtcp_attribute_line() {
    assert_line_print!(rtcp_attribute_line, "a=rtcp:65179 IN IP4 10.23.34.255");
    assert_line_print!(rtcp_attribute_line, "a=rtcp:65179 IN IP4 ::1");
}

// ///////////////////////

/// RtcpFeedback
///
/// This one is fun to parse
///<https://tools.ietf.org/html/rfc6642>
///<https://tools.ietf.org/html/rfc4585#section-4.2>
///<https://datatracker.ietf.org/doc/draft-ietf-mmusic-sdp-mux-attributes/16/?include_text=1>
/// eg `a=rtcp-fb:98 trr-int 100`
#[derive(Debug, PartialEq)]
pub struct Fb<'a> {
    pub payload: u32,
    pub val: FbVal<'a>,
}

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum FbVal<'a> {
    Ack(FbAckParam<'a>),
    Nack(FbNackParam<'a>),
    TrrInt(u32),
    RtcpFbId {
        id: &'a str,
        param: Option<FbParam<'a>>,
    },
}

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum FbParam<'a> {
    App(&'a str),
    Single(&'a str),
    Pair(&'a str, &'a str),
}

fn read_param(input: &str) -> IResult<&str, FbParam> {
    alt((
        map(preceded(tag("app"), wsf(read_string)), FbParam::App),
        map(
            tuple((wsf(read_string), wsf(read_string))),
            |(token, value)| FbParam::Pair(token, value),
        ),
        map(wsf(read_string), |value| FbParam::Single(value)),
    ))(input)
}

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum FbAckParam<'a> {
    Rpsi,
    Sli(Option<&'a str>),
    App(&'a str),
    Other(&'a str, Option<&'a str>),
}

fn read_ack_param(input: &str) -> IResult<&str, FbAckParam> {
    alt((
        map(tag("rpsi"), |_| FbAckParam::Rpsi),
        map(preceded(tag("app"), wsf(read_string)), FbAckParam::App),
        map(preceded(tag("sli"), opt(wsf(read_string))), FbAckParam::Sli),
        map(
            tuple((wsf(read_string), wsf(read_string))),
            |(token, value)| FbAckParam::Other(token, Some(value)),
        ),
    ))(input)
}

#[test]
fn test_rtcpfb_ack_param() {
    assert_line!(read_ack_param, "sli", FbAckParam::Sli(None));
    assert_line!(read_ack_param, "sli 5432", FbAckParam::Sli(Some("5432")));
}

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum FbNackParam<'a> {
    Pli,
    Sli,
    Rpsi,
    App(&'a str),
    Other(&'a str, &'a str),
}

fn read_nack_param(input: &str) -> IResult<&str, FbNackParam> {
    alt((
        map(tag("rpsi"), |_| FbNackParam::Rpsi),
        map(preceded(tag("app"), wsf(read_string)), FbNackParam::App),
        map(
            tuple((wsf(read_string), wsf(read_string))),
            |(token, value)| FbNackParam::Other(token, value),
        ),
    ))(input)
}

fn read_val(input: &str) -> IResult<&str, FbVal> {
    alt((
        map(preceded(tag("ack"), wsf(read_ack_param)), FbVal::Ack),
        map(preceded(tag("nack"), wsf(read_nack_param)), FbVal::Nack),
        map(preceded(tag("trr-int"), wsf(read_number)), FbVal::TrrInt),
        map(
            tuple((wsf(read_string), opt(wsf(read_param)))),
            |(id, param)| FbVal::RtcpFbId { id, param },
        ),
    ))(input)
}

#[test]
#[rustfmt::skip]
fn test_read_val() {
    assert_line!(read_val, "trr-int 100", FbVal::TrrInt(100), print);
    assert_line!(read_val, "ack sli", FbVal::Ack(FbAckParam::Sli(None)), print);
    assert_line!(read_val, "ack sli 5432", FbVal::Ack(FbAckParam::Sli(Some("5432"))), print);
    assert_line!(read_val, "nack rpsi", FbVal::Nack(FbNackParam::Rpsi), print);
    assert_line!(read_val, "goog-remb", FbVal:: RtcpFbId{id: "goog-remb", param: None}, print);
    assert_line!(read_val, "ccm", FbVal:: RtcpFbId{id: "ccm", param: None}, print);
    assert_line!(read_val, "ccm fir", FbVal:: RtcpFbId{id: "ccm", param: Some(FbParam::Single("fir"))}, print);
    assert_line!(read_val, "fb foo bar", FbVal:: RtcpFbId{id: "fb", param: Some(FbParam::Pair("foo", "bar"))}, print);
}

pub fn rtcpfb_attribute_line(input: &str) -> IResult<&str, Fb> {
    attribute("rtcp-fb", rtcpfb_attribute)(input)
}

fn rtcpfb_attribute(input: &str) -> IResult<&str, Fb> {
    map(
        tuple((
            read_number,   // payload
            wsf(read_val), // val
        )),
        |(payload, val)| Fb { payload, val },
    )(input)
}

#[test]
#[rustfmt::skip]
fn test_rtcpfb_line() {
    assert_line_print!(rtcpfb_attribute_line, "a=rtcp-fb:98 trr-int 100");
    assert_line_print!(rtcpfb_attribute_line, "a=rtcp-fb:98 ack sli");
    assert_line_print!(rtcpfb_attribute_line, "a=rtcp-fb:98 ack sli 5432");
    assert_line!(rtcpfb_attribute_line, "a=rtcp-fb:98 nack rpsi", Fb {payload: 98, val: FbVal::Nack(FbNackParam::Rpsi)}, print);

    assert_line!(rtcpfb_attribute_line, "a=rtcp-fb:96 goog-remb", Fb {payload: 96, val: FbVal::RtcpFbId{id: "goog-remb", param: None}}, print);
    assert_line!(rtcpfb_attribute_line, "a=rtcp-fb:96 transport-cc", Fb {payload: 96, val: FbVal::RtcpFbId{id: "transport-cc", param: None}}, print);
    assert_line!(
        rtcpfb_attribute_line,
        "a=rtcp-fb:96 ccm fir",
        Fb {
            payload: 96,
            val: FbVal::RtcpFbId{
                id: "ccm",
                param: Some(FbParam::Single("fir"))
            }
        }, print
    );
}
