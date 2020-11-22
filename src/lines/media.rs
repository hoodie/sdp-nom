#![allow(dead_code)]
use std::fmt;

use nom::{combinator::map, sequence::tuple, IResult};

use crate::parsers::*;
#[cfg(test)]
use crate::{assert_line, assert_line_print};

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

impl<'a> fmt::Display for Media<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "m={ty} {port} {protos}",
            ty = self.r#type,
            port = self.port,
            protos = self.protocol.join("/"),
        )?;
        for payload in &self.payloads {
            write!(f, " {}", payload)?;
        }
        Ok(())
    }
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
            payloads: vec![111, 103, 104, 9, 0, 8, 106, 105, 13, 110, 112, 113, 126],
        },
        print
    );
    assert_line_print!(
        media_line,
        "m=video 9 UDP/TLS/RTP/SAVPF 96 98 100 102 127 125 97 99 101 124"
    );
}

pub mod mid {
    use super::*;

    #[derive(Debug)]
    pub struct Mid<'a>(pub &'a str);

    pub fn mid_line(input: &str) -> IResult<&str, Mid> {
        attribute("mid", mid)(input)
    }

    pub fn mid(input: &str) -> IResult<&str, Mid> {
        map(read_string, Mid)(input)
    }

    impl<'a> fmt::Display for Mid<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "a=mid:{}", self.0)
        }
    }

    #[test]
    fn test_mid_line() {
        assert_line_print!(mid_line, "a=mid:1");
        assert_line_print!(mid_line, "a=mid:a1");
        assert_line_print!(mid_line, "a=mid:0");
        assert_line_print!(mid_line, "a=mid:audio")
    }
}

pub mod msid {
    use super::*;

    /// TODO: type this more strictly, if possible without `Vec`
    #[derive(Debug, PartialEq)]
    pub struct MsidSemantic<'a>(pub Vec<&'a str>);

    pub fn msid_semantic_line(input: &str) -> IResult<&str, MsidSemantic> {
        attribute("msid-semantic", msid_semantic)(input)
    }

    pub fn msid_semantic(input: &str) -> IResult<&str, MsidSemantic> {
        wsf(map(space_separated_strings, MsidSemantic))(input)
    }

    impl<'a> fmt::Display for MsidSemantic<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "a=msid-semantic:")?;
            for (i, x) in self.0.iter().enumerate() {
                if i > 0 {
                    write!(f, " ")?;
                }
                write!(f, "{}", x)?;
            }
            Ok(())
        }
    }

    #[test]
    fn test_msid_semantic_line() {
        assert_line!(
            msid_semantic_line,
            "a=msid-semantic: WMS lgsCFqt9kN2fVKw5wg3NKqGdATQoltEwOdMS",
            MsidSemantic(vec!["WMS", "lgsCFqt9kN2fVKw5wg3NKqGdATQoltEwOdMS"])
        );
        assert_line_print!(
            msid_semantic_line,
            "a=msid-semantic:WMS lgsCFqt9kN2fVKw5wg3NKqGdATQoltEwOdMS"
        );
    }

    #[derive(Debug, PartialEq)]
    pub struct Msid<'a>(pub Vec<&'a str>);

    pub fn msid_line(input: &str) -> IResult<&str, Msid> {
        attribute("msid", msid)(input)
    }

    pub fn msid(input: &str) -> IResult<&str, Msid> {
        wsf(map(space_separated_strings, Msid))(input)
    }

    impl<'a> fmt::Display for Msid<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "a=msid:")?;
            for (i, x) in self.0.iter().enumerate() {
                if i > 0 {
                    write!(f, " ")?;
                }
                write!(f, "{}", x)?;
            }
            Ok(())
        }
    }

    #[test]
    fn test_msid_line() {
        assert_line!(
            msid_line,
            "a=msid:47017fee-b6c1-4162-929c-a25110252400 f83006c5-a0ff-4e0a-9ed9-d3e6747be7d9",
            Msid(vec![
                "47017fee-b6c1-4162-929c-a25110252400",
                "f83006c5-a0ff-4e0a-9ed9-d3e6747be7d9"
            ]),
            print
        );
        assert_line_print!(
            msid_line,
            "a=msid:61317484-2ed4-49d7-9eb7-1414322a7aae f30bdb4a-5db8-49b5-bcdc-e0c9a23172e0"
        );
    }
}
