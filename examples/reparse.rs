use sdp_nom::{sdp_lines, sdp_lines_all, Session};

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
    let parsed_lines = sdp_lines_all(&input)
        .map(|res| {
            let (remainder, line) = res.unwrap();
            if !remainder.is_empty() {
                eprintln!("🙈 remainder {:?}", remainder);
            }
            line
        })
        .collect::<Vec<_>>();
    // eprintln!("parsed\n{:#?}", session);

    let reserialized_lines = parsed_lines
        .iter()
        .map(ToString::to_string)
        .collect::<String>();
    // eprintln!("reserialized\n{}", reserialized);
    let reparsed_lines = sdp_lines(&reserialized_lines).collect::<Vec<_>>();

    pretty_assertions::assert_eq!(parsed_lines, reparsed_lines);
}
