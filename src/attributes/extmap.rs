//!<https://tools.ietf.org/html/rfc8285>

use std::borrow::Cow;

use derive_into_owned::IntoOwned;
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
///<https://tools.ietf.org/html/rfc8285#section-8>
#[derive(Clone, IntoOwned, PartialEq, Eq)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct Extmap<'a> {
    pub value: u32,
    pub direction: Option<Direction>,
    pub uri: Cow<'a, str>,
    pub attributes: Vec<Cow<'a, str>>,
}

/// a=extmap:<value>["/"<direction>] <URI> <extensionattributes>
fn read_extmap(input: &str) -> IResult<&str, Extmap> {
    map(
        tuple((
            wsf(read_number),                             // <value>
            wsf(opt(preceded(tag("/"), read_direction))), // ["/"<direction>]
            wsf(cowify(read_string)),                     // <uri>
            wsf(read_as_cow_strings),                     // <extensionattributes>
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

#[test]
fn test_extmap() {
    assert_line!(
        read_extmap,
        "1/sendonly URI-toffset",
        Extmap {
            value: 1,
            direction: Some(Direction::SendOnly),
            uri: "URI-toffset".into(),
            attributes: vec![]
        }
    );
    assert_line!(
        read_extmap,
        "2 urn:ietf:params:rtp-hdrext:toffset",
        Extmap {
            value: 2,
            direction: None,
            uri: "urn:ietf:params:rtp-hdrext:toffset".into(),
            attributes: vec![]
        }
    );
    assert_line!(
        read_extmap,
        "3 urn:ietf:params:rtp-hdrext:encrypt urn:ietf:params:rtp-hdrext:smpte-tc 25@600/24",
        Extmap {
            value: 3,
            direction: None,
            uri: "urn:ietf:params:rtp-hdrext:encrypt".into(),
            attributes: vec![
                "urn:ietf:params:rtp-hdrext:smpte-tc".into(),
                "25@600/24".into()
            ]
        }
    );
    assert_line!(
        read_extmap,
        "4/recvonly urn:ietf:params:rtp-hdrext:encrypt URI-gps-string",
        Extmap {
            value: 4,
            direction: Some(Direction::RecvOnly),
            uri: "urn:ietf:params:rtp-hdrext:encrypt".into(),
            attributes: vec!["URI-gps-string".into()]
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
