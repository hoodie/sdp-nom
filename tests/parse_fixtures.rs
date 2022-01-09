use std::{
    env,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

use sdp_nom::Session;

fn with_all_fixtures<F>(
    sub_folder: impl AsRef<Path>,
    f: F,
) -> Result<(), Box<dyn std::error::Error>>
where
    F: Fn(&dyn AsRef<Path>), // -> Result<(), Box<dyn std::error::Error>>,
{
    let fixture_path = dbg!(PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("fixtures")
        .join(sub_folder));

    for path in fs::read_dir(fixture_path)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension().and_then(OsStr::to_str) == Some("sdp"))
        .map(|entry| entry.path())
    {
        eprintln!("fixture: {:?}", path.display());
        f(&path);
    }

    Ok(())
}

#[test]
fn parse_fixtures_root() {
    with_all_fixtures("", |path| {
        let fixture = std::fs::read_to_string(&path).unwrap();
        let session = Session::read_str(&fixture).into_owned();
        let reserialized = session.to_string();
        pretty_assertions::assert_eq!(fixture, reserialized)
    })
    .unwrap();
}
