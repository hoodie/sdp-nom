//! [6. SDP Attributes](https://tools.ietf.org/html/rfc4566#section-6)

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    combinator::map,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

pub mod candidate;
pub mod dtls;
pub mod extmap;
pub mod ice;
pub mod rtcp;
pub mod rtpmap;
pub mod ssrc;

pub use candidate::*;
pub use ice::*;
pub use ssrc::*;

use crate::parsers::*;
#[cfg(test)]
use crate::{assert_line, assert_line_print};

pub use bundle::*;
pub use control::*;
pub use direction::*;
pub use fingerprint::*;
pub use fmtp::*;
pub use rtcp_option::*;
pub use rtp::*;

#[derive(Debug)]
#[non_exhaustive]
pub enum SdpLine<'a> {
    // MsidSemantic(super::media::MsidSemantic<'a>),
    // Msid(super::media::Msid<'a>),
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
    Attribute { key: &'a str, val: &'a str },
}

pub mod generic {
    use super::*;

    pub fn lazy_attribute_line(input: &str) -> IResult<&str, (&str, &str)> {
        a_line(map(
            separated_pair(read_non_colon_string, tag(":"), is_not("\n")),
            |(key, val)| (key, val),
        ))(input)
    }
    #[test]
    fn test_lazy_attribute_line() {
        assert_line!(lazy_attribute_line, "a=foo:bar", ("foo", "bar"));
        assert_line!(
            lazy_attribute_line,
            "a=fmtp:111 minptime=10; useinbandfec=1",
            ("fmtp", "111 minptime=10; useinbandfec=1")
        );
        assert_line!(lazy_attribute_line, "a=setup:actpass", ("setup", "actpass"));
    }
}

pub mod bundle {
    use super::*;

    /// `a=group:BUNDLE 0 1`
    #[derive(Debug, PartialEq)]
    pub struct BundleGroup<'a>(pub Vec<&'a str>);

    pub fn bundle_group_line(input: &str) -> IResult<&str, BundleGroup> {
        attribute("group", bundle_group)(input)
    }

    fn bundle_group(input: &str) -> IResult<&str, BundleGroup> {
        preceded(
            tag("BUNDLE"),
            map(wsf(space_separated_strings), BundleGroup),
        )(input)
    }

    #[test]
    fn test_bundle_group_line() {
        assert_line!(
            bundle_group_line,
            "a=group:BUNDLE 0 1",
            BundleGroup(vec!["0", "1"]),
            print
        );
        assert_line!(
            bundle_group_line,
            "a=group:BUNDLE video",
            BundleGroup(vec!["video"]),
            print
        );
        assert_line!(
            bundle_group_line,
            "a=group:BUNDLE sdparta_0 sdparta_1 sdparta_2",
            BundleGroup(vec!["sdparta_0", "sdparta_1", "sdparta_2"]),
            print
        );
    }
}

pub mod rtp {
    use super::*;

    // a=rtpmap:110 opus/48000/2
    #[derive(Debug, PartialEq)]
    pub struct Rtp<'a> {
        pub payload: u32,
        pub codec: &'a str,
        pub rate: u32,
        pub encoding: u32,
    }

    pub fn rtp_attribute_line(input: &str) -> IResult<&str, Rtp> {
        attribute("rtpmap", rtp_attribute)(input)
    }

    fn rtp_attribute(input: &str) -> IResult<&str, Rtp> {
        map(
            tuple((
                wsf(read_number),                // payload
                wsf(read_non_slash_string),      // codec
                preceded(tag("/"), read_number), // rate
                preceded(tag("/"), read_number), // encoding
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
    #[derive(Debug, PartialEq)]
    pub struct Fmtp<'a> {
        pub payload: u32,
        pub config: &'a str,
    }

    pub fn fmtp_attribute_line(input: &str) -> IResult<&str, Fmtp> {
        attribute("fmtp", fmtp_attribute)(input)
    }

    fn fmtp_attribute(input: &str) -> IResult<&str, Fmtp> {
        map(
            tuple((
                read_number,       // payload
                wsf(is_not("\n")), // config
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
                config: "profile-level-id=24;object=23;bitrate=64000",
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
    #[derive(Debug, PartialEq)]
    pub struct Control<'a>(pub &'a str);

    pub fn control_attribute_line(input: &str) -> IResult<&str, Control> {
        attribute("control", control_attribute)(input)
    }

    fn control_attribute(input: &str) -> IResult<&str, Control> {
        map(read_string, Control)(input)
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

    #[derive(Debug)]
    pub struct Fingerprint<'a> {
        pub r#type: &'a str,
        pub hash: &'a str,
    }

    /// fingerprint
    pub fn fingerprint_line(input: &str) -> IResult<&str, Fingerprint> {
        attribute("fingerprint", fingerprint)(input)
    }

    /// fingerprint
    pub fn fingerprint(input: &str) -> IResult<&str, Fingerprint> {
        map(
            tuple((
                wsf(read_string), // type
                wsf(read_string), // hash
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

    #[derive(Debug)]
    pub struct Mid<'a>(pub &'a str);

    pub fn mid_line(input: &str) -> IResult<&str, Mid> {
        attribute("mid", mid)(input)
    }

    pub fn mid(input: &str) -> IResult<&str, Mid> {
        map(read_string, Mid)(input)
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
    #[derive(Debug, PartialEq)]
    pub struct MsidSemantic<'a>(pub Vec<&'a str>);

    pub fn msid_semantic_line(input: &str) -> IResult<&str, MsidSemantic> {
        attribute("msid-semantic", msid_semantic)(input)
    }

    pub fn msid_semantic(input: &str) -> IResult<&str, MsidSemantic> {
        wsf(map(space_separated_strings, MsidSemantic))(input)
    }

    #[test]
    fn test_msid_semantic_line() {
        assert_line!(
            msid_semantic_line,
            "a=msid-semantic: WMS lgsCFqt9kN2fVKw5wg3NKqGdATQoltEwOdMS",
            MsidSemantic(vec!["WMS", "lgsCFqt9kN2fVKw5wg3NKqGdATQoltEwOdMS"])
        );
        assert_line_print!(
            msid_semantic_line,
            "a=msid-semantic:WMS lgsCFqt9kN2fVKw5wg3NKqGdATQoltEwOdMS"
        );
    }

    #[derive(Debug, PartialEq)]
    pub struct Msid<'a>(pub Vec<&'a str>);

    pub fn msid_line(input: &str) -> IResult<&str, Msid> {
        attribute("msid", msid)(input)
    }

    pub fn msid(input: &str) -> IResult<&str, Msid> {
        wsf(map(space_separated_strings, Msid))(input)
    }

    #[test]
    fn test_msid_line() {
        assert_line!(
            msid_line,
            "a=msid:47017fee-b6c1-4162-929c-a25110252400 f83006c5-a0ff-4e0a-9ed9-d3e6747be7d9",
            Msid(vec![
                "47017fee-b6c1-4162-929c-a25110252400",
                "f83006c5-a0ff-4e0a-9ed9-d3e6747be7d9"
            ]),
            print
        );
        assert_line_print!(
            msid_line,
            "a=msid:61317484-2ed4-49d7-9eb7-1414322a7aae f30bdb4a-5db8-49b5-bcdc-e0c9a23172e0"
        );
    }
}
