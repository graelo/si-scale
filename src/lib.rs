//! # si-scale
//!
//! [![crate](https://img.shields.io/crates/v/si-scale.svg)](https://crates.io/crates/si-scale)
//! [![documentation](https://docs.rs/si-scale/badge.svg)](https://docs.rs/si-scale)
//! [![minimum rustc 1.8](https://img.shields.io/badge/rustc-1.50+-red.svg)](https://rust-lang.github.io/rfcs/2495-min-rust-version.html)
//! [![build status](https://github.com/u0xy/si-scale/workflows/master/badge.svg)](https://github.com/u0xy/si-scale/actions)
//!
//!
//! Format value with units according to SI ([système international d'unités](https://en.wikipedia.org/wiki/International_System_of_Units)).
//!
//! _Version requirement: rustc 1.50+_
//!
//! ```toml
//! [dependencies]
//! si-scale = "0.1"
//! ```
//!
//!
//! ## Examples
//!
//! Auto-format struct members:
//!
//! ```rust
//! // use si_scale::{units, units::seconds};
//!
//! // #[derive(Debug)]
//! // struct Sample {
//! //     #[units(seconds)]
//! //     value: u16,
//! // }
//! ```
//!
//!
//! # Overview
//!
//! This crate parses and formats numbers using the SI scales: from 1 y
//! (yocto, i.e. 1e-24) to 1 Y (Yotta, i.e. 1e24). It is agnostic of units
//! per-se; you can totally keep representing units with strings or
//! [uom](https://crates.io/crates/uom), or something else.
//!
//! The central representation is the `Value` type, which holds the mantissa,
//! the SI unit prefix (equivalent to an exponent), and the base which
//! represents the cases where "1 k" means 1000 (most common) and the cases
//! where "1 k" means 1024 (for kiB, MiB, etc).
//!
//!
//! ## The low-level function `Value::new()`
//!
//! The low-level function `Value::new()` converts any number convertible to
//! f64 into a `Value` using base 1000. The `Value` struct implements `From`
//! for common numbers and delegates to `Value::new()`, so they are equivalent
//! in practice. Here are a few examples.
//!
//! ```rust
//! use std::convert::From;
//! use si_scale::prelude::*;
//!
//! let actual = Value::from(0.123);
//! let expected = Value {
//!     mantissa: 123f64,
//!     prefix: Some(Prefix::Milli),
//!     base: Base::B1000,
//! };
//! assert_eq!(actual, expected);
//! assert_eq!(Value::new(0.123), expected);
//!
//! let actual: Value = 0.123.into();
//! assert_eq!(actual, expected);
//!
//! let actual: Value = 1300i32.into();
//! let expected = Value {
//!     mantissa: 1.3f64,
//!     prefix: Some(Prefix::Kilo),
//!     base: Base::B1000,
//! };
//! assert_eq!(actual, expected);
//!
//! let actual: Vec<Value> = vec![0.123f64, -1.5e28]
//!     .iter().map(|n| n.into()).collect();
//! let expected = vec![
//!     Value {
//!         mantissa: 123f64,
//!         prefix: Some(Prefix::Milli),
//!         base: Base::B1000,
//!     },
//!     Value {
//!         mantissa: -1.5e4f64,
//!         prefix: Some(Prefix::Yotta),
//!         base: Base::B1000,
//!     },
//! ];
//! assert_eq!(actual, expected);
//! ```
//!
//! As you can see in the last example, values which scale are outside of the
//! SI prefixes are represented using the closest SI prefix.
//!
//!
//! ## The low-level function `Value::new_with()`
//!
//! The low-level `Value::new_with()` performs the same as `Value::new()`
//! but also expects a base and constraints on the scales you want to use. In
//! comparison with the simple `Value::new()`, this allows base 1024 scaling
//! (for kiB, MiB, etc) and preventing upper scales for seconds or lower
//! scales for integral units such as bytes (e.g. avoid writing 1300 sec as
//! 1.3 ks or 0.415 B as 415 mB).
//!
//! ```rust
//! use si_scale::prelude::*;
//!
//! // Assume this is seconds, no kilo-seconds make sense.
//! let actual = Value::new_with(1234, Base::B1000, Some(&Constraint::UnitAndBelow));
//! let expected = Value {
//!     mantissa: 1234f64,
//!     prefix: Some(Prefix::Unit),
//!     base: Base::B1000,
//! };
//! assert_eq!(actual, expected);
//! ```
//!
//! Don't worry yet about the verbosity, the following parser helps with this.
//!
//!
//! ## Parser
//!
//! The parser ...
//!
//!
//! ## Run code-coverage
//!
//! Install the llvm-tools-preview component and grcov
//!
//! ```sh
//! rustup component add llvm-tools-preview
//! cargo install grcov
//! ```
//!
//! Install nightly
//!
//! ```sh
//! rustup toolchain install nightly
//! ```
//!
//! The following make invocation will switch to nigthly run the tests using
//! Cargo, and output coverage HTML report in `./coverage/`
//!
//! ```sh
//! make coverage
//! ```
//!
//! The coverage report is located in `./coverage/index.html`
//!
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

#[derive(Debug, PartialEq, Eq)]
pub enum SIUnitsError {
    ExponentParsing(String),
}

pub type Result<T> = std::result::Result<T, SIUnitsError>;

pub mod base;
pub mod prefix;
pub mod value;

pub mod prelude {
    pub use crate::base::Base;
    pub use crate::prefix::{Constraint, Prefix};
    pub use crate::value::Value;
}
