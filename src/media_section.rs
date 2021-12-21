use derive_into_owned::IntoOwned;

use crate::SdpLine;

#[derive(Debug, Default, IntoOwned)]
pub struct MediaSection<'a> {
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
