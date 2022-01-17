use derive_into_owned::IntoOwned;
use enum_as_inner::EnumAsInner;
use nom::{branch::alt, combinator::map, IResult};
use std::borrow::Cow;

use crate::{
    attributes::{attribute_line, AttributeLine},
    lines::{self, session_line, SessionLine},
    parsers::cowify,
};

/// Sdp Line
#[derive(Clone, IntoOwned, EnumAsInner, PartialEq)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub enum SdpLine<'a> {
    Session(SessionLine<'a>),
    Attribute(AttributeLine<'a>),
    Comment(Cow<'a, str>),
}

pub fn sdp_line(input: &str) -> IResult<&str, SdpLine> {
    alt((
        map(session_line, SdpLine::Session),
        map(attribute_line, SdpLine::Attribute),
        map(cowify(lines::comment::comment_line), SdpLine::Comment),
    ))(input)
}
