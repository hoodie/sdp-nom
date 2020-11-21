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

#[derive(Debug, PartialEq)]
pub struct Media<'a> {
    pub r#type: &'a str,
    pub port: u32,
    pub protocol: Vec<&'a str>,
    pub payloads: Vec<u32>,
}

pub fn media_line(input: &str) -> IResult<&str, Media> {
    line(
        "m=",
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

pub fn mid_line(input: &str) -> IResult<&str, Mid> {
    attribute("mid", mid)(input)
}

pub fn mid(input: &str) -> IResult<&str, Mid> {
    map(read_string, Mid)(input)
}

/// TODO: type this more strictly, if possible without `Vec`
#[derive(Debug)]
pub struct MsidSemantic<'a>(pub Vec<&'a str>);

pub fn msid_semantic_line(input: &str) -> IResult<&str, MsidSemantic> {
    attribute("msid-semantic", msid_semantic)(input)
}

pub fn msid_semantic(input: &str) -> IResult<&str, MsidSemantic> {
    wsf(map(space_separated_strings, MsidSemantic))(input)
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

pub fn msid_line(input: &str) -> IResult<&str, Msid> {
    attribute("msid", msid)(input)
}

pub fn msid(input: &str) -> IResult<&str, Msid> {
    wsf(map(space_separated_strings, Msid))(input)
}

#[test]
fn test_mid_line() {
    assert_line!(mid_line, "a=mid:1");
    assert_line!(mid_line, "a=mid:a1");
    assert_line!(mid_line, "a=mid:0");
    assert_line!(mid_line, "a=mid:audio")
}
