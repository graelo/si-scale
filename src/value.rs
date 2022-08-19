use crate::prefix::Prefix;
use std::convert::From;

use crate::base::Base;
use crate::prefix::Allowed;

/// Represents a float value using its mantissa and unit Prefix in a base.
///
/// With base = 1000, 1k = 1000, 1M = 1_000_000, 1m = 0.001, 1µ = 0.000_001,
/// etc.
///
/// | min         | max              | exponent | magnitude | prefix                |
/// | ---         | ---              | ---      | ---       | ----                  |
/// | ..          | ..               | -3       | -9        | `Some(Prefix::Nano)`  |
/// | 0.000\_001  | 0.001            | -2       | -6        | `Some(Prefix::Micro)` |
/// | 0.001       | 1                | -1       | -3        | `Some(Prefix::Milli)` |
/// | 1           | 1_000            | 0        | 0         | `Some(Prefix::Unit)`  |
/// | 1000        | 1\_000\_000      | 1        | 3         | `Some(Prefix::Kilo)`  |
/// | 1\_000\_000 | 1\_000\_000\_000 | 2        | 6         | `Some(Prefix::Mega)`  |
/// | ..          | ..               | 3        | 9         | `Some(Prefix::Tera)`  |
///
/// The base is usually 1000, but can also be 1024 (bibytes).
///
/// With base = 1024, 1ki = 1024, 1Mi = 1024 * 1024, etc.
///
/// # Example
///
/// ```
/// use std::convert::From;
/// use pretty_units::{base::Base, value::Value, prefix::Prefix};
///
/// let actual = Value::from(0.123);
/// let expected = Value {
///     mantissa: 123f64,
///     prefix: Some(Prefix::Milli),
///     base: Base::B1000,
/// };
/// assert_eq!(actual, expected);
/// ```
///
#[derive(Debug, PartialEq)]
pub struct Value {
    pub mantissa: f64,
    pub prefix: Option<Prefix>,
    pub base: Base,
}

impl Value {
    /// Returns a `Value` for the default base `B1000`, meaning `1 k = 1000`,
    /// `1 m = 1e-3`, etc.
    ///
    /// # Example
    ///
    /// ```
    /// use pretty_units::prelude::{Base, Prefix, Value};
    ///
    /// let actual = Value::new(-4.6e-5);
    /// let expected = Value {
    ///     mantissa: -46f64,
    ///     prefix: Some(Prefix::Micro),
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
    /// use pretty_units::prelude::{Base, Prefix, Value};
    ///
    /// let actual = Value::new(-4.3e-5);
    /// let expected = Value {
    ///     mantissa: -43.00000000000001f64,
    ///     prefix: Some(Prefix::Micro),
    ///     base: Base::B1000,
    /// };
    /// assert_eq!(actual, expected);
    /// ```
    ///
    pub fn new<F>(x: F) -> Self
    where
        F: Into<f64>,
    {
        Value::new_with(x, Base::B1000, Allowed::All)
    }

    /// Returns a `Value` for the provided base.
    ///
    /// # Example
    ///
    /// ```
    /// use pretty_units::prelude::{Allowed, Base, Prefix, Value};
    ///
    /// // 4 MiB
    /// let actual = Value::new_with(4 * 1024 * 1024, Base::B1024, Allowed::All);
    /// let expected = Value {
    ///     mantissa: 4f64,
    ///     prefix: Some(Prefix::Mega),
    ///     base: Base::B1024,
    /// };
    /// assert_eq!(actual, expected);
    /// ```
    ///
    pub fn new_with<F>(x: F, base: Base, allowed_prefixes: Allowed) -> Self
    where
        F: Into<f64>,
    {
        let x: f64 = x.into();

        // Closest integral exponent (multiple of 3)
        let exponent: i32 = base.integral_exponent_for(x);
        // Clamp the exponent using the allowed prefixes
        let prefix = allowed_prefixes.closest_prefix_below(exponent);

        let mantissa = match prefix {
            Some(prefix) => x / base.pow(prefix.exponent()),
            None => x,
        };

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
    /// use pretty_units::prelude::{Base, Prefix, Value};
    ///
    /// let value = Value {
    ///     mantissa: 1.3f64,
    ///     prefix: Some(Prefix::Unit),
    ///     base: Base::B1000,
    /// };
    /// assert_eq!(value.to_f64(), 1.3);
    /// ```
    ///
    pub fn to_f64(&self) -> f64 {
        match self.prefix {
            Some(prefix) => {
                let scale = self.base.pow(prefix.exponent());
                self.mantissa * scale
            }
            None => self.mantissa,
        }
    }

    /// Returns a number that represents the sign of self.
    ///
    /// - `1.0` if the number is positive, `+0.0` or `INFINITY`
    /// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    /// - `NAN` if the number is `NAN`
    ///
    pub fn signum(&self) -> f64 {
        self.mantissa.signum()
    }
}

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

impl_from_num_for_value!(u8);
impl_from_num_for_value!(i8);
impl_from_num_for_value!(u16);
impl_from_num_for_value!(i16);
impl_from_num_for_value!(u32);
impl_from_num_for_value!(i32);
// impl_from_num_for_value!(u64);
// impl_from_num_for_value!(i64);
// impl_from_num_for_value!(usize);
// impl_from_num_for_value!(isize);
impl_from_num_for_value!(f32);
impl_from_num_for_value!(f64);

impl From<Value> for f64 {
    fn from(value: Value) -> Self {
        value.to_f64()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn out_of_scale_values() {
        let actual = Value::new(1e-28);
        let expected = Value {
            mantissa: 1e-4f64,
            prefix: Some(Prefix::Yocto),
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::new(-1.5e28);
        let expected = Value {
            mantissa: -1.5e4f64,
            prefix: Some(Prefix::Yotta),
            base: Base::B1000,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn unit_values() {
        let actual = Value::new(1);
        let expected = Value {
            mantissa: 1f64,
            prefix: Some(Prefix::Unit),
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::new(-1.3);
        let expected = Value {
            mantissa: -1.3f64,
            prefix: Some(Prefix::Unit),
            base: Base::B1000,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn small_values() {
        let actual = Value::new(0.1);
        let expected = Value {
            mantissa: 100f64,
            prefix: Some(Prefix::Milli),
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::new(-0.1);
        let expected = Value {
            mantissa: -100f64,
            prefix: Some(Prefix::Milli),
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::new(0.001);
        let expected = Value {
            mantissa: 1f64,
            prefix: Some(Prefix::Milli),
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::new(-0.001);
        let expected = Value {
            mantissa: -1f64,
            prefix: Some(Prefix::Milli),
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::new(0.000_1);
        let expected = Value {
            mantissa: 100.00000000000001f64,
            prefix: Some(Prefix::Micro),
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::new(-0.000_1);
        let expected = Value {
            mantissa: -100.00000000000001f64,
            prefix: Some(Prefix::Micro),
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::new(-1e-4);
        let expected = Value {
            mantissa: -100.00000000000001f64,
            prefix: Some(Prefix::Micro),
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::new(-1e-8);
        let expected = Value {
            mantissa: -10f64,
            prefix: Some(Prefix::Nano),
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::new(-1e-23);
        let expected = Value {
            mantissa: -10f64,
            prefix: Some(Prefix::Yocto),
            base: Base::B1000,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn small_values_with_decimals() {
        let actual = Value::new(0.12345);
        let expected = Value {
            mantissa: 123.45f64,
            prefix: Some(Prefix::Milli),
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::new(-0.12345);
        let expected = Value {
            mantissa: -123.45f64,
            prefix: Some(Prefix::Milli),
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::new(0.01234);
        let expected = Value {
            mantissa: 12.34f64,
            prefix: Some(Prefix::Milli),
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::new(-0.01234);
        let expected = Value {
            mantissa: -12.34f64,
            prefix: Some(Prefix::Milli),
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::new(0.001234);
        let expected = Value {
            mantissa: 1.234f64,
            prefix: Some(Prefix::Milli),
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::new(-0.001234);
        let expected = Value {
            mantissa: -1.234f64,
            prefix: Some(Prefix::Milli),
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::new(0.000_123_400);
        let expected = Value {
            mantissa: 123.39999999999999f64,
            prefix: Some(Prefix::Micro),
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::new(-0.000_123_400);
        let expected = Value {
            mantissa: -123.39999999999999f64,
            prefix: Some(Prefix::Micro),
            base: Base::B1000,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn large_values() {
        let actual = Value::new(1234);
        let expected = Value {
            mantissa: 1.234f64,
            prefix: Some(Prefix::Kilo),
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::new(123_456);
        let expected = Value {
            mantissa: 123.456f64,
            prefix: Some(Prefix::Kilo),
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::new(123_456_000);
        let expected = Value {
            mantissa: 123.456f64,
            prefix: Some(Prefix::Mega),
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::new(-123_456_000);
        let expected = Value {
            mantissa: -123.456f64,
            prefix: Some(Prefix::Mega),
            base: Base::B1000,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn from_f64() {
        let actual = Value::from(0.1);
        let expected = Value {
            mantissa: 100f64,
            prefix: Some(Prefix::Milli),
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::from(-0.1);
        let expected = Value {
            mantissa: -100f64,
            prefix: Some(Prefix::Milli),
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::from(1.5);
        let expected = Value {
            mantissa: 1.5f64,
            prefix: Some(Prefix::Unit),
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::from(-1.5);
        let expected = Value {
            mantissa: -1.5f64,
            prefix: Some(Prefix::Unit),
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        let actual = Value::from(-1.5e28);
        let expected = Value {
            mantissa: -1.5e4f64,
            prefix: Some(Prefix::Yotta),
            base: Base::B1000,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn from_ref_f64() {
        let number = 0.1;
        let actual = Value::from(&number);
        let expected = Value {
            mantissa: 100f64,
            prefix: Some(Prefix::Milli),
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
        let actual = Value::new_with(1, Base::B1024, Allowed::All);
        let expected = Value {
            mantissa: 1f64,
            prefix: Some(Prefix::Unit),
            base: Base::B1024,
        };
        assert_eq!(actual, expected);

        let actual = Value::new_with(16, Base::B1024, Allowed::All);
        let expected = Value {
            mantissa: 16f64,
            prefix: Some(Prefix::Unit),
            base: Base::B1024,
        };
        assert_eq!(actual, expected);

        let actual = Value::new_with(1024, Base::B1024, Allowed::All);
        let expected = Value {
            mantissa: 1f64,
            prefix: Some(Prefix::Kilo),
            base: Base::B1024,
        };
        assert_eq!(actual, expected);

        let actual = Value::new_with(1.6 * 1024f32, Base::B1024, Allowed::All);
        let expected = Value {
            mantissa: 1.600000023841858f64,
            prefix: Some(Prefix::Kilo),
            base: Base::B1024,
        };
        assert_eq!(actual, expected);

        let actual = Value::new_with(16 * 1024 * 1024, Base::B1024, Allowed::All);
        let expected = Value {
            mantissa: 16f64,
            prefix: Some(Prefix::Mega),
            base: Base::B1024,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn values_with_prefix_constraints() {
        // For instance, seconds are never expressed as kilo-seconds, so
        // we must use constraints.
        let actual = Value::new_with(1325, Base::B1000, Allowed::UnitAndBelow);
        let expected = Value {
            mantissa: 1325f64,
            prefix: Some(Prefix::Unit),
            base: Base::B1000,
        };
        assert_eq!(actual, expected);

        // In the same spirit, there can be no milli-bytes.
        let actual = Value::new_with(0.015, Base::B1024, Allowed::UnitAndAbove);
        let expected = Value {
            mantissa: 0.015,
            prefix: Some(Prefix::Unit),
            base: Base::B1024,
        };
        assert_eq!(actual, expected);
    }
}
