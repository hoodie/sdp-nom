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

use std::fmt::Display;

use nom::{branch::alt, bytes::complete::tag, combinator::map, IResult};

pub mod attributes;
pub mod lines;
mod parsers;
#[cfg(test)]
mod tests;

#[cfg(test)]
#[macro_use]
mod assert;

use lines::{
    bandwidth::*, connection::*, email::*, media::*, origin::*, phone_number::*,
    session_information::*, session_name::*, timing::*, uri::*, version::*,
};

/// Sdp Line
#[derive(Debug)]
pub enum SdpLine<'a> {
    Session(SessionLine<'a>),
    Attribute(AttributeLine<'a>),
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

impl Display for SdpLine<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SdpLine::Session(session) => write!(f, "{}", session),
            SdpLine::Attribute(attribute) => write!(f, "{}", attribute),
        }
    }
}

impl Display for SessionLine<'_> {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionLine::Version(v)      => write!(f,"{}", v),
            SessionLine::Name(n)         => write!(f,"{}", n),
            SessionLine::Timing(t)       => write!(f,"{}", t),
            SessionLine::Origin(o)       => write!(f,"{}", o),
            SessionLine::BandWidth(b)    => write!(f,"{}", b),
            SessionLine::Uri(u)          => write!(f,"{}", u),
            SessionLine::PhoneNumber(p)  => write!(f,"{}", p),
            SessionLine::EmailAddress(e) => write!(f,"{}", e),
            SessionLine::Connection(c)   => write!(f,"{}", c),
            SessionLine::Description(d)  => write!(f,"{}", d),
            SessionLine::Media(m)        => write!(f,"{}", m),
        }
    }
}

impl Display for AttributeLine<'_> {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AttributeLine::Candidate(c)    => write!(f, "{}", c),
            AttributeLine::Ice(i)          => write!(f, "{}", i),
            AttributeLine::Mid(m)          => write!(f, "{}", m),
            AttributeLine::MsidSemantic(ms) => write!(f, "{}", ms),
            AttributeLine::Msid(m)         => write!(f, "{}", m),
            AttributeLine::RtpMap(r)       => write!(f, "{}", r),
            AttributeLine::PTime(p)        => write!(f, "{}", p),
            AttributeLine::Ssrc(s)         => write!(f, "{}", s),
            AttributeLine::BundleGroup(b)  => write!(f, "{}", b),
            AttributeLine::SsrcGroup(s)    => write!(f, "{}", s),
            AttributeLine::Fingerprint(fp) => write!(f, "{}", fp),
            AttributeLine::Direction(d)    => write!(f, "{}", d),
            AttributeLine::Rtp(r)          => write!(f, "{}", r),
            AttributeLine::Rtcp(r)         => write!(f, "{}", r),
            AttributeLine::Fmtp(fmtp)      => write!(f, "{}", fmtp),
            AttributeLine::RtcpFb(r)       => write!(f, "{}", r),
            AttributeLine::RtcpOption(r)   => write!(f, "{}", r),
            AttributeLine::Control(c)      => write!(f, "{}", c),
            AttributeLine::SetupRole(s)    => write!(f, "{}", s),
            AttributeLine::Extmap(e)       => write!(f, "{}", e),
            AttributeLine::BundleOnly      => write!(f, "a=bundle-only"),
            AttributeLine::EoC             => write!(f, "a=end-of-candidates"),
            AttributeLine::Attribute {
                key,
                val
            }                              => write!(f, "a={}:{}", key, val),
        }
    }
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
            map(attributes::generic::lazy_attribute_line,|(key,val)| AttributeLine::Attribute{key,val}),
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

impl std::fmt::Display for MediaSection<'_>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.lines {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for EagerSession<'_>{
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
