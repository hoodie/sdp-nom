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

#[derive(Copy, Clone, PartialEq)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub enum CandidateComponent {
    Rtp,
    Rtcp,
}

#[derive(Copy, Clone, PartialEq)]
#[cfg_attr(feature = "debug", derive(Debug))]
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

#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "debug", derive(Debug))]
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
#[derive(Clone, IntoOwned, PartialEq)]
#[cfg_attr(feature = "debug", derive(Debug))]
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

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use std::net::Ipv4Addr;

    use crate::{assert_line, assert_line_print};

    use super::*;

    #[test]
    fn parses_candidate_line() {
        assert_line_print!(candidate_line, "a=candidate:3348148302 1 udp 2113937151 192.0.2.1 56500 typ host");
        assert_line_print!(candidate_line, "a=candidate:3348148302 1 tcp 2113937151 192.0.2.1 56500 typ srflx");
        assert_line_print!(candidate_line, "a=candidate:3348148302 2 tcp 2113937151 192.0.2.1 56500 typ srflx");
        assert_line!(candidate_line, "a=candidate:1 1 TCP 2128609279 10.0.1.1 9 typ host tcptype active", Candidate {
            foundation: 1, component: CandidateComponent::Rtp, protocol: CandidateProtocol::Tcp, priority: 2128609279, addr: Ipv4Addr::new(10,0,1,1).into(), port: 9,
            r#type: CandidateType::Host, raddr: None, rport: None, tcptype: Some( Cow::from("active"),), generation: None, network_id: None, });
        assert_line_print!(candidate_line, "a=candidate:2 1 tcp 2124414975 10.0.1.1 8998 typ host tcptype passive");
        assert_line_print!(candidate_line, "a=candidate:3 1 tcp 2120220671 10.0.1.1 8999 typ host tcptype so");
        assert_line_print!(candidate_line, "a=candidate:4 1 tcp 1688207359 192.0.2.3 9 typ srflx raddr 10.0.1.1 rport 9 tcptype active");
        assert_line_print!(candidate_line, "a=candidate:5 1 tcp 1684013055 192.0.2.3 45664 typ srflx raddr 10.0.1.1 rport 8998 tcptype passive generation 5");
        assert_line_print!(candidate_line, "a=candidate:6 1 tcp 1692401663 192.0.2.3 45687 typ srflx raddr 10.0.1.1 rport 8999 tcptype so");
        assert_line!(candidate_line, "a=candidate:3348148302 1 UDP 2113937151 192.0.2.1 56500 typ relay");
        assert_line!(candidate_line, "a=candidate:3348148302 1 UDP 2113937151 192.0.2.1 56500 typ srflx");
        // assert_line!("a=candidate:3348148302 2 tcp 2113937151 ::1 56500 typ srflx ::1 1337", candidate_line); // FIXME: is this one compliant?
        assert_line_print!(candidate_line, "a=candidate:2791055836 1 udp 2122262783 2001:9e8:b0b:8400:c5e3:8776:82fc:7704 58605 typ host generation 0 network-id 2");
    }

    #[test]
    fn audio_lines() {

        let lines =[
            "a=candidate:1467250027 1 udp 2122260223 192.168.0.196 46243 typ host generation 0",
            "a=candidate:1467250027 2 udp 2122260222 192.168.0.196 56280 typ host generation 0",
            "a=candidate:435653019 1 tcp 1845501695 192.168.0.196 0 typ host tcptype active generation 0",
            "a=candidate:435653019 2 tcp 1845501695 192.168.0.196 0 typ host tcptype active generation 0",
            "a=candidate:1853887674 1 udp 1518280447 47.61.61.61 36768 typ srflx raddr 192.168.0.196 rport 36768 generation 0",
            "a=candidate:1853887674 2 udp 1518280447 47.61.61.61 36768 typ srflx raddr 192.168.0.196 rport 36768 generation 0",
            "a=candidate:750991856 2 udp 25108222 237.30.30.30 51472 typ relay raddr 47.61.61.61 rport 54763 generation 0",
            "a=candidate:750991856 1 udp 25108223 237.30.30.30 58779 typ relay raddr 47.61.61.61 rport 54761 generation 0",
            ];
        for line in &lines {
            assert_line!(*line, candidate_line);
        }

    }

    #[test]
    #[should_panic]
    #[ignore]
    fn fails_on_bad_ip() {
        candidate("candidate:3348148302 1 udp 2113937151 293.0.2.1 56500 typ host\n").unwrap();
    }
}
