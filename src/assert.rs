use std::fmt::Debug;

pub(crate) fn print_result<T: Debug>(input: &str, rest: &str, result: &T) {
    println!(
        "INPUT: {:?}\nLEFT:  {:?}\nRESULT: {:#?}",
        input, rest, result
    );
}

#[macro_export]
macro_rules! assert_line {
    ($line:expr) => {{
        let (rest, parsed) = crate::raw_sdp_lines(&$line).unwrap();
        crate::assert::print_result($line, rest, &parsed[0]);
        assert!(rest.is_empty(), "not parsed completely");
    }};

    ($parser:ident, $line:expr) => {
        assert_line!($line, $parser);
    };

    ($line:expr, $parser:ident) => {{
        let (rest, parsed) = $parser(&$line).unwrap();
        crate::assert::print_result($line, &rest, &parsed);
        assert!(rest.is_empty(), "not parsed completely");
    }};

    ($parser:ident, $line:expr, $expectation:expr) => {{
        let (rest, parsed) = $parser(&$line).unwrap();
        crate::assert::print_result($line, &rest, &parsed);
        pretty_assertions::assert_eq!(parsed, $expectation, "{:?} not parsed as expected", $line);
        assert!(rest.is_empty(), "not parsed completely");
    }};
}

#[macro_export]
macro_rules! assert_parse {
    ($line:expr, $parser:expr) => {{
        let res: IResult<_, _> = $parser(&$line);
        let (rest, parsed) = res.unwrap();

        crate::assert::print_result($line, &rest, &parsed);
        assert!(rest.is_empty(), "not parsed completely");
    }};
}
