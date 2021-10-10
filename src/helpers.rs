use crate::format_value;
use crate::{base::Base, prefix::Constraint, value::Value};

#[macro_export]
macro_rules! scale_fn {
    (
        $name:ident,
        base: $base_arg:ident,
        constraint: $constraint_arg:expr,
        mantissa_fmt: $mantissa_fmt:expr,
        unit: $unit_arg:literal
    ) => {
        pub fn $name<F>(x: F) -> String
        where
            F: Into<f64>,
        {
            let value = Value::new_with(x, $crate::base::Base::$base_arg, Some(&$constraint_arg));
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
        constraint: $constraint_arg:expr,
        mantissa_fmt: $mantissa_fmt:expr,
        groupings: $sep_arg:literal,
        unit: $unit_arg:literal
    ) => {
        pub fn $name<F>(x: F) -> String
        where
            F: Into<f64>,
        {
            let value = Value::new_with(x, $crate::base::Base::$base_arg, $constraint_arg);
            format!(
                "{}{}",
                $crate::format_value!(value, $mantissa_fmt, groupings: $sep_arg),
                $unit_arg
            )
        }
    };
}

/// Parses a number into a `Value` and displays it using 3 decimals and the
/// appropriate scale for seconds (`UnitAndBelow`), so that non-sensical
/// scales such as kilo-seconds may not appear.
///
/// ```
/// use si_scale::helpers::seconds;
///
/// let actual = format!("result is {}", seconds(1234.5678));
/// let expected = "result is 1234.568 s";
/// assert_eq!(actual, expected);
///
/// let actual = format!("result is {:>10}", seconds(12.34e-7));
/// let expected = "result is   1.234 µs";
/// assert_eq!(actual, expected);
/// ```
///
pub fn seconds<F>(x: F) -> String
where
    F: Into<f64>,
{
    let constraint = Constraint::UnitAndBelow;
    let value = Value::new_with(x, Base::B1000, constraint);
    format!("{}s", format_value!(value, "{:.3}"))
}

scale_fn!(bytes,
          base: B1000,
          constraint: Constraint::UnitAndAbove,
          mantissa_fmt: "{}",
          groupings: '_',
          unit: "B");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seconds() {
        let actual = format!("result is {}", seconds(1234.5678));
        let expected = "result is 1234.568 s";
        assert_eq!(actual, expected);

        let actual = format!("result is {:>10}", seconds(12.34e-7));
        let expected = "result is   1.234 µs";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_bytes() {
        let actual = format!("result is {}", bytes(12_345_678));
        let expected = "result is 12.345_678 MB";
        assert_eq!(actual, expected);

        let actual = format!("result is {:>10}", bytes(16));
        let expected = "result is       16 B";
        assert_eq!(actual, expected);

        let actual = format!("result is {}", bytes(0.12));
        let expected = "result is 0.12 B";
        assert_eq!(actual, expected);
    }
}
