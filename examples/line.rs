use sdp_nom::sdp_line;

fn main() {
    let content = "v=0
s=health hazard
c=IN IP4 127.0.0.1
m=vide 8851 RTP/SAVPF 321
a=rid 🤷
";

    for raw_line in content.lines() {
        let line = sdp_line(raw_line).unwrap().1;
        eprintln!("{:?}", line);
    }
}
