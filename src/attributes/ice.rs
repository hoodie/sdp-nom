use std::borrow::Cow;

use derive_into_owned::IntoOwned;
use nom::{branch::alt, bytes::complete::tag, combinator::map, IResult};

#[cfg(test)]
use crate::assert_line;
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

#[test]
fn test_ice_parameters() {
    assert_line!(
        ice_parameter_line,
        "a=ice-ufrag:Oyef7uvBlwafI3hT",
        IceParameter::Ufrag("Oyef7uvBlwafI3hT".into()),
        print
    );
    assert_line!(
        ice_parameter_line,
        "a=ice-pwd:T0teqPLNQQOf+5W+ls+P2p16",
        IceParameter::Pwd("T0teqPLNQQOf+5W+ls+P2p16".into()),
        print
    );
    assert_line!(
        ice_parameter_line,
        "a=ice-ufrag:x+m/",
        IceParameter::Ufrag("x+m/".into()),
        print
    );
    assert_line!(
        ice_parameter_line,
        "a=ice-pwd:Vf2pbpatEroIg6NAaVCIGL94",
        IceParameter::Pwd("Vf2pbpatEroIg6NAaVCIGL94".into()),
        print
    );
    assert_line!(
        ice_parameter_line,
        "a=ice-options:trickle",
        IceParameter::Options("trickle".into()),
        print
    );
    assert_line!(ice_parameter_line, "a=ice-lite", IceParameter::Lite, print);
}
