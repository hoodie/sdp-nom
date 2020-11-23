//! Minimal Candidate parser
//!
//! read [RFC5245 Section 15.1](https://tools.ietf.org/html/rfc5245#section-15.1)

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::{preceded, tuple},
    IResult,
};

use std::{fmt::Display, net::IpAddr};

use crate::parsers::{attribute, read_addr, read_number, read_string, wsf};

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
            typ,
            raddr,
            rport,
            tcptype,
            generation,
        )| Candidate {
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
        },
    )(input)
}

/// "a=Candidate"
pub fn candidate_line(input: &str) -> IResult<&str, Candidate> {
    attribute("candidate", candidate)(input)
}

impl Display for CandidateComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CandidateComponent::Rtp => write!(f, "1"),
            CandidateComponent::Rtcp => write!(f, "2"),
        }
    }
}

impl Display for CandidateProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CandidateProtocol::Tcp => write!(f, "tcp"),
            CandidateProtocol::Udp => write!(f, "udp"),
            CandidateProtocol::Dccp => write!(f, "dccp"),
        }
    }
}

impl Display for CandidateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CandidateType::Host => write!(f, "host"),
            CandidateType::Relay => write!(f, "relay"),
            CandidateType::Srflx => write!(f, "srflx"),
            CandidateType::Prflx => write!(f, "prflx"),
        }
    }
}

impl Display for Candidate<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "a=candidate:{} {} {} {} {} {} typ {}",
            self.foundation,
            self.component,
            self.protocol,
            self.priority,
            self.addr,
            self.port,
            self.typ,
        )?;
        if let Some(x) = self.raddr {
            write!(f, "{}", x)?;
        }
        if let Some(x) = self.rport {
            write!(f, "{}", x)?;
        }
        if let Some(x) = self.tcptype {
            write!(f, "{}", x)?;
        }
        if let Some(x) = self.generation {
            write!(f, "{}", x)?;
        }
        Ok(())
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use crate::{assert_line, assert_line_print};

    use super::*;

    #[test]
    fn parses_candidate_line() {
        assert_line_print!(candidate_line, "a=candidate:3348148302 1 udp 2113937151 192.0.2.1 56500 typ host");
        assert_line_print!(candidate_line, "a=candidate:3348148302 1 tcp 2113937151 192.0.2.1 56500 typ srflx");
        assert_line_print!(candidate_line, "a=candidate:3348148302 2 tcp 2113937151 192.0.2.1 56500 typ srflx");
        assert_line!(candidate_line, "a=candidate:3348148302 1 UDP 2113937151 192.0.2.1 56500 typ relay");
        assert_line!(candidate_line, "a=candidate:3348148302 1 UDP 2113937151 192.0.2.1 56500 typ srflx");
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
    #[should_panic]
    #[ignore]
    fn fails_on_bad_ip() {
        candidate("candidate:3348148302 1 udp 2113937151 293.0.2.1 56500 typ host\n").unwrap();
    }
}
