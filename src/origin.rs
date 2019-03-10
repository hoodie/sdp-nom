use nom::*;
use nom::types::CompleteStr;

use std::net::IpAddr;

use super::parsers::{
    read_addr,
    read_number,
    read_string,
};


#[derive(Debug)]
pub enum IpVer {
    Ip4, Ip6
}

/// Origin
/// 
/// o=- 20518 0 IN IP4 203.0.113.1
#[derive(Debug)]
pub struct Origin<'a> {
    user_name: &'a str,
    session_id: u32,
    session_version: u32,
    net_type: &'a str,
    ip_ver: IpVer,
    addr: IpAddr
}

named!{
    raw_parse_origin_line<CompleteStr, Origin>,
    ws!(
        do_parse!(
            tag!("o=") >>
            user_name: read_string >>
            session_id: read_number >>
            session_version: read_number >>
            net_type: read_string >>
            ip_ver: alt!(
                tag!("IP4") => {|_| IpVer::Ip4 } |
                tag!("IP6") => {|_| IpVer::Ip6 }
            ) >>
            addr: read_addr >>

            (Origin {
                user_name: &user_name,
                session_id,
                session_version,
                net_type: &net_type,
                ip_ver,
                addr
            })
        )
    )
}
