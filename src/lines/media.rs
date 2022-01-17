#![allow(dead_code)]

use std::borrow::Cow;

use derive_into_owned::IntoOwned;
use nom::{combinator::map, sequence::tuple, IResult};

use crate::parsers::*;
#[cfg(test)]
use crate::{assert_line, assert_line_print};

/// [RFC4566#5.14](https://datatracker.ietf.org/doc/html/rfc4566#section-5.14)
#[derive(Clone, IntoOwned, PartialEq)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct Media<'a> {
    pub r#type: Cow<'a, str>,
    pub port: u32,
    pub protocol: Vec<Cow<'a, str>>,
    pub payloads: Vec<Cow<'a, str>>,
}

pub fn media_line(input: &str) -> IResult<&str, Media> {
    line(
        "m=",
        wsf(map(
            tuple((
                wsf(cowify(read_string)),         // type
                wsf(read_number),                 // port
                wsf(slash_separated_cow_strings), // protocol
                wsf(read_as_cow_strings),         // payloads
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
            r#type: "video".into(),
            port: 51744,
            protocol: create_test_vec(&["RTP", "AVP"]),
            payloads: create_test_vec(&["126", "97", "98", "34", "31"]),
        },
        print
    );
    assert_line!(
        media_line,
        "m=audio 9 UDP/TLS/RTP/SAVPF 111 103 104 9 0 8 106 105 13 110 112 113 126",
        Media {
            r#type: "audio".into(),
            port: 9,
            protocol: create_test_vec(&["UDP", "TLS", "RTP", "SAVPF"]),
            payloads: create_test_vec(&[
                "111", "103", "104", "9", "0", "8", "106", "105", "13", "110", "112", "113", "126"
            ]),
        },
        print
    );
    assert_line_print!(
        media_line,
        "m=video 9 UDP/TLS/RTP/SAVPF 96 98 100 102 127 125 97 99 101 124"
    );
    assert_line_print!(media_line, "m=application 3238 UDP/BFCP *")
}
