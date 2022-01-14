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


If you are interested please contact me.

## features

| name     | what it does                                  | default |
| -------- | --------------------------------------------- | ------- |
| udisplay | use [ufmt][] to reserialize session and lines | **yes** |
| serde    | well serde support of course                  | no      |
| wee      | use [wee][] allocator                         | no      |

[nom]: https://docs.rs/nom
[ufmt]: https://docs.rs/ufmt
[wee]: https://docs.rs/wee_alloc
