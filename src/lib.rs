use nom::*;
use nom::types::CompleteStr;

mod parsers;
pub mod candidate;

use parsers::is_numeric;
use candidate::{Candidate, raw_parse_candidate_line};

/// "v=0"
named!{
    raw_version_line<CompleteStr, u32>,
    ws!(
        do_parse!(
            tag!("v=") >>
            version: map_res!(take_while1!(is_numeric), |i: CompleteStr| u32::from_str_radix(&i, 10)) >>
            (version)
        )
    )
}

named!(raw_sdp_line<CompleteStr, Vec<SdpLine> >,
    many0!(terminated!(
        alt!(
            raw_version_line => { |v| SdpLine::Version(v) }
            |
            raw_parse_candidate_line => { |c| SdpLine::Candidate(c)}
        )
    , opt!(multispace)))
);

#[derive(Debug)]
pub enum SdpLine {
    Version(u32),
    Candidate(Candidate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(raw_version_line(CompleteStr("v=0")), Ok((CompleteStr(""), 0)));
        assert_eq!(raw_version_line(CompleteStr("v=1")), Ok((CompleteStr(""), 1)))
    }
    
    #[test]
    fn parses_sdp() {
        println!("{:#?}", raw_sdp_line(CompleteStr(
        "v=0
        a=candidate:3348148302 1 udp 2113937151 192.0.2.1 56500 typ host
        a=candidate:3348148302 1 UDP 2113937151 192.0.2.1 56500 typ relay
        a=candidate:3348148302 1 UDP 2113937151 192.0.2.1 56500 typ srflx
        a=candidate:3348148302 1 tcp 2113937151 192.0.2.1 56500 typ srflx
        a=candidate:3348148302 2 tcp 2113937151 192.0.2.1 56500 typ srflx
        a=candidate:3348148302 2 tcp 2113937151 ::1 56500 typ srflx ::1 1337")).unwrap())
    }


}