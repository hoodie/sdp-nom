use nom::*;

use nom::types::CompleteStr;

pub fn is_not_space(c: char) -> bool { c != ' ' }

pub fn is_alphabetic(chr: char) -> bool {
  (chr as u8 >= 0x41 && chr as u8 <= 0x5A) || (chr as u8 >= 0x61 && chr as u8 <= 0x7A)
}

pub fn is_alphanumeric(chr: char) -> bool {
  is_alphabetic(chr)
}

named!(alphanumeric<CompleteStr, CompleteStr>, take_while1!(is_alphanumeric));
