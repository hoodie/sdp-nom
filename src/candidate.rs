//! Minimal Candidate parser
//!
//! read [RFC5245 Section 15.1](https://tools.ietf.org/html/rfc5245#section-15.1)

use nom::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::{preceded, tuple},
};

use std::net::IpAddr;

#[cfg(test)]
use crate::assert_line;
use crate::parsers::{read_addr, read_number, read_string, wsf};

#[derive(Debug)]
pub enum CandidateComponent {
    Rtp,
    Rtcp,
}

#[derive(Debug)]
pub enum CandidateProtocol {
    Tcp,
    Udp,
    Dccp,
}

#[derive(Debug)]
pub enum CandidateType {
    Host,
    Relay,
    Srflx,
    Prflx,
}

/// Candidate
///
/// https://tools.ietf.org/html/rfc5245#section-15.1
/// https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidateInit/candidate
///
///
/// candidate:3348148302 1 udp 2113937151 192.0.2.1 56500 typ host
/// candidate:3348148302 2 udp 2113937151 192.0.2.1 56501 typ host
// "candidate:1853887674 2 udp 1518280447 47.61.61.61 36768 typ srflx raddr 192.168.0.196 rport 36768 generation 0"
#[derive(Debug)]
pub struct Candidate<'a> {
    foundation: u32,
    component: CandidateComponent,
    protocol: CandidateProtocol,
    priority: u32,         // 2043278322
    addr: IpAddr,          // "192.168.0.56"
    port: u32,             // 44323
    typ: CandidateType,    // "host"
    raddr: Option<IpAddr>, // "192.168.0.56"
    rport: Option<u32>,    // 44323
    tcptype: Option<&'a str>,
    generation: Option<u32>,
}

pub(crate) fn candidate(input: &str) -> IResult<&str, Candidate> {
    preceded(
        tag("candidate:"),
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
                tag("typ"),
                // typ:
                wsf(alt((
                    map(tag("host"), |_| CandidateType::Host),
                    map(tag("relay"), |_| CandidateType::Relay),
                    map(tag("srflx"), |_| CandidateType::Srflx),
                    map(tag("prflx"), |_| CandidateType::Prflx),
                ))),
                //wsf(opt(read_addr)),                                // raddr
                opt(preceded(wsf(tag("raddr")), read_addr)), // raddr
                // wsf(opt(read_number)),                              // rport
                opt(preceded(wsf(tag("rport")), read_number)), // rport
                opt(preceded(wsf(tag("tcptype")), read_string)), // tcptype
                opt(preceded(wsf(tag("generation")), read_number)), // generation
            )),
            |(
                foundation,
                component,
                protocol,
                priority,
                addr,
                port,
                _,
                typ,
                raddr,
                rport,
                tcptype,
                generation,
            )| {
                Candidate {
                    foundation,
                    component,
                    protocol,
                    priority,
                    addr,
                    port,
                    typ,
                    raddr,
                    rport,
                    tcptype,
                    generation,
                }
            },
        ),
    )(input)
}

pub fn parse_candidate(raw: &str) -> Option<Candidate> {
    match candidate(raw) {
        Ok((_, candidate)) => Some(candidate),
        _ => None,
    }
}

/// "a=Candidate"
pub fn candidate_line(input: &str) -> IResult<&str, Candidate> {
    preceded(tag("a="), candidate)(input)
}

pub fn parse_candidate_line(raw: &str) -> Option<Candidate> {
    match candidate_line(raw) {
        Ok((_, candidate)) => Some(candidate),
        _ => None,
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use super::*;

    #[test]
    fn parses_candidate_line() {
        assert_line!("a=candidate:3348148302 1 udp 2113937151 192.0.2.1 56500 typ host", candidate_line);
        assert_line!("a=candidate:3348148302 1 UDP 2113937151 192.0.2.1 56500 typ relay", candidate_line);
        assert_line!("a=candidate:3348148302 1 UDP 2113937151 192.0.2.1 56500 typ srflx", candidate_line);
        assert_line!("a=candidate:3348148302 1 tcp 2113937151 192.0.2.1 56500 typ srflx", candidate_line);
        assert_line!("a=candidate:3348148302 2 tcp 2113937151 192.0.2.1 56500 typ srflx", candidate_line);
        // assert_line!("a=candidate:3348148302 2 tcp 2113937151 ::1 56500 typ srflx ::1 1337", candidate_line); // FIXME: is this one compliant?
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
    fn accepts_breaks() {
        let parsed_host = parse_candidate("candidate:3348148302 1 udp 2113937151 192.0.2.1 56500 typ host\n").unwrap();
        println!("{:#?}", parsed_host);
    }

    #[test]
    #[should_panic]
    #[ignore]
    fn fails_on_bad_ip() {
        candidate("candidate:3348148302 1 udp 2113937151 293.0.2.1 56500 typ host\n").unwrap();
    }
}
