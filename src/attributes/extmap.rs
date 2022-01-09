//!<https://tools.ietf.org/html/rfc8285>

use std::borrow::Cow;

use derive_into_owned::IntoOwned;
use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::{preceded, tuple},
    IResult,
};

use super::{read_direction, Direction};
use crate::parsers::*;

/// `a=extmap:<value>["/"<direction>] <URI> <extensionattributes>`
///<https://tools.ietf.org/html/rfc8285#section-8>
#[derive(Clone, Debug, IntoOwned, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct Extmap<'a> {
    pub value: u32,
    pub direction: Option<Direction>,
    pub uri: Cow<'a, str>,
    pub attributes: Vec<Cow<'a, str>>,
}

/// a=extmap:<value>["/"<direction>] <URI> <extensionattributes>
fn read_extmap(input: &str) -> IResult<&str, Extmap> {
    map(
        tuple((
            wsf(read_number),                             // <value>
            wsf(opt(preceded(tag("/"), read_direction))), // ["/"<direction>]
            wsf(cowify(read_string)),                     // <uri>
            wsf(read_as_cow_strings),                     // <extensionattributes>
        )),
        |(value, direction, uri, attributes)| Extmap {
            value,
            direction,
            uri,
            attributes,
        },
    )(input)
}

pub fn extmap_line(input: &str) -> IResult<&str, Extmap> {
    attribute("extmap", read_extmap)(input)
}
