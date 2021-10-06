use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

pub mod allowed;
pub use allowed::Allowed;

use crate::{Result, SIUnitsError};

/// Represents units' [SI prefixes](https://www.bipm.org/en/measurement-units/si-prefixes).
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum Prefix {
    /// "yocto" prefix, 1e-24
    Yocto = -24,
    /// "zepto" prefix, 1e-21
    Zepto = -21,
    /// "atto" prefix, 1e-18
    Atto = -18,
    /// "femto" prefix, 1e-15
    Femto = -15,
    /// "pico" prefix, 1e-12
    Pico = -12,
    /// "nano" prefix, 1e-9
    Nano = -9,
    /// "micro" prefix, 1e-6
    Micro = -6,
    /// "milli" prefix, 1e-3
    Milli = -3,
    /// unit prefix (empty), 1
    Unit = 0,
    /// "kilo" prefix, 1e3
    Kilo = 3,
    /// "mega" prefix, 1e6
    Mega = 6,
    /// "giga" prefix, 1e9
    Giga = 9,
    /// "tera" prefix, 1e12
    Tera = 12,
    /// "peta" prefix, 1e15
    Peta = 15,
    /// "exa" prefix, 1e18
    Exa = 18,
    /// "zetta" prefix, 1e21
    Zetta = 21,
    /// "yotta" prefix, 1e24
    Yotta = 24,
}

impl Prefix {
    /// Returns the exponent `e` for `base.pow(e)` to return the total
    /// scaling factor. See [`Base::pow()`][`crate::base::Base::pow()`].
    ///
    /// For instance,
    ///
    /// - if self is `-12` ("pico"), then `exponent()` returns `-12` so that
    /// `Base::B1000.pow(-12)` returns the scaling factor `1e-12`.
    ///
    /// - if self is `3` ("kilo"), then `exponent()` returns `3` so that
    /// `Base::B1024.pow(3)` returns the scaling factor `1024`.
    pub fn exponent(&self) -> i32 {
        *self as i32
    }
}

impl FromStr for Prefix {
    type Err = SIUnitsError;

    /// Converts a `&str` into a `Prefix` if conversion is successful,
    /// otherwise return an `Err`.
    ///
    /// # Example
    ///
    /// ```
    /// use std::str::FromStr;
    /// use pretty_units::prelude::Prefix;
    /// use pretty_units::Result;
    ///
    /// let actual= Prefix::from_str("y");
    /// let expected = Ok(Prefix::Yocto);
    /// assert_eq!(actual, expected);
    /// ```
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "Yocto" | "yocto" | "y" => Ok(Self::Yocto),
            "Zepto" | "zepto" | "z" => Ok(Self::Zepto),
            "Atto" | "atto" | "a" => Ok(Self::Atto),
            "Femto" | "femto" | "f" => Ok(Self::Femto),
            "Pico" | "pico" | "p" => Ok(Self::Pico),
            "Nano" | "nano" | "n" => Ok(Self::Nano),
            "Micro" | "micro" | "µ" => Ok(Self::Micro),
            "Milli" | "milli" | "m" => Ok(Self::Milli),
            "Kilo" | "kilo" | "k" => Ok(Self::Kilo),
            "Mega" | "mega" | "M" => Ok(Self::Mega),
            "Giga" | "giga" | "G" => Ok(Self::Giga),
            "Tera" | "tera" | "T" => Ok(Self::Tera),
            "Peta" | "peta" | "P" => Ok(Self::Peta),
            "Exa" | "exa" | "E" => Ok(Self::Exa),
            "Zetta" | "zetta" | "Z" => Ok(Self::Zetta),
            "Yotta" | "yotta" | "Y" => Ok(Self::Yotta),
            _ => Err(SIUnitsError::ExponentParsing(s.to_string())),
        }
    }
}

impl From<&Prefix> for &str {
    /// Converts a prefix into its displayable form (`&'static str`).
    ///
    /// # Example
    ///
    /// ```
    /// use pretty_units::prelude::Prefix;
    ///
    /// let pfx = Prefix::Tera;
    /// let a_string = format!("value: {} {}B", 1.5, pfx);
    ///
    /// assert_eq!(a_string, "value: 1.5 TB");
    /// ```
    ///
    fn from(prefix: &Prefix) -> &'static str {
        match prefix {
            Prefix::Yocto => "y",
            Prefix::Zepto => "z",
            Prefix::Atto => "a",
            Prefix::Femto => "f",
            Prefix::Pico => "p",
            Prefix::Nano => "n",
            Prefix::Micro => "µ",
            Prefix::Milli => "m",
            Prefix::Unit => "",
            Prefix::Kilo => "k",
            Prefix::Mega => "M",
            Prefix::Giga => "G",
            Prefix::Tera => "T",
            Prefix::Peta => "P",
            Prefix::Exa => "E",
            Prefix::Zetta => "Z",
            Prefix::Yotta => "Y",
        }
    }
}

impl fmt::Display for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: &'static str = self.into();
        write!(f, "{}", s)
    }
}

impl TryFrom<i32> for Prefix {
    type Error = SIUnitsError;

    /// Builds a `Prefix` from a `i32` if successful, otherwise returns a
    /// `SIUnitsError::ExponentParsing()` error.
    fn try_from(value: i32) -> Result<Self> {
        match value {
            -24 => Ok(Self::Yocto),
            -21 => Ok(Self::Zepto),
            -18 => Ok(Self::Atto),
            -15 => Ok(Self::Femto),
            -12 => Ok(Self::Pico),
            -9 => Ok(Self::Nano),
            -6 => Ok(Self::Micro),
            -3 => Ok(Self::Milli),
            0 => Ok(Self::Unit),
            3 => Ok(Self::Kilo),
            6 => Ok(Self::Mega),
            9 => Ok(Self::Giga),
            12 => Ok(Self::Tera),
            15 => Ok(Self::Peta),
            18 => Ok(Self::Exa),
            21 => Ok(Self::Zetta),
            24 => Ok(Self::Yotta),
            _ => Err(SIUnitsError::ExponentParsing(format!(
                "Provided value should be a multiple of 3, between -24 and 24, got `{}` instead",
                value
            ))),
        }
    }
}

// Currently commented out because not really useful.
//
// macro_rules! impl_try_from_num_for_siprefix {
//     ($t:ty) => {
//         impl TryFrom<$t> for Prefix {
//             type Error = SIUnitsError;

//             fn try_from(value: $t) -> Result<Self> {
//                 Prefix::try_from(value as i32)
//             }
//         }
//     };
// }

// impl_try_from_num_for_siprefix!(u8);
// impl_try_from_num_for_siprefix!(i8);
// impl_try_from_num_for_siprefix!(u16);
// impl_try_from_num_for_siprefix!(i16);
// impl_try_from_num_for_siprefix!(u32);
// // impl_try_from_num_for_siprefix!(i32);
// impl_try_from_num_for_siprefix!(u64);
// impl_try_from_num_for_siprefix!(i64);
// impl_try_from_num_for_siprefix!(usize);
// impl_try_from_num_for_siprefix!(isize);
// impl_try_from_num_for_siprefix!(f32);
// impl_try_from_num_for_siprefix!(f64);
