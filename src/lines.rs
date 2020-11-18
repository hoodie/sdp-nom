#![allow(dead_code)]
use nom::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    sequence::{preceded, separated_pair, tuple},
};

#[cfg(test)]
use crate::assert_line;
use crate::parsers::*;

/// "v=0"
pub(crate) fn version_line(input: &str) -> IResult<&str, u32> {
    preceded(tag("v="), wsf(read_number))(input)
}

#[test]
fn test_version_line() {
    assert_line!(version_line, "v=0");
    assert_line!(version_line, "v= 0");
}

/// `s=somename`
#[derive(Debug, PartialEq)]
pub struct Name<'a>(pub &'a str);

/// "s=somename"
pub(crate) fn name_line(input: &str) -> IResult<&str, Name> {
    preceded(tag("s="), map(wsf(read_string0), Name))(input)
}

#[test]
fn test_name_line() {
    assert_line!(name_line, "s=", Name(""));
    assert_line!(name_line, "s=testname", Name("testname"));
    assert_line!(name_line, "s= testname", Name("testname"));
    assert_line!(name_line, "s=testname ", Name("testname"));
}

#[derive(Debug, PartialEq)]
pub struct Description<'a>(pub &'a str);

/// "i=description"
pub(crate) fn description_line(input: &str) -> IResult<&str, Description> {
    // do_parse!(tag!("i=") >> description: read_string >> (Description(&description)))
    let (i, _) = tag("i=")(input)?;
    let (i, desc) = read_string(i)?;

    Ok((i, Description(desc)))
}

#[test]
fn test_description_line() {
    assert_line!(
        description_line,
        "i=testdescription",
        Description("testdescription")
    );
}

/// `t=0 0`
#[derive(Debug, PartialEq)]
pub struct Timing {
    start: u32,
    stop: u32,
}

/// "t=0 0"
pub(crate) fn timing_line(input: &str) -> IResult<&str, Timing> {
    wsf(|input| {
        let (input, _) = tag("t=")(input)?;
        let (input, start) = wsf(read_number)(input)?;
        let (input, stop) = wsf(read_number)(input)?;
        Ok((input, Timing { start, stop }))
    })(input)
}

#[test]
#[rustfmt::skip]
fn test_timing_line() {
    assert_line!(timing_line,"t=0 1", Timing { start: 0, stop: 1 });
    assert_line!(timing_line,"t=  2 3 ", Timing { start: 2, stop: 3 });
    assert_line!(timing_line,"t=23 42", Timing { start: 23, stop: 42 });
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
pub(crate) fn bandwidth_type(input: &str) -> IResult<&str, BandWidthType> {
    alt((
        map(tag("TIAS"), |_| BandWidthType::TIAS),
        map(tag("AS"), |_| BandWidthType::AS),
        map(tag("CT"), |_| BandWidthType::CT),
        map(tag("RR"), |_| BandWidthType::RR),
        map(tag("RS"), |_| BandWidthType::RS),
    ))(input)
}

#[derive(Debug, PartialEq)]
/// "b=AS:1024"
pub struct BandWidth {
    r#type: BandWidthType,
    limit: u32,
}

/// "b=AS:1024"
pub(crate) fn bandwidth_line(input: &str) -> IResult<&str, BandWidth> {
    preceded(
        tag("b="),
        map(
            separated_pair(bandwidth_type, tag(":"), read_number),
            |(r#type, limit)| (BandWidth { r#type, limit }),
        ),
    )(input)
}

#[test]
#[rustfmt::skip]
fn test_bandwidth_line() {
    assert_line!(
        bandwidth_line,"b=AS:30",
        BandWidth { r#type: BandWidthType::AS, limit: 30 }
    );
    assert_line!(
        bandwidth_line,"b=RR:1024",
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

pub(crate) fn media_line(input: &str) -> IResult<&str, Media> {
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
fn test_mline() {
    assert_line!(
        media_line,
        "m=video 51744 RTP/AVP 126 97 98 34 31",
        Media {
            r#type: "video",
            port: 51744,
            protocol: vec!["RTP", "AVP"],
            payloads: vec![126, 97, 98, 34, 31],
        }
    );
}

#[derive(Debug)]
pub struct Mid<'a>(pub &'a str);

pub(crate) fn mid_line(input: &str) -> IResult<&str, Mid> {
    preceded(tag("a=mid:"), map(read_string, Mid))(input)
}

#[derive(Debug)]
pub struct MsidSemantic<'a>(pub Vec<&'a str>);

pub(crate) fn msid_semantic_line(input: &str) -> IResult<&str, MsidSemantic> {
    preceded(
        tag("a=msid-semantic:"),
        wsf(map(space_separated_strings, MsidSemantic)),
    )(input)
}

#[test]
fn test_msid_semantic_line() {
    assert_line!(
        msid_semantic_line,
        "a=msid-semantic: WMS lgsCFqt9kN2fVKw5wg3NKqGdATQoltEwOdMS"
    );
}

#[derive(Debug)]
pub struct Msid<'a>(pub Vec<&'a str>);

pub(crate) fn msid_line(input: &str) -> IResult<&str, Msid> {
    preceded(tag("a=msid:"), wsf(map(space_separated_strings, Msid)))(input)
}

#[derive(Debug)]
pub struct Fingerprint<'a> {
    r#type: &'a str,
    hash: &'a str,
}

/// fingerprint
pub(crate) fn fingerprint_line(input: &str) -> IResult<&str, Fingerprint> {
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
fn test_fingerprint_line() {
    println!("{:?}",
        fingerprint_line("a=fingerprint:sha-256 19:E2:1C:3B:4B:9F:81:E6:B8:5C:F4:A5:A8:D8:73:04:BB:05:2F:70:9F:04:A9:0E:05:E9:26:33:E8:70:88:A2").unwrap()
    );
}

/// generic a line
pub(crate) fn a_line(input: &str) -> IResult<&str, Vec<&str>> {
    //do_parse!(tag!("a=") >> line: read_as_strings >> (line))
    preceded(tag("a="), read_as_strings)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_mid_line() {
        assert_line!(mid_line, "a=mid:1");
        assert_line!(mid_line, "a=mid:a1");
        assert_line!(mid_line, "a=mid:0");
        assert_line!(mid_line, "a=mid:audio")
    }
}
