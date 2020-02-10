`seek-ext`: convenience methods for `io::Seek`
==============================================

[![Build Status](https://img.shields.io/travis/LukasKalbertodt/seek-ext/master.svg)](https://travis-ci.org/LukasKalbertodt/seek-ext)
[![crates.io version](https://img.shields.io/crates/v/seek-ext.svg)](https://crates.io/crates/seek-ext)
[![docs](https://docs.rs/seek-ext/badge.svg)](https://docs.rs/seek-ext)

**This crate was created mainly to drive discussion about the inclusion of these convenience methods into the standard library. They already landed in `std`, albeit unstably. See [the tracking issue](https://github.com/rust-lang/rust/issues/59359). I am not planning on actually maintaining this crate, as the goal is to make this crate redundant.**

This crate offers the trait `SeekExt` which is implemented for all `T where T: io::Seek`. That way, some convenience methods are added to all types that implement `io::Seek`.



---

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
