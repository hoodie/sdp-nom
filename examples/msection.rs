use sdp_rs::{attributes::AttributeLine, session::SessionLine, Session};

fn read_from_args() -> Option<Session<'static>> {
    if let Some(arg) = std::env::args().nth(1) {
        println!("🎅 {}", arg);
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
    let session = read_from_args().unwrap();
    for line in session.lines {
        println!("{:?}", line);
    }

    for msection in session.media {
        println!("{:?}", msection.mline);

        println!("  fingerprints");
        for lines in msection.attributes_where(AttributeLine::into_fingerprint) {
            // println!("    {}", lines);
            println!("    {:?}", lines);
        }

        println!("  connection");
        for lines in msection.lines_where(SessionLine::into_connection) {
            // println!("    {}", lines);
            println!("    {:?}", lines);
        }

        println!("  lines");
        for lines in msection.lines() {
            println!("    {:?}", lines);
        }

        println!("  attributes");
        for attribute in msection.attributes() {
            println!("    {:?}", attribute);
        }
    }

    // let attributes: Vec<_> = msection.attributes().collect();
    // dbg!(attributes);
}
