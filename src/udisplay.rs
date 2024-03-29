use ufmt::{uWrite, uwrite, uwriteln, Formatter};

use crate::{
    attributes::{
        bundle::BundleGroup,
        candidate::{Candidate, CandidateComponent, CandidateProtocol, CandidateType},
        control::Control,
        direction::Direction,
        dtls::SetupRole,
        extmap::Extmap,
        fingerprint::Fingerprint,
        fmtp::Fmtp,
        ice::IceParameter,
        mid::Mid,
        msid::*,
        rtcp::*,
        rtcp_option::RtcpOption,
        rtp::Rtp,
        rtpmap::*,
        ssrc::{Ssrc, SsrcGroup, SsrcSemantic},
        AttributeLine,
    },
    lines::{
        bandwidth::*, connection::*, email::*, media::*, origin::*, phone_number::*,
        session_information::*, session_name::*, timing::*, uri::*, version::*, SessionLine,
    },
    media_section::MediaSection,
    parsers::IpVer,
    SdpLine, Session,
};
impl ufmt::uDisplay for Session<'_> {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        write_ln_option(f, &self.version)?;
        write_ln_option(f, &self.origin)?;
        write_ln_option(f, &self.name)?;
        write_ln_option(f, &self.timing)?;
        write_ln_option(f, &self.band_width)?;
        write_ln_option(f, &self.uri)?;
        write_ln_option(f, &self.phone_number)?;
        write_ln_option(f, &self.email_address)?;
        write_ln_option(f, &self.connection)?;
        write_ln_option(f, &self.description)?;

        for x in &self.attributes {
            uwriteln!(f, "{}", x)?;
        }

        for x in &self.media {
            uwrite!(f, "{}", x)?;
        }
        Ok(())
    }
}

fn write_ln_option<W>(
    f: &mut Formatter<'_, W>,
    content: &Option<impl ufmt::uDisplay>,
) -> Result<(), W::Error>
where
    W: uWrite + ?Sized,
{
    if let Some(ref x) = content {
        uwriteln!(f, "{}", x)?;
    }
    Ok(())
}

impl ufmt::uDisplay for MediaSection<'_> {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        uwriteln!(f, "{}", self.media())?;

        write_ln_option(f, &self.connection)?;

        write_ln_option(f, &self.rtcp)?;
        for candidate in &self.candidates {
            uwriteln!(f, "{}", candidate)?;
        }

        write_ln_option(f, &self.ice.ufrag.clone().map(IceParameter::Ufrag))?;

        write_ln_option(f, &self.ice.pwd.clone().map(IceParameter::Pwd))?;

        write_ln_option(f, &self.ice.options.clone().map(IceParameter::Options))?;

        write_ln_option(f, &self.fingerprint)?;
        write_ln_option(f, &self.setup_role)?;
        // uwriteln!(f, "{}", Mid(self.mid.clone()))?;
        write_ln_option(f, &self.mid.to_owned().map(Mid))?;

        write_ln_option(f, &self.p_time)?;
        for extmap in &self.extmap {
            uwriteln!(f, "{}", extmap)?;
        }

        write_ln_option(f, &self.bundle_group)?;
        if self.bundle_only {
            uwriteln!(f, "a=bundle-only")?;
        }
        write_ln_option(f, &self.direction)?;
        write_ln_option(f, &self.msid_semantic)?;
        write_ln_option(f, &self.msid)?;
        write_ln_option(f, &self.rtp)?;
        for rtcp_option in &self.rtcp_option {
            uwriteln!(f, "{}", rtcp_option)?;
        }

        let known_payloads = self
            .payloads
            .iter()
            .filter_map(|p| p.parse::<u32>().ok())
            .collect::<Vec<_>>();

        for payload in &known_payloads {
            for rtp in self.rtp_map.iter().filter(|r| r.payload == *payload) {
                uwriteln!(f, "{}", rtp)?;
            }
            for rtcp_fb in self.rtcp_fb.iter().filter(|r| r.payload == *payload) {
                uwriteln!(f, "{}", rtcp_fb)?;
            }
            for fmtp in self.fmtp.iter().filter(|r| r.payload == *payload) {
                uwriteln!(f, "{}", fmtp)?;
            }
        } // one more round for those not listed in the fmt field
        {
            for rtp in self
                .rtp_map
                .iter()
                .filter(|r| !known_payloads.contains(&r.payload))
            {
                uwriteln!(f, "{}", rtp)?;
            }
            for rtcp_fb in self
                .rtcp_fb
                .iter()
                .filter(|r| !known_payloads.contains(&r.payload))
            {
                uwriteln!(f, "{}", rtcp_fb)?;
            }
            for fmtp in self
                .fmtp
                .iter()
                .filter(|r| !known_payloads.contains(&r.payload))
            {
                uwriteln!(f, "{}", fmtp)?;
            }
        }

        write_ln_option(f, &self.ssrc_group)?;
        for ssrc in &self.ssrc {
            uwriteln!(f, "{}", ssrc)?;
        }

        write_ln_option(f, &self.control)?;

        for x in &self.attributes {
            uwriteln!(f, "{}", x)?;
        }

        Ok(())
    }
}
impl ufmt::uDisplay for SdpLine<'_> {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        match self {
            SdpLine::Session(session) => uwriteln!(f, "{}", session),
            SdpLine::Attribute(attribute) => uwriteln!(f, "{}", attribute),
            SdpLine::Comment(_) => Ok(()),
        }
    }
}
#[cfg(all(feature = "udisplay"))]
impl std::string::ToString for SdpLine<'_> {
    fn to_string(&self) -> String {
        let mut output = String::new();
        ufmt::uwrite!(output, "{}", self).unwrap();
        output
    }
}

impl ufmt::uDisplay for SessionLine<'_> {
    #[rustfmt::skip]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized, {
        match self {
            SessionLine::Version(v)      => uwrite!(f,"{}", v),
            SessionLine::Name(n)         => uwrite!(f,"{}", n),
            SessionLine::Timing(t)       => uwrite!(f,"{}", t),
            SessionLine::Origin(o)       => uwrite!(f,"{}", o),
            SessionLine::BandWidth(b)    => uwrite!(f,"{}", b),
            SessionLine::Uri(u)          => uwrite!(f,"{}", u),
            SessionLine::PhoneNumber(p)  => uwrite!(f,"{}", p),
            SessionLine::EmailAddress(e) => uwrite!(f,"{}", e),
            SessionLine::Connection(c)   => uwrite!(f,"{}", c),
            SessionLine::Description(d)  => uwrite!(f,"{}", d),
            SessionLine::Media(m)        => uwrite!(f,"{}", m),
        }
    }
}

impl ufmt::uDisplay for AttributeLine<'_> {
    #[rustfmt::skip]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized, {
        match self {
            AttributeLine::Candidate(c)    => uwrite!(f, "{}", c),
            AttributeLine::Ice(i)          => uwrite!(f, "{}", i),
            AttributeLine::Mid(m)          => uwrite!(f, "{}", m),
            AttributeLine::MsidSemantic(ms) => uwrite!(f, "{}", ms),
            AttributeLine::Msid(m)         => uwrite!(f, "{}", m),
            AttributeLine::RtpMap(r)       => uwrite!(f, "{}", r),
            AttributeLine::PTime(p)        => uwrite!(f, "{}", p),
            AttributeLine::Ssrc(s)         => uwrite!(f, "{}", s),
            AttributeLine::BundleGroup(b)  => uwrite!(f, "{}", b),
            AttributeLine::SsrcGroup(s)    => uwrite!(f, "{}", s),
            AttributeLine::Fingerprint(fp) => uwrite!(f, "{}", fp),
            AttributeLine::Direction(d)    => uwrite!(f, "{}", d),
            AttributeLine::Rtp(r)          => uwrite!(f, "{}", r),
            AttributeLine::Rtcp(r)         => uwrite!(f, "{}", r),
            AttributeLine::Fmtp(fmtp)      => uwrite!(f, "{}", fmtp),
            AttributeLine::RtcpFb(r)       => uwrite!(f, "{}", r),
            AttributeLine::RtcpOption(r)   => uwrite!(f, "{}", r),
            AttributeLine::Control(c)      => uwrite!(f, "{}", c),
            AttributeLine::SetupRole(s)    => uwrite!(f, "{}", s),
            AttributeLine::Extmap(e)       => uwrite!(f, "{}", e),
            AttributeLine::BundleOnly      => uwrite!(f, "a=bundle-only"),
            AttributeLine::EoC             => uwrite!(f, "a=end-of-candidates"),
            AttributeLine::KeyValue {
                key,
                val
            }                              => uwrite!(f, "a={}:{}", key.as_ref(), val.as_ref()),
            AttributeLine::KeyOnly(key)    => uwrite!(f, "a={}", key.as_ref()),
        }
    }
}

impl ufmt::uDisplay for BundleGroup<'_> {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        uwrite!(f, "a=group:BUNDLE")?;
        for v in &self.0 {
            uwrite!(f, " {}", v.as_ref())?;
        }
        Ok(())
    }
}
impl ufmt::uDisplay for Fmtp<'_> {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        uwrite!(f, "a=fmtp:{} {}", self.payload, self.config.as_ref())
    }
}
impl ufmt::uDisplay for Rtp<'_> {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        uwrite!(
            f,
            "a=rtpmap:{} {}/{}/{}",
            self.payload,
            self.codec.as_ref(),
            self.rate,
            self.encoding
        )
    }
}
impl ufmt::uDisplay for Control<'_> {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        uwrite!(f, "a=control:{}", self.0.as_ref())
    }
}
impl ufmt::uDisplay for Direction {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        match self {
            Direction::SendOnly => uwrite!(f, "a=sendonly"),
            Direction::SendRecv => uwrite!(f, "a=sendrecv"),
            Direction::RecvOnly => uwrite!(f, "a=recvonly"),
            Direction::Inactive => uwrite!(f, "a=inactive"),
        }
    }
}
impl ufmt::uDisplay for RtcpOption {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        match self {
            RtcpOption::RtcpMux => uwrite!(f, "a=rtcp-mux"),
            RtcpOption::RtcpMuxOnly => uwrite!(f, "a=rtcp-mux-only"),
            RtcpOption::RtcpRsize => uwrite!(f, "a=rtcp-rsize"),
        }
    }
}
impl ufmt::uDisplay for Fingerprint<'_> {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        uwrite!(
            f,
            "a=fingerprint:{} {}",
            self.r#type.as_ref(),
            self.hash.as_ref()
        )
    }
}
impl ufmt::uDisplay for Mid<'_> {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        uwrite!(f, "a=mid:{}", self.0.as_ref())
    }
}
impl ufmt::uDisplay for MsidSemantic<'_> {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        uwrite!(f, "a=msid-semantic: ")?;
        uwrite!(f, "{} ", self.semantic.as_ref())?;
        if let Some(ref token) = self.token {
            uwrite!(f, "{}", token.as_ref())?;
        }
        Ok(())
    }
}
impl ufmt::uDisplay for Msid<'_> {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        uwrite!(f, "a=msid:")?;
        for (i, x) in self.0.iter().enumerate() {
            if i > 0 {
                uwrite!(f, " ")?;
            }
            uwrite!(f, "{}", x.as_ref())?;
        }
        Ok(())
    }
}
impl ufmt::uDisplay for Version {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        uwrite!(f, "v={}", self.0)
    }
}
impl ufmt::uDisplay for SessionInformation<'_> {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        uwrite!(f, "i={}", self.0.as_ref())
    }
}
impl ufmt::uDisplay for SessionName<'_> {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        uwrite!(f, "s={}", self.0.as_ref())
    }
}

impl ufmt::uDisplay for Origin<'_> {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        uwrite!(
            f,
            "o={} {} {} {} {} {}",
            self.user_name.as_ref(),
            self.session_id,
            self.session_version,
            self.net_type.as_ref(),
            self.ip_ver,
            IpAddress(&self.addr)
        )
    }
}

impl ufmt::uDisplay for Media<'_> {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        uwrite!(
            f,
            "m={} {} {}",
            self.r#type.as_ref(),
            self.port,
            self.protocol.join("/").as_str(),
        )?;
        for payload in &self.payloads {
            uwrite!(f, " {}", payload.as_ref())?;
        }
        Ok(())
    }
}

struct IpAddress<'a>(&'a std::net::IpAddr);
#[allow(clippy::many_single_char_names)]
impl ufmt::uDisplay for IpAddress<'_> {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        match self.0 {
            std::net::IpAddr::V4(addr) => {
                let [a, b, c, d] = addr.octets();
                uwrite!(f, "{}.{}.{}.{}", a, b, c, d)
            }
            std::net::IpAddr::V6(addr) => {
                uwrite!(f, "{}", addr.to_string())
            }
        }
    }
}

impl ufmt::uDisplay for Connection {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        let Self { ip_ver, addr, mask } = self;
        if let Some(mask) = mask {
            uwrite!(f, "c=IN {} {}/{}", ip_ver, IpAddress(addr), mask)
        } else {
            uwrite!(f, "c=IN {} {}", ip_ver, IpAddress(addr))
        }
    }
}
impl ufmt::uDisplay for SsrcGroup {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        uwrite!(f, "a=ssrc-group:")?;
        match self.semantic {
            SsrcSemantic::FID => uwrite!(f, "FID")?,
            SsrcSemantic::FEC => uwrite!(f, "FEC")?,
        }
        for id in &self.ids {
            uwrite!(f, " {}", id)?;
        }
        Ok(())
    }
}
impl ufmt::uDisplay for Ssrc<'_> {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        uwrite!(
            f,
            "a=ssrc:{} {}:{}",
            self.id,
            self.attribute.as_ref(),
            self.value.as_ref()
        )
    }
}
impl ufmt::uDisplay for RtpMap<'_> {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        uwrite!(
            f,
            "a=rtpmap:{} {}",
            self.payload,
            self.encoding_name.as_ref()
        )?;
        if let Some(clock_rate) = self.clock_rate {
            uwrite!(f, "/{}", clock_rate)?;
        }
        if let Some(encoding) = self.encoding {
            uwrite!(f, "/{}", encoding)?;
        }
        Ok(())
    }
}
impl ufmt::uDisplay for PTime {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        match self {
            PTime::MaxPTime(x) => uwrite!(f, "a=maxptime:{}", x),
            PTime::MinPTime(x) => uwrite!(f, "a=minptime:{}", x),
            PTime::PTime(x) => uwrite!(f, "a=ptime:{}", x),
        }
    }
}

impl ufmt::uDisplay for FbAckParam<'_> {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        match self {
            FbAckParam::Rpsi => uwrite!(f, "rpsi"),
            FbAckParam::Sli(Some(x)) => uwrite!(f, "sli {}", x.as_ref()),
            FbAckParam::Sli(None) => uwrite!(f, "sli"),
            FbAckParam::App(x) => uwrite!(f, "app {}", x.as_ref()),
            FbAckParam::Other(k, Some(v)) => uwrite!(f, "{} {}", k.as_ref(), v.as_ref()),
            FbAckParam::Other(k, None) => uwrite!(f, "{}", k.as_ref()),
        }
    }
}

impl ufmt::uDisplay for FbNackParam<'_> {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        match self {
            FbNackParam::Pli => uwrite!(f, "pli"),
            FbNackParam::Rpsi => uwrite!(f, "rpsi"),
            FbNackParam::Sli => uwrite!(f, "sli"),
            FbNackParam::Other(k, v) => uwrite!(f, "{} {}", k.as_ref(), v.as_ref()),
            FbNackParam::App(x) => uwrite!(f, "app {}", x.as_ref()),
        }
    }
}

impl ufmt::uDisplay for FbParam<'_> {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        match self {
            FbParam::App(p) => uwrite!(f, "app {}", p.as_ref()),
            FbParam::Single(p) => uwrite!(f, "{}", p.as_ref()),
            FbParam::Pair(k, v) => uwrite!(f, "{} {}", k.as_ref(), v.as_ref()),
        }
    }
}

impl ufmt::uDisplay for FbVal<'_> {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        match self {
            FbVal::Ack(p) => uwrite!(f, "ack {}", p),
            FbVal::Nack(p) => uwrite!(f, "nack {}", p),
            FbVal::TrrInt(p) => uwrite!(f, "trr-int {}", p),
            FbVal::RtcpFbId {
                id,
                param: Some(param),
            } => uwrite!(f, "{} {}", id.as_ref(), param),
            FbVal::RtcpFbId { id, param: None } => uwrite!(f, "{}", id.as_ref()),
        }
    }
}

impl ufmt::uDisplay for Fb<'_> {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        uwrite!(f, "a=rtcp-fb:{} {}", self.payload, self.val)
    }
}
impl ufmt::uDisplay for NetType {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        uwrite!(f, "IN")
    }
}

impl ufmt::uDisplay for Rtcp {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        uwrite!(
            f,
            "a=rtcp:{} {} {} {}",
            self.port,
            self.net_type,
            self.ip_ver,
            IpAddress(&self.addr),
        )
    }
}

impl ufmt::uDisplay for IceParameter<'_> {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        match self {
            IceParameter::Ufrag(ufrag) => uwrite!(f, "a=ice-ufrag:{}", ufrag.as_ref()),
            IceParameter::Pwd(pwd) => uwrite!(f, "a=ice-pwd:{}", pwd.as_ref()),
            IceParameter::Options(options) => uwrite!(f, "a=ice-options:{}", options.as_ref()),
            IceParameter::Mismatch => uwrite!(f, "a=ice-mismatch"),
            IceParameter::Lite => uwrite!(f, "a=ice-lite"),
        }
    }
}

impl ufmt::uDisplay for Extmap<'_> {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        if let Some(direction) = self.direction {
            uwrite!(f, "a=extmap:{}/", self.value,)?;

            match direction {
                Direction::SendOnly => uwrite!(f, "sendonly")?,
                Direction::SendRecv => uwrite!(f, "sendrecv")?,
                Direction::RecvOnly => uwrite!(f, "recvonly")?,
                Direction::Inactive => uwrite!(f, "inactive")?,
            }

            uwrite!(f, " {}", self.uri.as_ref())?;
        } else {
            uwrite!(f, "a=extmap:{} {}", self.value, self.uri.as_ref())?;
        }
        for a in &self.attributes {
            uwrite!(f, " {}", a.as_ref())?;
        }
        Ok(())
    }
}
impl ufmt::uDisplay for BandWidthType {
    #[rustfmt::skip]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized, {
        use BandWidthType::*;
        uwrite!( f, "{}", match self { TIAS => "TIAS", AS => "AS", CT => "CT", RR => "RR", RS => "R" })
    }
}

impl ufmt::uDisplay for BandWidth {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        uwrite!(f, "b={}:{}", self.r#type, self.limit)
    }
}

impl ufmt::uDisplay for Uri<'_> {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        uwrite!(f, "u={}", self.0.as_ref())
    }
}
impl ufmt::uDisplay for EmailAddress<'_> {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        uwrite!(f, "e={}", self.0.as_ref())
    }
}
impl ufmt::uDisplay for PhoneNumber<'_> {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        uwrite!(f, "p={}", self.0.as_ref())
    }
}
impl ufmt::uDisplay for Timing {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        uwrite!(f, "t={} {}", self.start, self.stop)
    }
}

impl ufmt::uDisplay for IpVer {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        match self {
            IpVer::Ip4 => uwrite!(f, "IP4"),
            IpVer::Ip6 => uwrite!(f, "IP6"),
        }
    }
}
impl ufmt::uDisplay for CandidateComponent {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        match self {
            CandidateComponent::Rtp => uwrite!(f, "1"),
            CandidateComponent::Rtcp => uwrite!(f, "2"),
        }
    }
}

impl ufmt::uDisplay for CandidateProtocol {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        match self {
            CandidateProtocol::Tcp => uwrite!(f, "tcp"),
            CandidateProtocol::Udp => uwrite!(f, "udp"),
            CandidateProtocol::Dccp => uwrite!(f, "dccp"),
        }
    }
}

impl ufmt::uDisplay for CandidateType {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        match self {
            CandidateType::Host => uwrite!(f, "host"),
            CandidateType::Relay => uwrite!(f, "relay"),
            CandidateType::Srflx => uwrite!(f, "srflx"),
            CandidateType::Prflx => uwrite!(f, "prflx"),
        }
    }
}

impl ufmt::uDisplay for Candidate<'_> {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        uwrite!(
            f,
            "a=candidate:{} {} {} {} {} {} typ {}",
            self.foundation,
            self.component,
            self.protocol,
            self.priority,
            IpAddress(&self.addr),
            self.port,
            self.r#type,
        )?;
        if let Some(x) = self.raddr {
            uwrite!(f, " raddr {}", IpAddress(&x))?;
        }
        if let Some(x) = self.rport {
            uwrite!(f, " rport {}", x)?;
        }
        if let Some(x) = self.tcptype.as_ref() {
            uwrite!(f, " tcptype {}", x.as_ref())?;
        }
        if let Some(x) = self.generation {
            uwrite!(f, " generation {}", x)?;
        }
        if let Some(x) = self.network_id {
            uwrite!(f, " network-id {}", x)?;
        }
        Ok(())
    }
}

impl ufmt::uDisplay for SetupRole {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        match self {
            SetupRole::Active => uwrite!(f, "a=setup:active"),
            SetupRole::Passive => uwrite!(f, "a=setup:passive"),
            SetupRole::ActPass => uwrite!(f, "a=setup:actpass"),
        }
    }
}
