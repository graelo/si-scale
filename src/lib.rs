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
//! # Getting started
//!
//! This crate parses and formats numbers using the [SI
//! scales](https://en.wikipedia.org/wiki/International_System_of_Units): from
//! 1 y (yocto, i.e. 1e-24) to 1 Y (Yotta, i.e. 1e24). It is agnostic of units
//! per-se; you can totally keep representing units with strings or
//! [uom](https://crates.io/crates/uom), or something else.
//!
//! You can use one of the predefined helper functions to format numbers:
//! [`seconds()`][`crate::helpers::seconds()`],
//! [`bytes()`][`crate::helpers::bytes()`],
//! [`bibytes()`][`crate::helpers::bibytes()`]:
//!
//! ```
//! use si_scale::helpers::{seconds, seconds3};
//!
//! let actual = format!("{}", seconds(1.3e-5));
//! let expected = "13 µs";
//! assert_eq!(actual, expected);
//!
//! let actual = format!("{}", seconds3(1.3e-5));
//! let expected = "13.000 µs";
//! assert_eq!(actual, expected);
//! ```
//!
//! To define your own format function, use the
//! [`scale_fn!()`][`crate::scale_fn!()`] macro. For instance, let's define a
//! formatting function for bits per sec which prints the mantissa with 2
//! decimals, and also uses base 1024 (where 1 ki = 1024). Note that although
//! we define the function in a separate module, this is not a requirement.
//!
//! ```
//! mod unit_fmt {
//!     use si_scale::scale_fn;
//!     use si_scale::prelude::Value;
//!
//!     // defines the `bits_per_sec()` function
//!     scale_fn!(bits_per_sec,
//!               base: B1024,
//!               constraint: UnitAndAbove,
//!               mantissa_fmt: "{:.2}",
//!               groupings: '_',
//!               unit: "bit/s");
//! }
//!
//! use unit_fmt::bits_per_sec;
//!
//! fn main() {
//!     let x = 2.1 * 1024 as f32;
//!     let actual = format!("throughput: {:>15}", bits_per_sec(x));
//!     let expected = "throughput:    2.10 kibit/s";
//!     assert_eq!(actual, expected);
//!
//!     let x = 2;
//!     let actual = format!("throughput: {}", bits_per_sec(x));
//!     let expected = "throughput: 2.00 bit/s";
//!     assert_eq!(actual, expected);
//! }
//!
//! ```
//!
//! # Overview
//!
//! The central representation is the [`Value`][`crate::value::Value`] type,
//! which holds
//!
//! - the mantissa,
//! - the SI unit prefix (such as "kilo", "Mega", etc),
//! - and the base which represents the cases where "1 k" means 1000 (most
//! common) and the cases where "1 k" means 1024 (for kiB, MiB, etc).
//!
//! This crate provides 2 APIs: a low-level API, and a high-level API for
//! convenience.
//!
//! For the low-level API, the typical use case is
//!
//! - first parse a number into a [`Value`][`crate::value::Value`]. For doing
//! this, you have to specify the base, and maybe some constraint on the SI
//! scales. See [`Value::new()`][`crate::value::Value::new()`] and
//! [`Value::new_with()`][`crate::value::Value::new_with()`]
//!
//! - then display the `Value` either by yourself formatting the mantissa
//!   and prefix (implements the `fmt::Display` trait), or using the provided
//!   Formatter.
//!
//! For the high-level API, the typical use cases are
//!
//! 1. parse and display a number using the provided functions such as
//!    `bibytes()`, `bytes()` or `seconds()`, they will choose for each number
//!    the most appropriate SI scale.
//!
//! 2. In case you want the same control granularity as the low-level API
//!    (e.g. constraining the scale in some way, using some base, specific
//!    mantissa formatting), then you can build a custom function using the
//!    provided macro `scale_fn!()`. The existing functions such as
//!    `bibytes()`, `bytes()`, `seconds()` are all built using this same
//!    macro.
//!
//!
//! ## The high-level API
//!
//! The `seconds3()` function parses a number into a `Value` and displays it
//! using 3 decimals and the appropriate scale for seconds (`UnitAndBelow`),
//! so that non-sensical scales such as kilo-seconds may not appear. The
//! `seconds()` function does the same but formats the mantissa with the
//! default `"{}"`, so no decimals are printed for integer mantissa.
//!
//! ```
//! use si_scale::helpers::{seconds, seconds3};
//!
//! let actual = format!("result is {:>15}", seconds(1234.5678));
//! let expected = "result is     1234.5678 s";
//! assert_eq!(actual, expected);
//!
//! let actual = format!("result is {:>10}", seconds3(12.3e-7));
//! let expected = "result is   1.230 µs";
//! assert_eq!(actual, expected);
//! ```
//!
//! The `bytes()` function parses a number into a `Value` *using base 1000*
//! and displays it using 1 decimal and the appropriate scale for bytes
//! (`UnitAndAbove`), so that non-sensical scales such as milli-bytes may not
//! appear.
//!
//! ```
//! use si_scale::helpers::{bytes, bytes1};
//!
//! let actual = format!("result is {}", bytes1(12_345_678));
//! let expected = "result is 12.3 MB";
//! assert_eq!(actual, expected);
//!
//! let actual = format!("result is {:>10}", bytes(16));
//! let expected = "result is       16 B";
//! assert_eq!(actual, expected);
//!
//! let actual = format!("result is {}", bytes(0.12));
//! let expected = "result is 0.12 B";
//! assert_eq!(actual, expected);
//! ```
//!
//! The `bibytes1()` function parses a number into a `Value` *using base 1024*
//! and displays it using 1 decimal and the appropriate scale for bytes
//! (`UnitAndAbove`), so that non-sensical scales such as milli-bytes may not
//! appear.
//!
//! ```
//! use si_scale::helpers::{bibytes, bibytes1};
//!
//! let actual = format!("result is {}", bibytes1(12_345_678));
//! let expected = "result is 11.8 MiB";
//! assert_eq!(actual, expected);

//! let actual = format!("result is {}", bibytes(16 * 1024));
//! let expected = "result is 16 kiB";
//! assert_eq!(actual, expected);

//! let actual = format!("result is {:>10}", bibytes1(16));
//! let expected = "result is     16.0 B";
//! assert_eq!(actual, expected);

//! let actual = format!("result is {}", bibytes(0.12));
//! let expected = "result is 0.12 B";
//! assert_eq!(actual, expected);
//! ```
//!
//!
//! ## The low-level API
//!
//! ### Creating a `Value` with `Value::new()`
//!
//! The low-level function [`Value::new()`][`crate::value::Value::new()`]
//! converts any number convertible to f64 into a `Value` using base 1000. The
//! `Value` struct implements `From` for common numbers and delegates to
//! `Value::new()`, so they are equivalent in practice. Here are a few
//! examples.
//!
//! ```rust
//! use std::convert::From;
//! use si_scale::prelude::*;
//!
//! let actual = Value::from(0.123);
//! let expected = Value {
//!     mantissa: 123f64,
//!     prefix: Prefix::Milli,
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
//!     prefix: Prefix::Kilo,
//!     base: Base::B1000,
//! };
//! assert_eq!(actual, expected);
//!
//! let actual: Vec<Value> = vec![0.123f64, -1.5e28]
//!     .iter().map(|n| n.into()).collect();
//! let expected = vec![
//!     Value {
//!         mantissa: 123f64,
//!         prefix: Prefix::Milli,
//!         base: Base::B1000,
//!     },
//!     Value {
//!         mantissa: -1.5e4f64,
//!         prefix: Prefix::Yotta,
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
//! ### Creating a `Value` with `Value::new_with()`
//!
//! The low-level [`Value::new_with()`][`crate::value::Value::new_with()`]
//! operates similarly to [`Value::new()`][`crate::value::Value::new()`] but
//! also expects a base and a constraint on the scales you want to use. In
//! comparison with the simple `Value::new()`, this allows base 1024 scaling
//! (for kiB, MiB, etc) and preventing upper scales for seconds or lower
//! scales for integral units such as bytes (e.g. avoid writing 1300 sec as
//! 1.3 ks or 0.415 B as 415 mB).
//!
//! ```rust
//! use si_scale::prelude::*;
//!
//! // Assume this is seconds, no kilo-seconds make sense.
//! let actual = Value::new_with(1234, Base::B1000, Constraint::UnitAndBelow);
//! let expected = Value {
//!     mantissa: 1234f64,
//!     prefix: Prefix::Unit,
//!     base: Base::B1000,
//! };
//! assert_eq!(actual, expected);
//! ```
//!
//! Don't worry yet about the verbosity, the following parser helps with this.
//!
//!
//! ### Formatting values
//!
//! In this example, the number `x` is converted into a value and displayed
//! using the most appropriate SI prefix. The user chose to constrain the
//! prefix to be anything lower than `Unit` (1) because kilo-seconds make
//! no sense.
//!
//! ```
//! use si_scale::format_value;
//! # fn main() {
//! use si_scale::{value::Value, base::Base, prefix::Constraint};
//!
//! let x = 1234.5678;
//! let v = Value::new_with(x, Base::B1000, Constraint::UnitAndBelow);
//! let unit = "s";
//!
//! let actual = format!(
//!     "result is {}{u}",
//!     format_value!(v, "{:.5}", groupings: '_'),
//!     u = unit
//! );
//! let expected = "result is 1_234.567_80 s";
//! assert_eq!(actual, expected);
//! # }
//! ```
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
pub mod format;
pub mod helpers;
pub mod prefix;
pub mod value;

pub mod prelude {
    pub use crate::base::Base;
    pub use crate::prefix::{Constraint, Prefix};
    pub use crate::value::Value;
}
