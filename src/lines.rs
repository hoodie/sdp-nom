#![allow(dead_code)]

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    combinator::map,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

use std::fmt;

pub mod connection;
pub mod media;
pub mod origin;

#[cfg(test)]
use crate::assert_line;
use crate::parsers::*;

pub mod version {
    use super::*;

    #[derive(Debug, PartialEq)]
    pub struct Version(u32);

    /// "v=0"
    pub fn version_line(input: &str) -> IResult<&str, Version> {
        preceded(tag("v="), map(wsf(read_number), Version))(input)
    }

    impl fmt::Display for Version {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "v={}", self.0)
        }
    }

    #[test]
    fn test_version_line() {
        assert_line!(version_line, "v=0", Version(0), print);
        assert_line!(version_line, "v= 0");
    }
}

pub mod session_name {
    use super::*;

    /// `s=somename`
    ///
    /// https://tools.ietf.org/html/rfc4566#section-5.3
    #[derive(Debug, PartialEq)]
    pub struct SessionName<'a>(pub &'a str);

    /// "s=somename"
    pub fn name_line(input: &str) -> IResult<&str, SessionName> {
        preceded(tag("s="), map(wsf(read_string0), SessionName))(input)
    }

    impl<'a> std::fmt::Display for SessionName<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "s={}", self.0)
        }
    }

    #[test]
    fn test_name_line() {
        assert_line!(name_line, "s=", SessionName(""), print);
        assert_line!(name_line, "s=testname", SessionName("testname"), print);
        assert_line!(name_line, "s= testname", SessionName("testname"));
        assert_line!(name_line, "s=testname ", SessionName("testname"));
    }
}

pub mod session_information {
    use super::*;

    /// `i=<session description>`
    #[derive(Debug, PartialEq)]
    pub struct SessionInformation<'a>(pub &'a str);

    /// SessionInformation "i=description"
    pub fn description_line(input: &str) -> IResult<&str, SessionInformation> {
        line("i=", map(read_string, SessionInformation))(input)
    }

    impl<'a> std::fmt::Display for SessionInformation<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "i={}", self.0)
        }
    }

    #[test]
    fn test_description_line() {
        assert_line!(
            description_line,
            "i=testdescription",
            SessionInformation("testdescription"),
            print
        );
    }
}

pub mod uri {
    use super::*;
    /// Uri `u=<uri>`
    #[derive(Debug, PartialEq)]
    pub struct Uri<'a>(pub &'a str);

    /// "i=description"
    pub fn uri_line(input: &str) -> IResult<&str, Uri> {
        line("u=", map(read_string, Uri))(input)
    }

    impl fmt::Display for Uri<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    #[test]
    fn test_uri_line() {
        assert_line!(
            uri_line,
            "u=https://parse-my.sdp",
            Uri("https://parse-my.sdp")
        );
    }
}

pub mod email {
    use super::*;

    /// Email `e=<email-address>`
    #[derive(Debug, PartialEq)]
    pub struct EmailAddress<'a>(pub &'a str);

    /// "e=email@example.com"
    pub fn email_address_line(input: &str) -> IResult<&str, EmailAddress> {
        line("e=", wsf(map(read_string, EmailAddress)))(input)
    }

    impl<'a> std::fmt::Display for EmailAddress<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "e={}", self.0)
        }
    }

    #[test]
    fn test_email_address_line() {
        assert_line!(
            email_address_line,
            "e=test@example.com",
            EmailAddress("test@example.com"),
            print
        );
    }
}

// ////////////////////////

pub mod phone_number {
    use super::*;
    /// Email `p=<phone-number>`
    #[derive(Debug, PartialEq)]
    pub struct PhoneNumber<'a>(pub &'a str);

    /// "i=description"
    pub fn phone_number_line(input: &str) -> IResult<&str, PhoneNumber> {
        line("p=", map(take_while(|_| true), PhoneNumber))(input)
    }

    #[test]
    fn test_phone_number_line() {
        assert_line!(
            phone_number_line,
            "p=0118 999 881 999 119 7253",
            PhoneNumber("0118 999 881 999 119 7253"),
            print
        );
    }
    impl<'a> std::fmt::Display for PhoneNumber<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "p={}", self.0)
        }
    }
}

pub mod timing {
    use super::*;

    /// `t=0 0`
    #[derive(Debug, PartialEq)]
    pub struct Timing {
        start: u32,
        stop: u32,
    }

    /// "t=0 0"
    pub fn timing_line(input: &str) -> IResult<&str, Timing> {
        line(
            "t=",
            wsf(map(
                tuple((wsf(read_number), wsf(read_number))),
                |(start, stop)| Timing { start, stop },
            )),
        )(input)
    }

    #[test]
    #[rustfmt::skip]
    fn test_timing_line() {
        assert_line!(timing_line,"t=0 1", Timing { start: 0, stop: 1 }, print);
        assert_line!(timing_line,"t=  2 3 ", Timing { start: 2, stop: 3 });
        assert_line!(timing_line,"t=  2  3 ", Timing { start: 2, stop: 3 });
        assert_line!(timing_line,"t=23 42", Timing { start: 23, stop: 42 }, print);
    }

    impl fmt::Display for Timing {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "t={} {}", self.start, self.stop)
        }
    }
}

pub mod bandwidth {
    use super::*;
    #[derive(Debug, PartialEq)]
    pub enum BandWidthType {
        TIAS,
        AS,
        CT,
        RR,
        RS,
    }
    // TIAS|AS|CT|RR|RS
    pub fn bandwidth_type(input: &str) -> IResult<&str, BandWidthType> {
        alt((
            map(tag("TIAS"), |_| BandWidthType::TIAS),
            map(tag("AS"), |_| BandWidthType::AS),
            map(tag("CT"), |_| BandWidthType::CT),
            map(tag("RR"), |_| BandWidthType::RR),
            map(tag("RS"), |_| BandWidthType::RS),
        ))(input)
    }

    #[derive(Debug, PartialEq)]
    /// "b=AS:1024"
    pub struct BandWidth {
        r#type: BandWidthType,
        limit: u32,
    }

    /// "b=AS:1024"
    pub fn bandwidth_line(input: &str) -> IResult<&str, BandWidth> {
        line("b=", bandwidth)(input)
    }

    /// "AS:1024"
    pub fn bandwidth(input: &str) -> IResult<&str, BandWidth> {
        map(
            separated_pair(bandwidth_type, tag(":"), read_number),
            |(r#type, limit)| (BandWidth { r#type, limit }),
        )(input)
    }

    impl fmt::Display for BandWidthType {
        #[rustfmt::skip]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use BandWidthType::*;
        write!( f, "{}", match self { TIAS => "TIAS", AS => "AS", CT => "CT", RR => "RR", RS => "R" })
    }
    }

    impl fmt::Display for BandWidth {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "b={}:{}", self.r#type, self.limit)
        }
    }

    #[test]
    #[rustfmt::skip]
    fn test_bandwidth_line() {
        assert_line!(
            bandwidth_line,"b=AS:30",
            BandWidth { r#type: BandWidthType::AS, limit: 30 }, print
        );
        assert_line!(
            bandwidth_line,"b=RR:1024",
            BandWidth { r#type: BandWidthType::RR, limit: 1024 }, print
        );
    }
}
