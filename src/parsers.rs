#![allow(dead_code)]
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while1},
    character::{
        complete::{multispace0, space1},
        is_digit,
    },
    combinator::{complete, map, map_res, opt},
    error::ParseError,
    multi::many0,
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult, Parser,
};

use std::{borrow::Cow, net::IpAddr};

#[cfg(test)]
pub fn create_test_vec(strs: &[&str]) -> Vec<Cow<'static, str>> {
    strs.iter().map(|&s| Cow::from(s.to_owned())).collect()
}

pub fn is_not_space(c: char) -> bool {
    c != ' '
}

pub fn is_alphabetic(chr: u8) -> bool {
    (0x41..=0x5A).contains(&chr) || (0x61..=0x7A).contains(&chr)
}

pub fn is_alphanumeric(chr: char) -> bool {
    is_alphabetic(chr as u8) || is_digit(chr as u8)
}

pub fn is_numeric(chr: char) -> bool {
    is_digit(chr as u8)
}

pub fn ws<'a, O, E: ParseError<&'a str>, F: Parser<&'a str, O, E>>(
    f: F,
) -> impl Parser<&'a str, O, E> {
    delimited(multispace0, f, multispace0)
}

pub fn wsf<'a, O, E: ParseError<&'a str>, F: Parser<&'a str, O, E>>(
    f: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E> {
    delimited(multispace0, f, multispace0)
}

pub fn cowify<'a, E: ParseError<&'a str>, F: Parser<&'a str, &'a str, E>>(
    f: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, Cow<'a, str>, E> {
    map(f, Cow::from)
}

pub fn a_line<'a, O, E: ParseError<&'a str>, F: Parser<&'a str, O, E>>(
    f: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E> {
    line("a=", f)
}

pub fn attribute<'a, O, E: ParseError<&'a str>, F: Parser<&'a str, O, E>>(
    attribute_kind: &'a str,
    f: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E> {
    line(
        "a=",
        map(separated_pair(tag(attribute_kind), tag(":"), f), |(_, x)| x),
    )
}

pub fn attribute_p<'a, O, E: ParseError<&'a str>, F: Parser<&'a str, O, E>>(
    p: F,
    f: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E> {
    line("a=", map(separated_pair(p, tag(":"), f), |(_, x)| x))
}

pub fn line<'a, O, E: ParseError<&'a str>, F: Parser<&'a str, O, E>>(
    prefix: &'a str,
    f: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E> {
    complete(preceded(tag(prefix), f))
}

pub fn read_number(input: &str) -> IResult<&str, u32> {
    map_res(
        take_while1(|c: char| -> bool { c != ' ' && c != ':' && c != '/' }),
        |i: &str| i.parse::<u32>(),
    )(input)
}

pub fn read_big_number(input: &str) -> IResult<&str, u64> {
    map_res(
        take_while1(|c: char| -> bool { c != ' ' && c != ':' && c != '/' }),
        |i: &str| (i).parse::<u64>(),
    )(input)
}

pub fn read_everything(input: &str) -> IResult<&str, &str> {
    map(take_while(|_| true), str::trim)(input)
}

pub fn read_string0(input: &str) -> IResult<&str, &str> {
    take_while(is_not_space)(input)
}

pub fn read_string(input: &str) -> IResult<&str, &str> {
    take_while1(is_not_space)(input)
}

pub fn read_non_colon_string(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| -> bool { c != ' ' && c != ':' })(input)
}

pub fn read_non_slash_string(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| -> bool { c != ' ' && c != '/' })(input)
}

pub fn slash_separated_strings(input: &str) -> IResult<&str, Vec<&str>> {
    many0(terminated(read_non_slash_string, opt(tag("/"))))(input)
}

pub fn slash_separated_cow_strings(input: &str) -> IResult<&str, Vec<Cow<'_, str>>> {
    many0(terminated(cowify(read_non_slash_string), opt(tag("/"))))(input)
}

pub fn space_separated_cow_strings(input: &str) -> IResult<&str, Vec<Cow<'_, str>>> {
    many0(terminated(cowify(read_string), multispace0))(input)
}

pub fn space_separated_strings(input: &str) -> IResult<&str, Vec<&str>> {
    many0(terminated(read_string, multispace0))(input)
}

pub fn read_addr(input: &str) -> IResult<&str, IpAddr> {
    map_res(take_while1(|c| c != ' ' && c != '/'), str::parse)(input)
}

#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[non_exhaustive]
pub enum IpVer {
    Ip4,
    Ip6,
}

pub fn read_ipver(input: &str) -> IResult<&str, IpVer> {
    alt((
        map(tag("IP4"), |_| IpVer::Ip4),
        map(tag("IP6"), |_| IpVer::Ip6),
    ))(input)
}

pub fn read_as_strings(input: &str) -> IResult<&str, Vec<&str>> {
    many0(terminated(read_string, opt(space1)))(input)
}

pub fn read_as_cow_strings(input: &str) -> IResult<&str, Vec<Cow<'_, str>>> {
    many0(terminated(cowify(read_string), opt(space1)))(input)
}

pub fn read_as_numbers(input: &str) -> IResult<&str, Vec<u32>> {
    many0(terminated(read_number, opt(space1)))(input)
}
