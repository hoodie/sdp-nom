use nom::*;
use nom::types::CompleteStr;

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
pub(crate) struct Rtp<'a> {
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
    println!("{:?}", raw_rtp_attribute_line("a=rtpmap:110 opus/48000/2".into()).unwrap());
}