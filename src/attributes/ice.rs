use std::borrow::Cow;

use derive_into_owned::IntoOwned;
use nom::{branch::alt, bytes::complete::tag, combinator::map, IResult};

use crate::parsers::*;

#[derive(Clone, Debug, IntoOwned, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[non_exhaustive]
pub enum IceParameter<'a> {
    Ufrag(Cow<'a, str>),
    Pwd(Cow<'a, str>),
    Options(Cow<'a, str>),
    Mismatch,
    Lite,
}

#[derive(Debug, Default, IntoOwned)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct Ice<'a> {
    pub ufrag: Option<Cow<'a, str>>,
    pub pwd: Option<Cow<'a, str>>,
    pub options: Option<Cow<'a, str>>,
}

pub fn ice_parameter_line(input: &str) -> IResult<&str, IceParameter> {
    alt((
        attribute("ice-ufrag", map(cowify(read_string), IceParameter::Ufrag)),
        attribute("ice-pwd", map(cowify(read_string), IceParameter::Pwd)),
        attribute(
            "ice-options",
            map(cowify(read_string), IceParameter::Options),
        ),
        a_line(map(tag("ice-mismatch"), |_| IceParameter::Mismatch)),
        a_line(map(tag("ice-lite"), |_| IceParameter::Lite)),
    ))(input)
}
