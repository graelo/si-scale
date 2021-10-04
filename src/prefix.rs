use std::convert::TryFrom;
use std::str::FromStr;

use crate::{Result, SIUnitsError};

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
    /// multiplication factor.
    ///
    /// For instance, if self is `-12` ("pico"), then `base_exponent()`
    /// returns 4, for `1000.pow(-4)` to be the multiplication factor:
    /// `1e-12`.
    pub fn base_exponent(&self) -> i32 {
        *self as i32 / 3
    }
}

impl FromStr for Prefix {
    type Err = SIUnitsError;

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

impl TryFrom<i32> for Prefix {
    type Error = SIUnitsError;

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
