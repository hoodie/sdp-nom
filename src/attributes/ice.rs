use nom::{branch::alt, combinator::map, IResult};

#[cfg(test)]
use crate::assert_line;
use crate::parsers::*;

#[derive(Debug, PartialEq)]
pub enum IceParameter<'a> {
    Ufrag(&'a str),
    Pwd(&'a str),
    Options(&'a str),
}

pub fn ice_parameter_line(input: &str) -> IResult<&str, IceParameter> {
    alt((
        attribute("ice-ufrag", map(read_string, IceParameter::Ufrag)),
        attribute("ice-pwd", map(read_string, IceParameter::Pwd)),
        attribute("ice-options", map(read_string, IceParameter::Options)),
    ))(input)
}

#[test]
fn test_ice_parameters() {
    assert_line!(
        ice_parameter_line,
        "a=ice-ufrag:Oyef7uvBlwafI3hT",
        IceParameter::Ufrag("Oyef7uvBlwafI3hT")
    );
    assert_line!(
        ice_parameter_line,
        "a=ice-pwd:T0teqPLNQQOf+5W+ls+P2p16",
        IceParameter::Pwd("T0teqPLNQQOf+5W+ls+P2p16")
    );
    assert_line!(
        ice_parameter_line,
        "a=ice-ufrag:x+m/",
        IceParameter::Ufrag("x+m/")
    );
    assert_line!(
        ice_parameter_line,
        "a=ice-pwd:Vf2pbpatEroIg6NAaVCIGL94",
        IceParameter::Pwd("Vf2pbpatEroIg6NAaVCIGL94")
    );
    assert_line!(
        ice_parameter_line,
        "a=ice-options:trickle",
        IceParameter::Options("trickle")
    );
}
