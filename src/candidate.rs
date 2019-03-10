use nom::*;
use nom::types::CompleteStr;

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
pub struct Candidate<'a> {
    foundation: u32,
    component: u8, // "rtp" (the number 1 is encoded to this string; 2 becomes "rtcp")
    protocol: &'a str,  // "udp", "tcp" or "dccp"
    priority: u32, // 2043278322
    ip: &'a str,       // "192.168.0.56"
    port: u32,     // 44323
    typ: &'a str// "host"
}

named!{
  raw_parse_candidate<CompleteStr, Candidate>,
    ws!(
    do_parse!(
        tag!("candidate:") >>
        foundation: map_res!(take_while1!(is_not_space), |i: CompleteStr| u32::from_str_radix(&i, 10)) >>
        component: map_res!(take_while1!(is_not_space), |i: CompleteStr| u8::from_str_radix(&i, 10)) >>
        protocol: take_while1!(is_not_space) >>
        priority: map_res!(take_while1!(is_not_space), |i: CompleteStr| u32::from_str_radix(&i, 10)) >>
        ip: take_while1!(is_not_space) >>
        port: map_res!(take_while1!(is_not_space), |i: CompleteStr| u32::from_str_radix(&i, 10)) >>
        tag!("typ") >>
        typ: alphanumeric >>


        (Candidate {
            foundation,
            component,
            protocol: &protocol,
            priority,
            ip: &ip,
            port,
            typ: &typ,
        })
        )
    )
}

pub fn parse_candidate(raw: &str) -> IResult<CompleteStr, Candidate> {
    raw_parse_candidate(CompleteStr(raw))
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let parsed_host = parse_candidate("candidate:3348148302 1 udp 2113937151 192.0.2.1 56500 typ host").unwrap();
        let parsed_relay = parse_candidate("candidate:3348148302 1 udp 2113937151 192.0.2.1 56500 typ relay").unwrap();
        println!("{:#?}", parsed_host);
        println!("{:#?}", parsed_relay);
    }
}