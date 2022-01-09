//! Minimal Candidate parser
//!
//! read [RFC5245 Section 15.1](https://tools.ietf.org/html/rfc5245#section-15.1)

use derive_into_owned::IntoOwned;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::{preceded, tuple},
    IResult,
};

use std::{borrow::Cow, net::IpAddr};

use crate::parsers::{attribute, cowify, read_addr, read_number, read_string, wsf};

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub enum CandidateComponent {
    Rtp,
    Rtcp,
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[non_exhaustive]
pub enum CandidateProtocol {
    Tcp,
    Udp,
    Dccp,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[non_exhaustive]
pub enum CandidateType {
    Host,
    Relay,
    Srflx,
    Prflx,
}

/// Candidate
///
/// <https://tools.ietf.org/html/rfc5245#section-15.1>
/// <https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidateInit/candidate>
///
///
/// candidate:3348148302 1 udp 2113937151 192.0.2.1 56500 typ host
/// candidate:3348148302 2 udp 2113937151 192.0.2.1 56501 typ host
// "candidate:1853887674 2 udp 1518280447 47.61.61.61 36768 typ srflx raddr 192.168.0.196 rport 36768 generation 0"
#[derive(Clone, Debug, IntoOwned, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct Candidate<'a> {
    pub foundation: u32,
    pub component: CandidateComponent,
    pub protocol: CandidateProtocol,
    pub priority: u32,         // 2043278322
    pub addr: IpAddr,          // "192.168.0.56"
    pub port: u32,             // 44323
    pub r#type: CandidateType, // "host"
    pub raddr: Option<IpAddr>, // "192.168.0.56"
    pub rport: Option<u32>,    // 44323
    pub tcptype: Option<Cow<'a, str>>,
    pub generation: Option<u32>,
    pub network_id: Option<u32>,
}

pub fn candidate(input: &str) -> IResult<&str, Candidate> {
    map(
        tuple((
            wsf(read_number), // foundation
            // component:
            wsf(alt((
                map(tag("1"), |_| CandidateComponent::Rtp),
                map(tag("2"), |_| CandidateComponent::Rtcp),
            ))),
            // protocol:
            wsf(alt((
                map(alt((tag("UDP"), tag("udp"))), |_| CandidateProtocol::Udp),
                map(alt((tag("TCP"), tag("tcp"))), |_| CandidateProtocol::Tcp),
                map(alt((tag("DCCP"), tag("dccp"))), |_| CandidateProtocol::Dccp),
            ))),
            wsf(read_number), // priority
            wsf(read_addr),   // addr
            wsf(read_number), // port
            preceded(
                tag("typ"),
                // typ:
                wsf(alt((
                    map(tag("host"), |_| CandidateType::Host),
                    map(tag("relay"), |_| CandidateType::Relay),
                    map(tag("srflx"), |_| CandidateType::Srflx),
                    map(tag("prflx"), |_| CandidateType::Prflx),
                ))),
            ),
            opt(preceded(wsf(tag("raddr")), read_addr)), // raddr
            opt(preceded(wsf(tag("rport")), read_number)), // rport
            opt(preceded(wsf(tag("tcptype")), cowify(read_string))), // tcptype
            opt(preceded(wsf(tag("generation")), read_number)), // generation
            opt(preceded(wsf(tag("network-id")), read_number)), // generation
        )),
        |(
            foundation,
            component,
            protocol,
            priority,
            addr,
            port,
            r#type,
            raddr,
            rport,
            tcptype,
            generation,
            network_id,
        )| Candidate {
            foundation,
            component,
            protocol,
            priority,
            addr,
            port,
            r#type,
            raddr,
            rport,
            tcptype,
            generation,
            network_id,
        },
    )(input)
}

/// "a=Candidate"
pub fn candidate_line(input: &str) -> IResult<&str, Candidate> {
    attribute("candidate", candidate)(input)
}
