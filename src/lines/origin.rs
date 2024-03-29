use derive_into_owned::IntoOwned;
use nom::{combinator::map, sequence::tuple, IResult};

use std::{borrow::Cow, net::IpAddr};

use crate::parsers::{
    cowify, line, read_addr, read_big_number, read_ipver, read_number, read_string, wsf, IpVer,
};
#[cfg(test)]
use crate::{assert_line, assert_line_print};

/// Origin
///
/// o=- 20518 0 IN IP4 203.0.113.1
#[derive(Clone, IntoOwned, PartialEq, Eq)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct Origin<'a> {
    pub user_name: Cow<'a, str>,
    pub session_id: u64,
    pub session_version: u32,
    pub net_type: Cow<'a, str>,
    pub ip_ver: IpVer,
    pub addr: IpAddr,
}

pub fn origin(input: &str) -> IResult<&str, Origin> {
    map(
        tuple((
            wsf(cowify(read_string)), // user_name
            wsf(read_big_number),     // session_id
            wsf(read_number),         // session_version
            wsf(cowify(read_string)), // net_type
            wsf(read_ipver),          // ip_ver
            wsf(read_addr),           // addr
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

pub fn origin_line(input: &str) -> IResult<&str, Origin> {
    line("o=", origin)(input)
}

#[test]
fn parses_candidates() {
    assert_line!(
        origin_line,
        "o=test 4962303333179871722 1 IN IP4 0.0.0.0",
        Origin {
            user_name: "test".into(),
            session_id: 4962303333179871722,
            session_version: 1,
            net_type: "IN".into(),
            ip_ver: IpVer::Ip4,
            addr: "0.0.0.0".parse().unwrap(),
        },
        print
    );
    assert_line_print!(origin_line, "o=- 4962303333179871722 1 IN IP4 0.0.0.0");
}
