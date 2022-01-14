use std::{
    env,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

use sdp_nom::{sdp_lines, sdp_lines_all, Session};

fn with_all_fixtures<F>(
    sub_folder: impl AsRef<Path>,
    f: F,
) -> Result<(), Box<dyn std::error::Error>>
where
    F: Fn(&Path), // -> Result<(), Box<dyn std::error::Error>>,
{
    let fixture_path = dbg!(PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("fixtures")
        .join(sub_folder));

    for path in fs::read_dir(fixture_path)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension().and_then(OsStr::to_str) == Some("sdp"))
        .map(|entry| entry.path())
    {
        f(&path);
    }

    Ok(())
}

#[test]
#[cfg(feature = "udisplay")]
fn parse_fixtures_root() {
    with_all_fixtures("", |path| {
        let fixture = std::fs::read_to_string(&path).unwrap();
        let session = Session::read_str(&fixture);
        eprintln!("parsed\n{:#?}", session);

        let reserialized = session.to_string();
        eprintln!("reserialized\n{}", reserialized);
        let reparsed = Session::read_str(&reserialized);

        eprintln!("fixture: {:?}", path.display());
        pretty_assertions::assert_eq!(fixture, reserialized);
        pretty_assertions::assert_eq!(session, reparsed);
    })
    .unwrap();
}

#[test]
#[cfg(feature = "udisplay")]
fn parse_fixtures_sdp_transform() {
    with_all_fixtures("sdp_transform", |path| {
        let fixture = std::fs::read_to_string(&path).unwrap();
        let session = Session::read_str(&fixture);
        eprintln!("parsed\n{:#?}", session);

        let reserialized = session.to_string();
        eprintln!("reserialized\n{}", reserialized);
        let reparsed = Session::read_str(&reserialized);

        eprintln!("fixture: {:?}", path.display());
        pretty_assertions::assert_eq!(session, reparsed);
    })
    .unwrap();
}

#[test]
#[cfg(feature = "udisplay")]
fn parse_fixtures_sdp_transform_lazy() {
    with_all_fixtures("sdp_transform", |path| {
        let fixture = std::fs::read_to_string(&path).unwrap();
        let parsed_lines = sdp_lines_all(&fixture)
            .map(|res| {
                eprintln!("fixture: {:?}", path.display());
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

        eprintln!("fixture: {:?}", path.display());
        pretty_assertions::assert_eq!(parsed_lines, reparsed_lines);
    })
    .unwrap();
}
