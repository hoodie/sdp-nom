//! https://tools.ietf.org/html/rfc8285

use std::fmt;

use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::{preceded, tuple},
    IResult,
};

use super::{read_direction, Direction};
#[cfg(test)]
use crate::assert_line;
use crate::parsers::*;

/// `a=extmap:<value>["/"<direction>] <URI> <extensionattributes>`
/// https://tools.ietf.org/html/rfc8285#section-8
#[derive(Debug, PartialEq)]
pub struct Extmap<'a> {
    pub value: u32,
    pub direction: Option<Direction>,
    pub uri: &'a str,
    pub attributes: Vec<&'a str>,
}

/// a=extmap:<value>["/"<direction>] <URI> <extensionattributes>
fn read_extmap(input: &str) -> IResult<&str, Extmap> {
    map(
        tuple((
            wsf(read_number),                             // <value>
            wsf(opt(preceded(tag("/"), read_direction))), // ["/"<direction>]
            wsf(read_string),                             // <uri>
            wsf(read_as_strings),                         // <extensionattributes>
        )),
        |(value, direction, uri, attributes)| Extmap {
            value,
            direction,
            uri,
            attributes,
        },
    )(input)
}

pub fn extmap_line(input: &str) -> IResult<&str, Extmap> {
    attribute("extmap", read_extmap)(input)
}
impl fmt::Display for Extmap<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(direction) = self.direction {
            write!(f, "a=extmap:{}/{} {}", self.value, direction, self.uri)?;
        } else {
            write!(f, "a=extmap:{} {}", self.value, self.uri)?;
        }
        for a in &self.attributes {
            write!(f, " {}", a)?;
        }
        Ok(())
    }
}

#[test]
fn test_extmap() {
    assert_line!(
        read_extmap,
        "1/sendonly URI-toffset",
        Extmap {
            value: 1,
            direction: Some(Direction::SendOnly),
            uri: "URI-toffset",
            attributes: vec![]
        }
    );
    assert_line!(
        read_extmap,
        "2 urn:ietf:params:rtp-hdrext:toffset",
        Extmap {
            value: 2,
            direction: None,
            uri: "urn:ietf:params:rtp-hdrext:toffset",
            attributes: vec![]
        }
    );
    assert_line!(
        read_extmap,
        "3 urn:ietf:params:rtp-hdrext:encrypt urn:ietf:params:rtp-hdrext:smpte-tc 25@600/24",
        Extmap {
            value: 3,
            direction: None,
            uri: "urn:ietf:params:rtp-hdrext:encrypt",
            attributes: vec!["urn:ietf:params:rtp-hdrext:smpte-tc", "25@600/24"]
        }
    );
    assert_line!(
        read_extmap,
        "4/recvonly urn:ietf:params:rtp-hdrext:encrypt URI-gps-string",
        Extmap {
            value: 4,
            direction: Some(Direction::RecvOnly),
            uri: "urn:ietf:params:rtp-hdrext:encrypt",
            attributes: vec!["URI-gps-string"]
        }
    );
}
#[test]
fn test_extmap_line() {
    assert_line!(extmap_line, "a=extmap:1/sendonly URI-toffset");
    assert_line!(extmap_line, "a=extmap:2 urn:ietf:params:rtp-hdrext:toffset");
    assert_line!(
        extmap_line,
        "a=extmap:3 urn:ietf:params:rtp-hdrext:encrypt urn:ietf:params:rtp-hdrext:smpte-tc 25@600/24"
    );
    assert_line!(
        extmap_line,
        "a=extmap:4/recvonly urn:ietf:params:rtp-hdrext:encrypt URI-gps-string"
    );
    assert_line!(
        extmap_line,
        "a=extmap:2/sendrecv http://example.com/082005/ext.htm#xmeta short"
    );
}
