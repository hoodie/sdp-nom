use sdp_rs::LazySession;

fn get_session(content: &str) -> LazySession<'static> {
    LazySession::read_str(content).into_owned()
}
fn main() {
    let content = "v=0
o=- 7089656826184809091 2 IN IP4 127.0.0.1
s=-
t=0 0
a=group:BUNDLE video
a=msid-semantic: WMS
m=video 9 RTP/SAVPF 96 97 98 99 100 101 102 124 127 123 125
c=IN IP4 0.0.0.0";
    let session = get_session(content);
    println!("{}", session.to_string());
    // println!("{:#?}", session);
    let msection = session.media.first();
    //let mline = session.media.first().and_then(|m| m.lines.first()).and_then(|line| line.as_session());
    dbg!(msection);
}
