#![warn(missing_docs)]
#![allow(clippy::needless_doctest_main)]
#![doc = include_str!("../README.md")]

/// Error type used by this crate.
#[derive(Debug, PartialEq, Eq)]
pub enum SIUnitsError {
    /// Indicates an error occurred when parsing the exponent.
    ExponentParsing(String),
}

/// Result type used by this crate.
pub type Result<T> = std::result::Result<T, SIUnitsError>;

pub mod base;
pub mod format;
pub mod helpers;
pub mod prefix;
pub mod value;

/// Holds first-class citizens of this crate, for convenience.
pub mod prelude {
    pub use crate::base::Base;
    pub use crate::prefix::{Constraint, Prefix};
    pub use crate::value::{IntoF64, Value};
}
