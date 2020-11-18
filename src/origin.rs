use nom::*;
use nom::{
    bytes::complete::tag,
    combinator::map,
    sequence::{preceded, tuple},
};

use std::net::IpAddr;

#[cfg(test)]
use crate::assert_line;
use crate::parsers::{
    read_addr, read_big_number, read_ipver, read_number, read_string, wsf, IpVer,
};

/// Origin
///
/// o=- 20518 0 IN IP4 203.0.113.1
#[derive(Debug)]
pub struct Origin<'a> {
    pub user_name: &'a str,
    pub session_id: u64,
    pub session_version: u32,
    pub net_type: &'a str,
    pub ip_ver: IpVer,
    pub addr: IpAddr,
}

pub(crate) fn origin(input: &str) -> IResult<&str, Origin> {
    map(
        tuple((
            wsf(read_string),     // user_name
            wsf(read_big_number), // session_id
            wsf(read_number),     // session_version
            wsf(read_string),     // net_type
            wsf(read_ipver),      // ip_ver
            wsf(read_addr),       // addr
        )),
        |(user_name, session_id, session_version, net_type, ip_ver, addr)| Origin {
            user_name,
            session_id,
            session_version,
            net_type,
            ip_ver,
            addr,
        },
    )(input)
}

pub(crate) fn origin_line(input: &str) -> IResult<&str, Origin> {
    // ws!(do_parse!(tag!("o=") >> origin: origin >> (origin)))
    preceded(tag("o="), wsf(origin))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_candidates() {
        assert_line!(origin_line, "o=test 4962303333179871722 1 IN IP4 0.0.0.0");
        assert_line!(origin_line, "o=- 4962303333179871722 1 IN IP4 0.0.0.0");
    }
}
