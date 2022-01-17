//! <https://tools.ietf.org/html/rfc4572>

use nom::{branch::alt, bytes::complete::tag, combinator::map, IResult};

#[cfg(test)]
use crate::assert_line;
use crate::parsers::*;

#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "debug", derive(Debug))]
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

#[test]
fn test_setup_role() {
    assert_line!(read_setup_role, "active", SetupRole::Active);
    assert_line!(read_setup_role, "passive", SetupRole::Passive);
}

pub fn setup_role_line(input: &str) -> IResult<&str, SetupRole> {
    attribute("setup", read_setup_role)(input)
}

#[test]
fn test_setup_role_line() {
    assert_line!(setup_role_line, "a=setup:active", SetupRole::Active, print);
    assert_line!(
        setup_role_line,
        "a=setup:actpass",
        SetupRole::ActPass,
        print
    );
    assert_line!(
        setup_role_line,
        "a=setup:passive",
        SetupRole::Passive,
        print
    );
}
