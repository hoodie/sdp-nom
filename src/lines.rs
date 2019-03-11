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

#[derive(Debug)]
pub struct Name<'a>(pub &'a str);

/// "s=somename"
named!{
    pub(crate) raw_name_line<CompleteStr, Name>,
    ws!(
        do_parse!(
            tag!("s=") >>
            name: read_string >>
            (Name(&name))
        )
    )
}


#[derive(Debug)]
pub struct Timing {
    start: u32,
    stop: u32,
}

/// "s=somename"
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
pub struct Mid<'a>(pub &'a str);

named!{
    pub(crate) raw_mid_line<CompleteStr, Mid>,
    do_parse!(
        tag!("a=mid:") >>
        mid: read_string >>

        (Mid(&mid))
    )
}

#[derive(Debug)]
pub enum Direction {
    SendOnly,
    SendRecv,
    RecvOnly,
    Inactive
}

named!{
    pub(crate) raw_direction_line<CompleteStr, Direction>,
    do_parse!(
        tag!("a=") >>
        direction: alt!(
            tag!("sendrecv") => { |_| Direction::SendRecv } |
            tag!("sendonly") => { |_| Direction::SendOnly } |
            tag!("recvonly") => { |_| Direction::RecvOnly } |
            tag!("inactive") => { |_| Direction::Inactive}
        )>>

        (direction)
    )
}


/// generic a line
named!{
    pub(crate) raw_a_line<CompleteStr, Vec<String>>,
    do_parse!(
        tag!("a=") >>
        line: map!(
            read_as_strings,
            |vs| vs
                .into_iter()
                .map(|s| ToString::to_string(&s))
                .collect()
        ) >>

        (line)
    )
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