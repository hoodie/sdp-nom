use nom::*;
use nom::types::CompleteStr;

use std::net::IpAddr;

use super::parsers::{
    read_addr,
    read_ipver,
    read_number,
    read_string,
    IpVer,
};


/// Origin
/// 
/// o=- 20518 0 IN IP4 203.0.113.1
#[derive(Debug)]
pub struct Origin<'a> {
    pub user_name: &'a str,
    pub session_id: u32,
    pub session_version: u32,
    pub net_type: &'a str,
    pub ip_ver: IpVer,
    pub addr: IpAddr
}

named!{
    pub(crate) raw_parse_origin_line<CompleteStr, Origin>,
    ws!(
        do_parse!(
            tag!("o=") >>
            user_name: read_string >>
            session_id: read_number >>
            session_version: read_number >>
            net_type: read_string >>
            ip_ver: read_ipver >>
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
