use derive_into_owned::IntoOwned;

use crate::{
    attributes::AttributeLine,
    lines::media::Media,
    session::{SdpLine, SessionLine},
};

#[derive(Clone, Debug, IntoOwned)]
pub struct MediaSection<'a> {
    pub mline: Media<'a>,
    pub lines: Vec<SdpLine<'a>>,
}

#[cfg(feature = "display")]
impl std::fmt::Display for MediaSection<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.lines {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

impl<'a> MediaSection<'a> {
    pub fn lines(&self) -> impl Iterator<Item = &SessionLine<'a>> {
        self.lines.iter().filter_map(SdpLine::as_session)
    }

    pub fn attributes(&self) -> impl Iterator<Item = &AttributeLine<'a>> {
        self.lines.iter().filter_map(SdpLine::as_attribute)
    }

    pub fn lines_where<'b, T: 'a, F: 'a>(&'a self, f: F) -> Vec<T>
    where
        F: Fn(SessionLine<'static>) -> Result<T, SessionLine<'static>>,
        T: Clone,
        'a: 'b,
    {
        self.lines()
            .map(|x| x.clone().into_owned())
            .filter_map(|x| f(x).ok())
            .collect()
    }

    pub fn attributes_where<'b, T: 'a, F: 'a>(&'a self, f: F) -> Vec<T>
    where
        F: Fn(AttributeLine<'static>) -> Result<T, AttributeLine<'static>>,
        T: Clone,
        'a: 'b,
    {
        self.attributes()
            .map(|x| x.clone().into_owned())
            .filter_map(|x| f(x).ok())
            .collect()
    }
}
