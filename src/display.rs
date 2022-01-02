use std::fmt;

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
    parsers::IpVer,
    sdp_line::SdpLine,
};

impl fmt::Display for SdpLine<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SdpLine::Session(session) => write!(f, "{}", session),
            SdpLine::Attribute(attribute) => write!(f, "{}", attribute),
            SdpLine::Comment(_) => Ok(()),
        }
    }
}

impl fmt::Display for SessionLine<'_> {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

impl fmt::Display for AttributeLine<'_> {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

impl fmt::Display for BundleGroup<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "a=group:BUNDLE")?;
        for v in &self.0 {
            write!(f, " {}", v)?;
        }
        Ok(())
    }
}
impl fmt::Display for Fmtp<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "a=fmtp:{} {}", self.payload, self.config)
    }
}
impl fmt::Display for Rtp<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "a=rtpmap:{} {}/{}/{}",
            self.payload, self.codec, self.rate, self.encoding
        )
    }
}
impl fmt::Display for Control<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "a=control:{}", self.0)
    }
}
impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::SendOnly => write!(f, "a=sendonly"),
            Direction::SendRecv => write!(f, "a=sendrecv"),
            Direction::RecvOnly => write!(f, "a=recvonly"),
            Direction::Inactive => write!(f, "a=inactive"),
        }
    }
}
impl fmt::Display for RtcpOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RtcpOption::RtcpMux => write!(f, "a=rtcp-mux"),
            RtcpOption::RtcpMuxOnly => write!(f, "a=rtcp-mux-only"),
            RtcpOption::RtcpRsize => write!(f, "a=rtcp-rsize"),
        }
    }
}
impl fmt::Display for Fingerprint<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "a=fingerprint:{} {}", self.r#type, self.hash)
    }
}
impl fmt::Display for Mid<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "a=mid:{}", self.0)
    }
}
impl fmt::Display for MsidSemantic<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "a=msid-semantic:")?;
        for (i, x) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            write!(f, "{}", x)?;
        }
        Ok(())
    }
}
impl fmt::Display for Msid<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "a=msid:")?;
        for (i, x) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            write!(f, "{}", x)?;
        }
        Ok(())
    }
}
impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "v={}", self.0)
    }
}
impl fmt::Display for SessionInformation<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "i={}", self.0)
    }
}
impl fmt::Display for SessionName<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "s={}", self.0)
    }
}
impl fmt::Display for Origin<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "o={name} {id} {version} {nt} {ipver} {addr}",
            name = self.user_name,
            id = self.session_id,
            version = self.session_version,
            nt = self.net_type,
            ipver = self.ip_ver,
            addr = self.addr
        )
    }
}

impl fmt::Display for Media<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "m={ty} {port} {protos}",
            ty = self.r#type,
            port = self.port,
            protos = self.protocol.join("/"),
        )?;
        for payload in &self.payloads {
            write!(f, " {}", payload)?;
        }
        Ok(())
    }
}

impl fmt::Display for Connection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { ip_ver, addr, mask } = self;
        if let Some(mask) = mask {
            write!(f, "c=IN {} {}/{}", ip_ver, addr, mask)
        } else {
            write!(f, "c=IN {} {}", ip_ver, addr)
        }
    }
}
impl fmt::Display for SsrcGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "a=ssrc-group:")?;
        match self.semantic {
            SsrcSemantic::FID => write!(f, "FID")?,
            SsrcSemantic::FEC => write!(f, "FEC")?,
        }
        for id in &self.ids {
            write!(f, " {}", id)?;
        }
        Ok(())
    }
}
impl fmt::Display for Ssrc<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "a=ssrc:{} {}:{}", self.id, self.attribute, self.value)
    }
}
impl fmt::Display for RtpMap<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "a=rtpmap:{} {}", self.payload_type, self.encoding_name)?;
        if let Some(clock_rate) = self.clock_rate {
            write!(f, "/{}", clock_rate)?;
        }
        if let Some(encoding) = self.encoding {
            write!(f, "/{}", encoding)?;
        }
        Ok(())
    }
}
impl fmt::Display for PTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PTime::MaxPTime(x) => write!(f, "a=maxptime:{}", x),
            PTime::MinPTime(x) => write!(f, "a=minptime:{}", x),
            PTime::PTime(x) => write!(f, "a=ptime:{}", x),
        }
    }
}

impl fmt::Display for FbAckParam<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FbAckParam::Rpsi => write!(f, "rpsi"),
            FbAckParam::Sli(Some(x)) => write!(f, "sli {}", x),
            FbAckParam::Sli(None) => write!(f, "sli"),
            FbAckParam::App(x) => write!(f, "app {}", x),
            FbAckParam::Other(k, Some(v)) => write!(f, "{} {}", k, v),
            FbAckParam::Other(k, None) => write!(f, "{}", k),
        }
    }
}

impl fmt::Display for FbNackParam<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FbNackParam::Pli => write!(f, "pli"),
            FbNackParam::Rpsi => write!(f, "rpsi"),
            FbNackParam::Sli => write!(f, "sli"),
            FbNackParam::Other(k, v) => write!(f, "{} {}", k, v),
            FbNackParam::App(x) => write!(f, "app {}", x),
        }
    }
}

impl fmt::Display for FbParam<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FbParam::App(p) => write!(f, "app {}", p),
            FbParam::Single(p) => write!(f, "{}", p),
            FbParam::Pair(k, v) => write!(f, "{} {}", k, v),
        }
    }
}

impl fmt::Display for FbVal<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FbVal::Ack(p) => write!(f, "ack {}", p),
            FbVal::Nack(p) => write!(f, "nack {}", p),
            FbVal::TrrInt(p) => write!(f, "trr-int {}", p),
            FbVal::RtcpFbId {
                id,
                param: Some(param),
            } => write!(f, "{} {}", id, param),
            FbVal::RtcpFbId { id, param: None } => write!(f, "{}", id),
        }
    }
}

impl fmt::Display for Fb<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "a=rtcp-fb:{} {}", self.payload, self.val)
    }
}
impl fmt::Display for NetType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IN")
    }
}

impl fmt::Display for Rtcp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "a=rtcp:{} {} {} {}",
            self.port, self.net_type, self.ip_ver, self.addr
        )
    }
}

impl fmt::Display for IceParameter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IceParameter::Ufrag(ufrag) => write!(f, "a=ice-ufrag:{}", ufrag),
            IceParameter::Pwd(pwd) => write!(f, "a=ice-pwd:{}", pwd),
            IceParameter::Options(options) => write!(f, "a=ice-options:{}", options),
            IceParameter::Mismatch => write!(f, "a=ice-mismatch"),
            IceParameter::Lite => write!(f, "a=ice-lite"),
        }
    }
}

impl fmt::Display for Extmap<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(direction) = self.direction {
            write!(f, "a=extmap:{}/{} {}", self.value, direction, self.uri)?;
        } else {
            write!(f, "a=extmap:{} {}", self.value, self.uri)?;
        }
        for a in &self.attributes {
            write!(f, " {}", a)?;
        }
        Ok(())
    }
}
impl fmt::Display for BandWidthType {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use BandWidthType::*;
        write!( f, "{}", match self { TIAS => "TIAS", AS => "AS", CT => "CT", RR => "RR", RS => "R" })
    }
}

impl fmt::Display for BandWidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "b={}:{}", self.r#type, self.limit)
    }
}

impl fmt::Display for Uri<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl fmt::Display for EmailAddress<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "e={}", self.0)
    }
}
impl fmt::Display for PhoneNumber<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "p={}", self.0)
    }
}
impl fmt::Display for Timing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "t={} {}", self.start, self.stop)
    }
}

impl fmt::Display for IpVer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IpVer::Ip4 => write!(f, "IP4"),
            IpVer::Ip6 => write!(f, "IP6"),
        }
    }
}
impl fmt::Display for CandidateComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CandidateComponent::Rtp => write!(f, "1"),
            CandidateComponent::Rtcp => write!(f, "2"),
        }
    }
}

impl fmt::Display for CandidateProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CandidateProtocol::Tcp => write!(f, "tcp"),
            CandidateProtocol::Udp => write!(f, "udp"),
            CandidateProtocol::Dccp => write!(f, "dccp"),
        }
    }
}

impl fmt::Display for CandidateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CandidateType::Host => write!(f, "host"),
            CandidateType::Relay => write!(f, "relay"),
            CandidateType::Srflx => write!(f, "srflx"),
            CandidateType::Prflx => write!(f, "prflx"),
        }
    }
}

impl fmt::Display for Candidate<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "a=candidate:{} {} {} {} {} {} typ {}",
            self.foundation,
            self.component,
            self.protocol,
            self.priority,
            self.addr,
            self.port,
            self.r#type,
        )?;
        if let Some(x) = self.raddr {
            write!(f, "{}", x)?;
        }
        if let Some(x) = self.rport {
            write!(f, "{}", x)?;
        }
        if let Some(x) = self.tcptype.as_ref() {
            write!(f, "{}", x)?;
        }
        if let Some(x) = self.generation {
            write!(f, "{}", x)?;
        }
        Ok(())
    }
}

impl fmt::Display for SetupRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SetupRole::Active => write!(f, "a=setup:active"),
            SetupRole::Passive => write!(f, "a=setup:passive"),
            SetupRole::ActPass => write!(f, "a=setup:actpass"),
        }
    }
}
