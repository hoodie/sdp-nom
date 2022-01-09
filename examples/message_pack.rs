#[cfg(feature = "msg_pack")]
fn read_from_args() -> Option<(Session<'static>, String)> {
    if let Some(arg) = std::env::args().nth(1) {
        if let Ok(content) = std::fs::read_to_string(arg) {
            Some((Session::read_str(&content).into_owned(), content))
        } else {
            None
        }
    } else {
        println!("no input! please pass a file path as first parameter");
        None
    }
}

fn main() {
    cfg_if::cfg_if! {
        if #[cfg(feature = "msg_pack")] {
            use pretty_assertions::assert_eq;
            use sdp_nom::Session;
            let (session, content) = read_from_args().unwrap();

            let packed = rmp_serde::to_vec(&session).unwrap();

            let text_size = content.as_bytes().len() as u32;
            let pack_size = packed.len() as u32;

            println!("text size:   {:?}B", text_size);
            println!("packed size: {:?}B", pack_size);
            println!(
                "ratio:       {:?}B",
                f64::from(pack_size) / f64::from(text_size)
            );

            let new_session: Session = rmp_serde::from_read_ref(&packed).unwrap();
            assert_eq!(session, new_session);
            assert_eq!(content, new_session.to_string());
        } else {
            panic!("please use --feature \"msg_pack\"")
        }
    }
}
