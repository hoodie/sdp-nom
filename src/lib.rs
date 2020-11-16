#![allow(unused_imports)]
use nom::{
    branch::{alt, Alt},
    bytes::complete::{escaped, tag, take_while},
    character::complete::{alphanumeric1 as alphanumeric, char, one_of},
    combinator::{cut, map, opt, value},
    error::{context, convert_error, ContextError, ErrorKind, ParseError, VerboseError},
    multi::separated_list0,
    number::complete::double,
    sequence::{delimited, preceded, separated_pair, terminated},
    Err, IResult,
};

pub mod attributes;
pub mod candidate;
pub mod connection;
pub mod lines;
pub mod origin;
mod parsers;

use attributes::*;
use candidate::*;
use connection::*;
use lines::*;
use origin::*;
use parsers::*;

#[derive(Debug)]
pub enum SdpLine<'a> {
    Version(u32),
    Name(Name<'a>),
    Timing(Timing),
    Origin(Origin<'a>),
    Candidate(Candidate),
    Connection(Connection),
    Media(Media<'a>),
    Mid(Mid<'a>),
    Msid(Msid<'a>),
    Ssrc(Ssrc<'a>),
    Fingerprint(Fingerprint<'a>),
    Direction(Direction),
    BundleOnly,
    EoC,
    Aline(Vec<&'a str>),
}

pub fn raw_sdp_line(input: &str) -> IResult<&str, SdpLine> {
    alt((
        map(raw_version_line, SdpLine::Version),
        map(raw_name_line, SdpLine::Name),
        map(raw_timing_line, SdpLine::Timing),
        map(raw_origin_line, SdpLine::Origin),
        map(raw_candidate_line, SdpLine::Candidate),
        map(raw_connection_line, SdpLine::Connection),
        map(raw_mid_line, SdpLine::Mid),
        map(raw_msid_line, SdpLine::Msid),
        map(raw_media_line, SdpLine::Media),
        map(raw_ssrc_line, SdpLine::Ssrc),
        map(raw_fingerprint_line, SdpLine::Fingerprint),
        map(raw_direction_line, SdpLine::Direction),
        map(tag("a=bundle-only"), |_| SdpLine::BundleOnly),
        map(raw_a_line, SdpLine::Aline),
        map(tag("a=end-of-candidates"), |_| SdpLine::EoC),
    ))(input)
}

pub fn raw_sdp_lines(input: &str) -> IResult<&str, Vec<SdpLine>> {
    use nom::{character::complete::multispace0, multi::many0, sequence::terminated};
    many0(terminated(raw_sdp_line, multispace0))(input)
}

pub fn sdp_line(raw: &str) -> Option<SdpLine> {
    match raw_sdp_line(raw) {
        Ok((_, line)) => Some(line),
        _ => None,
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
            .map(|line| (sdp_line(line), line))
            .for_each(|sdp_line| println!("{:?}", sdp_line));
    }

    #[test]
    fn test_version() {
        assert_eq!(raw_version_line("v=0"), Ok(("", 0)));
        assert_eq!(raw_version_line("v=1"), Ok(("", 1)))
    }

    #[test]
    fn parses_sdp() {
        println!(
            "{}",
            raw_sdp_lines(
                "v=0
        a=candidate:3348148302 1 udp 2113937151 192.0.2.1 56500 typ host
        a=candidate:3348148302 1 UDP 2113937151 192.0.2.1 56500 typ relay
        a=candidate:3348148302 1 UDP 2113937151 192.0.2.1 56500 typ srflx
        a=candidate:3348148302 1 tcp 2113937151 192.0.2.1 56500 typ srflx
        a=candidate:3348148302 2 tcp 2113937151 192.0.2.1 56500 typ srflx
        a=candidate:3348148302 2 tcp 2113937151 ::1 56500 typ srflx ::1 1337"
            )
            .unwrap()
            .0
        )
    }
}
