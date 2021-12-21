#![allow(dead_code)]

use derive_into_owned::IntoOwned;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    combinator::map,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

use std::borrow::Cow;

pub mod connection;
pub mod media;
pub mod origin;

#[cfg(test)]
use crate::assert_line;
use crate::parsers::*;

pub mod version {
    use super::*;

    #[derive(Debug, IntoOwned, PartialEq)]
    pub struct Version(pub u32);

    /// "v=0"
    pub fn version_line(input: &str) -> IResult<&str, Version> {
        preceded(tag("v="), map(wsf(read_number), Version))(input)
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
    /// <https://tools.ietf.org/html/rfc4566#section-5.3>
    #[derive(Debug, IntoOwned, PartialEq)]
    pub struct SessionName<'a>(pub Cow<'a, str>);

    /// "s=somename"
    pub fn name_line(input: &str) -> IResult<&str, SessionName> {
        preceded(tag("s="), map(wsf(cowify(read_string0)), SessionName))(input)
    }

    #[test]
    fn test_name_line() {
        assert_line!(name_line, "s=", SessionName("".into()), print);
        assert_line!(
            name_line,
            "s=testname",
            SessionName("testname".into()),
            print
        );
        assert_line!(name_line, "s= testname", SessionName("testname".into()));
        assert_line!(name_line, "s=testname ", SessionName("testname".into()));
    }
}

pub mod session_information {
    use super::*;

    /// `i=<session description>`
    #[derive(Debug, IntoOwned, PartialEq)]
    pub struct SessionInformation<'a>(pub Cow<'a, str>);

    /// SessionInformation "i=description"
    pub fn description_line(input: &str) -> IResult<&str, SessionInformation> {
        line("i=", map(cowify(read_string), SessionInformation))(input)
    }

    #[test]
    fn test_description_line() {
        assert_line!(
            description_line,
            "i=testdescription",
            SessionInformation("testdescription".into()),
            print
        );
    }
}

pub mod uri {
    use super::*;
    /// Uri `u=<uri>`
    #[derive(Debug, IntoOwned, PartialEq)]
    pub struct Uri<'a>(pub Cow<'a, str>);

    /// "i=description"
    pub fn uri_line(input: &str) -> IResult<&str, Uri> {
        line("u=", map(cowify(read_string), Uri))(input)
    }

    #[test]
    fn test_uri_line() {
        assert_line!(
            uri_line,
            "u=https://parse-my.sdp",
            Uri("https://parse-my.sdp".into())
        );
    }
}

pub mod email {

    use super::*;

    /// Email `e=<email-address>`
    #[derive(Debug, IntoOwned, PartialEq)]
    pub struct EmailAddress<'a>(pub Cow<'a, str>);

    /// "e=email@example.com"
    pub fn email_address_line(input: &str) -> IResult<&str, EmailAddress> {
        line("e=", wsf(map(cowify(read_string), EmailAddress)))(input)
    }

    #[test]
    fn test_email_address_line() {
        assert_line!(
            email_address_line,
            "e=test@example.com",
            EmailAddress("test@example.com".into()),
            print
        );
    }
}

// ////////////////////////

pub mod phone_number {
    use super::*;
    /// Email `p=<phone-number>`
    #[derive(Debug, IntoOwned, PartialEq)]
    pub struct PhoneNumber<'a>(pub Cow<'a, str>);

    /// "i=description"
    pub fn phone_number_line(input: &str) -> IResult<&str, PhoneNumber> {
        line("p=", map(cowify(take_while(|_| true)), PhoneNumber))(input)
    }

    #[test]
    fn test_phone_number_line() {
        assert_line!(
            phone_number_line,
            "p=0118 999 881 999 119 7253",
            PhoneNumber("0118 999 881 999 119 7253".into()),
            print
        );
    }
}

pub mod timing {
    use super::*;

    /// `t=0 0`
    #[derive(Debug, PartialEq)]
    pub struct Timing {
        pub start: u32,
        pub stop: u32,
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
}

pub mod bandwidth {
    use super::*;
    #[derive(Debug, PartialEq)]
    #[non_exhaustive]
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
        pub r#type: BandWidthType,
        pub limit: u32,
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

pub mod comment {
    use super::*;

    pub fn comment_line(input: &str) -> IResult<&str, &str> {
        preceded(tag(";"), wsf(read_everything))(input)
    }

    #[test]
    fn test_read_comment() {
        assert_line!(
            comment_line,
            "; this should not be part of the document",
            "this should not be part of the document"
        )
    }
}
