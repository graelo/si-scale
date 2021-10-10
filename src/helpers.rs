//! The helpers functions provide number parsing and correct SI formatting for
//! various units. They are probably the most used functions in this crate.
//!
//! You can extend with your own units and formatting using the
//! [`scale_fn!()`] macro.
//!
//! The `seconds()` function parses a number into a `Value` and displays it
//! using 3 decimals and the appropriate scale for seconds (`UnitAndBelow`),
//! so that non-sensical scales such as kilo-seconds may not appear.
//!
//! ```
//! use si_scale::helpers::{seconds, seconds3};
//!
//! let actual = format!("result is {}", seconds(1234.5678));
//! let expected = "result is 1234.5678 s";
//! assert_eq!(actual, expected);
//!
//! let actual = format!("result is {:>10}", seconds3(12.3e-7));
//! let expected = "result is   1.230 µs";
//! assert_eq!(actual, expected);
//! ```
//!
//! The `bytes1()` function parses a number into a `Value` *using base 1000*
//! and displays it using 1 decimal and the appropriate scale for bytes
//! (`UnitAndAbove`), so that non-sensical scales such as milli-bytes may not
//! appear.
//!
//! ```
//! use si_scale::helpers::bytes1;
//!
//! let actual = format!("result is {}", bytes1(12_345_678));
//! let expected = "result is 12.3 MB";
//! assert_eq!(actual, expected);
//!
//! let actual = format!("result is {:>10}", bytes1(16));
//! let expected = "result is     16.0 B";
//! assert_eq!(actual, expected);
//!
//! let actual = format!("result is {}", bytes1(0.12));
//! let expected = "result is 0.1 B";
//! assert_eq!(actual, expected);
//! ```
//!
//! The `bibytes1()` function parses a number into a `Value` *using base 1024*
//! and displays it using 1 decimal and the appropriate scale for bytes
//! (`UnitAndAbove`), so that non-sensical scales such as milli-bytes may not
//! appear.
//!
//! ```
//! use si_scale::helpers::bibytes1;
//!
//! let actual = format!("result is {}", bibytes1(12_345_678));
//! let expected = "result is 11.8 MiB";
//! assert_eq!(actual, expected);

//! let actual = format!("result is {}", bibytes1(16 * 1024));
//! let expected = "result is 16.0 kiB";
//! assert_eq!(actual, expected);

//! let actual = format!("result is {:>10}", bibytes1(16));
//! let expected = "result is     16.0 B";
//! assert_eq!(actual, expected);

//! let actual = format!("result is {}", bibytes1(0.12));
//! let expected = "result is 0.1 B";
//! assert_eq!(actual, expected);
//! ```

use crate::value::Value;

#[macro_export]
macro_rules! scale_fn {
    (
        $name:ident,
        base: $base_arg:ident,
        constraint: $constraint_arg:ident,
        mantissa_fmt: $mantissa_fmt:expr,
        unit: $unit_arg:literal
    ) => {
        pub fn $name<F>(x: F) -> String
        where
            F: Into<f64>,
        {
            let value = $crate::value::Value::new_with(
                x,
                $crate::base::Base::$base_arg,
                $crate::prefix::Constraint::$constraint_arg,
            );
            format!(
                "{}{}",
                $crate::format_value!(value, $mantissa_fmt),
                $unit_arg
            )
        }
    };

    (
        $name:ident,
        base: $base_arg:ident,
        constraint: $constraint_arg:ident,
        mantissa_fmt: $mantissa_fmt:expr,
        groupings: $sep_arg:literal,
        unit: $unit_arg:literal
    ) => {
        pub fn $name<F>(x: F) -> String
        where
            F: Into<f64>,
        {
            let value = Value::new_with(
                x,
                $crate::base::Base::$base_arg,
                $crate::prefix::Constraint::$constraint_arg,
            );
            format!(
                "{}{}",
                $crate::format_value!(value, $mantissa_fmt, groupings: $sep_arg),
                $unit_arg
            )
        }
    };
}

// seconds
//
scale_fn!(seconds,
          base: B1000,
          constraint: UnitAndBelow,
          mantissa_fmt: "{}",
          unit: "s");

scale_fn!(seconds3,
          base: B1000,
          constraint: UnitAndBelow,
          mantissa_fmt: "{:.3}",
          unit: "s");

// bytes
//
scale_fn!(bytes,
          base: B1000,
          constraint: UnitAndAbove,
          mantissa_fmt: "{}",
          groupings: '_',
          unit: "B");

scale_fn!(bytes_,
          base: B1000,
          constraint: UnitOnly,
          mantissa_fmt: "{}",
          groupings: '_',
          unit: "B");

scale_fn!(bytes1,
          base: B1000,
          constraint: UnitAndAbove,
          mantissa_fmt: "{:.1}",
          groupings: '_',
          unit: "B");

// bibytes
//
scale_fn!(bibytes,
          base: B1024,
          constraint: UnitAndAbove,
          mantissa_fmt: "{}",
          groupings: '_',
          unit: "B");

scale_fn!(bibytes1,
          base: B1024,
          constraint: UnitAndAbove,
          mantissa_fmt: "{:.1}",
          groupings: '_',
          unit: "B");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seconds() {
        let actual = format!("result is {}", seconds(1234.5678));
        let expected = "result is 1234.5678 s";
        assert_eq!(actual, expected);

        let actual = format!("result is {:>10}", seconds(12.4e-7));
        let expected = "result is    1.24 µs";
        assert_eq!(actual, expected);

        let actual = format!("result is {}", seconds(12e-7));
        let expected = "result is 1.2 µs";
        assert_eq!(actual, expected);

        let actual = format!("result is {}", seconds(1.0));
        let expected = "result is 1 s";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_seconds3() {
        let actual = format!("result is {}", seconds3(1234.5678));
        let expected = "result is 1234.568 s";
        assert_eq!(actual, expected);

        let actual = format!("result is {:>10}", seconds3(12.4e-7));
        let expected = "result is   1.240 µs";
        assert_eq!(actual, expected);

        let actual = format!("result is {}", seconds3(12e-7));
        let expected = "result is 1.200 µs";
        assert_eq!(actual, expected);

        let actual = format!("result is {}", seconds3(1.0));
        let expected = "result is 1.000 s";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_bytes1() {
        let actual = format!("result is {}", bytes1(12_345_678));
        let expected = "result is 12.3 MB";
        assert_eq!(actual, expected);

        let actual = format!("result is {:>10}", bytes1(16));
        let expected = "result is     16.0 B";
        assert_eq!(actual, expected);

        let actual = format!("result is {}", bytes1(0.12));
        let expected = "result is 0.1 B";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_bibytes() {
        let actual = format!("result is {}", bibytes1(12_345_678));
        let expected = "result is 11.8 MiB";
        assert_eq!(actual, expected);

        let actual = format!("result is {}", bibytes1(16 * 1024));
        let expected = "result is 16.0 kiB";
        assert_eq!(actual, expected);

        let actual = format!("result is {:>10}", bibytes1(16));
        let expected = "result is     16.0 B";
        assert_eq!(actual, expected);

        let actual = format!("result is {}", bibytes1(0.12));
        let expected = "result is 0.1 B";
        assert_eq!(actual, expected);
    }
}
