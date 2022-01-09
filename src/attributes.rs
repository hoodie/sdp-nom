//! [SDP Attributes](https://tools.ietf.org/html/rfc4566#section-6) aka lines that start with `a=`

use derive_into_owned::IntoOwned;
use enum_as_inner::EnumAsInner;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    combinator::map,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};
use std::borrow::Cow;

pub mod candidate;
pub mod dtls;
pub mod extmap;
pub mod ice;
pub mod rtcp;
pub mod rtpmap;
pub mod ssrc;

use crate::parsers::*;

pub use bundle::*;
pub use candidate::*;
pub use control::*;
pub use direction::*;
pub use fingerprint::*;
pub use fmtp::*;
pub use ice::*;
pub use rtcp_option::*;
pub use rtp::*;
pub use ssrc::*;

#[derive(Clone, Debug, IntoOwned, EnumAsInner, PartialEq)]
#[non_exhaustive]
pub enum AttributeLine<'a> {
    /// `a=candidate:1853887674 2 udp 1518280447 0.0.0.0 36768 typ srflx raddr 192.168.0.196 rport 36768 generation 0`
    Candidate(candidate::Candidate<'a>),
    Ice(ice::IceParameter<'a>),
    Mid(mid::Mid<'a>),
    MsidSemantic(msid::MsidSemantic<'a>),
    Msid(msid::Msid<'a>),
    RtpMap(rtpmap::RtpMap<'a>),
    PTime(rtpmap::PTime),
    Ssrc(Ssrc<'a>),
    BundleGroup(BundleGroup<'a>),
    SsrcGroup(SsrcGroup),
    Fingerprint(Fingerprint<'a>),
    Direction(Direction),
    Rtp(Rtp<'a>),
    Rtcp(rtcp::Rtcp),
    Fmtp(Fmtp<'a>),
    RtcpFb(rtcp::Fb<'a>),
    RtcpOption(RtcpOption),
    Control(Control<'a>),
    SetupRole(dtls::SetupRole),
    Extmap(extmap::Extmap<'a>),
    BundleOnly,
    EoC,
    KeyValue {
        key: Cow<'a, str>,
        val: Cow<'a, str>,
    },
    KeyOnly(Cow<'a, str>),
}
pub fn attribute_line(input: &str) -> IResult<&str, AttributeLine> {
    alt((
        alt((
            map(mid::mid_line, AttributeLine::Mid),
            map(msid::msid_semantic_line, AttributeLine::MsidSemantic),
            map(msid::msid_line, AttributeLine::Msid),
            map(bundle::bundle_group_line, AttributeLine::BundleGroup),
            map(ice::ice_parameter_line, AttributeLine::Ice),
            map(ssrc::ssrc_line, AttributeLine::Ssrc),
            map(ssrc::ssrc_group_line, AttributeLine::SsrcGroup),
            map(rtpmap::rtpmap_line, AttributeLine::RtpMap),
            map(rtpmap::read_p_time, AttributeLine::PTime),
            map(fingerprint_line, AttributeLine::Fingerprint),
        )),
        alt((
            map(candidate::candidate_line, AttributeLine::Candidate),
            map(direction::direction_line, AttributeLine::Direction),
            map(extmap::extmap_line, AttributeLine::Extmap),
            map(dtls::setup_role_line, AttributeLine::SetupRole),
            map(rtp_attribute_line, AttributeLine::Rtp),
            map(rtcp::rtcp_attribute_line, AttributeLine::Rtcp),
            map(fmtp_attribute_line, AttributeLine::Fmtp),
            map(control_attribute_line, AttributeLine::Control),
            map(rtcp::rtcpfb_attribute_line, AttributeLine::RtcpFb),
            map(rtp_option_line, AttributeLine::RtcpOption),
            map(generic::key_val_attribute_line, |(key, val)| {
                AttributeLine::KeyValue { key, val }
            }),
            map(tag("a=bundle-only"), |_| AttributeLine::BundleOnly),
            map(tag("a=end-of-candidates"), |_| AttributeLine::EoC),
            map(generic::key_only_attribute_line, AttributeLine::KeyOnly),
        )),
    ))(input)
}

pub mod generic {
    use super::*;

    pub fn key_val_attribute_line(input: &str) -> IResult<&str, (Cow<'_, str>, Cow<'_, str>)> {
        a_line(map(
            separated_pair(
                cowify(read_non_colon_string),
                tag(":"),
                cowify(is_not("\n")),
            ),
            |(key, val)| (key, val),
        ))(input)
    }

    pub fn key_only_attribute_line(input: &str) -> IResult<&str, Cow<'_, str>> {
        a_line(cowify(is_not("\n")))(input)
    }
}

pub mod bundle {
    use super::*;

    /// `a=group:BUNDLE 0 1`
    #[derive(Clone, Debug, IntoOwned, PartialEq)]
    pub struct BundleGroup<'a>(pub Vec<Cow<'a, str>>);

    pub fn bundle_group_line(input: &str) -> IResult<&str, BundleGroup> {
        attribute("group", bundle_group)(input)
    }

    fn bundle_group(input: &str) -> IResult<&str, BundleGroup> {
        preceded(
            tag("BUNDLE"),
            map(wsf(space_separated_cow_strings), BundleGroup),
        )(input)
    }
}

pub mod rtp {
    use super::*;

    // a=rtpmap:110 opus/48000/2
    #[derive(Clone, Debug, IntoOwned, PartialEq)]
    pub struct Rtp<'a> {
        pub payload: u32,
        pub codec: Cow<'a, str>,
        pub rate: u32,
        pub encoding: u32,
    }

    pub fn rtp_attribute_line(input: &str) -> IResult<&str, Rtp> {
        attribute("rtpmap", rtp_attribute)(input)
    }

    fn rtp_attribute(input: &str) -> IResult<&str, Rtp> {
        map(
            tuple((
                wsf(read_number),                   // payload
                wsf(cowify(read_non_slash_string)), // codec
                preceded(tag("/"), read_number),    // rate
                preceded(tag("/"), read_number),    // encoding
            )),
            |(payload, codec, rate, encoding)| Rtp {
                payload,
                codec,
                rate,
                encoding,
            },
        )(input)
    }
}

pub mod fmtp {
    use super::*;
    ///<https://tools.ietf.org/html/rfc4588#section-8.1>
    /// `a=fmtp:108 profile-level-id=24;object=23;bitrate=64000`
    #[derive(Clone, Debug, IntoOwned, PartialEq)]
    pub struct Fmtp<'a> {
        pub payload: u32,
        pub config: Cow<'a, str>,
    }

    pub fn fmtp_attribute_line(input: &str) -> IResult<&str, Fmtp> {
        attribute("fmtp", fmtp_attribute)(input)
    }

    fn fmtp_attribute(input: &str) -> IResult<&str, Fmtp> {
        map(
            tuple((
                read_number,               // payload
                cowify(wsf(is_not("\n"))), // config
            )),
            |(payload, config)| (Fmtp { payload, config }),
        )(input)
    }
}
pub mod control {
    use super::*;

    /// `a=control:streamid=0`
    #[derive(Clone, Debug, IntoOwned, PartialEq)]

    pub struct Control<'a>(pub Cow<'a, str>);

    pub fn control_attribute_line(input: &str) -> IResult<&str, Control> {
        attribute("control", control_attribute)(input)
    }

    fn control_attribute(input: &str) -> IResult<&str, Control> {
        map(cowify(read_string), Control)(input)
    }
}
pub mod direction {
    use super::*;

    /// Direction
    ///
    /// `a=sendrecv`
    /// `a=sendonly`
    /// `a=recvonly`
    /// `a=inactive`
    #[derive(Debug, PartialEq, Clone, Copy)]
    #[non_exhaustive]
    pub enum Direction {
        SendOnly,
        SendRecv,
        RecvOnly,
        Inactive,
    }

    pub fn read_direction(input: &str) -> IResult<&str, Direction> {
        alt((
            map(tag("sendrecv"), |_| Direction::SendRecv),
            map(tag("sendonly"), |_| Direction::SendOnly),
            map(tag("recvonly"), |_| Direction::RecvOnly),
            map(tag("inactive"), |_| Direction::Inactive),
        ))(input)
    }

    /// `a=sendrecv`
    pub fn direction_line(input: &str) -> IResult<&str, Direction> {
        a_line(wsf(read_direction))(input)
    }
}
pub mod rtcp_option {
    use super::*;

    #[derive(Clone, Debug, PartialEq)]
    #[non_exhaustive]
    pub enum RtcpOption {
        RtcpMux,
        RtcpMuxOnly,
        RtcpRsize,
    }

    pub fn rtp_option(input: &str) -> IResult<&str, RtcpOption> {
        alt((
            map(tag("rtcp-rsize"), |_| RtcpOption::RtcpRsize),
            map(tag("rtcp-mux-only"), |_| RtcpOption::RtcpMuxOnly),
            map(tag("rtcp-mux"), |_| RtcpOption::RtcpMux),
        ))(input)
    }
    pub fn rtp_option_line(input: &str) -> IResult<&str, RtcpOption> {
        a_line(rtp_option)(input)
    }
}
pub mod fingerprint {
    use super::*;

    #[derive(Clone, Debug, IntoOwned, PartialEq)]
    pub struct Fingerprint<'a> {
        pub r#type: Cow<'a, str>,
        pub hash: Cow<'a, str>,
    }

    /// fingerprint
    pub fn fingerprint_line(input: &str) -> IResult<&str, Fingerprint> {
        attribute("fingerprint", fingerprint)(input)
    }

    /// fingerprint
    pub fn fingerprint(input: &str) -> IResult<&str, Fingerprint> {
        map(
            tuple((
                cowify(wsf(read_string)), // type
                cowify(wsf(read_string)), // hash
            )),
            |(r#type, hash)| Fingerprint { r#type, hash },
        )(input)
    }
}

pub mod mid {
    use super::*;

    #[derive(Clone, Debug, IntoOwned, PartialEq)]

    pub struct Mid<'a>(pub Cow<'a, str>);

    pub fn mid_line(input: &str) -> IResult<&str, Mid> {
        attribute("mid", mid)(input)
    }

    pub fn mid(input: &str) -> IResult<&str, Mid> {
        map(cowify(read_string), Mid)(input)
    }
}

pub mod msid {
    use nom::{character::complete::multispace1, combinator::opt};

    use super::*;

    /// TODO: type this more strictly, if possible without `Vec`
    #[derive(Clone, Debug, derive_into_owned::IntoOwned, PartialEq)]
    pub struct MsidSemantic<'a> {
        pub semantic: Cow<'a, str>,
        pub token: Option<Cow<'a, str>>,
    }

    pub fn msid_semantic_line(input: &str) -> IResult<&str, MsidSemantic> {
        attribute("msid-semantic", msid_semantic)(input)
    }

    pub fn msid_semantic(input: &str) -> IResult<&str, MsidSemantic> {
        wsf(map(
            tuple((cowify(read_string), multispace1, opt(cowify(read_string)))),
            |(semantic, _, token)| MsidSemantic { semantic, token },
        ))(input)
    }

    #[derive(Clone, Debug, IntoOwned, PartialEq)]
    pub struct Msid<'a>(pub Vec<Cow<'a, str>>);

    pub fn msid_line(input: &str) -> IResult<&str, Msid> {
        attribute("msid", msid)(input)
    }

    pub fn msid(input: &str) -> IResult<&str, Msid> {
        wsf(map(space_separated_cow_strings, Msid))(input)
    }
}
