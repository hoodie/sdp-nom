//! Minimal Candidate parser
//! 
//! read [RFC5245 Section 15.1](https://tools.ietf.org/html/rfc5245#section-15.1)
use nom::*;
use nom::types::CompleteStr;

use std::net::IpAddr;

use super::parsers::is_not_space;

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
    priority: u32, // 2043278322
    addr: IpAddr,       // "192.168.0.56"
    port: u32,     // 44323
    typ: CandidateType, // "host"
    raddr: Option<IpAddr>,       // "192.168.0.56"
    rport: Option<u32>,     // 44323
}

#[derive(Debug)]
pub enum CandidateComponent{
    Rtp, Rtcp
}

#[derive(Debug)]
pub enum CandidateProtocol {
    Tcp, Udp, Dccp
}

#[derive(Debug)]
pub enum CandidateType {
    Host, Relay, Srflx, Prflx
}

named!{
    raw_parse_candidate<CompleteStr, Candidate>,
    ws!(
        do_parse!(
            tag!("candidate:") >>
            foundation: map_res!(take_while1!(is_not_space), |i: CompleteStr| u32::from_str_radix(&i, 10)) >>

            component: alt!(
                tag!("1") => {|_| CandidateComponent::Rtp } |
                tag!("2") => {|_| CandidateComponent::Rtcp }
            ) >>

            protocol: alt!(
                alt!(tag!("UDP") | tag!("udp")) => { |_| CandidateProtocol::Udp} |
                alt!(tag!("TCP") | tag!("tcp")) => { |_| CandidateProtocol::Tcp} |
                alt!(tag!("DCCP") | tag!("dccp"))        => { |_| CandidateProtocol::Dccp}
            ) >>

            priority: map_res!(take_while1!(is_not_space), |i: CompleteStr| u32::from_str_radix(&i, 10)) >>

            addr: map_res!(take_while1!(is_not_space), |i: CompleteStr| i.parse() ) >>

            port: map_res!(take_while1!(is_not_space), |i: CompleteStr| u32::from_str_radix(&i, 10)) >>

            tag!("typ") >>
            typ: alt!(
                tag!("host") => { |_| CandidateType::Host } |
                tag!("relay") => { |_| CandidateType::Relay} |
                tag!("srflx") => { |_| CandidateType::Srflx} |
                tag!("prflx") => { |_| CandidateType::Prflx}
            ) >>

            raddr: opt!(map_res!(take_while1!(is_not_space), |i: CompleteStr| i.parse() )) >>
            rport: opt!(map_res!(take_while1!(is_not_space), |i: CompleteStr| u32::from_str_radix(&i, 10))) >>

            (Candidate {
                foundation,
                component,
                protocol,
                priority,
                addr, port,
                typ,
                raddr, rport,
            })
        )
    )
}

pub fn parse_candidate(raw: &str) -> Option<Candidate> {
    match raw_parse_candidate(CompleteStr(raw)) {
        Ok((_, candidate)) => Some(candidate),
        _ => None
    }
}

/// "a=Candidate"
named!{
    pub raw_parse_candidate_line<CompleteStr, Candidate>,
    ws!(
        do_parse!(
            tag!("a=") >>
            candidate: raw_parse_candidate >>
            (candidate)
        )
    )
}

pub fn parse_candidate_line(raw: &str) -> Option<Candidate> {
    match raw_parse_candidate_line(CompleteStr(raw)) {
        Ok((_, candidate)) => Some(candidate),
        _ => None
    }
}


named!{
    raw_parse_candidate_lines <CompleteStr, Vec<Candidate>>,
    many0!(terminated!(raw_parse_candidate_line, opt!(multispace)))
}

#[cfg(test)]
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
        println!("{:?}", parse_candidate_line("a=candidate:3348148302 1 udp 2113937151 192.0.2.1 56500 typ host").unwrap());
        println!("{:?}", parse_candidate_line("a=candidate:3348148302 1 UDP 2113937151 192.0.2.1 56500 typ relay").unwrap());
        println!("{:?}", parse_candidate_line("a=candidate:3348148302 1 UDP 2113937151 192.0.2.1 56500 typ srflx").unwrap());
        println!("{:?}", parse_candidate_line("a=candidate:3348148302 1 tcp 2113937151 192.0.2.1 56500 typ srflx").unwrap());
        println!("{:?}", parse_candidate_line("a=candidate:3348148302 2 tcp 2113937151 192.0.2.1 56500 typ srflx").unwrap());
        println!("{:?}", parse_candidate_line("a=candidate:3348148302 2 tcp 2113937151 ::1 56500 typ srflx ::1 1337").unwrap());
    }

    #[test]
    fn parses_candidate_lines() {
        println!("{:#?}", raw_parse_candidate_lines(CompleteStr(
        "a=candidate:3348148302 1 udp 2113937151 192.0.2.1 56500 typ host
        a=candidate:3348148302 1 UDP 2113937151 192.0.2.1 56500 typ relay
        a=candidate:3348148302 1 UDP 2113937151 192.0.2.1 56500 typ srflx
        a=candidate:3348148302 1 tcp 2113937151 192.0.2.1 56500 typ srflx
        a=candidate:3348148302 2 tcp 2113937151 192.0.2.1 56500 typ srflx
        a=candidate:3348148302 2 tcp 2113937151 ::1 56500 typ srflx ::1 1337")))
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
        raw_parse_candidate(CompleteStr("candidate:3348148302 1 udp 2113937151 293.0.2.1 56500 typ host\n")).unwrap();
    }
}