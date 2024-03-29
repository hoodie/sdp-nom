use sdp_nom::{attributes::CandidateProtocol, Session};

fn read_from_args() -> Option<Session<'static>> {
    if let Some(arg) = std::env::args().nth(1) {
        if let Ok(content) = std::fs::read_to_string(arg) {
            Some(Session::read_str(&content).into_owned())
        } else {
            None
        }
    } else {
        println!("no input! please pass a file path as first parameter");
        None
    }
}

fn main() {
    let mut session = read_from_args().unwrap();
    session.media = session
        .media
        .into_iter()
        .map(|mut media| {
            media.candidates.retain(|c| c.protocol == CandidateProtocol::Tcp);
            media
        })
        .collect();

    cfg_if::cfg_if! {
        if #[cfg(feature = "serde")] {
            println!("{}", serde_json::to_string_pretty(&session).unwrap());
        } else {
            // println!("{:#?}", session);
            print!("{}", session.to_string());
        }
    }
}
