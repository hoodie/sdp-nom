use nom::*;
use nom::types::CompleteStr;

mod parsers;
pub mod lines;
pub mod candidate;
pub mod origin;
pub mod connection;

use lines::*;
use origin::*;
use candidate::*;
use connection::*;

#[derive(Debug)]
pub enum SdpLine<'a> {
    Version(u32),
    Name(Name<'a>),
    Timing(Timing),
    Origin(Origin<'a>),
    Connection(Connection),
    Candidate(Candidate),
    Mid(Mid<'a>),
    Direction(Direction),
    EoC,
    Aline(Vec<CompleteStr<'a>>)
}

named!(raw_sdp_line<CompleteStr, SdpLine >,
    alt!(
        raw_version_line => { |v| SdpLine::Version(v) } |
        raw_name_line => { |v| SdpLine::Name(v) } |
        raw_timing_line => { |t| SdpLine::Timing(t) } |
        raw_origin_line => { |o| SdpLine::Origin(o)} |
        raw_connection_line => { |c| SdpLine::Connection(c)} |
        raw_mid_line=> { |m| SdpLine::Mid(m)} |
        raw_direction_line => { |d| SdpLine::Direction(d)} |
        raw_candidate_line => { |c| SdpLine::Candidate(c)} |
        tag!("a=end-of-candidates") => { |_| SdpLine::EoC}
        // | raw_a_line => { |v| SdpLine::Aline(v)}
    )
);

named!(raw_sdp_lines<CompleteStr, Vec<SdpLine> >,
    many0!(terminated!(raw_sdp_line, opt!(multispace)))
);

pub fn sdp_line(raw: &str) -> Option<SdpLine> {
    match raw_sdp_line(CompleteStr(raw)) {
        Ok((_, line)) => Some(line),
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_by_line() {
        let jsep_sdp = include_str!("../sdp-transform/test/jsep.sdp");
        jsep_sdp
            .lines()
            .map(|line| (sdp_line(line), line) )
            .for_each(|sdp_line| println!("{:?}", sdp_line));

    }

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