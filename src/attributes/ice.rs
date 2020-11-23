use nom::{branch::alt, combinator::map, IResult};
use std::fmt;

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

impl fmt::Display for IceParameter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IceParameter::Ufrag(ufrag) => write!(f, "a=ice-ufrag:{}", ufrag),
            IceParameter::Pwd(pwd) => write!(f, "a=ice-pwd:{}", pwd),
            IceParameter::Options(options) => write!(f, "a=ice-options:{}", options),
        }
    }
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
}
