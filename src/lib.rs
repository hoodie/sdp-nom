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

#![deny(
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    // unused_qualifications
)]
// #![warn(missing_docs)]

use nom::{branch::alt, bytes::complete::tag, combinator::map, IResult};

#[cfg_attr(feature="wee_alloc", global_allocator)]
#[cfg(feature="wee_alloc")]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub mod attributes;
pub mod lines;
mod parsers;
#[cfg(test)]
mod tests;

#[cfg(test)]
#[macro_use]
mod assert;
#[cfg(any(feature = "display", test))]
mod display;

#[cfg(feature = "ufmt")]
mod udisplay;

use lines::{
    bandwidth::*, connection::*, email::*, media::*, origin::*, phone_number::*,
    session_information::*, session_name::*, timing::*, uri::*, version::*,
};

/// Sdp Line
#[derive(Debug)]
pub enum SdpLine<'a> {
    Session(SessionLine<'a>),
    Attribute(AttributeLine<'a>),
    Comment(&'a str),
}

/// Session Line
#[derive(Debug)]
pub enum SessionLine<'a> {
    /// `v=0`
    Version(Version),

    /// `s=-`
    Name(SessionName<'a>),

    /// `t=0 0`
    Timing(Timing),

    /// `o=- 20518 0 IN IP4 203.0.113.1`
    Origin(Origin<'a>),

    /// `b=AS:1024`
    BandWidth(BandWidth),

    /// `u=`
    Uri(Uri<'a>),

    /// `p=0118 999 881 999 119 7253`
    PhoneNumber(PhoneNumber<'a>),

    /// "e=email@example.com"
    EmailAddress(EmailAddress<'a>),

    /// `c=IN IP4 10.23.42.137`
    Connection(Connection),

    Description(SessionInformation<'a>),

    /// `m=video 51744 RTP/AVP 126 97 98 34 31
    Media(Media<'a>),
}

#[derive(Debug)]
pub enum AttributeLine<'a> {
    /// `a=candidate:1853887674 2 udp 1518280447 0.0.0.0 36768 typ srflx raddr 192.168.0.196 rport 36768 generation 0`
    Candidate(attributes::Candidate<'a>),
    Ice(attributes::IceParameter<'a>),
    Mid(attributes::mid::Mid<'a>),
    MsidSemantic(attributes::msid::MsidSemantic<'a>),
    Msid(attributes::msid::Msid<'a>),
    RtpMap(attributes::rtpmap::RtpMap<'a>),
    PTime(attributes::rtpmap::PTime),
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
    SetupRole(attributes::dtls::SetupRole),
    Extmap(attributes::extmap::Extmap<'a>),
    BundleOnly,
    EoC,
    Attribute {
        key: &'a str,
        val: &'a str,
    },
}

pub fn sdp_line(input: &str) -> IResult<&str, SdpLine> {
    alt((
        map(session_line, SdpLine::Session),
        map(attribute_line, SdpLine::Attribute),
        map(lines::comment::comment_line, SdpLine::Comment),
    ))(input)
}

fn session_line(input: &str) -> IResult<&str, SessionLine> {
    alt((
        // two levels of `alt` because it's not implemented for such large tuples
        map(version_line, SessionLine::Version),
        map(name_line, SessionLine::Name),
        map(description_line, SessionLine::Description),
        map(bandwidth_line, SessionLine::BandWidth),
        map(uri_line, SessionLine::Uri),
        map(timing_line, SessionLine::Timing),
        map(phone_number_line, SessionLine::PhoneNumber),
        map(email_address_line, SessionLine::EmailAddress),
        map(origin_line, SessionLine::Origin),
        map(connection_line, SessionLine::Connection),
        map(media_line, SessionLine::Media),
    ))(input)
}

pub fn attribute_line_lazy(input: &str) -> IResult<&str, AttributeLine> {
    map(attributes::generic::lazy_attribute_line, |(key, val)| {
        AttributeLine::Attribute { key, val }
    })(input)
}

pub fn attribute_line(input: &str) -> IResult<&str, AttributeLine> {
    use attributes::*;
    use fingerprint::fingerprint_line;
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
            map(attributes::generic::lazy_attribute_line, |(key, val)| {
                AttributeLine::Attribute { key, val }
            }),
            map(tag("a=bundle-only"), |_| AttributeLine::BundleOnly),
            map(tag("a=end-of-candidates"), |_| AttributeLine::EoC),
        )),
    ))(input)
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

#[cfg(all(feature = "display", feature = "udisplay"))]
compile_error!("The features \"display\" and \"udisplay\" can not be enabled together.");

#[cfg(feature = "display")]
impl std::fmt::Display for MediaSection<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.lines {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

#[cfg(feature = "display")]
impl std::fmt::Display for EagerSession<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.lines {
            writeln!(f, "{}", line)?;
        }
        for msection in &self.media {
            write!(f, "{}", msection)?;
        }
        Ok(())
    }
}

#[cfg(feature = "udisplay")]
impl std::string::ToString for EagerSession<'_> {
    fn to_string(&self) -> String {
        let mut output = String::new();
        ufmt::uwrite!(output, "{}", self).unwrap();
        output
    }
}

type ParseError<'a> = nom::Err<nom::error::Error<&'a str>>;

#[derive(Debug, Default)]
struct ParserState<'a> {
    current_msecion: Option<MediaSection<'a>>,
    lines: Vec<SdpLine<'a>>,
    media: Vec<MediaSection<'a>>,
    failed: Option<nom::Err<nom::error::Error<&'a str>>>,
}

#[cfg(feature = "udisplay")]
impl std::string::ToString for EagerSession<'_> {
    fn to_string(&self) -> String {
        let mut output = String::new();
        ufmt::uwrite!(output, "{}", self).unwrap();
        output
    }
}

impl<'a> std::convert::TryFrom<&'a String> for EagerSession<'a> {
    type Error = ParseError<'a>;

    fn try_from(sdp: &'a String) -> Result<EagerSession<'a>, Self::Error> {
        EagerSession::try_from(sdp.as_str())
    }
}

impl<'a> std::convert::TryFrom<&'a str> for EagerSession<'a> {
    type Error = ParseError<'a>;

    fn try_from(sdp: &'a str) -> Result<EagerSession<'a>, Self::Error> {
        let mut state = {
            sdp.lines().fold(ParserState::default(), |mut state, line| {
                if state.failed.is_some() {
                    return state;
                }
                match sdp_line(&line) {
                    Ok((_, parsed)) => {
                        if matches!(parsed, SdpLine::Session(SessionLine::Media(_))) {
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
                    Err(e) => state.failed = Some(e),
                }
                state
            })
        };

        if let Some(err) = state.failed {
            return Err(err);
        }
        if let Some(m) = state.current_msecion.take() {
            state.media.push(m);
        }
        Ok(EagerSession {
            media: state.media,
            lines: state.lines,
        })
    }
}

impl<'a> EagerSession<'a> {
    pub fn read_str(sdp: &'a str) -> EagerSession<'a> {
        let mut state = {
            sdp.lines().fold(ParserState::default(), |mut state, line| {
                if let Ok((_, parsed)) = sdp_line(&line) {
                    if matches!(parsed, SdpLine::Session(SessionLine::Media(_))) {
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
