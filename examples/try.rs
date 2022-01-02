use sdp_rs::LazySession;
use std::convert::TryFrom;

fn main() {
    let mut err_count = 1;
    if let Some(arg) = std::env::args().nth(1) {
        if let Ok(content) = std::fs::read_to_string(arg) {
            err_count = 0;
            LazySession::try_from(&content).unwrap();
        }
    }
    println!("{} errors", err_count);
    std::process::exit(err_count);
}
