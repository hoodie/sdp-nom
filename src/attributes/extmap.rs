//! https://tools.ietf.org/html/rfc8285

use nom::*;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_till1},
    combinator::{map, opt},
    sequence::{preceded, separated_pair, tuple},
};

use super::{read_direction, Direction};
#[cfg(test)]
use crate::assert_line;
use crate::parsers::*;

/// https://tools.ietf.org/html/rfc8285#section-8
#[derive(Debug, PartialEq)]
pub struct ExtmapValue<'a> {
    pub value: u32,
    pub direction: Option<Direction>,
    pub extension_name: &'a str,
    pub attributes: Vec<&'a str>,
}

/// a=extmap:<value>["/"<direction>] <URI> <extensionattributes>
fn extmap(input: &str) -> IResult<&str, ExtmapValue> {
    preceded(
        tag("extmap:"),
        map(
            tuple((
                wsf(read_number),                             // <value>
                wsf(opt(preceded(tag("/"), read_direction))), // ["/"<direction>]
                wsf(read_string),                             // <uri>
                wsf(read_as_strings),                         // <extensionattributes>
            )),
            |(value, direction, uri, attributes)| ExtmapValue {
                value,
                direction,
                extension_name: uri,
                attributes,
            },
        ),
    )(input)
}

pub(crate) fn extmap_line(input: &str) -> IResult<&str, ExtmapValue> {
    a_line(extmap)(input)
}

#[test]
fn test_extmap() {
    assert_line!(
        extmap,
        "extmap:1/sendonly URI-toffset",
        ExtmapValue {
            value: 1,
            direction: Some(Direction::SendOnly),
            extension_name: "URI-toffset",
            attributes: vec![]
        }
    );
    assert_line!(
        extmap,
        "extmap:2 urn:ietf:params:rtp-hdrext:toffset",
        ExtmapValue {
            value: 2,
            direction: None,
            extension_name: "urn:ietf:params:rtp-hdrext:toffset",
            attributes: vec![]
        }
    );
    assert_line!(
        extmap,
        "extmap:3 urn:ietf:params:rtp-hdrext:encrypt urn:ietf:params:rtp-hdrext:smpte-tc 25@600/24",
        ExtmapValue {
            value: 3,
            direction: None,
            extension_name: "urn:ietf:params:rtp-hdrext:encrypt",
            attributes: vec!["urn:ietf:params:rtp-hdrext:smpte-tc", "25@600/24"]
        }
    );
    assert_line!(
        extmap,
        "extmap:4/recvonly urn:ietf:params:rtp-hdrext:encrypt URI-gps-string",
        ExtmapValue {
            value: 4,
            direction: Some(Direction::RecvOnly),
            extension_name: "urn:ietf:params:rtp-hdrext:encrypt",
            attributes: vec!["URI-gps-string"]
        }
    );
}
#[test]
#[ignore]
fn test_extmap_line() {
    assert_line!(extmap_line, "a=extmap:1/sendonly URI-toffset");
    assert_line!(extmap_line, "a=extmap:2 urn:ietf:params:rtp-hdrext:toffset");
    assert_line!(extmap_line, "a=extmap:3 urn:ietf:params:rtp-hdrext:encrypt urn:ietf:params:rtp-hdrext:smpte-tc 25@600/24");
    assert_line!(
        extmap_line,
        "a=extmap:4/recvonly urn:ietf:params:rtp-hdrext:encrypt URI-gps-string"
    );
}
