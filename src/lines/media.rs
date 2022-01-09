#![allow(dead_code)]

use std::borrow::Cow;

use derive_into_owned::IntoOwned;
use nom::{combinator::map, sequence::tuple, IResult};

use crate::parsers::*;

#[derive(Clone, Debug, IntoOwned, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct Media<'a> {
    pub r#type: Cow<'a, str>,
    pub port: u32,
    pub protocol: Vec<Cow<'a, str>>,
    pub payloads: Vec<Cow<'a, str>>,
}

pub fn media_line(input: &str) -> IResult<&str, Media> {
    line(
        "m=",
        wsf(map(
            tuple((
                wsf(cowify(read_string)),         // type
                wsf(read_number),                 // port
                wsf(slash_separated_cow_strings), // protocol
                wsf(read_as_cow_strings),         //payloads
            )),
            |(r#type, port, protocol, payloads)| Media {
                r#type,
                port,
                protocol,
                payloads,
            },
        )),
    )(input)
}
