use sdp_nom::Session;

fn read_from_args() -> Option<String> {
    if let Some(arg) = std::env::args().nth(1) {
        std::fs::read_to_string(arg).ok()
    } else {
        None
    }
}

fn main() {
    let content = read_from_args().unwrap();
    let session = Session::read_str(&content);

    cfg_if::cfg_if! {
        if #[cfg(feature = "serde")] {
            println!("{}", serde_json::to_string_pretty(&session).unwrap());
        } else {
            // println!("{:#?}", session);
            print!("{}", session.to_string());
        }
    }
}
