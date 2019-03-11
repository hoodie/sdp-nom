use nom::*;
use nom::types::CompleteStr;

use std::net::IpAddr;

use super::parsers::{
    read_addr,
    read_ipver,
    read_big_number,
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
    pub session_id: u64,
    pub session_version: u32,
    pub net_type: &'a str,
    pub ip_ver: IpVer,
    pub addr: IpAddr
}

named!{
    pub(crate) raw_origin<CompleteStr, Origin>,
    ws!(
        do_parse!(
            user_name: read_string >>
            session_id: read_big_number >>
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

named!{
    pub(crate) raw_origin_line<CompleteStr, Origin>,
    ws!(
        do_parse!(
            tag!("o=") >>
            origin: raw_origin >>

            (origin)
        )
    )
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_candidates() {
        println!("{:?}", raw_origin_line("o=test 4962303333179871722 1 IN IP4 0.0.0.0".into()).unwrap());
        println!("{:?}", raw_origin_line("o=- 4962303333179871722 1 IN IP4 0.0.0.0".into()).unwrap());
    }
}