///

/// [6. SDP Attributes](https://tools.ietf.org/html/rfc4566#section-6)
use nom::*;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_till1},
    combinator::{map, opt},
    sequence::{preceded, separated_pair, tuple},
};

pub mod dtls_parameters;
pub mod extmap;
pub mod rtcpfb;

use std::net::IpAddr;

#[cfg(test)]
use crate::assert_line;
use crate::parsers::*;

#[derive(Debug, PartialEq)]
pub struct Attribute<'a> {
    kind: AttributeKind<'a>,
    value: &'a str,
}

#[derive(Debug, PartialEq)]
pub enum AttributeKind<'a> {
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
pub(crate) fn generic_attribute_line(input: &str) -> IResult<&str, Attribute> {
    line(
        "a=",
        map(
            separated_pair(attribute_kind, tag(":"), is_not("\n")),
            |(kind, value)| Attribute { kind, value },
        ),
    )(input)
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

#[derive(Debug, PartialEq)]
pub struct BundleGroup<'a>(pub Vec<&'a str>);

pub(crate) fn bundle_group_line(input: &str) -> IResult<&str, BundleGroup> {
    preceded(
        tag("a=group:BUNDLE"),
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

pub(crate) fn rtp_attribute_line(input: &str) -> IResult<&str, Rtp> {
    preceded(
        tag("a=rtpmap:"),
        map(
            tuple((
                wsf(read_number),           // payload
                wsf(read_non_slash_string), // codec
                preceded(tag("/"), read_number), // rate
                preceded(tag("/"), read_number), // encoding
            )),
            |(payload, codec, rate, encoding)| Rtp {
                payload,
                codec,
                rate,
                encoding,
            },
        ),
    )(input)
}

#[test]
fn test_rtp_attribute_line() {
    assert_line!("a=rtpmap:110 opus/48000/2", rtp_attribute_line);
}

// a=fmtp:108 profile-level-id=24;object=23;bitrate=64000
#[derive(Debug, PartialEq)]
pub struct Fmtp<'a> {
    payload: u32,
    config: &'a str,
}

pub(crate) fn fmtp_attribute_line(input: &str) -> IResult<&str, Fmtp> {
    preceded(
        tag("a=fmtp:"),
        map(
            tuple((
                read_number,       // payload
                wsf(is_not("\n")), // config
            )),
            |(payload, config)| (Fmtp { payload, config }),
        ),
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

// a=control:streamid=0
#[derive(Debug, PartialEq)]
pub struct Control<'a>(&'a str);

pub(crate) fn control_attribute_line(input: &str) -> IResult<&str, Control> {
    preceded(tag("a=control:"), map(read_string, Control))(input)
}

#[test]
fn test_control_attribute_line() {
    assert_line!(control_attribute_line, "a=control:streamid=0");
}

// ///////////////////////

/// Rtcp
///
/// https://tools.ietf.org/html/rfc3605
/// `a=rtcp:65179 IN IP4 10.23.34.567`
#[derive(Debug, PartialEq)]
pub struct Rtcp {
    port: u32,
    net_type: NetType,
    ip_ver: IpVer,
    addr: IpAddr,
}

pub(crate) fn rtcp_attribute_line(input: &str) -> IResult<&str, Rtcp> {
    preceded(
        tag("a=rtcp:"),
        map(
            tuple((
                wsf(read_number),   // port
                wsf(read_net_type), // net_type
                wsf(read_ipver),    // ip_ver
                wsf(read_addr),     // addr
            )),
            |(port, net_type, ip_ver, addr)| Rtcp {
                port,
                net_type,
                ip_ver,
                addr,
            },
        ),
    )(input)
}

#[test]
fn test_rtcp_attribute_line() {
    assert_line!(rtcp_attribute_line, "a=rtcp:65179 IN IP4 10.23.34.255");
    assert_line!(rtcp_attribute_line, "a=rtcp:65179 IN IP4 ::1");
}

// ///////////////////////

// ///////////////////////

// a=extmap:2 urn:ietf:params:rtp-hdrext:toffset
#[derive(Debug, PartialEq)]
pub struct Ext<'a> {
    value: u32,
    direction: Option<Direction>,
    uri: Option<&'a str>,
    extended: Option<&'a str>,
}

pub fn ext_attribute_line(input: &str) -> IResult<&str, Ext> {
    preceded(
        tag("a=extmap:"),
        map(
            tuple((
                read_number, // value: >>
                //direction:
                opt(preceded(tag("/"), read_direction)),
                wsf(opt(read_string)), //uri
                wsf(opt(read_string)), // extended
            )),
            |(value, direction, uri, extended)| Ext {
                value,
                direction,
                uri,
                extended,
            },
        ),
    )(input)
}

#[test]
fn test_ext_line() {
    assert_line!(
        ext_attribute_line,
        "a=extmap:2 urn:ietf:params:rtp-hdrext:toffset"
    );
    assert_line!(
        ext_attribute_line,
        "a=extmap:1 http://example.com/082005/ext.htm#ttime"
    );
    assert_line!(
        ext_attribute_line,
        "a=extmap:1 http://example.com/082005/ext.htm#ttime",
        Ext {
            value: 1,
            direction: None,
            uri: Some("http://example.com/082005/ext.htm#ttime"),
            extended: None,
        }
    );
    assert_line!(
        ext_attribute_line,
        "a=extmap:2/sendrecv http://example.com/082005/ext.htm#xmeta short",
        Ext {
            value: 2,
            direction: Some(Direction::SendRecv),
            uri: Some("http://example.com/082005/ext.htm#xmeta"),
            extended: Some("short")
        }
    )
}

// ///////////////////////

#[derive(Debug, PartialEq)]
pub enum NetType {
    IN,
}

pub(crate) fn read_net_type(input: &str) -> IResult<&str, NetType> {
    map(tag("IN"), |_| NetType::IN)(input)
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

pub(crate) fn read_direction(input: &str) -> IResult<&str, Direction> {
    alt((
        map(tag("sendrecv"), |_| Direction::SendRecv),
        map(tag("sendonly"), |_| Direction::SendOnly),
        map(tag("recvonly"), |_| Direction::RecvOnly),
        map(tag("inactive"), |_| Direction::Inactive),
    ))(input)
}

pub(crate) fn direction_line(input: &str) -> IResult<&str, Direction> {
    preceded(tag("a="), wsf(read_direction))(input)
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

pub(crate) fn read_rtp_option(input: &str) -> IResult<&str, RtcpOption> {
    a_line(alt((
        map(tag("rtcp-rsize"), |_| RtcpOption::RtcpRsize),
        map(tag("rtcp-mux-only"), |_| RtcpOption::RtcpMuxOnly),
        map(tag("rtcp-mux"), |_| RtcpOption::RtcpMux),
    )))(input)
}
#[test]
fn test_read_rtp_option() {
    assert_line!(read_rtp_option, "a=rtcp-mux", RtcpOption::RtcpMux);
    assert_line!(read_rtp_option, "a=rtcp-mux-only", RtcpOption::RtcpMuxOnly);
    assert_line!(read_rtp_option, "a=rtcp-rsize", RtcpOption::RtcpRsize);
}
