use std::fmt::Debug;

pub(crate) fn print_result<T: Debug>(input: &str, rest: &str, result: &T) {
    println!(
        "INPUT: {:?}\nLEFT:  {:?}\nRESULT: {:#?}",
        input, rest, result
    );
}

pub(crate) fn print_leftover(input: &str, rest: &str) {
    println!("INPUT: {:?}\nLEFT:  {:?}", input, rest);
}

#[macro_export]
macro_rules! assert_line {
    ($line:expr) => {{
        let (rest, _parsed) = crate::sdp_line(&$line).unwrap();
        if !rest.is_empty() {
            crate::assert::print_leftover($line, rest);
        }
        assert!(rest.is_empty(), "not parsed completely");
    }};

    ($parser:ident, $line:expr) => {
        assert_line!($line, $parser);
    };

    ($line:expr, $parser:ident) => {{
        let (rest, _parsed) = $parser(&$line).unwrap();
        if !rest.is_empty() {
            crate::assert::print_leftover($line, rest);
        }
        assert!(rest.is_empty(), "not parsed completely");
    }};

    ($parser:ident, $line:expr, $expectation:expr) => {{
        let (rest, parsed) = $parser(&$line).unwrap();
        crate::assert::print_result($line, &rest, &parsed);
        pretty_assertions::assert_eq!(parsed, $expectation, "{:?} not parsed as expected", $line);
        assert!(rest.is_empty(), "not parsed completely");
    }};
}
