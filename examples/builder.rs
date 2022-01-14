use std::borrow::Cow;

use sdp_nom::{
    lines::{session_name::SessionName, version::Version},
    session::SessionBuilder,
    Session,
};

fn main() {
    let session = SessionBuilder::default()
        .version(Version(0))
        .name(SessionName(Cow::from("-")))
        .build();

    println!("{:#?}", session);
}
