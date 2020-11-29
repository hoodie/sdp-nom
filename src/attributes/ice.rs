use nom::{branch::alt, bytes::complete::tag, combinator::map, IResult};

#[cfg(test)]
use crate::assert_line;
use crate::parsers::*;

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum IceParameter<'a> {
    Ufrag(&'a str),
    Pwd(&'a str),
    Options(&'a str),
    Mismatch,
    Lite,
}

pub fn ice_parameter_line(input: &str) -> IResult<&str, IceParameter> {
    alt((
        attribute("ice-ufrag", map(read_string, IceParameter::Ufrag)),
        attribute("ice-pwd", map(read_string, IceParameter::Pwd)),
        attribute("ice-options", map(read_string, IceParameter::Options)),
        a_line(map(tag("ice-mismatch"), |_| IceParameter::Mismatch)),
        a_line(map(tag("ice-lite"), |_| IceParameter::Lite)),
    ))(input)
}

#[test]
fn test_ice_parameters() {
    assert_line!(
        ice_parameter_line,
        "a=ice-ufrag:Oyef7uvBlwafI3hT",
        IceParameter::Ufrag("Oyef7uvBlwafI3hT"),
        print
    );
    assert_line!(
        ice_parameter_line,
        "a=ice-pwd:T0teqPLNQQOf+5W+ls+P2p16",
        IceParameter::Pwd("T0teqPLNQQOf+5W+ls+P2p16"),
        print
    );
    assert_line!(
        ice_parameter_line,
        "a=ice-ufrag:x+m/",
        IceParameter::Ufrag("x+m/"),
        print
    );
    assert_line!(
        ice_parameter_line,
        "a=ice-pwd:Vf2pbpatEroIg6NAaVCIGL94",
        IceParameter::Pwd("Vf2pbpatEroIg6NAaVCIGL94"),
        print
    );
    assert_line!(
        ice_parameter_line,
        "a=ice-options:trickle",
        IceParameter::Options("trickle"),
        print
    );
    assert_line!(ice_parameter_line, "a=ice-lite", IceParameter::Lite, print);
}
