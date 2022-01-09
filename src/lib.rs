//! # Nom based SDP parser
//!
//!
//! ## Implementation status:
//! * [x] [Protocol Version](https://tools.ietf.org/html/rfc4566#section-5.1) (`"v="`) [`u32`]
//! * [x] [Origin](https://tools.ietf.org/html/rfc4566#section-5.2) (`"o="`) [`Origin`][`crate::lines::origin::Origin`]
//! * [x] [Session Name](https://tools.ietf.org/html/rfc4566#section-5.3) (`"s="`) [`SessionName`][`crate::lines::session_name::SessionName`]
//! * [x] [Session Information](https://tools.ietf.org/html/rfc4566#section-5.4) (`"i="`) [`SessionInformation`][`crate::lines::session_information::SessionInformation`]
//! * [x] [URI](https://tools.ietf.org/html/rfc4566#section-5.5) (`"u="`) [`Uri`][`crate::lines::uri::Uri`]
//! * [x] [Email Address and Phone Number](https://tools.ietf.org/html/rfc4566#section-5.6) (`"e="` and `"p="`) [`EmailAddress`][`crate::lines::email::EmailAddress`] [`PhoneNumber`][`crate::lines::phone_number::PhoneNumber`]
//! * [x] [Connection Data](https://tools.ietf.org/html/rfc4566#section-5.7) (`"c="`) [`Connection`][`crate::lines::connection::Connection`]
//! * [x] [Bandwidth](https://tools.ietf.org/html/rfc4566#section-5.8) (`"b="`) [`BandWidth`][`crate::lines::bandwidth::BandWidth`]
//! * [x] [Timing](https://tools.ietf.org/html/rfc4566#section-5.9) (`"t="`) [`Timing`][`crate::lines::timing::Timing`]
//! * [ ] [Repeat Times](https://tools.ietf.org/html/rfc4566#section-5.10) (`"r="`)
//! * [ ] [Time Zones](https://tools.ietf.org/html/rfc4566#section-5.11) (`"z="`)
//! * [ ] [Encryption Keys](https://tools.ietf.org/html/rfc4566#section-5.12) (`"k="`)
//! * [x] [Attributes](https://tools.ietf.org/html/rfc4566#section-5.13) (`"a="`)
//! * [x] [Media Descriptions](https://tools.ietf.org/html/rfc4566#section-5.14) (`"m="`) [`Media`][`crate::lines::media::Media`]
//! * [ ] [SDP Attributes](https://tools.ietf.org/html/rfc4566#section-6.0)

#![deny(
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    clippy::clone_on_copy
)]
// #![warn(missing_docs)]

#[cfg_attr(feature = "wee_alloc", global_allocator)]
#[cfg(feature = "wee_alloc")]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub mod attributes;
pub mod lines;
mod sdp_line;

pub mod media_section;
pub mod session;

mod parsers;
#[cfg(test)]
mod tests;

#[cfg(test)]
#[macro_use]
mod assert;

#[cfg(feature = "ufmt")]
mod udisplay;

pub use crate::{
    sdp_line::{sdp_line, SdpLine},
    session::Session,
};

#[cfg(feature = "ufmt")]
pub use crate::session::ufmt_to_string;

pub fn sdp_lines(sdp: &str) -> impl Iterator<Item = SdpLine<'_>> {
    sdp.lines().filter_map(|line| match sdp_line(line) {
        Ok((_, parsed)) => Some(parsed),
        Err(_) => None,
    })
}

pub fn sdp_lines_all(sdp: &str) -> impl Iterator<Item = Result<(&str, SdpLine<'_>), String>> {
    sdp.lines()
        .map(|l| nom::Finish::finish(sdp_line(l)).map_err(|e| e.to_string()))
}
