<div align="center">

# sdp-nom

[![build](https://img.shields.io/github/workflow/status/hoodie/sdp-nom/Continuous%20Integration)](https://github.com/hoodie/sdp-nom/actions?query=workflow%3A"Continuous+Integration")
[![Crates.io](https://img.shields.io/crates/d/sdp-nom)](https://crates.io/crates/sdp-nom)
[![contributors](https://img.shields.io/github/contributors/hoodie/sdp-nom)](https://github.com/hoodie/sdp-nom/graphs/contributors)
![maintenance](https://img.shields.io/maintenance/yes/2022)

[![version](https://img.shields.io/crates/v/sdp-nom)](https://crates.io/crates/sdp-nom/)
[![documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/sdp-nom/)
[![license](https://img.shields.io/crates/l/sdp-nom.svg?style=flat)](https://crates.io/crates/sdp-nom/)

</div>

This is a [nom][]-based SDP parser.
It can parse and reserialize and is currently optimized for a very small wasm footprint (hence the use of [ufmt][]).
You don't need reserialization? Just build with `--no-default-features`.

## Why?

There is already [mozilla's webrtc-sdp][mozilla-sdp], [sdp from webrtc.rs][webrtc_rs] and [sdp-types],
which are all very good, why make another?
This is still very much a "for fun" project, so don't mind me!

## cargo-features

| name     | what it does                                  | default |
| -------- | --------------------------------------------- | ------- |
| udisplay | use [ufmt][] to reserialize session and lines | **yes** |
| debug    | provide `Debug` formatting for all types      | **yes** |
| serde    | well serde support of course                  | no      |
| wee      | use [wee][] allocator                         | no      |

## Objectives

With this parser we try to implement a wasm-friendly, low-copy and high-level-nom parser.

### WASM-Friendly

By using [wee][] and [ufmt][] we try to achieve a small binary size.
Further Debug-printing is a feature that can be disabled.
Further concrete wasm-related work is on the horizon.

### low copy

Zero copy seems a bit out of reach but this parser tries to enable as much as possible without actually copying over the content of the sdp.
This is achieved by reading any string into a [`Cow`][] and only optionally creating a `'static` copy of the content.

### functional high-level parser-combinators

SDP is a weird standard,
there is no container format specification other than that lines start with a `char` followed by `=`.
Basically every line follows it's own rules.
Therefore every line has its own parser like in this example:

```rust
/// Email `e=<email-address>`
pub struct EmailAddress<'a>(pub Cow<'a, str>);

/// "e=email@example.com"
pub fn email_address_line(input: &str) -> IResult<&str, EmailAddress> {
    line("e=", wsf(map(cowify(read_string), EmailAddress)))(input)
}
```

## License

icalendar-rs is licensed under either of

* Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Any help in form of descriptive and friendly [issues](https://github.com/hoodie/sdp-nom/issues) or comprehensive pull requests are welcome! 

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in sdp-nom by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[nom]: https://docs.rs/nom
[ufmt]: https://docs.rs/ufmt
[wee]: https://docs.rs/wee_alloc
[mozilla-sdp]: https://crates.io/crates/webrtc-sdp
[webrtc_rs]: https://crates.io/crates/sdp
[sdp-types]: https://crates.io/crates/sdp-types
[`cow`]: https://doc.rust-lang.org/stable/std/borrow/enum.Cow.html

