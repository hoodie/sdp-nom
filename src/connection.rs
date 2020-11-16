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

use std::net::IpAddr;

use super::parsers::{read_addr, read_ipver, wsf, IpVer};

#[derive(Debug)]
pub struct Connection {
    pub ip_ver: IpVer,
    pub addr: IpAddr,
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
                    read_ipver, // ip_ver
                    read_addr,  // addr
                )),
                |(ip_ver, addr)| (Connection { ip_ver, addr }),
            ),
        ),
    )(input)
}
