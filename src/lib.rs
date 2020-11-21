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
pub mod connection;
pub mod lines;
pub mod media;
pub mod origin;
mod parsers;
#[cfg(test)]
#[macro_use]
mod assert;

use connection::*;
use lines::*;
use media::*;
use origin::*;

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

    Ice(attributes::IceParameter<'a>),

    /// `candidate:1853887674 2 udp 1518280447 0.0.0.0 36768 typ srflx raddr 192.168.0.196 rport 36768 generation 0`
    Candidate(attributes::Candidate<'a>),

    /// `c=IN IP4 10.23.42.137`
    Connection(Connection),

    Description(SessionInformation<'a>),

    /// `m=video 51744 RTP/AVP 126 97 98 34 31
    Media(Media<'a>),
    Mid(Mid<'a>),
    MsidSemantic(MsidSemantic<'a>),
    Msid(Msid<'a>),
    RtpMap(attributes::codec::RtpMap<'a>),
    PTime(attributes::codec::PTime),

    Ssrc(attributes::Ssrc<'a>),
    BundleGroup(attributes::BundleGroup<'a>),
    SsrcGroup(attributes::SsrcGroup),
    Fingerprint(attributes::Fingerprint<'a>),
    Direction(attributes::Direction),
    Rtp(attributes::Rtp<'a>),
    Rtcp(attributes::rtcp::Rtcp),
    Fmtp(attributes::Fmtp<'a>),
    RtcpFb(attributes::rtcp::Fb<'a>),
    RtcpOption(attributes::RtcpOption),
    Control(attributes::Control<'a>),
    SetupRole(attributes::dtls_parameters::SetupRole),
    Extmap(attributes::extmap::Extmap<'a>),
    BundleOnly,
    EoC,
    // Aline(Vec<&'a str>), // catch all, don't use
}

pub fn sdp_line(input: &str) -> IResult<&str, SdpLine> {
    alt((
        alt((
            // two levels of `alt` because it's not implemented for such large tuples
            map(version_line, SdpLine::Version),
            map(name_line, SdpLine::Name),
            map(description_line, SdpLine::Description),
            map(bandwidth_line, SdpLine::BandWidth),
            map(timing_line, SdpLine::Timing),
            map(origin_line, SdpLine::Origin),
            map(connection_line, SdpLine::Connection),
            map(media_line, SdpLine::Media),
            map(mid_line, SdpLine::Mid),
            map(msid_semantic_line, SdpLine::MsidSemantic),
            map(msid_line, SdpLine::Msid),
        )),
        alt((
            map(attributes::bundle_group_line, SdpLine::BundleGroup),
            map(attributes::ice_parameter_line, SdpLine::Ice),
            map(attributes::ssrc_line, SdpLine::Ssrc),
            map(attributes::ssrc_group_line, SdpLine::SsrcGroup),
            map(attributes::codec::rtpmap_line, SdpLine::RtpMap),
            map(attributes::codec::read_p_time, SdpLine::PTime),
            map(attributes::fingerprint_line, SdpLine::Fingerprint),
            map(attributes::candidate_line, SdpLine::Candidate),
            map(attributes::direction_line, SdpLine::Direction),
            map(attributes::extmap::extmap_line, SdpLine::Extmap),
            map(
                attributes::dtls_parameters::setup_role_line,
                SdpLine::SetupRole,
            ),
            map(attributes::rtp_attribute_line, SdpLine::Rtp),
            map(attributes::rtcp::rtcp_attribute_line, SdpLine::Rtcp),
            map(attributes::fmtp_attribute_line, SdpLine::Fmtp),
            map(attributes::control_attribute_line, SdpLine::Control),
            map(attributes::rtcp::rtcpfb_attribute_line, SdpLine::RtcpFb),
            map(attributes::rtp_option_line, SdpLine::RtcpOption),
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

#[derive(Debug, Default)]
pub struct MediaSection<'a> {
    pub lines: Vec<SdpLine<'a>>,
}
#[derive(Debug, Default)]
pub struct EagerSession<'a> {
    pub lines: Vec<SdpLine<'a>>,
    pub media: Vec<MediaSection<'a>>,
}

#[derive(Debug, Default)]
struct ParserState<'a> {
    current_msecion: Option<MediaSection<'a>>,
    lines: Vec<SdpLine<'a>>,
    media: Vec<MediaSection<'a>>,
}

impl<'a> EagerSession<'a> {
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(sdp: &'a str) -> EagerSession<'a> {
        let mut state = {
            sdp.lines().fold(ParserState::default(), |mut state, line| {
                match sdp_line(&line) {
                    Ok((_, parsed)) => {
                        if matches!(parsed, SdpLine::Media(_)) {
                            if let Some(m) = state.current_msecion.take() {
                                state.media.push(m);
                            }
                            let mut new_m_section = MediaSection::default();
                            new_m_section.lines.push(parsed);
                            state.current_msecion = Some(new_m_section);
                        } else if let Some(ref mut msection) = state.current_msecion {
                            msection.lines.push(parsed);
                        } else {
                            state.lines.push(parsed);
                        }
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                }
                state
            })
        };
        if let Some(m) = state.current_msecion.take() {
            state.media.push(m);
        }
        EagerSession {
            media: state.media,
            lines: state.lines,
        }
    }
}
