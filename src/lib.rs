//! # pretty-units
//!
//! [![crate](https://img.shields.io/crates/v/pretty-units.svg)](https://crates.io/crates/pretty-units)
//! [![documentation](https://docs.rs/pretty-units/badge.svg)](https://docs.rs/pretty-units)
//! [![minimum rustc 1.8](https://img.shields.io/badge/rustc-1.25+-red.svg)](https://rust-lang.github.io/rfcs/2495-min-rust-version.html)
//! [![build status](https://github.com/u0xy/pretty-units/workflows/master/badge.svg)](https://github.com/u0xy/pretty-units/actions)
//!
//!
//! Format value with units according to SI ([système international d'unités](https://en.wikipedia.org/wiki/International_System_of_Units)).
//!
//! _Version requirement: rustc 1.26+_
//!
//! ```toml
//! [dependencies]
//! pretty-units = "0.1"
//! ```
//!
//!
//! ## Examples
//!
//! Auto-format struct members:
//!
//! ```rust,ignore
//! use pretty_units::{units, units::seconds};
//!
//! #[derive(Debug)]
//! struct Sample {
//!     #[units(seconds)]
//!     value: u16,
//! }
//! ```
//!
//!
//! ## License
//!
//! Licensed under either of
//!
//!  * [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
//!  * [MIT license](http://opensource.org/licenses/MIT)
//!
//! at your option.
//!
//!
//! ### Contribution
//!
//! Unless you explicitly state otherwise, any contribution intentionally submitted
//! for inclusion in the work by you, as defined in the Apache-2.0 license, shall
//! be dual licensed as above, without any additional terms or conditions.

/// Placeholder struct.
pub struct Unit {}
