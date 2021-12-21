use derive_into_owned::IntoOwned;
use nom::{branch::alt, combinator::map, IResult};
use std::borrow::Cow;

use crate::{
    attributes::{attribute_line, AttributeLine},
    lines,
    media_section::MediaSection,
    parsers::cowify,
};
use lines::{
    bandwidth::*, connection::*, email::*, media::*, origin::*, phone_number::*,
    session_information::*, session_name::*, timing::*, uri::*, version::*,
};

/// Sdp Line
#[derive(Debug, IntoOwned)]
#[non_exhaustive]
pub enum SdpLine<'a> {
    Session(SessionLine<'a>),
    Attribute(AttributeLine<'a>),
    Comment(Cow<'a, str>),
}

pub fn sdp_line(input: &str) -> IResult<&str, SdpLine> {
    alt((
        map(session_line, SdpLine::Session),
        map(attribute_line, SdpLine::Attribute),
        map(cowify(lines::comment::comment_line), SdpLine::Comment),
    ))(input)
}

/// Session Line
#[derive(Debug, IntoOwned)]
#[non_exhaustive]
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

/**********/

#[derive(Debug, Default, IntoOwned)]
pub struct Session<'a> {
    pub lines: Vec<SdpLine<'a>>,
    pub media: Vec<MediaSection<'a>>,
}

#[cfg(feature = "display")]
impl std::fmt::Display for Session<'_> {
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
impl std::string::ToString for Session<'_> {
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

impl<'a> std::convert::TryFrom<&'a String> for Session<'a> {
    type Error = ParseError<'a>;

    fn try_from(sdp: &'a String) -> Result<Session<'a>, Self::Error> {
        Session::try_from(sdp.as_str())
    }
}

impl<'a> std::convert::TryFrom<&'a str> for Session<'a> {
    type Error = ParseError<'a>;

    fn try_from(sdp: &'a str) -> Result<Session<'a>, Self::Error> {
        let mut state = {
            sdp.lines().fold(ParserState::default(), |mut state, line| {
                if state.failed.is_some() {
                    return state;
                }
                match sdp_line(line) {
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
        Ok(Session {
            media: state.media,
            lines: state.lines,
        })
    }
}

impl<'a> Session<'a> {
    pub fn read_str(sdp: &'a str) -> Session<'a> {
        let mut state = {
            sdp.lines().fold(ParserState::default(), |mut state, line| {
                if let Ok((_, parsed)) = sdp_line(line) {
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
        Session {
            media: state.media,
            lines: state.lines,
        }
    }
}
