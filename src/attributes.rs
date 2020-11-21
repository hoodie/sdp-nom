///

/// [6. SDP Attributes](https://tools.ietf.org/html/rfc4566#section-6)
use nom::*;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_till1},
    combinator::{map, opt},
    sequence::{preceded, separated_pair, tuple},
};

pub mod candidate;
pub mod dtls;
pub mod extmap;
pub mod ice;
pub mod rtcp;
pub mod rtpmap;
pub mod ssrc;

pub use candidate::*;
pub use ice::*;
pub use ssrc::*;

#[cfg(test)]
use crate::assert_line;
use crate::parsers::*;

#[derive(Debug, PartialEq)]
pub struct Attribute<'a> {
    kind: AttributeKind<'a>,
    value: &'a str,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum AttributeKind<'a> {
    Rtp,
    Fmtp,
    Control,
    Rtcp,
    RtcpFbTrrInt,
    RtcpFb,
    Ext,
    Crypto,
    Setup,
    // Mid,
    // Msid,
    Ptime,
    MaxPtime,
    // Direction,
    IceLite,
    IceUFrag,
    IcePwd,
    // Fingerprint,
    Candidates,
    // EndOfCandidates,
    RemoteCandidates,
    IceOptions,
    Ssrcs,
    SsrcGroups,
    MsidSemantic,
    Groups,
    RtcpMux,
    RtcpRsize,
    Sctpmap,
    XGoogleFlag,
    Rids,
    Imageattrs,
    Simulcast,
    Simulcast03,
    Framerate,
    SourceFilter,
    BundleOnly,
    Label,
    SctpPort,
    MaxMessageSize,
    Other(&'a str),
}

#[allow(dead_code)]
fn attribute_kind(input: &str) -> IResult<&str, AttributeKind> {
    alt((
        map(tag("setup"), |_| AttributeKind::Setup),
        map(take_till1(|i| i == ':'), AttributeKind::Other),
    ))(input)
}

#[allow(dead_code)]
pub fn generic_attribute_line(input: &str) -> IResult<&str, Attribute> {
    a_line(map(
        separated_pair(attribute_kind, tag(":"), is_not("\n")),
        |(kind, value)| Attribute { kind, value },
    ))(input)
}

#[test]
fn test_generic_attribute_line() {
    assert_line!(
        generic_attribute_line,
        "a=foo:bar",
        Attribute {
            kind: AttributeKind::Other("foo"),
            value: "bar"
        }
    );
    assert_line!(
        generic_attribute_line,
        "a=fmtp:111 minptime=10; useinbandfec=1",
        Attribute {
            kind: AttributeKind::Other("fmtp"),
            value: "111 minptime=10; useinbandfec=1"
        }
    );
    assert_line!(
        generic_attribute_line,
        "a=setup:actpass",
        Attribute {
            kind: AttributeKind::Setup,
            value: "actpass"
        }
    );
}

// ///////////////////////

/// `a=group:BUNDLE 0 1`
#[derive(Debug, PartialEq)]
pub struct BundleGroup<'a>(pub Vec<&'a str>);

pub fn bundle_group_line(input: &str) -> IResult<&str, BundleGroup> {
    attribute("group", bundle_group)(input)
}

fn bundle_group(input: &str) -> IResult<&str, BundleGroup> {
    preceded(
        tag("BUNDLE"),
        map(wsf(space_separated_strings), BundleGroup),
    )(input)
}

#[test]
fn test_bundle_group_line() {
    assert_line!(
        bundle_group_line,
        "a=group:BUNDLE 0 1",
        BundleGroup(vec!["0", "1"])
    );
    assert_line!(
        bundle_group_line,
        "a=group:BUNDLE video",
        BundleGroup(vec!["video"])
    );
    assert_line!(
        bundle_group_line,
        "a=group:BUNDLE sdparta_0 sdparta_1 sdparta_2",
        BundleGroup(vec!["sdparta_0", "sdparta_1", "sdparta_2"])
    );
}

// ///////////////////////

// a=rtpmap:110 opus/48000/2
#[derive(Debug, PartialEq)]
pub struct Rtp<'a> {
    payload: u32,
    codec: &'a str,
    rate: u32,
    encoding: u32,
}

pub fn rtp_attribute_line(input: &str) -> IResult<&str, Rtp> {
    attribute("rtpmap", rtp_attribute)(input)
}

fn rtp_attribute(input: &str) -> IResult<&str, Rtp> {
    map(
        tuple((
            wsf(read_number),                // payload
            wsf(read_non_slash_string),      // codec
            preceded(tag("/"), read_number), // rate
            preceded(tag("/"), read_number), // encoding
        )),
        |(payload, codec, rate, encoding)| Rtp {
            payload,
            codec,
            rate,
            encoding,
        },
    )(input)
}

#[test]
fn test_rtp_attribute_line() {
    assert_line!("a=rtpmap:110 opus/48000/2", rtp_attribute_line);
}

/// https://tools.ietf.org/html/rfc4588#section-8.1
/// `a=fmtp:108 profile-level-id=24;object=23;bitrate=64000`
#[derive(Debug, PartialEq)]
pub struct Fmtp<'a> {
    payload: u32,
    config: &'a str,
}

pub fn fmtp_attribute_line(input: &str) -> IResult<&str, Fmtp> {
    attribute("fmtp", fmtp_attribute)(input)
}

fn fmtp_attribute(input: &str) -> IResult<&str, Fmtp> {
    map(
        tuple((
            read_number,       // payload
            wsf(is_not("\n")), // config
        )),
        |(payload, config)| (Fmtp { payload, config }),
    )(input)
}

#[test]
fn test_fmtp_attribute_line() {
    assert_line!(
        fmtp_attribute_line,
        "a=fmtp:108 profile-level-id=24;object=23;bitrate=64000",
        Fmtp {
            payload: 108,
            config: "profile-level-id=24;object=23;bitrate=64000",
        }
    );
    assert_line!(
        fmtp_attribute_line,
        "a=fmtp:111 minptime=10; useinbandfec=1"
    );
}

// ///////////////////////

/// `a=control:streamid=0`
#[derive(Debug, PartialEq)]
pub struct Control<'a>(&'a str);

pub fn control_attribute_line(input: &str) -> IResult<&str, Control> {
    attribute("control", control_attribute)(input)
}

fn control_attribute(input: &str) -> IResult<&str, Control> {
    map(read_string, Control)(input)
}

#[test]
fn test_control_attribute_line() {
    assert_line!(control_attribute_line, "a=control:streamid=0");
}

// ///////////////////////

/// a=extmap:2 urn:ietf:params:rtp-hdrext:toffset
#[derive(Debug, PartialEq)]
pub struct Extmap<'a> {
    value: u32,
    direction: Option<Direction>,
    uri: Option<&'a str>,
    extended: Option<&'a str>,
}

/// Direction
///
/// `a=sendrecv`
/// `a=sendonly`
/// `a=recvonly`
/// `a=inactive`
#[derive(Debug, PartialEq)]
pub enum Direction {
    SendOnly,
    SendRecv,
    RecvOnly,
    Inactive,
}

pub fn read_direction(input: &str) -> IResult<&str, Direction> {
    alt((
        map(tag("sendrecv"), |_| Direction::SendRecv),
        map(tag("sendonly"), |_| Direction::SendOnly),
        map(tag("recvonly"), |_| Direction::RecvOnly),
        map(tag("inactive"), |_| Direction::Inactive),
    ))(input)
}

/// `a=sendrecv`
pub fn direction_line(input: &str) -> IResult<&str, Direction> {
    a_line(wsf(read_direction))(input)
}
#[test]
fn test_direction_line() {
    assert_line!(read_direction, "sendrecv", Direction::SendRecv);
    assert_line!(direction_line, "a=sendrecv", Direction::SendRecv);

    assert_line!(read_direction, "sendonly", Direction::SendOnly);
    assert_line!(direction_line, "a=sendonly", Direction::SendOnly);

    assert_line!(read_direction, "recvonly", Direction::RecvOnly);
    assert_line!(direction_line, "a=recvonly", Direction::RecvOnly);

    assert_line!(read_direction, "inactive", Direction::Inactive);
    assert_line!(direction_line, "a=inactive", Direction::Inactive);
}

#[derive(Debug, PartialEq)]
pub enum RtcpOption {
    RtcpMux,
    RtcpMuxOnly,
    RtcpRsize,
}

pub fn rtp_option(input: &str) -> IResult<&str, RtcpOption> {
    alt((
        map(tag("rtcp-rsize"), |_| RtcpOption::RtcpRsize),
        map(tag("rtcp-mux-only"), |_| RtcpOption::RtcpMuxOnly),
        map(tag("rtcp-mux"), |_| RtcpOption::RtcpMux),
    ))(input)
}
pub fn rtp_option_line(input: &str) -> IResult<&str, RtcpOption> {
    a_line(rtp_option)(input)
}
#[test]
fn test_read_rtp_option() {
    assert_line!(rtp_option_line, "a=rtcp-mux", RtcpOption::RtcpMux);
    assert_line!(rtp_option_line, "a=rtcp-mux-only", RtcpOption::RtcpMuxOnly);
    assert_line!(rtp_option_line, "a=rtcp-rsize", RtcpOption::RtcpRsize);
}

#[derive(Debug)]
pub struct Fingerprint<'a> {
    r#type: &'a str,
    hash: &'a str,
}

/// fingerprint
pub fn fingerprint_line(input: &str) -> IResult<&str, Fingerprint> {
    attribute("fingerprint", fingerprint)(input)
}

/// fingerprint
pub fn fingerprint(input: &str) -> IResult<&str, Fingerprint> {
    map(
        tuple((
            wsf(read_string), // type
            wsf(read_string), // hash
        )),
        |(r#type, hash)| Fingerprint { r#type, hash },
    )(input)
}

#[test]
fn test_fingerprint_line() {
    println!("{:?}",
        fingerprint_line("a=fingerprint:sha-256 19:E2:1C:3B:4B:9F:81:E6:B8:5C:F4:A5:A8:D8:73:04:BB:05:2F:70:9F:04:A9:0E:05:E9:26:33:E8:70:88:A2").unwrap()
    );
}
