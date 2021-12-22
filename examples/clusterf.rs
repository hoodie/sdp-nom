use sdp_rs::Session;

fn main() {
    println!("example");
    let content = "v=+
o=l
s=-
t=;
m=o 1 DTLS/SCTP
a=sctp-port";
    let session = Session::read_str(content);
    println!("{}", session.to_string());
    println!("{:#?}", session);
}
