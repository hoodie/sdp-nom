#![allow(dead_code)]
use nom::*;
use nom::types::CompleteStr;

use std::net::IpAddr;

pub fn is_not_space(c: char) -> bool { c != ' ' }

pub fn is_alphabetic(chr: u8) -> bool {
  (chr >= 0x41 && chr <= 0x5A) || (chr >= 0x61 && chr <= 0x7A)
}

pub fn is_alphanumeric(chr: char) -> bool {
  is_alphabetic(chr as u8) || is_digit(chr as u8)
}

pub fn is_numeric(chr: char) -> bool {
  is_digit(chr as u8)
}

named!(alphanumeric<CompleteStr, CompleteStr>, take_while1!(is_alphanumeric));

named!{
    pub read_number<CompleteStr, u32>,
    do_parse!(
      n: map_res!(take_while1!(is_not_space), |i: CompleteStr| u32::from_str_radix(&i, 10)) >>
      (n)
    )
}

named!{
    pub read_string<CompleteStr, CompleteStr>,
    do_parse!(
      s: take_while1!(is_not_space) >>
      (s)
    )
}

named!{
    pub read_addr<CompleteStr, IpAddr>,
    do_parse!(
      addr: map_res!(take_while1!(is_not_space), |i: CompleteStr| i.parse() ) >>
      (addr)
    )
}
