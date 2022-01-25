use sdp_nom::Session;

fn main() {
    let content = "v=0
s=health hazard
c=IN IP4 127.0.0.1
m=vide 8851 RTP/SAVPF 321
a=rid 🤷
";

    let session = Session::read_str(content);

    cfg_if::cfg_if! {
        if #[cfg(feature = "display")] {
            println!("{}", serde_json::to_string_pretty(&session).unwrap());
        } else if #[cfg(feature = "debug")] {
            println!("{:#?}", session);
            // print!("{}", session.to_string());
        } else {
            panic!("not enough features activated")
        }
    }
}
