use derive_into_owned::IntoOwned;

use crate::{
    lines::SessionLine,
    sdp_line::{sdp_line, SdpLine},
    LazyMediaSection,
};

#[derive(Debug, Default, IntoOwned)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct LazySession<'a> {
    pub lines: Vec<SdpLine<'a>>,
    pub media: Vec<LazyMediaSection<'a>>,
}

#[cfg(feature = "display")]
impl std::fmt::Display for LazySession<'_> {
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
impl std::string::ToString for LazySession<'_> {
    fn to_string(&self) -> String {
        let mut output = String::new();
        ufmt::uwrite!(output, "{}", self).unwrap();
        output
    }
}

type ParseError<'a> = nom::Err<nom::error::Error<&'a str>>;

#[derive(Debug, Default)]
struct ParserState<'a> {
    current_msecion: Option<LazyMediaSection<'a>>,
    lines: Vec<SdpLine<'a>>,
    media: Vec<LazyMediaSection<'a>>,
    failed: Option<nom::Err<nom::error::Error<&'a str>>>,
}

impl<'a> std::convert::TryFrom<&'a String> for LazySession<'a> {
    type Error = ParseError<'a>;

    fn try_from(sdp: &'a String) -> Result<LazySession<'a>, Self::Error> {
        LazySession::try_from(sdp.as_str(), true)
    }
}

impl<'a> std::convert::TryFrom<&'a str> for LazySession<'a> {
    type Error = ParseError<'a>;

    fn try_from(sdp: &'a str) -> Result<LazySession<'a>, Self::Error> {
        LazySession::try_from(sdp, true)
    }
}

impl<'a> LazySession<'a> {
    fn try_from(sdp: &'a str, fallible: bool) -> Result<LazySession<'a>, ParseError<'a>> {
        let mut state = {
            sdp.lines().fold(ParserState::default(), |mut state, line| {
                if state.failed.is_some() {
                    return state;
                }
                match sdp_line(line) {
                    Ok((_, parsed)) => {
                        if let SdpLine::Session(SessionLine::Media(mline)) = parsed {
                            if let Some(m) = state.current_msecion.take() {
                                state.media.push(m);
                            }
                            let new_m_section = LazyMediaSection {
                                mline,
                                lines: Default::default(),
                            };
                            state.current_msecion = Some(new_m_section);
                        } else if let Some(ref mut msection) = state.current_msecion {
                            msection.lines.push(parsed);
                        } else {
                            state.lines.push(parsed);
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
        if let Some(m) = state.current_msecion.take() {
            state.media.push(m);
        }
        Ok(LazySession {
            media: state.media,
            lines: state.lines,
        })
    }

    pub fn read_str(sdp: &'a str) -> LazySession<'a> {
        Self::try_from(sdp, false).expect("unfallible should mean this never unwraps")
    }
}
