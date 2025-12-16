//! Represents a float value using its mantissa and unit Prefix in a base.
//!
//! With base = 1000, 1k = 1000, 1M = 1_000_000, 1m = 0.001, 1µ = 0.000_001,
//! etc.
//!
//! | min (incl.) | max (excl.)      | magnitude | prefix          |
//! | ---         | ---              | ---       | ----            |
//! | ..          | ..               | -24       | `Prefix::Yocto` |
//! | ..          | ..               | -21       | `Prefix::Zepto` |
//! | ..          | ..               | -18       | `Prefix::Atto`  |
//! | ..          | ..               | -15       | `Prefix::Femto` |
//! | ..          | ..               | -12       | `Prefix::Pico`  |
//! | ..          | ..               | -9        | `Prefix::Nano`  |
//! | 0.000\_001  | 0.001            | -6        | `Prefix::Micro` |
//! | 0.001       | 1                | -3        | `Prefix::Milli` |
//! | 1           | 1_000            | 0         | `Prefix::Unit`  |
//! | 1000        | 1\_000\_000      | 3         | `Prefix::Kilo`  |
//! | 1\_000\_000 | 1\_000\_000\_000 | 6         | `Prefix::Mega`  |
//! | ..          | ..               | 9         | `Prefix::Giga`  |
//! | ..          | ..               | 12        | `Prefix::Tera`  |
//! | ..          | ..               | 15        | `Prefix::Peta`  |
//! | ..          | ..               | 18        | `Prefix::Exa`   |
//! | ..          | ..               | 21        | `Prefix::Zetta` |
//! | ..          | ..               | 24        | `Prefix::Yotta` |
//!
//! The base is usually 1000, but can also be 1024 (bibytes).
//!
//! With base = 1024, 1ki = 1024, 1Mi = 1024 * 1024, etc.
//!
//! # Example
//!
//! ```
//! use std::convert::From;
//! use si_scale::{base::Base, value::Value, prefix::Prefix};
//!
//! let actual = Value::from(0.123);
//! let expected = Value {
//!     mantissa: 123f64,
//!     prefix: Prefix::Milli,
//!     base: Base::B1000,
//! };
//! assert_eq!(actual, expected);
//! ```

use crate::prefix::Prefix;
use std::convert::From;
use std::fmt;

use crate::base::Base;
use crate::prefix::Constraint;

/// A trait for types that can be converted to `f64`.
///
/// This trait enables uniform handling of all numeric types, including those
/// like `u64`, `i64`, `usize`, and `isize` that don't implement `Into<f64>`
/// because the conversion may be lossy.
pub trait IntoF64 {
    /// Converts self to `f64`.
    fn into_f64(self) -> f64;
}

macro_rules! impl_into_f64_lossless {
    ($($t:ty),*) => {
        $(
            impl IntoF64 for $t {
                #[inline]
                fn into_f64(self) -> f64 {
                    self.into()
                }
            }
        )*
    };
}

macro_rules! impl_into_f64_lossy {
    ($($t:ty),*) => {
        $(
            impl IntoF64 for $t {
                #[inline]
                fn into_f64(self) -> f64 {
                    self as f64
                }
            }
        )*
    };
}

impl_into_f64_lossless!(u8, i8, u16, i16, u32, i32, f32, f64);
impl_into_f64_lossy!(u64, i64, usize, isize);

/// Defines the representation of the value.
#[derive(Debug, PartialEq)]
pub struct Value {
    /// Mantissa of the value after scaling.
    pub mantissa: f64,

    /// Prefix indicating the scale.
    pub prefix: Prefix,

    /// Indicates if the base is `1000` or `1024`.
    pub base: Base,
}

impl Value {
    /// Returns a `Value` for the default base `B1000`, meaning `1 k = 1000`,
    /// `1 m = 1e-3`, etc.
    ///
    /// # Example
    ///
    /// ```
    /// use si_scale::prelude::{Base, Prefix, Value};
    ///
    /// let actual = Value::new(-4.6e-5);
    /// let expected = Value {
    ///     mantissa: -46f64,
    ///     prefix: Prefix::Micro,
    ///     base: Base::B1000,
    /// };
    /// assert_eq!(actual, expected);
    /// ```
    ///
    /// # Note
    ///
    /// As always the case in floating point operations, you may encounter
    /// approximate representations. For instance:
    ///
    /// ```
    /// use si_scale::prelude::{Base, Prefix, Value};
    ///
    /// let actual = Value::new(-4.3e-5);
    /// let expected = Value {
    ///     mantissa: -43.00000000000001f64,
    ///     prefix: Prefix::Micro,
    ///     base: Base::B1000,
    /// };
    /// assert_eq!(actual, expected);
    /// ```
    ///
    pub fn new<F>(x: F) -> Self
    where
        F: IntoF64,
    {
        Value::new_with(x, Base::B1000, Constraint::None)
    }

    /// Returns a `Value` for the provided base.
    ///
    /// # Example
    ///
    /// ```
    /// use si_scale::prelude::{Constraint, Base, Prefix, Value};
    ///
    /// // 4 MiB
    /// let actual = Value::new_with(4 * 1024 * 1024, Base::B1024, Constraint::None);
    /// let expected = Value {
    ///     mantissa: 4f64,
    ///     prefix: Prefix::Mega,
    ///     base: Base::B1024,
    /// };
    /// assert_eq!(actual, expected);
    /// ```
    ///
    // #[deprecated(
    //     since = "0.2.0",
    //     note = "please use the `Value::constraint()` and `Value::base()` methods instead"
    // )]
    // #[allow(deprecated)]
    pub fn new_with<F, C>(x: F, base: Base, prefix_constraint: C) -> Self
    where
        F: IntoF64,
        C: AsRef<Constraint>,
    {
        let x: f64 = x.into_f64();

        // Closest integral exponent (multiple of 3)
        let exponent: i32 = base.integral_exponent_for(x);
        // Clamp the exponent using the constraint on prefix
        let prefix = Self::closest_prefix_for(exponent, prefix_constraint);

        let mantissa = x / base.pow(prefix.exponent());

        Value {
            mantissa,
            base,
            prefix,
        }
    }

    /// Converts `self` to a `f64`.
    ///
    /// # Example
    ///
    /// ```
    /// use si_scale::prelude::{Base, Prefix, Value};
    ///
    /// let value = Value {
    ///     mantissa: 1.3f64,
    ///     prefix: Prefix::Unit,
    ///     base: Base::B1000,
    /// };
    /// assert_eq!(value.to_f64(), 1.3);
    /// ```
    ///
    pub fn to_f64(&self) -> f64 {
        let scale = self.base.pow(self.prefix.exponent());
        self.mantissa * scale
    }

    /// Returns a number that represents the sign of self.
    ///
    /// - `1.0` if the number is positive, `+0.0` or `INFINITY`
    /// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    /// - `NAN` if the number is `NAN`
    ///
    /// # Example
    ///
    /// ```
    /// use std::convert::From;
    /// use si_scale::value::Value;
    ///
    /// let number = -1.5e3f32;
    /// let value: Value = number.into();
    ///
    /// assert_eq!(value.signum(), number.signum() as f64);
    /// ```
    ///
    pub fn signum(&self) -> f64 {
        self.mantissa.signum()
    }

    /// Returns the closest prefix for the provided exponent, respecting the
    /// optional constraint.
    ///
    /// # Panics
    ///
    /// - If the `Custom` allowed prefixes is an empty vector.
    ///
    fn closest_prefix_for<C: AsRef<Constraint>>(exponent: i32, constraint: C) -> Prefix {
        use std::convert::TryFrom;

        match constraint.as_ref() {
            Constraint::None => {
                Prefix::try_from(exponent.clamp(Prefix::Yocto as i32, Prefix::Yotta as i32))
                    .unwrap_or(Prefix::Unit)
            }
            Constraint::UnitOnly => Prefix::Unit,
            Constraint::UnitAndAbove => {
                Prefix::try_from(exponent.clamp(Prefix::Unit as i32, Prefix::Yotta as i32))
                    .unwrap_or(Prefix::Unit)
            }
            Constraint::UnitAndBelow => {
                Prefix::try_from(exponent.clamp(Prefix::Yocto as i32, Prefix::Unit as i32))
                    .unwrap_or(Prefix::Unit)
            }
            Constraint::Custom(allowed_prefixes) => {
                if allowed_prefixes.is_empty() {
                    panic!("At least one prefix should be allowed");
                }
                let smallest_prefix = *allowed_prefixes.first().unwrap();
                if exponent < smallest_prefix as i32 {
                    return smallest_prefix;
                }
                allowed_prefixes
                    .iter()
                    .take_while(|&&prefix| prefix as i32 <= exponent)
                    .cloned()
                    .last()
                    .unwrap_or(Prefix::Unit)
            }
        }
    }
}

//
// From Value -> f64
//

impl From<Value> for f64 {
    fn from(value: Value) -> Self {
        value.to_f64()
    }
}

//
// From number -> Value
//

macro_rules! impl_from_num_for_value {
    ($t:ty) => {
        impl From<$t> for Value {
            fn from(x: $t) -> Self {
                Value::new(x)
            }
        }

        impl From<&$t> for Value {
            fn from(x: &$t) -> Self {
                Value::new(*x)
            }
        }
    };
}

//
// Display (simplistic)
//

impl fmt::Display for Value {
    /// A basic but limited way to display the value; it does not allow
    /// mantissa formatting. Consider using the
    /// [`format_value!()`][`crate::format_value`] macro instead.
    ///
    /// # Example
    ///
    /// ```
    /// use std::convert::From;
    /// use si_scale::prelude::Value;
    ///
    /// let value: Value = 5.3e5.into();
    ///
    /// let actual = format!("{}", value);
    /// let expected = "530 k".to_string();
    /// assert_eq!(actual, expected);
    /// ```
    ///
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.prefix {
            Prefix::Unit => write!(f, "{}", self.mantissa),
            _ => write!(f, "{} {}", self.mantissa, self.prefix),
        }
    }
}

impl_from_num_for_value!(u8);
impl_from_num_for_value!(i8);
impl_from_num_for_value!(u16);
impl_from_num_for_value!(i16);
impl_from_num_for_value!(u32);
impl_from_num_for_value!(i32);
impl_from_num_for_value!(u64);
impl_from_num_for_value!(i64);
impl_from_num_for_value!(usize);
impl_from_num_for_value!(isize);
impl_from_num_for_value!(f32);
impl_from_num_for_value!(f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn out_of_scale_values() {
        let actual = Value::new(1e-28);
        let expected = Value {
            mantissa: 1e-4f64,
            prefix: Prefix::Yocto,
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::new(-1.5e28);
        let expected = Value {
            mantissa: -1.5e4f64,
            prefix: Prefix::Yotta,
            base: Base::B1000,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn unit_values() {
        let actual = Value::new(1);
        let expected = Value {
            mantissa: 1f64,
            prefix: Prefix::Unit,
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::new(-1.3);
        let expected = Value {
            mantissa: -1.3f64,
            prefix: Prefix::Unit,
            base: Base::B1000,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn small_values() {
        let actual = Value::new(0.1);
        let expected = Value {
            mantissa: 100f64,
            prefix: Prefix::Milli,
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::new(-0.1);
        let expected = Value {
            mantissa: -100f64,
            prefix: Prefix::Milli,
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::new(0.001);
        let expected = Value {
            mantissa: 1f64,
            prefix: Prefix::Milli,
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::new(-0.001);
        let expected = Value {
            mantissa: -1f64,
            prefix: Prefix::Milli,
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::new(0.000_1);
        let expected = Value {
            mantissa: 100.00000000000001f64,
            prefix: Prefix::Micro,
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::new(-0.000_1);
        let expected = Value {
            mantissa: -100.00000000000001f64,
            prefix: Prefix::Micro,
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::new(-1e-4);
        let expected = Value {
            mantissa: -100.00000000000001f64,
            prefix: Prefix::Micro,
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::new(-1e-8);
        let expected = Value {
            mantissa: -10f64,
            prefix: Prefix::Nano,
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::new(-1e-23);
        let expected = Value {
            mantissa: -10f64,
            prefix: Prefix::Yocto,
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::new(0.12345);
        let expected = Value {
            mantissa: 123.45f64,
            prefix: Prefix::Milli,
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::new(-0.12345);
        let expected = Value {
            mantissa: -123.45f64,
            prefix: Prefix::Milli,
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::new(0.01234);
        let expected = Value {
            mantissa: 12.34f64,
            prefix: Prefix::Milli,
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::new(-0.01234);
        let expected = Value {
            mantissa: -12.34f64,
            prefix: Prefix::Milli,
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::new(0.001234);
        let expected = Value {
            mantissa: 1.234f64,
            prefix: Prefix::Milli,
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::new(-0.001234);
        let expected = Value {
            mantissa: -1.234f64,
            prefix: Prefix::Milli,
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::new(0.000_123_400);
        let expected = Value {
            mantissa: 123.39999999999999f64,
            prefix: Prefix::Micro,
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::new(-0.000_123_400);
        let expected = Value {
            mantissa: -123.39999999999999f64,
            prefix: Prefix::Micro,
            base: Base::B1000,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn large_values() {
        let actual = Value::new(1234);
        let expected = Value {
            mantissa: 1.234f64,
            prefix: Prefix::Kilo,
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::new(123_456);
        let expected = Value {
            mantissa: 123.456f64,
            prefix: Prefix::Kilo,
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::new(123_456_000);
        let expected = Value {
            mantissa: 123.456f64,
            prefix: Prefix::Mega,
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::new(-123_456_000);
        let expected = Value {
            mantissa: -123.456f64,
            prefix: Prefix::Mega,
            base: Base::B1000,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn from_numbers() {
        let actual = Value::from(0.1f32);
        let expected = Value {
            mantissa: 100.00000149011612f64,
            prefix: Prefix::Milli,
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::from(-0.1);
        let expected = Value {
            mantissa: -100f64,
            prefix: Prefix::Milli,
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::from(1.5);
        let expected = Value {
            mantissa: 1.5f64,
            prefix: Prefix::Unit,
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::from(-1.5);
        let expected = Value {
            mantissa: -1.5f64,
            prefix: Prefix::Unit,
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::from(15u32);
        let expected = Value {
            mantissa: 15f64,
            prefix: Prefix::Unit,
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::from(-1.5e28);
        let expected = Value {
            mantissa: -1.5e4f64,
            prefix: Prefix::Yotta,
            base: Base::B1000,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn from_ref_number() {
        let number = 0.1;
        let actual = Value::from(&number);
        let expected = Value {
            mantissa: 100f64,
            prefix: Prefix::Milli,
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let number = 10_000_000u32;
        let actual = Value::from(&number);
        let expected = Value {
            mantissa: 10f64,
            prefix: Prefix::Mega,
            base: Base::B1000,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn into_f64() {
        let x = 0.1;
        let actual: f64 = Value::from(x).into();
        let expected = x;
        assert_eq!(actual, expected);

        let x = -0.1;
        let actual: f64 = Value::from(x).into();
        let expected = x;
        assert_eq!(actual, expected);

        let x = 1.500;
        let actual: f64 = Value::from(x).into();
        let expected = x;
        assert_eq!(actual, expected);

        let x = -1.500;
        let actual: f64 = Value::from(x).into();
        let expected = x;
        assert_eq!(actual, expected);
    }

    #[test]
    fn large_value_with_base_1024() {
        let actual = Value::new_with(1, Base::B1024, Constraint::None);
        let expected = Value {
            mantissa: 1f64,
            prefix: Prefix::Unit,
            base: Base::B1024,
        };
        assert_eq!(actual, expected);

        let actual = Value::new_with(16, Base::B1024, Constraint::None);
        let expected = Value {
            mantissa: 16f64,
            prefix: Prefix::Unit,
            base: Base::B1024,
        };
        assert_eq!(actual, expected);

        let actual = Value::new_with(1024, Base::B1024, Constraint::None);
        let expected = Value {
            mantissa: 1f64,
            prefix: Prefix::Kilo,
            base: Base::B1024,
        };
        assert_eq!(actual, expected);

        let actual = Value::new_with(1.6 * 1024f32, Base::B1024, Constraint::None);
        let expected = Value {
            mantissa: 1.600000023841858f64,
            prefix: Prefix::Kilo,
            base: Base::B1024,
        };
        assert_eq!(actual, expected);

        let actual = Value::new_with(16 * 1024 * 1024, Base::B1024, Constraint::None);
        let expected = Value {
            mantissa: 16f64,
            prefix: Prefix::Mega,
            base: Base::B1024,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn values_with_prefix_constraints() {
        // For instance, seconds are never expressed as kilo-seconds, so
        // we must use constraints.
        let actual = Value::new_with(1325, Base::B1000, Constraint::UnitAndBelow);
        let expected = Value {
            mantissa: 1325f64,
            prefix: Prefix::Unit,
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        // In the same spirit, there can be no milli-bytes.
        let actual = Value::new_with(0.015, Base::B1024, Constraint::UnitAndAbove);
        let expected = Value {
            mantissa: 0.015,
            prefix: Prefix::Unit,
            base: Base::B1024,
        };
        assert_eq!(actual, expected);

        // The `UnitOnly` constraint prevents any scaling
        let actual = Value::new_with(0.015, Base::B1000, Constraint::UnitOnly);
        let expected = Value {
            mantissa: 0.015,
            prefix: Prefix::Unit,
            base: Base::B1000,
        };
        assert_eq!(actual, expected);
    }

    /// If no prefix constraint is set, then the function returns the best
    /// prefix if the exponent is a multiple of 3 and between `-24` and `24`
    /// (incl.).
    #[test]
    fn closest_prefix_without_constraint() {
        let exponent = -24;
        let actual = Value::closest_prefix_for(exponent, Constraint::None);
        let expected = Prefix::Yocto;
        assert_eq!(actual, expected);

        let exponent = 0;
        let actual = Value::closest_prefix_for(exponent, Constraint::None);
        let expected = Prefix::Unit;
        assert_eq!(actual, expected);

        let exponent = 24;
        let actual = Value::closest_prefix_for(exponent, Constraint::None);
        let expected = Prefix::Yotta;
        assert_eq!(actual, expected);

        let exponent = 30;
        let actual = Value::closest_prefix_for(exponent, Constraint::None);
        let expected = Prefix::Yotta;
        assert_eq!(actual, expected);

        let exponent = 1; // should never happen
        let actual = Value::closest_prefix_for(exponent, Constraint::None);
        let expected = Prefix::Unit;
        assert_eq!(actual, expected);
    }

    /// If the allowed prefixes are `Constraint::UnitAndAbove`, the function
    /// returns the corresponding prefix if the exponent is greater or equal
    /// than `0`.
    #[test]
    fn closest_prefix_with_unitandabove() {
        let constraint = Constraint::UnitAndAbove;

        let exponent = -24;
        let actual = Value::closest_prefix_for(exponent, &constraint);
        let expected = Prefix::Unit;
        assert_eq!(actual, expected);

        let exponent = 0;
        let actual = Value::closest_prefix_for(exponent, &constraint);
        let expected = Prefix::Unit;
        assert_eq!(actual, expected);

        let exponent = 24;
        let actual = Value::closest_prefix_for(exponent, &constraint);
        let expected = Prefix::Yotta;
        assert_eq!(actual, expected);

        let exponent = 30;
        let actual = Value::closest_prefix_for(exponent, &constraint);
        let expected = Prefix::Yotta;
        assert_eq!(actual, expected);

        let exponent = 1; // should never happen
        let actual = Value::closest_prefix_for(exponent, &constraint);
        let expected = Prefix::Unit;
        assert_eq!(actual, expected);
    }

    /// If the allowed prefixes are `Constraint::UnitAndBelow`, the function
    /// returns the corresponding prefix if the exponent is smaller or equal
    /// than `0`.
    #[test]
    fn closest_prefix_with_unitandbelow() {
        let constraint = Constraint::UnitAndBelow;

        let exponent = -24;
        let actual = Value::closest_prefix_for(exponent, &constraint);
        let expected = Prefix::Yocto;
        assert_eq!(actual, expected);

        let exponent = 0;
        let actual = Value::closest_prefix_for(exponent, &constraint);
        let expected = Prefix::Unit;
        assert_eq!(actual, expected);

        let exponent = 24;
        let actual = Value::closest_prefix_for(exponent, &constraint);
        let expected = Prefix::Unit;
        assert_eq!(actual, expected);

        let exponent = -30;
        let actual = Value::closest_prefix_for(exponent, &constraint);
        let expected = Prefix::Yocto;
        assert_eq!(actual, expected);

        let exponent = -1; // should never happen
        let actual = Value::closest_prefix_for(exponent, &constraint);
        let expected = Prefix::Unit;
        assert_eq!(actual, expected);
    }

    /// If the allowed prefixes are `Constraint::Custom(...)`, the function
    /// returns the corresponding prefix if the exponent matches one of them.
    #[test]
    fn closest_prefix_with_custom() {
        let constraint = Constraint::Custom(vec![Prefix::Milli, Prefix::Unit, Prefix::Kilo]);

        let exponent = -24;
        let actual = Value::closest_prefix_for(exponent, &constraint);
        let expected = Prefix::Milli;
        assert_eq!(actual, expected);

        let exponent = -3;
        let actual = Value::closest_prefix_for(exponent, &constraint);
        let expected = Prefix::Milli;
        assert_eq!(actual, expected);

        let exponent = 0;
        let actual = Value::closest_prefix_for(exponent, &constraint);
        let expected = Prefix::Unit;
        assert_eq!(actual, expected);

        let exponent = 3;
        let actual = Value::closest_prefix_for(exponent, &constraint);
        let expected = Prefix::Kilo;
        assert_eq!(actual, expected);

        let exponent = 24;
        let actual = Value::closest_prefix_for(exponent, &constraint);
        let expected = Prefix::Kilo;
        assert_eq!(actual, expected);

        let exponent = -30;
        let actual = Value::closest_prefix_for(exponent, &constraint);
        let expected = Prefix::Milli;
        assert_eq!(actual, expected);

        let exponent = -1; // should never happen
        let actual = Value::closest_prefix_for(exponent, &constraint);
        let expected = Prefix::Milli;
        assert_eq!(actual, expected);
    }

    #[test]
    #[should_panic]
    fn closest_prefix_with_custom_empty() {
        let constraint = Constraint::Custom(vec![]);

        let exponent = 3;
        Value::closest_prefix_for(exponent, constraint);
    }
}
