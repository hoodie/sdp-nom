//! [SDP Attributes](https://tools.ietf.org/html/rfc4566#section-6) aka lines that start with `a=`

use derive_into_owned::IntoOwned;
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
#[cfg(test)]
use crate::{assert_line, assert_line_print};

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

// #[derive(Debug)]
// #[non_exhaustive]
// pub enum SdpLine<'a> {
//     // MsidSemantic(super::media::MsidSemantic<'a>),
//     // Msid(super::media::Msid<'a>),
//     RtpMap(rtpmap::RtpMap<'a>),
//     PTime(rtpmap::PTime),
//
//     Ssrc(Ssrc<'a>),
//     BundleGroup(BundleGroup<'a>),
//     SsrcGroup(SsrcGroup),
//     Fingerprint(Fingerprint<'a>),
//     Direction(Direction),
//     Rtp(Rtp<'a>),
//     Rtcp(rtcp::Rtcp),
//     Fmtp(Fmtp<'a>),
//     RtcpFb(rtcp::Fb<'a>),
//     RtcpOption(RtcpOption),
//     Control(Control<'a>),
//     SetupRole(dtls::SetupRole),
//     Extmap(extmap::Extmap<'a>),
//     BundleOnly,
//     EoC,
//     Attribute {
//         key: Cow<'a, str>,
//         val: Cow<'a, str>,
//     },
// }

#[derive(Debug, IntoOwned)]
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
    Attribute {
        key: Cow<'a, str>,
        val: Cow<'a, str>,
    },
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
            map(generic::lazy_attribute_line, |(key, val)| {
                AttributeLine::Attribute { key, val }
            }),
            map(tag("a=bundle-only"), |_| AttributeLine::BundleOnly),
            map(tag("a=end-of-candidates"), |_| AttributeLine::EoC),
        )),
    ))(input)
}

pub mod generic {
    use super::*;

    pub fn lazy_attribute_line(input: &str) -> IResult<&str, (Cow<'_, str>, Cow<'_, str>)> {
        a_line(map(
            separated_pair(
                cowify(read_non_colon_string),
                tag(":"),
                cowify(is_not("\n")),
            ),
            |(key, val)| (key, val),
        ))(input)
    }
    #[test]
    fn test_lazy_attribute_line() {
        assert_line!(
            lazy_attribute_line,
            "a=foo:bar",
            ("foo".into(), "bar".into())
        );
        assert_line!(
            lazy_attribute_line,
            "a=fmtp:111 minptime=10; useinbandfec=1",
            ("fmtp".into(), "111 minptime=10; useinbandfec=1".into())
        );
        assert_line!(
            lazy_attribute_line,
            "a=setup:actpass",
            ("setup".into(), "actpass".into())
        );
    }
}

pub mod bundle {
    use super::*;

    /// `a=group:BUNDLE 0 1`
    #[derive(Debug, IntoOwned, PartialEq)]
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

    #[test]
    fn test_bundle_group_line() {
        assert_line!(
            bundle_group_line,
            "a=group:BUNDLE 0 1",
            BundleGroup(create_test_vec(&["0", "1"])),
            print
        );
        assert_line!(
            bundle_group_line,
            "a=group:BUNDLE video",
            BundleGroup(create_test_vec(&["video"])),
            print
        );
        assert_line!(
            bundle_group_line,
            "a=group:BUNDLE sdparta_0 sdparta_1 sdparta_2",
            BundleGroup(create_test_vec(&["sdparta_0", "sdparta_1", "sdparta_2"])),
            print
        );
    }
}

pub mod rtp {
    use super::*;

    // a=rtpmap:110 opus/48000/2
    #[derive(Debug, IntoOwned, PartialEq)]
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

    #[test]
    fn test_rtp_attribute_line() {
        assert_line!("a=rtpmap:110 opus/48000/2", rtp_attribute_line);
    }
}

pub mod fmtp {
    use super::*;
    ///<https://tools.ietf.org/html/rfc4588#section-8.1>
    /// `a=fmtp:108 profile-level-id=24;object=23;bitrate=64000`
    #[derive(Debug, IntoOwned, PartialEq)]
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

    #[test]
    fn test_fmtp_attribute_line() {
        assert_line!(
            fmtp_attribute_line,
            "a=fmtp:108 profile-level-id=24;object=23;bitrate=64000",
            Fmtp {
                payload: 108,
                config: "profile-level-id=24;object=23;bitrate=64000".into(),
            },
            print
        );
        assert_line_print!(
            fmtp_attribute_line,
            "a=fmtp:111 minptime=10; useinbandfec=1"
        );
    }
}
pub mod control {
    use super::*;

    /// `a=control:streamid=0`
    #[derive(Debug, IntoOwned, PartialEq)]
    pub struct Control<'a>(pub Cow<'a, str>);

    pub fn control_attribute_line(input: &str) -> IResult<&str, Control> {
        attribute("control", control_attribute)(input)
    }

    fn control_attribute(input: &str) -> IResult<&str, Control> {
        map(cowify(read_string), Control)(input)
    }

    #[test]
    fn test_control_attribute_line() {
        assert_line_print!(control_attribute_line, "a=control:streamid=0");
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

    #[test]
    fn test_direction_line() {
        assert_line!(read_direction, "sendrecv", Direction::SendRecv);
        assert_line!(direction_line, "a=sendrecv", Direction::SendRecv);

        assert_line!(read_direction, "sendonly", Direction::SendOnly);
        assert_line!(direction_line, "a=sendonly", Direction::SendOnly);

        assert_line!(read_direction, "recvonly", Direction::RecvOnly);
        assert_line!(direction_line, "a=recvonly", Direction::RecvOnly);

        assert_line!(read_direction, "inactive", Direction::Inactive);
        assert_line!(direction_line, "a=inactive", Direction::Inactive);
    }
}
pub mod rtcp_option {
    use super::*;

    #[derive(Debug, PartialEq)]
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

    #[test]
    fn test_read_rtp_option() {
        assert_line!(rtp_option_line, "a=rtcp-mux", RtcpOption::RtcpMux, print);
        assert_line!(
            rtp_option_line,
            "a=rtcp-mux-only",
            RtcpOption::RtcpMuxOnly,
            print
        );
        assert_line!(
            rtp_option_line,
            "a=rtcp-rsize",
            RtcpOption::RtcpRsize,
            print
        );
    }
}
pub mod fingerprint {
    use super::*;

    #[derive(Debug, IntoOwned)]
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

    #[test]
    fn test_fingerprint_line() {
        assert_line_print!(
            fingerprint_line,
            "a=fingerprint:sha-256 19:E2:1C:3B:4B:9F:81:E6:B8:5C:F4:A5:A8:D8:73:04:BB:05:2F:70:9F:04:A9:0E:05:E9:26:33:E8:70:88:A2");
    }
}

pub mod mid {
    use super::*;

    #[derive(Debug, IntoOwned)]
    pub struct Mid<'a>(pub Cow<'a, str>);

    pub fn mid_line(input: &str) -> IResult<&str, Mid> {
        attribute("mid", mid)(input)
    }

    pub fn mid(input: &str) -> IResult<&str, Mid> {
        map(cowify(read_string), Mid)(input)
    }

    #[test]
    fn test_mid_line() {
        assert_line_print!(mid_line, "a=mid:1");
        assert_line_print!(mid_line, "a=mid:a1");
        assert_line_print!(mid_line, "a=mid:0");
        assert_line_print!(mid_line, "a=mid:audio")
    }
}

pub mod msid {
    use super::*;

    /// TODO: type this more strictly, if possible without `Vec`
    #[derive(Debug, derive_into_owned::IntoOwned, PartialEq)]
    pub struct MsidSemantic<'a>(pub Vec<Cow<'a, str>>);

    pub fn msid_semantic_line(input: &str) -> IResult<&str, MsidSemantic> {
        attribute("msid-semantic", msid_semantic)(input)
    }

    pub fn msid_semantic(input: &str) -> IResult<&str, MsidSemantic> {
        wsf(map(space_separated_cow_strings, MsidSemantic))(input)
    }

    #[test]
    fn test_msid_semantic_line() {
        assert_line!(
            msid_semantic_line,
            "a=msid-semantic: WMS lgsCFqt9kN2fVKw5wg3NKqGdATQoltEwOdMS",
            MsidSemantic(create_test_vec(&[
                "WMS",
                "lgsCFqt9kN2fVKw5wg3NKqGdATQoltEwOdMS"
            ]))
        );
        assert_line_print!(
            msid_semantic_line,
            "a=msid-semantic:WMS lgsCFqt9kN2fVKw5wg3NKqGdATQoltEwOdMS"
        );
    }

    #[derive(Debug, IntoOwned, PartialEq)]
    pub struct Msid<'a>(pub Vec<Cow<'a, str>>);

    pub fn msid_line(input: &str) -> IResult<&str, Msid> {
        attribute("msid", msid)(input)
    }

    pub fn msid(input: &str) -> IResult<&str, Msid> {
        wsf(map(space_separated_cow_strings, Msid))(input)
    }

    #[test]
    fn test_msid_line() {
        assert_line!(
            msid_line,
            "a=msid:47017fee-b6c1-4162-929c-a25110252400 f83006c5-a0ff-4e0a-9ed9-d3e6747be7d9",
            Msid(vec![
                "47017fee-b6c1-4162-929c-a25110252400".into(),
                "f83006c5-a0ff-4e0a-9ed9-d3e6747be7d9".into()
            ]),
            print
        );
        assert_line_print!(
            msid_line,
            "a=msid:61317484-2ed4-49d7-9eb7-1414322a7aae f30bdb4a-5db8-49b5-bcdc-e0c9a23172e0"
        );
    }
}
