#![allow(dead_code)]

use nom::{combinator::map, sequence::tuple, IResult};

use crate::parsers::*;
#[cfg(test)]
use crate::{assert_line, assert_line_print};

#[derive(Debug, PartialEq)]
pub struct Media<'a> {
    pub r#type: &'a str,
    pub port: u32,
    pub protocol: Vec<&'a str>,
    pub payloads: Vec<&'a str>,
}

pub fn media_line(input: &str) -> IResult<&str, Media> {
    line(
        "m=",
        wsf(map(
            tuple((
                wsf(read_string),             // type
                wsf(read_number),             // port
                wsf(slash_separated_strings), // protocol
                wsf(read_as_strings),         //payloads
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
            payloads: vec!["126", "97", "98", "34", "31"],
        },
        print
    );
    assert_line!(
        media_line,
        "m=audio 9 UDP/TLS/RTP/SAVPF 111 103 104 9 0 8 106 105 13 110 112 113 126",
        Media {
            r#type: "audio",
            port: 9,
            protocol: vec!["UDP", "TLS", "RTP", "SAVPF"],
            payloads: vec![
                "111", "103", "104", "9", "0", "8", "106", "105", "13", "110", "112", "113", "126"
            ],
        },
        print
    );
    assert_line_print!(
        media_line,
        "m=video 9 UDP/TLS/RTP/SAVPF 96 98 100 102 127 125 97 99 101 124"
    );
    assert_line_print!(media_line, "m=application 3238 UDP/BFCP *")
}
