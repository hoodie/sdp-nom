use nom::*;
use nom::types::CompleteStr;

use crate::parsers::*;

/// "v=0"
named!{
    pub(crate) raw_version_line<CompleteStr, u32>,
    ws!(
        do_parse!(
            tag!("v=") >>
            version: read_number >>
            (version)
        )
    )
}

#[derive(Debug, PartialEq)]
pub struct Name<'a>(pub &'a str);

/// "s=somename"
named!{
    pub(crate) raw_name_line<CompleteStr, Name>,
    do_parse!(tag!("s=") >> name: read_string >> (Name(&name)))
}

#[test]
fn test_raw_name_line() {
    assert_eq!(raw_name_line("s=testname".into()).unwrap().1, Name("testname"));
}

#[derive(Debug, PartialEq)]
pub struct Description<'a>(pub &'a str);

/// "i=description"
named!{
    pub(crate) raw_description_line<CompleteStr, Description>,
    do_parse!(tag!("i=") >> description: read_string >> (Description(&description)))
}

#[test]
fn test_raw_description_line() {
    assert_eq!(raw_description_line("i=testdescription".into()).unwrap().1, Description("testdescription"));
}






#[derive(Debug)]
pub struct Timing {
    start: u32,
    stop: u32,
}

/// "t=0 0"
named!{
    pub(crate) raw_timing_line<CompleteStr, Timing>,
    ws!(
        do_parse!(
            tag!("t=") >>
            start: read_number >>
            stop: read_number >>
            (Timing{ start, stop })
        )
    )
}




#[derive(Debug)]
pub enum BandWidthType {
    TIAS, AS, CT, RR, RS
}
// TIAS|AS|CT|RR|RS
named!{
    pub(crate) raw_bandwidth_type<CompleteStr, BandWidthType>,
    do_parse!(
        bandwidthtype: alt!(
            tag!("TIAS") => { |_| BandWidthType::TIAS } |
            tag!("AS") => { |_| BandWidthType::AS } |
            tag!("CT") => { |_| BandWidthType::CT } |
            tag!("RR") => { |_| BandWidthType::RR } |
            tag!("RS") => { |_| BandWidthType::RS }
        )>>

        (bandwidthtype)
    )
}

#[derive(Debug)]
pub struct BandWidth {
    r#type: BandWidthType,
    limit: u32,
}

/// "s=somename"
named!{
    pub(crate) raw_bandwidth_line<CompleteStr, BandWidth>,
    ws!(
        do_parse!(
            tag!("b=") >>
            r#type: raw_bandwidth_type >>
            limit: read_number >>
            (BandWidth{ r#type, limit})
        )
    )
}





#[derive(Debug, PartialEq)]
pub struct Media<'a> {
    r#type: &'a str,
    port: u32,
    protocol: Vec<&'a str>,
    payloads: Vec<u32>, 
}

named!{
    pub(crate) raw_media_line<CompleteStr, Media>,
    ws!(
        do_parse!(
            tag!("m=") >>
            t: read_string >>
            port: read_number >>
            protocol: slash_separated_strings >>
            payloads: read_as_numbers >>
            (Media { r#type: &t, port, protocol, payloads })
        )
    )
}

#[test]
fn test_raw_mline() {
    let parsed = raw_media_line("m=video 51744 RTP/AVP 126 97 98 34 31".into()).unwrap();
    let expected = Media {
        r#type: "video",
        port: 51744,
        protocol: vec!["RTP".into(), "AVP".into()],
        payloads: vec![126, 97, 98, 34, 31], 
    };

    println!("{:?}", parsed.1);
    assert_eq!(parsed.1, expected);
}



#[derive(Debug)]
pub struct Mid<'a>(pub &'a str);

named!{
    pub(crate) raw_mid_line<CompleteStr, Mid>,
    do_parse!(tag!("a=mid:") >> mid: read_string >> (Mid(&mid)))
}





#[derive(Debug)]
pub struct Msid<'a>(pub Vec<&'a str>);

named!{
    pub(crate) raw_msid_line<CompleteStr, Msid>,
    do_parse!(tag!("a=msid:") >> msids: read_as_strings >> (Msid(msids)))
}





named!{
    pub(crate) raw_direction_line<CompleteStr, Direction>,
    do_parse!(tag!("a=") >> direction: read_direction >> (direction))
}





#[derive(Debug)]
pub struct Ssrc<'a> {
    id: u64,
    attribute: &'a str,
    value: &'a str,
}

/// ssrc
named!{
    pub(crate) raw_ssrc_line<CompleteStr, Ssrc>,
    ws!(
        do_parse!(
            tag!("a=ssrc:") >> id: read_big_number  >>
            attribute: read_non_colon_string >>
            tag!(":") >> value: read_string >>
            (Ssrc { id, attribute: &attribute, value: &value })
        )
    )
}

#[test]
fn parse_ssrc_line() {
    println!("{:?}",
        raw_ssrc_line("a=ssrc:1366781084 cname:EocUG1f0fcg/yvY7".into()).unwrap()
    );
}





#[derive(Debug)]
pub struct Fingerprint<'a> {
    r#type: &'a str,
    hash: &'a str,
}

/// ssrc
named!{
    pub(crate) raw_fingerprint_line<CompleteStr, Fingerprint>,
    ws!(
        do_parse!(
            tag!("a=fingerprint:") >>
            typ: read_string >>
            hash: read_string >>
            (Fingerprint {
                r#type: &typ,
                hash: &hash
            })
        )
    )
}

#[test]
fn parse_fingerprint_line() {
    println!("{:?}",
        raw_fingerprint_line("a=fingerprint:sha-256 19:E2:1C:3B:4B:9F:81:E6:B8:5C:F4:A5:A8:D8:73:04:BB:05:2F:70:9F:04:A9:0E:05:E9:26:33:E8:70:88:A2".into()).unwrap()
    );
}

/// generic a line
named!{
    pub(crate) raw_a_line<CompleteStr, Vec<&str>>,
    do_parse!(tag!("a=") >> line: read_as_strings >> (line))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_mid_line() {
        println!("{:?}", raw_mid_line("a=mid:1".into()).unwrap());
        println!("{:?}", raw_mid_line("a=mid:a1".into()).unwrap());
        println!("{:?}", raw_mid_line("a=mid:0".into()).unwrap());
        println!("{:?}", raw_mid_line("a=mid:audio".into()).unwrap());
    }
}