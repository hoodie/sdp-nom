//! <https://tools.ietf.org/html/rfc4572>

use nom::{branch::alt, bytes::complete::tag, combinator::map, IResult};

use crate::parsers::*;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[non_exhaustive]
pub enum SetupRole {
    Active,
    Passive,
    ActPass,
}

fn read_setup_role(input: &str) -> IResult<&str, SetupRole> {
    alt((
        map(tag("active"), |_| SetupRole::Active),
        map(tag("passive"), |_| SetupRole::Passive),
        map(tag("actpass"), |_| SetupRole::ActPass),
    ))(input)
}

pub fn setup_role_line(input: &str) -> IResult<&str, SetupRole> {
    attribute("setup", read_setup_role)(input)
}
