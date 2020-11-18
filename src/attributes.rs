use nom::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::{preceded, tuple},
};

use std::net::IpAddr;

#[cfg(test)]
use crate::assert_line;
use crate::parsers::*;

pub enum Attribute {
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
    Invalid,
}

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
                tag("/"),
                read_number, // rate
                tag("/"),
                read_number, // encoding
            )),
            |(payload, codec, _, rate, _, encoding)| Rtp {
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
                read_number,      // payload
                wsf(read_string), // config
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
    )
}

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

/// RtcpFeedback
///
/// https://datatracker.ietf.org/doc/draft-ietf-mmusic-sdp-mux-attributes/16/?include_text=1
/// eg `a=rtcp-fb:98 trr-int 100`
#[derive(Debug, PartialEq)]
pub struct RtcpFb {
    payload: u32,
    r#type: RtcpFbType,
    subtype: Option<RtcpFbSubType>,
    value: Option<u32>,
}

#[derive(Debug, PartialEq)]
pub enum RtcpFbType {
    Ack,
    Nack,
    TrrInt,
}

#[derive(Debug, PartialEq)]
pub enum RtcpFbSubType {
    Rpsi,
    App,
    Pli,
    Sli,
}

pub(crate) fn rtcpfb_attribute_line(input: &str) -> IResult<&str, RtcpFb> {
    preceded(
        tag("a=rtcp-fb:"),
        map(
            tuple((
                read_number, // payload
                //r#type:
                wsf(alt((
                    map(tag("ack"), |_| RtcpFbType::Ack),
                    map(tag("nack"), |_| RtcpFbType::Nack), // | tag!("trr-int") => { |_| RtcpFbType::TrrInt }
                    map(tag("trr-int"), |_| RtcpFbType::TrrInt),
                ))),
                // subtype:
                opt(wsf(alt((
                    map(tag("rpsi"), |_| RtcpFbSubType::Rpsi),
                    map(tag("app"), |_| RtcpFbSubType::App),
                    map(tag("pli"), |_| RtcpFbSubType::Pli),
                    map(tag("sli"), |_| RtcpFbSubType::Sli),
                )))),
                opt(read_number), // value
            )),
            |(payload, r#type, subtype, value)| RtcpFb {
                payload,
                r#type,
                subtype,
                value,
            },
        ),
    )(input)
}

#[test]
fn test_rtcpfb_line() {
    assert_line!(rtcpfb_attribute_line, "a=rtcp-fb:98 trr-int 100");
    assert_line!(rtcpfb_attribute_line, "a=rtcp-fb:98 ack sli");
    assert_line!(rtcpfb_attribute_line, "a=rtcp-fb:98 ack sli 5432");
    assert_line!(rtcpfb_attribute_line, "a=rtcp-fb:98 nack rpsi");
}

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
                opt(map(tuple((tag("/"), read_direction)), |(_, d)| d)),
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

#[derive(Debug, PartialEq)]
pub enum NetType {
    IN,
}

pub(crate) fn read_net_type(input: &str) -> IResult<&str, NetType> {
    map(tag("IN"), |_| NetType::IN)(input)
}

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
