use nom::*;
use nom::types::CompleteStr;

use std::net::IpAddr;

use crate::parsers::*;

pub enum Attribute {
    Rtp,
    Fmtp,
    Control,
    Rtcp,
    RtcpFbTrrInt,
    RtcpFb,
    Ext,
    Crypto,
    Setup,
    // Mid,
    // Msid,
    Ptime,
    MaxPtime,
    // Direction,
    IceLite,
    IceUFrag,
    IcePwd,
    // Fingerprint,
    Candidates,
    // EndOfCandidates,
    RemoteCandidates,
    IceOptions,
    Ssrcs,
    SsrcGroups,
    MsidSemantic,
    Groups,
    RtcpMux,
    RtcpRsize,
    Sctpmap,
    XGoogleFlag,
    Rids,
    Imageattrs,
    Simulcast,
    Simulcast03,
    Framerate,
    SourceFilter,
    BundleOnly,
    Label,
    SctpPort,
    MaxMessageSize,
    Invalid,
}




// a=rtpmap:110 opus/48000/2
#[derive(Debug, PartialEq)]
pub struct Rtp<'a> {
    payload: u32,
    codec: &'a str,
    rate: u32,
    encoding: u32,
}

named!{
    raw_rtp_attribute_line<CompleteStr, Rtp>,
    do_parse!(
        tag!("a=") >> tag!("rtpmap:") >> 
        payload: read_number >> tag!(" ") >> 
        codec: read_non_slash_string >> tag!("/") >> 
        rate: read_number >> tag!("/") >> 
        encoding: read_number >> 
        (Rtp { payload, codec, rate, encoding })
    )
}

#[test]
fn test_raw_rtp_attribute_line() {
    println!("{:?}", raw_rtp_attribute_line("a=rtpmap:110 opus/48000/2".into()).unwrap().1);
}





// a=fmtp:108 profile-level-id=24;object=23;bitrate=64000
#[derive(Debug, PartialEq)]
pub struct Fmtp<'a> {
    payload: u32,
    config: &'a str,
}

named!{
    raw_fmtp_attribute_line<CompleteStr, Fmtp>,
    ws!(
        do_parse!(
            tag!("a=") >> tag!("fmtp:") >> 
            payload: read_number >>
            config: read_string >>
            (Fmtp { payload, config, })
        )
    )
}

#[test]
fn test_raw_fmtp_attribute_line() {
    println!("{:?}", raw_fmtp_attribute_line("a=fmtp:108 profile-level-id=24;object=23;bitrate=64000".into()).unwrap().1);
}







// a=control:streamid=0
#[derive(Debug, PartialEq)]
pub struct Control<'a> (&'a str);

named!{
    raw_control_attribute_line<CompleteStr, Control>,
    do_parse!(tag!("a=") >> tag!("control:") >> control: read_string >> ( Control(control) ))
}

#[test]
fn test_raw_control_attribute_line() {
    println!("{:?}", raw_control_attribute_line("a=control:streamid=0".into()).unwrap().1);
}






/// Rtcp
/// 
/// https://tools.ietf.org/html/rfc3605
/// `a=rtcp:65179 IN IP4 10.23.34.567`
#[derive(Debug, PartialEq)]
pub struct Rtcp {
    port: u32,
    net_type: NetType,
    ip_ver: IpVer,
    addr: IpAddr,
}

named!{
    raw_rtcp_attribute_line<CompleteStr, Rtcp>,
    ws!(
        do_parse!(
            tag!("a=") >> tag!("rtcp:") >>
            port: read_number >>
            net_type: read_net_type >>
            ip_ver: read_ipver >>
            addr: read_addr >>
            ( Rtcp { port, net_type, ip_ver, addr } )
        )
    )
}

#[test]
fn test_raw_rtcp_attribute_line() {
    println!("{:?}", raw_rtcp_attribute_line("a=rtcp:65179 IN IP4 10.23.34.255".into()).unwrap().1);
    println!("{:?}", raw_rtcp_attribute_line("a=rtcp:65179 IN IP4 ::1".into()).unwrap().1);
}






/// RtcpFeedback
/// 
/// https://datatracker.ietf.org/doc/draft-ietf-mmusic-sdp-mux-attributes/16/?include_text=1
/// eg `a=rtcp-fb:98 trr-int 100`
#[derive(Debug, PartialEq)]
pub struct RtcpFb {
    payload: u32,
    r#type: RtcpFbType,
    subtype: Option<RtcpFbSubType>,
    value: Option<u32>,
}

#[derive(Debug, PartialEq)]
pub enum RtcpFbType {
    Ack,
    Nack,
    TrrInt
}

#[derive(Debug, PartialEq)]
pub enum RtcpFbSubType {
    Rpsi,
    App,
    Pli,
    Sli,
}

named!{
    raw_rtcpfb_attribute_line<CompleteStr, RtcpFb>,
    alt!(
    ws!(
        do_parse!(
            tag!("a=") >> tag!("rtcp-fb:") >>
            payload: read_number >>
            r#type: alt!(
                tag!("ack")  => { |_| RtcpFbType::Ack} |
                tag!("nack") => { |_| RtcpFbType::Nack}
                // | tag!("trr-int") => { |_| RtcpFbType::TrrInt }
            ) >>
            subtype: opt!(alt!(
                tag!("rpsi")  => { |_| RtcpFbSubType::Rpsi} |
                tag!("app") => { |_| RtcpFbSubType::App} |
                tag!("pli") => { |_| RtcpFbSubType::Pli} |
                tag!("sli") => { |_| RtcpFbSubType::Sli}
            )) >>
            ( RtcpFb {
                payload, r#type, subtype, value: None
            } )
        )
    ) | ws!(
        do_parse!(
            tag!("a=") >> tag!("rtcp-fb:") >>
            payload: read_number >>
            tag!("trr-int") >>
            value: read_number >>

            ( RtcpFb {
                payload, r#type: RtcpFbType::TrrInt,
                subtype: None,
                value: Some(value),
            } )
        )
    )
    )
}

#[test]
fn test_raw_rtcpfb_line() {
    println!("{:?}", raw_rtcpfb_attribute_line("a=rtcp-fb:98 trr-int 100".into()).unwrap().1);
    println!("{:?}", raw_rtcpfb_attribute_line("a=rtcp-fb:98 ack sli".into()).unwrap().1);
    println!("{:?}", raw_rtcpfb_attribute_line("a=rtcp-fb:98 ack sli 5432".into()).unwrap().1);
    println!("{:?}", raw_rtcpfb_attribute_line("a=rtcp-fb:98 nack rpsi".into()).unwrap().1);
}







// a=extmap:2 urn:ietf:params:rtp-hdrext:toffset
#[derive(Debug, PartialEq)]
pub struct Ext<'a> {
    value: u32,
    direction: Option<Direction>,
    uri: Option<&'a str>,
    extended: Option<&'a str>,
}

named!{
    pub raw_ext_attribute_line<CompleteStr, Ext>,
    ws!(
        do_parse!(
            tag!("a=") >> tag!("extmap:") >>
            value: read_number >>
            direction: opt!(map!(tuple!(tag!("/"), read_direction), |(_, d)| d) ) >>
            uri: opt!(read_string) >>
            extended: opt!(read_string) >>

            ( Ext{ value, direction, uri, extended, } )
        )
    )
}


#[test]
fn test_raw_ext_line() {
    println!("{:?}", raw_ext_attribute_line("a=extmap:2 urn:ietf:params:rtp-hdrext:toffset".into()).unwrap().1);
    println!("{:?}", raw_ext_attribute_line("a=extmap:1 http://example.com/082005/ext.htm#ttime".into()).unwrap().1);
    println!("{:?}", raw_ext_attribute_line("a=extmap:2/sendrecv http://example.com/082005/ext.htm#xmeta short".into()).unwrap().1);
}