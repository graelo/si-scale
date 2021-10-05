//! # pretty-units
//!
//! [![crate](https://img.shields.io/crates/v/pretty-units.svg)](https://crates.io/crates/pretty-units)
//! [![documentation](https://docs.rs/pretty-units/badge.svg)](https://docs.rs/pretty-units)
//! [![minimum rustc 1.8](https://img.shields.io/badge/rustc-1.50+-red.svg)](https://rust-lang.github.io/rfcs/2495-min-rust-version.html)
//! [![build status](https://github.com/u0xy/pretty-units/workflows/master/badge.svg)](https://github.com/u0xy/pretty-units/actions)
//!
//!
//! Format value with units according to SI ([système international d'unités](https://en.wikipedia.org/wiki/International_System_of_Units)).
//!
//! _Version requirement: rustc 1.50+_
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
//! ```rust
//! // use pretty_units::{units, units::seconds};
//!
//! // #[derive(Debug)]
//! // struct Sample {
//! //     #[units(seconds)]
//! //     value: u16,
//! // }
//! use std::convert::From;
//! use pretty_units::prelude::*;
//!
//! let actual = Value::from(0.123);
//! let expected = Value {
//!     mantissa: 123f64,
//!     base: Base::B1000,
//!     prefix: Some(Prefix::Milli),
//! };
//! assert_eq!(actual, expected);
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
    pub use crate::prefix::{Allowed, Prefix};
    pub use crate::value::Value;
}
