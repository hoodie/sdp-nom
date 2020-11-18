use nom::*;
use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, take_while, take_while1},
    character::{
        complete::{anychar, char, multispace0, none_of, space1},
        is_digit,
    },
    combinator::{map, map_res, opt},
    error::ParseError,
    multi::many0,
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    Parser,
};
use pretty_assertions::assert_eq;

use std::net::{IpAddr, Ipv4Addr};

#[cfg(test)]
use crate::assert_line;
use crate::parsers::{read_addr, read_ipver, read_number, wsf, IpVer};

#[derive(Debug, PartialEq)]
pub struct Connection {
    pub ip_ver: IpVer,
    pub addr: IpAddr,
    pub mask: Option<u32>,
}

/// Connection "c=IN IP4 10.23.42.137"
///
pub(crate) fn raw_connection_line(input: &str) -> IResult<&str, Connection> {
    preceded(
        tag("c="),
        preceded(
            wsf(tag("IN")),
            map(
                tuple((
                    wsf(read_ipver), // ip_ver
                    read_addr,       // addr
                    opt(tag("/")),
                    opt(read_number),
                )),
                |(ip_ver, addr, _, mask)| (Connection { ip_ver, addr, mask }),
            ),
        ),
    )(input)
}

#[test]
#[rustfmt::skip]
fn test_raw_connection_line() {
    assert_line!(
        raw_connection_line,
        "c=IN IP6 fe80::5a55:caff:fe1a:e187",
        Connection {
            ip_ver: IpVer::Ip6,
            addr: "fe80::5a55:caff:fe1a:e187".parse().unwrap(),
            mask: None,
        }
    );
    assert_line!(
        raw_connection_line,
        "c=IN IP4 10.23.42.137/32",
        Connection {
            ip_ver: IpVer::Ip4,
            addr: IpAddr::V4(Ipv4Addr::new(10, 23, 42, 137)),
            mask: Some(32),
        }
    );
    assert_line!(
        raw_connection_line,
        "c=IN IP4 10.23.42.137",
        Connection {
            ip_ver: IpVer::Ip4,
            addr: IpAddr::V4(Ipv4Addr::new(10, 23, 42, 137)),
            mask: None,
        }
    );
}
