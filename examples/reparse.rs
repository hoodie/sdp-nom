use sdp_nom::Session;

fn read_from_args() -> Option<String> {
    if let Some(arg) = std::env::args().nth(1) {
        std::fs::read_to_string(arg).ok()
    } else {
        println!("no input! please pass a file path as first parameter");
        None
    }
}

fn main() {
    let input = read_from_args().unwrap();
    let session = Session::read_str(&input);
    eprintln!("parsed\n{:#?}", session);

    let reserialized = session.to_string();
    eprintln!("reserialized\n{}", reserialized);
    let reparsed = Session::read_str(&reserialized);

    pretty_assertions::assert_eq!(session, reparsed);
}
