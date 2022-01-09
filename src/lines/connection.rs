use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::{preceded, tuple},
    IResult,
};

use std::net::IpAddr;

use crate::parsers::{line, read_addr, read_ipver, read_number, wsf, IpVer};

/// Connection "c=IN IP4 10.23.42.137"
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct Connection {
    pub ip_ver: IpVer,
    pub addr: IpAddr,
    pub mask: Option<u32>,
}

/// Connection "c=IN IP4 10.23.42.137"
///
pub fn connection_line(input: &str) -> IResult<&str, Connection> {
    line(
        "c=",
        preceded(
            wsf(tag("IN")),
            map(
                tuple((
                    wsf(read_ipver), // ip_ver
                    read_addr,       // addr
                    opt(preceded(tag("/"), read_number)),
                )),
                |(ip_ver, addr, mask)| (Connection { ip_ver, addr, mask }),
            ),
        ),
    )(input)
}
