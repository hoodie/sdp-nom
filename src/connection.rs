use nom::*;
use nom::types::CompleteStr;

use std::net::IpAddr;

use super::parsers::{
    read_addr,
    read_ipver,
    IpVer,
};

#[derive(Debug)]
pub struct Connection {
    pub ip_ver: IpVer,
    pub addr: IpAddr,
}

/// Connection "c=IN IP4 10.23.42.137"
/// 
named!{
    pub(crate) raw_connection_line<CompleteStr, Connection>,
    ws!(
        do_parse!(
            tag!("c=") >>
            tag!("IN") >>
            ip_ver: read_ipver >>
            addr: read_addr >> 

            (Connection {
                ip_ver,
                addr,
            })
        )
    )
}
