#![allow(dead_code)]

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

use crate::parsers::*;

/// "v=0"
pub(crate) fn raw_version_line(input: &str) -> IResult<&str, u32> {
    preceded(tag("v="), wsf(read_number))(input)
}

#[test]
fn test_raw_version_line() {
    assert_eq!(raw_version_line("v=0").unwrap().1, 0);
    assert_eq!(raw_version_line("v= 0").unwrap().1, 0);
}

#[derive(Debug, PartialEq)]
pub struct Name<'a>(pub &'a str);

/// "s=somename"
pub(crate) fn raw_name_line(input: &str) -> IResult<&str, Name> {
    preceded(tag("s="), map(wsf(read_string), Name))(input)
}

#[test]
fn test_raw_name_line() {
    assert_eq!(raw_name_line("s=testname").unwrap().1, Name("testname"));
    assert_eq!(raw_name_line("s= testname").unwrap().1, Name("testname"));
    assert_eq!(raw_name_line("s=testname ").unwrap().1, Name("testname"));
}

#[derive(Debug, PartialEq)]
pub struct Description<'a>(pub &'a str);

/// "i=description"
pub(crate) fn raw_description_line(input: &str) -> IResult<&str, Description> {
    // do_parse!(tag!("i=") >> description: read_string >> (Description(&description)))
    let (i, _) = tag("i=")(input)?;
    let (i, desc) = read_string(i)?;

    Ok((i, Description(desc)))
}

#[test]
#[rustfmt::skip]
fn test_raw_description_line() {
    assert_eq!(raw_description_line("i=testdescription").unwrap().1, Description("testdescription"));
}

#[derive(Debug, PartialEq)]
pub struct Timing {
    start: u32,
    stop: u32,
}

/// "t=0 0"
pub(crate) fn raw_timing_line(input: &str) -> IResult<&str, Timing> {
    wsf(|input| {
        let (input, _) = tag("t=")(input)?;
        let (input, start) = wsf(read_number)(input)?;
        let (input, stop) = wsf(read_number)(input)?;
        Ok((input, Timing { start, stop }))
    })(input)
}

#[test]
#[rustfmt::skip]
fn test_raw_timing_line() {
    assert_eq!(raw_timing_line("t=0 1").unwrap().1, Timing { start: 0, stop: 1 });
    assert_eq!(raw_timing_line("t=  2 3 ").unwrap().1, Timing { start: 2, stop: 3 });
    assert_eq!(raw_timing_line("t=23 42").unwrap().1, Timing { start: 23, stop: 42 });
}

#[derive(Debug, PartialEq)]
pub enum BandWidthType {
    TIAS,
    AS,
    CT,
    RR,
    RS,
}
// TIAS|AS|CT|RR|RS
pub(crate) fn raw_bandwidth_type(input: &str) -> IResult<&str, BandWidthType> {
    alt((
        map(tag("TIAS"), |_| BandWidthType::TIAS),
        map(tag("AS"), |_| BandWidthType::AS),
        map(tag("CT"), |_| BandWidthType::CT),
        map(tag("RR"), |_| BandWidthType::RR),
        map(tag("RS"), |_| BandWidthType::RS),
    ))(input)
}

#[derive(Debug, PartialEq)]
pub struct BandWidth {
    r#type: BandWidthType,
    limit: u32,
}

/// "b=AS:1024"
pub(crate) fn raw_bandwidth_line(input: &str) -> IResult<&str, BandWidth> {
    preceded(
        tag("b="),
        map(
            separated_pair(raw_bandwidth_type, tag(":"), read_number),
            |(r#type, limit)| (BandWidth { r#type, limit }),
        ),
    )(input)
}

#[test]
#[rustfmt::skip]
fn test_raw_bandwidth_line() {
    assert_eq!(
        raw_bandwidth_line("b=AS:30").unwrap().1,
        BandWidth { r#type: BandWidthType::AS, limit: 30 }
    );
    assert_eq!(
        raw_bandwidth_line("b=RR:1024").unwrap().1,
        BandWidth { r#type: BandWidthType::RR, limit: 1024 }
    );
}

#[derive(Debug, PartialEq)]
pub struct Media<'a> {
    r#type: &'a str,
    port: u32,
    protocol: Vec<&'a str>,
    payloads: Vec<u32>,
}

pub(crate) fn raw_media_line(input: &str) -> IResult<&str, Media> {
    preceded(
        tag("m="),
        wsf(map(
            tuple((
                wsf(read_string),             // type
                wsf(read_number),             // port
                wsf(slash_separated_strings), // protocol
                wsf(read_as_numbers),         //payloads
            )),
            |(r#type, port, protocol, payloads)| Media {
                r#type,
                port,
                protocol,
                payloads,
            },
        )),
    )(input)
}

#[test]
fn test_raw_mline() {
    let parsed = raw_media_line("m=video 51744 RTP/AVP 126 97 98 34 31").unwrap();
    let expected = Media {
        r#type: "video",
        port: 51744,
        protocol: vec!["RTP", "AVP"],
        payloads: vec![126, 97, 98, 34, 31],
    };

    assert_eq!(parsed.1, expected);
}

#[derive(Debug)]
pub struct Mid<'a>(pub &'a str);

pub(crate) fn raw_mid_line(input: &str) -> IResult<&str, Mid> {
    preceded(tag("a=mid:"), map(read_string, Mid))(input)
}

#[derive(Debug)]
pub struct Msid<'a>(pub Vec<&'a str>);

pub(crate) fn raw_msid_line(input: &str) -> IResult<&str, Msid> {
    preceded(tag("a=msid:"), map(read_as_strings, Msid))(input)
}

pub(crate) fn raw_direction_line(input: &str) -> IResult<&str, Direction> {
    preceded(tag("a="), wsf(read_direction))(input)
}

#[derive(Debug, PartialEq)]
pub struct Ssrc<'a> {
    id: u64,
    attribute: &'a str,
    value: &'a str,
}

/// ssrc
pub(crate) fn raw_ssrc_line(input: &str) -> IResult<&str, Ssrc> {
    preceded(
        tag("a=ssrc:"),
        map(
            tuple((
                wsf(read_big_number), // id
                multispace0,
                read_non_colon_string, //attribute
                tag(":"),
                wsf(read_string), // value
            )),
            |(id, _, attribute, _, value)| Ssrc {
                id,
                attribute: &attribute,
                value: &value,
            },
        ),
    )(input)
}

#[test]
#[rustfmt::skip]
fn parse_ssrc_line() {
    assert_eq!(
        raw_ssrc_line("a=ssrc:1366781084 cname:EocUG1f0fcg/yvY7").unwrap().1,
        Ssrc { id: 1366781084, attribute: "cname", value: "EocUG1f0fcg/yvY7" }
    );
    assert_eq!(
        raw_ssrc_line("a=ssrc: 1366781084 cname: EocUG1f0fcg/yvY7").unwrap().1,
        Ssrc { id: 1366781084, attribute: "cname", value: "EocUG1f0fcg/yvY7" }
    );
}

#[derive(Debug)]
pub struct Fingerprint<'a> {
    r#type: &'a str,
    hash: &'a str,
}

/// fingerprint
pub(crate) fn raw_fingerprint_line(input: &str) -> IResult<&str, Fingerprint> {
    preceded(
        tag("a=fingerprint:"),
        map(
            tuple((
                wsf(read_string), // type
                wsf(read_string), // hash
            )),
            |(r#type, hash)| Fingerprint { r#type, hash },
        ),
    )(input)
}

#[test]
fn parse_fingerprint_line() {
    println!("{:?}",
        raw_fingerprint_line("a=fingerprint:sha-256 19:E2:1C:3B:4B:9F:81:E6:B8:5C:F4:A5:A8:D8:73:04:BB:05:2F:70:9F:04:A9:0E:05:E9:26:33:E8:70:88:A2").unwrap()
    );
}

/// generic a line
pub(crate) fn raw_a_line(input: &str) -> IResult<&str, Vec<&str>> {
    //do_parse!(tag!("a=") >> line: read_as_strings >> (line))
    preceded(tag("a="), read_as_strings)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_mid_line() {
        println!("{:?}", raw_mid_line("a=mid:1").unwrap());
        println!("{:?}", raw_mid_line("a=mid:a1").unwrap());
        println!("{:?}", raw_mid_line("a=mid:0").unwrap());
        println!("{:?}", raw_mid_line("a=mid:audio").unwrap());
    }
}
