//! Rtcp<https://tools.ietf.org/html/rfc3605>
// ///////////////////////
use derive_into_owned::IntoOwned;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::{preceded, tuple},
    IResult,
};

use std::{borrow::Cow, net::IpAddr};

use crate::parsers::*;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
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
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
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

// ///////////////////////

/// RtcpFeedback
///
/// This one is fun to parse
///<https://tools.ietf.org/html/rfc6642>
///<https://tools.ietf.org/html/rfc4585#section-4.2>
///<https://datatracker.ietf.org/doc/draft-ietf-mmusic-sdp-mux-attributes/16/?include_text=1>
/// eg `a=rtcp-fb:98 trr-int 100`
#[derive(Clone, Debug, IntoOwned, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct Fb<'a> {
    pub payload: u32,
    pub val: FbVal<'a>,
}

#[derive(Clone, Debug, IntoOwned, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[non_exhaustive]
pub enum FbVal<'a> {
    Ack(FbAckParam<'a>),
    Nack(FbNackParam<'a>),
    TrrInt(u32),
    RtcpFbId {
        id: Cow<'a, str>,
        param: Option<FbParam<'a>>,
    },
}

#[derive(Clone, Debug, IntoOwned, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[non_exhaustive]
pub enum FbParam<'a> {
    App(Cow<'a, str>),
    Single(Cow<'a, str>),
    Pair(Cow<'a, str>, Cow<'a, str>),
}

fn read_param(input: &str) -> IResult<&str, FbParam> {
    alt((
        map(preceded(tag("app"), wsf(cowify(read_string))), FbParam::App),
        map(
            tuple((wsf(cowify(read_string)), wsf(cowify(read_string)))),
            |(token, value)| FbParam::Pair(token, value),
        ),
        map(wsf(cowify(read_string)), FbParam::Single),
    ))(input)
}

#[derive(Clone, Debug, IntoOwned, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[non_exhaustive]
pub enum FbAckParam<'a> {
    Rpsi,
    Sli(Option<Cow<'a, str>>),
    App(Cow<'a, str>),
    Other(Cow<'a, str>, Option<Cow<'a, str>>),
}

fn read_ack_param(input: &str) -> IResult<&str, FbAckParam> {
    alt((
        map(tag("rpsi"), |_| FbAckParam::Rpsi),
        map(
            preceded(tag("app"), wsf(cowify(read_string))),
            FbAckParam::App,
        ),
        map(
            preceded(tag("sli"), opt(wsf(cowify(read_string)))),
            FbAckParam::Sli,
        ),
        map(
            tuple((wsf(cowify(read_string)), wsf(cowify(read_string)))),
            |(token, value)| FbAckParam::Other(token, Some(value)),
        ),
    ))(input)
}

#[derive(Clone, Debug, IntoOwned, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[non_exhaustive]
pub enum FbNackParam<'a> {
    Pli,
    Sli,
    Rpsi,
    App(Cow<'a, str>),
    Other(Cow<'a, str>, Cow<'a, str>),
}

fn read_nack_param(input: &str) -> IResult<&str, FbNackParam> {
    alt((
        map(tag("rpsi"), |_| FbNackParam::Rpsi),
        map(
            preceded(tag("app"), wsf(cowify(read_string))),
            FbNackParam::App,
        ),
        map(
            tuple((wsf(cowify(read_string)), wsf(cowify(read_string)))),
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
            tuple((wsf(cowify(read_string)), opt(wsf(read_param)))),
            |(id, param)| FbVal::RtcpFbId { id, param },
        ),
    ))(input)
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
