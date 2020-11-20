//! https://tools.ietf.org/html/rfc4572

use nom::*;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_till1},
    combinator::{map, opt},
    sequence::{preceded, separated_pair, tuple},
};

#[cfg(test)]
use crate::assert_line;
use crate::parsers::*;

#[derive(Debug, PartialEq)]
pub enum SetupRole {
    Active,
    Passive,
    ActPass,
}

fn setup_role(input: &str) -> IResult<&str, SetupRole> {
    alt((
        map(tag("active"), |_| SetupRole::Active),
        map(tag("passive"), |_| SetupRole::Passive),
        map(tag("actpass"), |_| SetupRole::ActPass),
    ))(input)
}

#[test]
fn test_setup_role() {
    assert_line!(setup_role, "active", SetupRole::Active);
    assert_line!(setup_role, "passive", SetupRole::Passive);
}

pub(crate) fn setup_role_line(input: &str) -> IResult<&str, SetupRole> {
    a_line(preceded(tag("setup:"), setup_role))(input)
}
#[test]
fn test_setup_role_line() {
    assert_line!(setup_role_line, "a=setup:active", SetupRole::Active);
    assert_line!(setup_role_line, "a=setup:passive", SetupRole::Passive);
}
