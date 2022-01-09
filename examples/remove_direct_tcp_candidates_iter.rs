use sdp_nom::{attributes::CandidateProtocol, sdp_lines, ufmt_to_string};

fn read_from_args() -> Option<String> {
    if let Some(arg) = std::env::args().nth(1) {
        if let Ok(content) = std::fs::read_to_string(arg) {
            Some(content)
        } else {
            None
        }
    } else {
        println!("no input! please pass a file path as first parameter");
        None
    }
}

fn main() {
    let content = read_from_args().unwrap();
    for line in sdp_lines(&content).filter(|line| {
        line.as_attribute()
            .and_then(|a| a.as_candidate())
            .map(|candidate| candidate.protocol != CandidateProtocol::Tcp)
            .unwrap_or(true)
    }) {
        cfg_if::cfg_if! {
            if #[cfg(feature = "serde")] {
                println!("{}", serde_json::to_string_pretty(&line).unwrap());
            } else {
                println!("{}", ufmt_to_string(&line));
            }
        }
    }
}
