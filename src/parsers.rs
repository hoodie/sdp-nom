#![allow(dead_code)]
use nom::*;
use nom::types::CompleteStr;

use std::net::IpAddr;

pub(crate) fn is_not_space(c: char) -> bool { c != ' ' }

pub(crate) fn is_alphabetic(chr: u8) -> bool {
    (chr >= 0x41 && chr <= 0x5A) || (chr >= 0x61 && chr <= 0x7A)
}

pub(crate) fn is_alphanumeric(chr: char) -> bool {
    is_alphabetic(chr as u8) || is_digit(chr as u8)
}

pub(crate) fn is_numeric(chr: char) -> bool {
    is_digit(chr as u8)
}

named!(alphanumeric<CompleteStr, CompleteStr>, take_while1!(is_alphanumeric));

named!{
    pub(crate) read_number<CompleteStr, u32>,
    do_parse!(n: map_res!(take_while1!(is_not_space), |i: CompleteStr| u32::from_str_radix(&i, 10)) >> (n))
}

named!{
    pub(crate) read_big_number<CompleteStr, u64>,
    do_parse!(n: map_res!(take_while1!(is_not_space), |i: CompleteStr| u64::from_str_radix(&i, 10)) >> (n))
}

named!{
    pub(crate) read_string<CompleteStr, &str>,
    do_parse!(s: map!( take_while1!(is_not_space), |cs| &**cs) >> (s))
}

named!{
    pub(crate) read_non_colon_string<CompleteStr, &str>,
    do_parse!(s: map!(take_while1!(|c: char| -> bool { c != ' ' && c != ':' }), |cs| &**cs) >> (s))
}

named!{
    pub(crate) read_non_slash_string<CompleteStr, &str>,
    do_parse!(s: map!(take_while1!(|c: char| -> bool { c != ' ' && c != '/' }), |cs| &**cs) >> (s))
}

named!{
    pub(crate) slash_separated_strings<CompleteStr, Vec<&str>>,
    many0!(terminated!(read_non_slash_string, opt!(tag!("/"))))
}


named!{
    pub(crate) read_addr<CompleteStr, IpAddr>,
    do_parse!(addr: map_res!(take_while1!(is_not_space), |i: CompleteStr| i.parse() ) >> (addr))
}


#[derive(Debug)]
pub enum IpVer {
    Ip4, Ip6
}

named!{
    pub(crate) read_ipver<CompleteStr, IpVer>,
    do_parse!(
        ip_ver: alt!(
            tag!("IP4") => {|_| IpVer::Ip4 } |
            tag!("IP6") => {|_| IpVer::Ip6 }
        ) >>
        (ip_ver)
    )
}

named!{
    pub(crate) read_as_strings<CompleteStr, Vec<&str>>,
    many0!(terminated!(read_string, opt!(space1)))
}

named!{
    pub(crate) read_as_numbers<CompleteStr, Vec<u32>>,
    many0!(terminated!(read_number, opt!(space1)))
}

