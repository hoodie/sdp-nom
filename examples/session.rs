use sdp_rs::Session;

fn main() {
    let content = "v=0
o=- 7089656826184809091 2 IN IP4 127.0.0.1
s=-
t=0 0
a=group:BUNDLE video
a=msid-semantic: WMS
m=video 9 RTP/SAVPF 96 97 98 99 100 101 102 124 127 123 125
c=IN IP4 0.0.0.0";
    let session = Session::read_str(content);
    println!("{}", session.to_string());
    cfg_if::cfg_if! {
        if #[cfg(feature = "serde")] {
            println!("{}", serde_json::to_string_pretty(&session).unwrap());
        } else {
            println!("{:#?}", session);
        }
    }
}
