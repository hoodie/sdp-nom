use derive_into_owned::IntoOwned;

use crate::{
    attributes::AttributeLine,
    lines::{
        bandwidth::BandWidth, connection::Connection, email::EmailAddress, origin::Origin,
        phone_number::PhoneNumber, session_information::SessionInformation,
        session_name::SessionName, timing::Timing, uri::Uri, version::Version, SessionLine,
    },
    media_section::MediaSection,
    sdp_line, SdpLine,
};

#[derive(Debug, Default, IntoOwned, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct Session<'a> {
    /// `v=0`
    pub version: Option<Version>,

    /// `s=-`
    pub name: Option<SessionName<'a>>,

    /// `t=0 0`
    pub timing: Option<Timing>,

    /// `o=- 20518 0 IN IP4 203.0.113.1`
    pub origin: Option<Origin<'a>>,

    /// `b=AS:1024`
    pub band_width: Option<BandWidth>,

    /// `u=`
    pub uri: Option<Uri<'a>>,

    /// `p=0118 999 881 999 119 7253`
    pub phone_number: Option<PhoneNumber<'a>>,

    /// "e=email@example.com"
    pub email_address: Option<EmailAddress<'a>>,

    /// `c=IN IP4 10.23.42.137`
    pub connection: Option<Connection>,

    pub description: Option<SessionInformation<'a>>,

    pub attributes: Vec<AttributeLine<'a>>,
    pub media: Vec<MediaSection<'a>>,
}

type ParseError<'a> = nom::Err<nom::error::Error<&'a str>>;

#[derive(Debug, Default)]
struct ParserState<'a> {
    session: Session<'a>,
    current_msection: Option<MediaSection<'a>>,
    failed: Option<nom::Err<nom::error::Error<&'a str>>>,
}

impl<'a> Session<'a> {
    fn add_line(&mut self, line: SdpLine<'a>) {
        use SessionLine::*;
        match line {
            //crate::SdpLine::Session(Session)       => todo!(),
            SdpLine::Session(Version(version)) => self.version = Some(version),
            SdpLine::Session(Name(session_name)) => self.name = Some(session_name),
            SdpLine::Session(Timing(timing)) => self.timing = Some(timing),
            SdpLine::Session(Origin(origin)) => self.origin = Some(origin),
            SdpLine::Session(BandWidth(bw)) => self.band_width = Some(bw),
            SdpLine::Session(Uri(uri)) => self.uri = Some(uri),
            SdpLine::Session(PhoneNumber(phone)) => self.phone_number = Some(phone),
            SdpLine::Session(EmailAddress(email)) => self.email_address = Some(email),
            SdpLine::Session(Connection(connection)) => self.connection = Some(connection),
            SdpLine::Session(Description(info)) => self.description = Some(info),
            SdpLine::Session(Media(_)) => unreachable!(),
            SdpLine::Attribute(a) => self.attributes.push(a),
            SdpLine::Comment(_) => {}
        }
    }

    fn try_from(sdp: &'a str, fallible: bool) -> Result<Session<'a>, ParseError<'a>> {
        let mut state = {
            sdp.lines().fold(ParserState::default(), |mut state, line| {
                if state.failed.is_some() {
                    return state;
                }
                match sdp_line(line) {
                    Ok((_, parsed)) => {
                        if let SdpLine::Session(SessionLine::Media(mline)) = parsed {
                            if let Some(m) = state.current_msection.take() {
                                state.session.media.push(m);
                            }
                            let new_m_section = MediaSection::from(mline);
                            state.current_msection = Some(new_m_section);
                        } else if let Some(ref mut msection) = state.current_msection {
                            msection.add_line(parsed);
                        } else {
                            state.session.add_line(parsed);
                        }
                    }
                    Err(e) => {
                        if fallible {
                            state.failed = Some(e)
                        }
                    }
                }
                state
            })
        };

        if let Some(err) = state.failed {
            return Err(err);
        }
        if let Some(m) = state.current_msection.take() {
            state.session.media.push(m);
        }
        Ok(state.session)
    }

    pub fn read_str(sdp: &'a str) -> Session<'a> {
        Self::try_from(sdp, false).expect("unfallible should mean this never unwraps")
    }

    pub fn modify_media<F>(mut self, f: F) -> Self
    where
        F: Fn(MediaSection) -> MediaSection,
    {
        self.media = self.media.into_iter().map(f).collect();
        self
    }
}

#[cfg(all(feature = "udisplay", not(feature = "display")))]
impl std::string::ToString for Session<'_> {
    fn to_string(&self) -> String {
        let mut output = String::new();
        ufmt::uwrite!(output, "{}", self).unwrap();
        output
    }
}

#[cfg(all(feature = "udisplay", not(feature = "display")))]
pub fn ufmt_to_string<U: ufmt::uDisplay>(stuff: &U) -> String {
    let mut output = String::new();
    ufmt::uwrite!(output, "{}", stuff).unwrap();
    output
}
