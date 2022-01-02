use sdp_rs::{attributes::AttributeLine, lines::SessionLine, LazySession};

fn read_from_args() -> Option<LazySession<'static>> {
    if let Some(arg) = std::env::args().nth(1) {
        println!("🎅 {}", arg);
        if let Ok(content) = std::fs::read_to_string(arg) {
            Some(LazySession::read_str(&content).into_owned())
        } else {
            None
        }
    } else {
        println!("no input! please pass a file path as first parameter");
        None
    }
}

fn main() {
    let session = read_from_args().unwrap();
    for line in session.lines {
        println!("{:?}", line);
    }

    for msection in session.media {
        println!("{:?}", msection.mline);

        println!("  fingerprints");
        for line in msection.attributes_where(AttributeLine::into_fingerprint) {
            // println!("    {}", lines);
            println!("    {:?}", line);
        }

        println!("  connection");
        for line in msection.lines_where(SessionLine::into_connection) {
            // println!("    {}", lines);
            println!("    {:?}", line);
        }

        println!("  lines");
        for line in msection.lines() {
            println!("    {:?}", line);
        }

        println!("  attributes");
        for attribute in msection.attributes() {
            println!("    {:?}", attribute);
            println!("    {}\n", attribute);
        }
    }

    // let attributes: Vec<_> = msection.attributes().collect();
    // dbg!(attributes);
}
