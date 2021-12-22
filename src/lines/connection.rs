use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::{preceded, tuple},
    IResult,
};

use std::net::IpAddr;

#[cfg(test)]
use std::net::Ipv4Addr;

#[cfg(test)]
use crate::assert_line;
use crate::parsers::{line, read_addr, read_ipver, read_number, wsf, IpVer};

/// Connection "c=IN IP4 10.23.42.137"
#[derive(Clone, Debug, PartialEq)]
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

#[test]
fn test_connection_line() {
    assert_line!(
        connection_line,
        "c=IN IP6 fe80::5a55:caff:fe1a:e187",
        Connection {
            ip_ver: IpVer::Ip6,
            addr: "fe80::5a55:caff:fe1a:e187".parse().unwrap(),
            mask: None,
        },
        print
    );
    assert_line!(
        connection_line,
        "c=IN IP4 10.23.42.137/32",
        Connection {
            ip_ver: IpVer::Ip4,
            addr: IpAddr::V4(Ipv4Addr::new(10, 23, 42, 137)),
            mask: Some(32),
        },
        print
    );
    assert_line!(
        connection_line,
        "c=IN IP4 10.23.42.137",
        Connection {
            ip_ver: IpVer::Ip4,
            addr: IpAddr::V4(Ipv4Addr::new(10, 23, 42, 137)),
            mask: None,
        },
        print
    );
}
