//! # Nom based SDP parser
//!
//!
//! ## Implementation status:
//! * ☒️ [Protocol Version](https://tools.ietf.org/html/rfc4566#section-5.1) (`"v="`) [`u32`]
//! * ☒️ [Origin](https://tools.ietf.org/html/rfc4566#section-5.2) (`"o="`) [`Origin`]
//! * ☒ [Session Name](https://tools.ietf.org/html/rfc4566#section-5.3) (`"s="`) [`SessionName`]
//! * ☒ [Session Information](https://tools.ietf.org/html/rfc4566#section-5.4) (`"i="`) [`SessionInformation`]
//! * ☒ [URI](https://tools.ietf.org/html/rfc4566#section-5.5) (`"u="`) [`Uri`]
//! * ☒ [Email Address and Phone Number](https://tools.ietf.org/html/rfc4566#section-5.6) (`"e="` and `"p="`) [`EmailAddress`] [`PhoneNumber`]
//! * ☒ [Connection Data](https://tools.ietf.org/html/rfc4566#section-5.7) (`"c="`) [`Connection`]
//! * ☒ [Bandwidth](https://tools.ietf.org/html/rfc4566#section-5.8) (`"b="`) [`BandWidth`]
//! * ☒ [Timing](https://tools.ietf.org/html/rfc4566#section-5.9) (`"t="`) [`Timing`]
//! * ☐ [Repeat Times](https://tools.ietf.org/html/rfc4566#section-5.10) (`"r="`)
//! * ☐ [Time Zones](https://tools.ietf.org/html/rfc4566#section-5.11) (`"z="`)
//! * ☐ [Encryption Keys](https://tools.ietf.org/html/rfc4566#section-5.12) (`"k="`)
//! * ☐ [Attributes](https://tools.ietf.org/html/rfc4566#section-5.13) (`"a="`)
//! * ☒ [Media Descriptions](https://tools.ietf.org/html/rfc4566#section-5.14) (`"m="`) [`Media`]
//! * ☐ [SDP Attributes](https://tools.ietf.org/html/rfc4566#section-6.0)

#![allow(unused_imports)]
use nom::{branch::alt, bytes::complete::tag, combinator::map, IResult};

pub mod attributes;
pub mod candidate;
pub mod connection;
pub mod ice;
pub mod lines;
pub mod media;
pub mod origin;
mod parsers;
pub mod ssrc;
#[cfg(test)]
#[macro_use]
mod assert;

use attributes::*;
use candidate::*;
use connection::*;
use ice::*;
use lines::*;
use media::*;
use origin::*;
use ssrc::*;

#[derive(Debug)]
pub enum SdpLine<'a> {
    /// `v=0`
    Version(u32),

    /// `s=-`
    Name(SessionName<'a>),

    /// `t=0 0`
    Timing(Timing),

    /// `o=- 20518 0 IN IP4 203.0.113.1`
    Origin(Origin<'a>),

    /// `b=AS:1024`
    BandWidth(BandWidth),

    Ice(IceParameter<'a>),

    /// `candidate:1853887674 2 udp 1518280447 0.0.0.0 36768 typ srflx raddr 192.168.0.196 rport 36768 generation 0`
    Candidate(Candidate<'a>),

    /// `c=IN IP4 10.23.42.137`
    Connection(Connection),

    BundleGroup(BundleGroup<'a>),

    /// `m=video 51744 RTP/AVP 126 97 98 34 31
    Media(Media<'a>),
    Mid(Mid<'a>),
    MsidSemantic(MsidSemantic<'a>),
    Msid(Msid<'a>),
    Ssrc(Ssrc<'a>),
    Fingerprint(Fingerprint<'a>),
    Description(SessionInformation<'a>),
    Direction(Direction),
    Rtp(Rtp<'a>),
    Rtcp(Rtcp),
    Fmtp(Fmtp<'a>),
    RtcpFb(RtcpFb),
    Control(Control<'a>),
    BundleOnly,
    EoC,
    // Aline(Vec<&'a str>), // catch all, don't use
}

pub fn sdp_line(input: &str) -> IResult<&str, SdpLine> {
    alt((
        alt((
            // two levels of `alt` because it's not implemented for such large tuples
            map(version_line, SdpLine::Version),
            map(bandwidth_line, SdpLine::BandWidth),
            map(name_line, SdpLine::Name),
            map(timing_line, SdpLine::Timing),
            map(origin_line, SdpLine::Origin),
            map(bundle_group_line, SdpLine::BundleGroup),
            map(ice_parameter_line, SdpLine::Ice),
            map(candidate_line, SdpLine::Candidate),
            map(connection_line, SdpLine::Connection),
            map(mid_line, SdpLine::Mid),
            map(msid_semantic_line, SdpLine::MsidSemantic),
            map(msid_line, SdpLine::Msid),
            map(media_line, SdpLine::Media),
            map(ssrc_line, SdpLine::Ssrc),
            map(fingerprint_line, SdpLine::Fingerprint),
            map(direction_line, SdpLine::Direction),
            map(description_line, SdpLine::Description),
        )),
        alt((
            map(rtp_attribute_line, SdpLine::Rtp),
            map(rtcp_attribute_line, SdpLine::Rtcp),
            map(fmtp_attribute_line, SdpLine::Fmtp),
            map(control_attribute_line, SdpLine::Control),
            map(rtcpfb_attribute_line, SdpLine::RtcpFb),
            map(tag("a=bundle-only"), |_| SdpLine::BundleOnly),
            map(tag("a=end-of-candidates"), |_| SdpLine::EoC),
            // map(a_line, SdpLine::Aline),
        )),
    ))(input)
}
#[cfg(test)]
#[ctor::ctor]
fn init_color_backtrace() {
    color_backtrace::install();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_by_line() {
        let jsep_sdp = include_str!("../sdp-transform/test/jsep.sdp");
        jsep_sdp
            .lines()
            .map(|line| (sdp_line(line), line))
            .for_each(|sdp_line| println!("{:?}", sdp_line));
    }

    #[test]
    fn test_version() {
        assert_eq!(version_line("v=0"), Ok(("", 0)));
        assert_eq!(version_line("v=1"), Ok(("", 1)))
    }

    #[test]
    #[ignore = "still red"]
    fn anatomy() {
        //! every exaple from https://webrtchacks.com/sdp-anatomy/

        let anatomy_examples = [
            // Global Lines
            "o=- 4611731400430051336 2 IN IP4 127.0.0.1",
            "s=-",
            "t=0 0",
            "a=group:BUNDLE 0 1",
            "a=msid-semantic: WMS lgsCFqt9kN2fVKw5wg3NKqGdATQoltEwOdMS",
            "m=audio 58779 UDP/TLS/RTP/SAVPF 111 103 104 9 0 8 106 105 13 126",
            "c=IN IP4 217.130.243.155",
            "a=rtcp:51472 IN IP4 217.130.243.155",

            // Audio Lines
            "a=candidate:1467250027 1 udp 2122260223 192.168.0.196 46243 typ host generation 0",
            "a=candidate:1467250027 2 udp 2122260222 192.168.0.196 56280 typ host generation 0",
            "a=candidate:435653019 1 tcp 1845501695 192.168.0.196 0 typ host tcptype active generation 0",
            "a=candidate:435653019 2 tcp 1845501695 192.168.0.196 0 typ host tcptype active generation 0",
            "a=candidate:1853887674 1 udp 1518280447 47.61.61.61 36768 typ srflx raddr 192.168.0.196 rport 36768 generation 0",
            "a=candidate:1853887674 2 udp 1518280447 47.61.61.61 36768 typ srflx raddr 192.168.0.196 rport 36768 generation 0",
            "a=candidate:750991856 2 udp 25108222 237.30.30.30 51472 typ relay raddr 47.61.61.61 rport 54763 generation 0",
            "a=candidate:750991856 1 udp 25108223 237.30.30.30 58779 typ relay raddr 47.61.61.61 rport 54761 generation 0",

            // ICE Parameters
            "a=ice-ufrag:Oyef7uvBlwafI3hT",
            "a=ice-pwd:T0teqPLNQQOf+5W+ls+P2p16",

            // DTLS Parameters
            "a=fingerprint:sha-256 49:66:12:17:0D:1C:91:AE:57:4C:C6:36:DD:D5:97:D2:7D:62:C9:9A:7F:B9:A3:F4:70:03:E7:43:91:73:23:5E",
            "a=setup:actpass",
            "a=mid:0",
            "a=extmap:1 urn:ietf:params:rtp-hdrext:ssrc-audio-level",
            "a=extmap:3 http://www.webrtc.org/experiments/rtp-hdrext/abs-send-time",
            "a=sendrecv",
            "a=rtcp-mux",

            // Codec Parameters
            "a=rtpmap:111 opus/48000/2",
            "a=fmtp:111 minptime=10; useinbandfec=1",
            "a=rtpmap:103 ISAC/16000",
            "a=rtpmap:104 ISAC/32000",
            "a=rtpmap:9 G722/8000",
            "a=rtpmap:0 PCMU/8000",
            "a=rtpmap:8 PCMA/8000",
            "a=rtpmap:106 CN/32000",
            "a=rtpmap:105 CN/16000",
            "a=rtpmap:13 CN/8000",
            "a=rtpmap:126 telephone-event/8000",
            "a=maxptime:60",

            //SSRC Parameters
            "a=ssrc:3570614608 cname:4TOk42mSjXCkVIa6",
            "a=ssrc:3570614608 msid:lgsCFqt9kN2fVKw5wg3NKqGdATQoltEwOdMS 35429d94-5637-4686-9ecd-7d0622261ce8",
            "a=ssrc:3570614608 mslabel:lgsCFqt9kN2fVKw5wg3NKqGdATQoltEwOdMS",
            "a=ssrc:3570614608 label:35429d94-5637-4686-9ecd-7d0622261ce8",

            // Video Lines
            "m=video 60372 UDP/TLS/RTP/SAVPF 100 101 116 117 96",
            "c=IN IP4 217.130.243.155",
            "a=rtcp:64891 IN IP4 217.130.243.155",

            // ICE Candidates
            "a=candidate:1467250027 1 udp 2122260223 192.168.0.196 56143 typ host generation 0",
            "a=candidate:1467250027 2 udp 2122260222 192.168.0.196 58874 typ host generation 0",
            "a=candidate:435653019 1 tcp 1518280447 192.168.0.196 0 typ host tcptype active generation 0",
            "a=candidate:435653019 2 tcp 1518280446 192.168.0.196 0 typ host tcptype active generation 0",
            "a=candidate:1853887674 1 udp 1518280447 47.61.61.61 36768 typ srflx raddr 192.168.0.196 rport 36768 generation 0",
            "a=candidate:1853887674 1 udp 1518280447 47.61.61.61 36768 typ srflx raddr 192.168.0.196 rport 36768 generation 0",
            "a=candidate:750991856 1 udp 25108223 237.30.30.30 60372 typ relay raddr 47.61.61.61 rport 54765 generation 0",
            "a=candidate:750991856 2 udp 25108222 237.30.30.30 64891 typ relay raddr 47.61.61.61 rport 54767 generation 0",

            // ICE Parameters
            "a=ice-ufrag:Oyef7uvBlwafI3hT",
            "a=ice-pwd:T0teqPLNQQOf+5W+ls+P2p16",

            // DTLS Parameters
            "a=fingerprint:sha-256 49:66:12:17:0D:1C:91:AE:57:4C:C6:36:DD:D5:97:D2:7D:62:C9:9A:7F:B9:A3:F4:70:03:E7:43:91:73:23:5E",
            "a=setup:actpass",
            "a=mid:1",
            "a=extmap:2 urn:ietf:params:rtp-hdrext:toffset",
            "a=extmap:3 http://www.webrtc.org/experiments/rtp-hdrext/abs-send-time",
            "a=extmap:4 urn:3gpp:video-orientation",
            "a=sendrecv",
            "a=rtcp-mux",

            // Codec Parameters
            "a=rtpmap:100 VP8/90000",
            "a=rtcp-fb:100 ccm fir",
            "a=rtcp-fb:100 nack",
            "a=rtcp-fb:100 nack pli",
            "a=rtcp-fb:100 goog-remb",
            "a=rtpmap:101 VP9/90000",
            "a=rtcp-fb:101 ccm fir",
            "a=rtcp-fb:101 nack",
            "a=rtcp-fb:101 nack pli",
            "a=rtcp-fb:101 goog-remb",
            "a=rtpmap:116 red/90000",
            "a=rtpmap:117 ulpfec/90000",
            "a=rtpmap:96 rtx/90000",
            "a=fmtp:96 apt=100",

            // SSRC Parameters
            "a=ssrc-group:FID 2231627014 632943048",
            "a=ssrc:2231627014 cname:4TOk42mSjXCkVIa6",
            "a=ssrc:2231627014 msid:lgsCFqt9kN2fVKw5wg3NKqGdATQoltEwOdMS daed9400-d0dd-4db3-b949-422499e96e2d",
            "a=ssrc:2231627014 mslabel:lgsCFqt9kN2fVKw5wg3NKqGdATQoltEwOdMS",
            "a=ssrc:2231627014 label:daed9400-d0dd-4db3-b949-422499e96e2d",
            "a=ssrc:632943048 cname:4TOk42mSjXCkVIa6",
            "a=ssrc:632943048 msid:lgsCFqt9kN2fVKw5wg3NKqGdATQoltEwOdMS daed9400-d0dd-4db3-b949-422499e96e2d",
        ];
        for (i, line) in anatomy_examples.iter().enumerate() {
            print!("{}.", i);
            assert_line!(line);
        }
    }
}
