use nom::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::{preceded, tuple},
};

#[cfg(test)]
use crate::assert_line;
use crate::parsers::*;

#[derive(Debug, PartialEq)]
pub enum PTime {
    MaxPTime(u32),
    MinPTime(u32),
    PTime(u32),
}

pub(crate) fn read_p_time(input: &str) -> IResult<&str, PTime> {
    a_line(alt((
        preceded(tag("ptime:"), map(read_number, PTime::PTime)),
        preceded(tag("minptime:"), map(read_number, PTime::MinPTime)),
        preceded(tag("maxptime:"), map(read_number, PTime::MaxPTime)),
    )))(input)
}

#[test]
fn test_read_p_time() {
    assert_line!(read_p_time, "a=ptime:1", PTime::PTime(1));
    assert_line!(read_p_time, "a=minptime:20", PTime::MinPTime(20));
    assert_line!(read_p_time, "a=maxptime:120", PTime::MaxPTime(120));
}

/// RtpMap
/// `a=rtpmap:<payload type> <encoding name>/<clock rate> [/<encoding` parameters>]
/// https://tools.ietf.org/html/rfc4566#section-6
#[derive(Debug, PartialEq)]
pub struct RtpMap<'a> {
    payload_type: u32,
    encoding_name: &'a str,
    clock_rate: u32,
    encoding: Option<u32>,
}

pub(crate) fn rtpmap_line(input: &str) -> IResult<&str, RtpMap> {
    a_line(preceded(
        tag("rtpmap:"),
        map(
            tuple((
                read_number,                               // payload_typ
                preceded(tag(" "), read_non_slash_string), // encoding_name
                preceded(tag("/"), read_number),           // clock_rate
                opt(preceded(
                    tag("/"),
                    read_number, // encoding
                )),
            )),
            |(payload_type, encoding_name, clock_rate, encoding)| RtpMap {
                payload_type,
                encoding_name,
                clock_rate,
                encoding,
            },
        ),
    ))(input)
}

#[test]
fn test_rtpmap_line() {
    assert_line!(
        rtpmap_line,
        "a=rtpmap:96 VP8/90000",
        RtpMap {
            payload_type: 96,
            encoding_name: "VP8",
            clock_rate: 90000,
            encoding: None,
        }
    );
    assert_line!(
        rtpmap_line,
        "a=rtpmap:97 rtx/90000",
        RtpMap {
            payload_type: 97,
            encoding_name: "rtx",
            clock_rate: 90000,
            encoding: None,
        }
    );
    assert_line!(
        rtpmap_line,
        "a=rtpmap:111 opus/48000/2",
        RtpMap {
            payload_type: 111,
            encoding_name: "opus",
            clock_rate: 48000,
            encoding: Some(2),
        }
    );
    assert_line!(rtpmap_line, "a=rtpmap:98 VP9/90000");
    assert_line!(rtpmap_line, "a=rtpmap:99 rtx/90000");
    assert_line!(rtpmap_line, "a=rtpmap:100 H264/90000");
    assert_line!(rtpmap_line, "a=rtpmap:101 rtx/90000");
    assert_line!(rtpmap_line, "a=rtpmap:102 H264/90000");
    assert_line!(rtpmap_line, "a=rtpmap:124 rtx/90000");
    assert_line!(rtpmap_line, "a=rtpmap:127 red/90000");
    assert_line!(rtpmap_line, "a=rtpmap:123 rtx/90000");
    assert_line!(rtpmap_line, "a=rtpmap:125 ulpfec/90000");
    assert_line!(
        rtpmap_line,
        "a=rtpmap:113 telephone-event/16000",
        RtpMap {
            payload_type: 113,
            encoding_name: "telephone-event",
            clock_rate: 16000,
            encoding: None,
        }
    );
}
