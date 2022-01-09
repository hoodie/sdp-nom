use std::borrow::Cow;

use derive_into_owned::IntoOwned;

use crate::{
    attributes::{
        candidate, dtls, extmap, ice::IceParameter, msid, rtcp, rtpmap, AttributeLine, BundleGroup,
        Control, Direction, Fingerprint, Fmtp, Ice, RtcpOption, Rtp, Ssrc, SsrcGroup,
    },
    lazy_media_section::LazyMediaSection,
    lines::{connection::Connection, media::Media, SessionLine},
    SdpLine,
};

#[derive(Debug, Default, IntoOwned)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct MediaSection<'a> {
    pub r#type: Cow<'a, str>,
    pub port: u32,
    pub protocol: Vec<Cow<'a, str>>,
    pub payloads: Vec<Cow<'a, str>>,

    pub connection: Option<Connection>,
    pub candidates: Vec<candidate::Candidate<'a>>,
    // pub ice: Vec<ice::IceParameter<'a>>,
    pub ice: Ice<'a>,
    pub mid: Cow<'a, str>,
    pub msid_semantic: Option<msid::MsidSemantic<'a>>,
    pub msid: Option<msid::Msid<'a>>,
    pub rtp_map: Vec<rtpmap::RtpMap<'a>>,
    pub p_time: Option<rtpmap::PTime>,
    pub ssrc: Vec<Ssrc<'a>>,
    pub bundle_group: Option<BundleGroup<'a>>,
    pub bundle_only: bool,
    pub ssrc_group: Option<SsrcGroup>,
    pub fingerprint: Option<Fingerprint<'a>>,
    pub direction: Option<Direction>,
    pub rtp: Option<Rtp<'a>>,
    pub rtcp: Option<rtcp::Rtcp>,
    pub fmtp: Vec<Fmtp<'a>>,
    pub rtcp_fb: Vec<rtcp::Fb<'a>>,
    pub rtcp_option: Vec<RtcpOption>,
    pub control: Option<Control<'a>>,
    pub setup_role: Option<dtls::SetupRole>,
    pub extmap: Vec<extmap::Extmap<'a>>,

    pub attributes: Vec<AttributeLine<'a>>,
}

impl<'a> MediaSection<'a> {
    pub fn media(&self) -> Media<'a> {
        Media {
            r#type: self.r#type.clone(),
            port: self.port.clone(),
            protocol: self.protocol.clone(),
            payloads: self.payloads.clone(),
        }
    }
    pub(crate) fn add_line(&mut self, line: SdpLine<'a>) {
        use AttributeLine::*;
        use SessionLine::*;
        match line {
            SdpLine::Session(Media(_)) => unreachable!(),
            SdpLine::Session(SessionLine::Connection(connection)) => {
                assert!(self.connection.replace(connection).is_none())
            }
            SdpLine::Session(session) => println!("🔥 {:#?}", session),

            SdpLine::Attribute(Candidate(candidate)) => self.candidates.push(candidate),
            SdpLine::Attribute(Ice(IceParameter::Options(o))) => self.ice.options = Some(o),
            SdpLine::Attribute(Ice(IceParameter::Ufrag(o))) => self.ice.ufrag = Some(o),
            SdpLine::Attribute(Ice(IceParameter::Pwd(o))) => self.ice.pwd = Some(o),
            SdpLine::Attribute(attr @ Ice(_)) => self.attributes.push(attr),
            SdpLine::Attribute(Mid(mid)) => self.mid = mid.0,
            SdpLine::Attribute(MsidSemantic(msid_semantic)) => {
                debug_assert!(self.msid_semantic.replace(msid_semantic).is_none())
            }
            SdpLine::Attribute(Msid(msid)) => {
                debug_assert!(self.msid.replace(msid).is_none())
            }
            SdpLine::Attribute(RtpMap(rtp_map)) => self.rtp_map.push(rtp_map),
            SdpLine::Attribute(PTime(p_time)) => {
                debug_assert!(self.p_time.replace(p_time).is_none())
            }
            SdpLine::Attribute(Ssrc(ssrc)) => self.ssrc.push(ssrc),
            SdpLine::Attribute(BundleGroup(bundle_group)) => {
                debug_assert!(self.bundle_group.replace(bundle_group).is_none())
            }
            SdpLine::Attribute(SsrcGroup(ssrc_group)) => {
                debug_assert!(self.ssrc_group.replace(ssrc_group).is_none())
            }
            SdpLine::Attribute(Fingerprint(fingerprint)) => {
                debug_assert!(self.fingerprint.replace(fingerprint).is_none())
            }
            SdpLine::Attribute(Direction(direction)) => {
                debug_assert!(self.direction.replace(direction).is_none())
            }
            SdpLine::Attribute(Rtp(rtp)) => debug_assert!(self.rtp.replace(rtp).is_none()),
            SdpLine::Attribute(Rtcp(rtcp)) => {
                debug_assert!(self.rtcp.replace(rtcp).is_none())
            }
            SdpLine::Attribute(Fmtp(fmtp)) => self.fmtp.push(fmtp),
            SdpLine::Attribute(RtcpFb(rtcp_fb)) => self.rtcp_fb.push(rtcp_fb),
            SdpLine::Attribute(RtcpOption(rtcp_option)) => self.rtcp_option.push(rtcp_option),
            SdpLine::Attribute(Control(control)) => {
                debug_assert!(self.control.replace(control).is_none())
            }
            SdpLine::Attribute(SetupRole(setup_role)) => {
                debug_assert!(self.setup_role.replace(setup_role).is_none())
            }
            SdpLine::Attribute(Extmap(extmap)) => self.extmap.push(extmap),
            SdpLine::Attribute(AttributeLine::BundleOnly) => self.bundle_only = true,
            SdpLine::Attribute(attr) => self.attributes.push(attr),
            // SdpLine::Attribute(AttributeLine::EoC        => todo!(),
            // SdpLine::Attribute(AttributeLine::Attribute {
            //     key: Cow<'a, str>,
            //     val: Cow<'a, str>,
            // } => todo!(),
            SdpLine::Comment(_) => {}
        }
    }
}

impl<'a> From<Media<'a>> for MediaSection<'a> {
    fn from(mline: Media<'a>) -> Self {
        Self {
            r#type: mline.r#type,
            port: mline.port,
            protocol: mline.protocol,
            payloads: mline.payloads,
            ..Default::default()
        }
    }
}

impl<'a> From<LazyMediaSection<'a>> for MediaSection<'a> {
    fn from(lazy: LazyMediaSection<'a>) -> Self {
        let mut section = MediaSection::from(lazy.mline);

        for line in lazy.lines {
            section.add_line(line);
        }
        section
    }
}
