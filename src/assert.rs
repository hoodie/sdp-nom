use std::fmt;

pub fn print_result<T: fmt::Debug>(input: &str, rest: &str, result: &T) {
    println!(
        "INPUT: {:?}\nLEFT:  {:?}\nRESULT: {:#?}",
        input, rest, result
    );
}

pub fn print_leftover(input: &str, rest: &str) {
    println!("INPUT: {:?}\nLEFT:  {:?}", input, rest);
}

trait AssertLinify {
    fn assert_linify(_: Self);
}

impl<T: fmt::Display> AssertLinify for T {
    fn assert_linify(_: T) {
        panic!("{:?} is Debug", std::any::type_name::<T>());
    }
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
        cfg_if::cfg_if! {
            if #[cfg(feature = "debug")] {
                crate::assert::print_result($line, &rest, &parsed);
                pretty_assertions::assert_eq!(parsed, $expectation, "{:?} not parsed as expected", $line);
            } else {
                crate::assert::print_leftover($line, &rest);
                assert!(parsed == $expectation);
            }
        }
        assert!(rest.is_empty(), "not parsed completely");
    }};

    ($parser:ident, $line:expr, $expectation:expr, print) => {{
        let (rest, parsed) = $parser(&$line).unwrap();
        
        cfg_if::cfg_if! {
            if #[cfg(feature = "debug")] {
                crate::assert::print_result($line, &rest, &parsed);
                pretty_assertions::assert_eq!(parsed, $expectation, "{:?} not parsed as expected", $line);
            } else {
                crate::assert::print_leftover($line, &rest);
                assert!(parsed == $expectation);
            }
        }
        assert!(rest.is_empty(), "not parsed completely");

        cfg_if::cfg_if! {
            if #[cfg(feature = "udisplay")] {
                let serialized = {
                    let mut output = String::new();
                    ufmt::uwrite!(output, "{}", parsed).unwrap();
                    output
                };

                pretty_assertions::assert_eq!($line, serialized);
            }
        }
    }};
}

// just for debugging, prints and fails
#[macro_export]
macro_rules! assert_line_dbg {
    ($parser:ident, $line:expr) => {{
        let (rest, _parsed) = $parser(&$line).unwrap();
        if !rest.is_empty() {
            crate::assert::print_leftover($line, rest);
        }
        panic!("{:#?}", _parsed);
    }};
}

#[macro_export]
macro_rules! assert_line_print {
    ($parser:ident, $line:expr) => {{
        let (rest, _parsed) = $parser(&$line).unwrap();
        if !rest.is_empty() {
            crate::assert::print_leftover($line, rest);
        }
        assert!(rest.is_empty(), "not parsed completely");

        cfg_if::cfg_if! {
            if #[cfg(feature = "udisplay")] {
                let serialized = {
                    let mut output = String::new();
                    ufmt::uwrite!(output, "{}", _parsed).unwrap();
                    output
                };
                assert_eq!($line, serialized);
            }
        }
    }};
}
