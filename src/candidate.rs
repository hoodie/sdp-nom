//! Minimal Candidate parser
//!
//! read [RFC5245 Section 15.1](https://tools.ietf.org/html/rfc5245#section-15.1)

use nom::*;
use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, take_while, take_while1},
    character::{
        complete::{anychar, char, multispace0, none_of, space1},
        is_digit,
    },
    combinator::{map, map_res, opt},
    error::ParseError,
    multi::many0,
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    Parser,
};

use std::net::IpAddr;

#[cfg(test)]
use crate::assert_line;
use crate::parsers::{read_addr, read_number, wsf};

/// Candidate
///
/// https://tools.ietf.org/html/rfc5245#section-15.1
/// https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidateInit/candidate
///
///
/// candidate:3348148302 1 udp 2113937151 192.0.2.1 56500 typ host
/// candidate:3348148302 2 udp 2113937151 192.0.2.1 56501 typ host
#[derive(Debug)]
pub struct Candidate {
    foundation: u32,
    component: CandidateComponent,
    protocol: CandidateProtocol,
    priority: u32,         // 2043278322
    addr: IpAddr,          // "192.168.0.56"
    port: u32,             // 44323
    typ: CandidateType,    // "host"
    raddr: Option<IpAddr>, // "192.168.0.56"
    rport: Option<u32>,    // 44323
}

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

pub(crate) fn raw_candidate(input: &str) -> IResult<&str, Candidate> {
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
                wsf(opt(read_addr)),   // raddr
                wsf(opt(read_number)), // rport
            )),
            |(foundation, component, protocol, priority, addr, port, _, typ, raddr, rport)| {
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
                }
            },
        ),
    )(input)
}

pub fn parse_candidate(raw: &str) -> Option<Candidate> {
    match raw_candidate(raw) {
        Ok((_, candidate)) => Some(candidate),
        _ => None,
    }
}

/// "a=Candidate"
pub fn raw_candidate_line(input: &str) -> IResult<&str, Candidate> {
    preceded(tag("a="), raw_candidate)(input)
}

pub fn parse_candidate_line(raw: &str) -> Option<Candidate> {
    match raw_candidate_line(raw) {
        Ok((_, candidate)) => Some(candidate),
        _ => None,
    }
}

// fn raw_parse_candidate_lines(input: &str) -> IResult<&str, Vec<Candidate>> {
//     many0(terminated(raw_candidate_line, opt(multispace0)))(input)
// }

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use super::*;

    #[test]
    fn parses_candidates() {
        println!("{:?}", parse_candidate("candidate:3348148302 1 udp 2113937151 192.0.2.1 56500 typ host").unwrap());
        println!("{:?}", parse_candidate("candidate:3348148302 1 UDP 2113937151 192.0.2.1 56500 typ relay").unwrap());
        println!("{:?}", parse_candidate("candidate:3348148302 1 UDP 2113937151 192.0.2.1 56500 typ srflx").unwrap());
        println!("{:?}", parse_candidate("candidate:3348148302 1 tcp 2113937151 192.0.2.1 56500 typ srflx").unwrap());
        println!("{:?}", parse_candidate("candidate:3348148302 2 tcp 2113937151 192.0.2.1 56500 typ srflx").unwrap());
        println!("{:?}", parse_candidate("candidate:3348148302 2 tcp 2113937151 ::1 56500 typ srflx ::1 1337").unwrap());
    }

    #[test]
    fn parses_candidate_line() {
        assert_line!("a=candidate:3348148302 1 udp 2113937151 192.0.2.1 56500 typ host", raw_candidate_line);
        assert_line!("a=candidate:3348148302 1 UDP 2113937151 192.0.2.1 56500 typ relay", raw_candidate_line);
        assert_line!("a=candidate:3348148302 1 UDP 2113937151 192.0.2.1 56500 typ srflx", raw_candidate_line);
        assert_line!("a=candidate:3348148302 1 tcp 2113937151 192.0.2.1 56500 typ srflx", raw_candidate_line);
        assert_line!("a=candidate:3348148302 2 tcp 2113937151 192.0.2.1 56500 typ srflx", raw_candidate_line);
        assert_line!("a=candidate:3348148302 2 tcp 2113937151 ::1 56500 typ srflx ::1 1337", raw_candidate_line);
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
        raw_candidate("candidate:3348148302 1 udp 2113937151 293.0.2.1 56500 typ host\n").unwrap();
    }
}
