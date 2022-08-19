//! The `format_value` macro.

/// Formats a [`Value`][`crate::value::Value`]'s mantissa and unit prefix (but
/// not the unit itself). Because it simply delegates to
/// [`format_args!()`][`std::format_args`], the output should be consumed by
/// macros such as `println!()`, `write!()`, etc.
///
/// It provides more control than the `Display` implementation in
/// [`Value`][`crate::value::Value`] because you can provide the number
/// formatting.
///
/// # Example
///
/// ```
/// use si_scale::{value::Value, format_value};
///
/// let x = 3.4e-12f32;
/// let v: Value = x.into();
/// let unit = "F"; // just something displayable.
///
/// let actual = format!("result is {}{u}",
///     format_value!(v, "{:>8.2}"), u = unit
/// );
/// let expected = "result is     3.40 pF";
/// assert_eq!(actual, expected);
///
/// // left alignment
///
/// let actual = format!("result is {}{u}",
///     format_value!(v, "{:<8.3}"), u = unit
/// );
/// let expected = "result is 3.400    pF";
/// assert_eq!(actual, expected);
/// ```
///
/// Additionally, you can provide a symbol for thousands' groupings.
///
/// # Example
///
/// In this example, the number `x` is converted into a value and displayed
/// using the most appropriate SI prefix. The user chose to constrain the
/// prefix to be anything lower than `Unit` (1) because kilo-seconds make
/// no sense.
///
/// ```
/// use si_scale::format_value;
/// # fn main() {
/// use si_scale::{value::Value, base::Base, prefix::Constraint};
///
/// let x = 1234.5678;
/// let v = Value::new_with(x, Base::B1000, Constraint::UnitAndBelow);
/// let unit = "s";
///
/// let actual = format!(
///     "result is {}{u}",
///     format_value!(v, "{:.5}", groupings: '_'),
///     u = unit
/// );
/// let expected = "result is 1_234.567_80 s";
/// assert_eq!(actual, expected);
/// # }
/// ```
///
#[macro_export]
macro_rules! format_value {
    ($name:ident, $fmt_str:literal) => {
        format_args! {
            concat!($fmt_str, " {}{}"),
            $name.mantissa,
            $name.prefix,
            match $name.base {
                $crate::base::Base::B1000 => "",
                $crate::base::Base::B1024 => if $name.prefix == $crate::prefix::Prefix::Unit {""} else {"i"},
            },
        }
    };

    ($name:ident, $fmt_str:literal, groupings: $separator:expr) => {
        format_args! {
            "{} {}{}",
            $crate::format::separated_float(&format!($fmt_str, $name.mantissa), $separator),
            $name.prefix,
            match $name.base {
                $crate::base::Base::B1000 => "",
                $crate::base::Base::B1024 => if $name.prefix == $crate::prefix::Prefix::Unit {""} else {"i"},
            },
        }
    };

    ($name:ident, $fmt_str:literal, groupings: $separator:expr, no_unit) => {
        format_args! {
            "{}{}{}{}",
            $crate::format::separated_float(&format!($fmt_str, $name.mantissa), $separator),
            match $name.prefix {
                $crate::prefix::Prefix::Unit => "",
                _=> " "
            },
            $name.prefix,
            match $name.base {
                $crate::base::Base::B1000 => "",
                $crate::base::Base::B1024 => if $name.prefix == $crate::prefix::Prefix::Unit {""} else {"i"},
            },
        }
    };
}

/// Given a input `&str` representing a digit (float or int), this function
/// returns a `String` in which thousands separators are inserted both on the
/// integral part and the fractional part.
///
pub fn separated_float(input: &str, separator: char) -> String {
    let idx = match input.find('.') {
        Some(i) => i,
        None => input.len(),
    };

    let int_part = &input[..idx];
    let frac_part = &input[idx..];

    let int_part_separated = separate_thousands_backward(int_part, separator);
    let frac_part_separated = separate_thousands_forward(frac_part, separator);
    int_part_separated + &frac_part_separated
}

fn separate_thousands_backward(input: &str, separator: char) -> String {
    let mut output = String::with_capacity(input.len() + input.len() / 4);
    let mut pos = 0;
    for ch in input.chars().rev() {
        if ch.is_ascii_digit() {
            // don't push a sep on first char
            if pos > 1 && pos % 3 == 0 {
                output.push(separator);
            }
            pos += 1;
        }
        output.push(ch);
    }
    output.chars().rev().collect()
}

fn separate_thousands_forward(input: &str, separator: char) -> String {
    let mut output = String::with_capacity(input.len() + input.len() / 4);
    let mut pos = 0;
    for ch in input.chars() {
        if ch.is_ascii_digit() {
            // don't push a sep on first char
            if pos > 1 && pos % 3 == 0 {
                output.push(separator);
            }
            pos += 1;
        }
        output.push(ch);
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::format_value;
    use crate::value::Value;

    #[test]
    fn format_value_without_groupings() {
        let x = 3.4e-12f32;
        let v: Value = x.into();
        let unit = "F"; // just something displayable.

        let actual = format!("result is {}{u}", format_value!(v, "{:>8.2}"), u = unit);
        let expected = "result is     3.40 pF";
        assert_eq!(actual, expected);

        let actual = format!("result is {}{u}", format_value!(v, "{:<8.3}"), u = unit);
        let expected = "result is 3.400    pF";
        assert_eq!(actual, expected);
    }

    #[test]
    fn format_value_with_groupings() {
        let x = 1234.5678;
        let v: Value = x.into();
        let unit = "m"; // just something displayable.

        let actual = format!(
            "result is {}{u}",
            format_value!(v, "{:.7}", groupings: '_'),
            u = unit
        );
        let expected = "result is 1.234_567_8 km";
        assert_eq!(actual, expected);

        use crate::base::Base;
        use crate::prefix::Constraint;

        let v = Value::new_with(x, Base::B1000, Constraint::UnitAndBelow);
        let unit = "s";

        let actual = format!(
            "result is {}{u}",
            format_value!(v, "{:.5}", groupings: '_'),
            u = unit
        );
        let expected = "result is 1_234.567_80 s";
        assert_eq!(actual, expected);
    }

    #[test]
    fn separate_float() {
        let actual: String = separated_float("123456.123456", '_');
        let expected = "123_456.123_456";
        assert_eq!(actual, expected);

        let actual: String = separated_float("123456789.123456789", '_');
        let expected = "123_456_789.123_456_789";
        assert_eq!(actual, expected);

        let actual: String = separated_float("1234567.1234567", '_');
        let expected = "1_234_567.123_456_7";
        assert_eq!(actual, expected);

        let actual: String = separated_float("--1234567.1234567++", '_');
        let expected = "--1_234_567.123_456_7++";
        assert_eq!(actual, expected);
    }

    #[test]
    fn int_part_with_separate_thousands_backward() {
        let actual = separate_thousands_backward("123456", '_');
        let expected = "123_456";
        assert_eq!(actual, expected);

        let actual = separate_thousands_backward("  123456..", '_');
        let expected = "  123_456..";
        assert_eq!(actual, expected);
    }

    #[test]
    fn frac_part_with_separate_thousands_forward() {
        let actual = separate_thousands_forward(".123456789", '_');
        let expected = ".123_456_789";
        assert_eq!(actual, expected);

        let actual = separate_thousands_forward(".1234567--", '_');
        let expected = ".123_456_7--";
        assert_eq!(actual, expected);
    }

    #[test]
    fn format_zero_value() {
        let x = 0.0f32;
        let v: Value = x.into();
        let unit = "F"; // just something displayable.

        let actual = format!("result is {}{u}", format_value!(v, "{:>8.2}"), u = unit);
        let expected = "result is     0.00 F";
        assert_eq!(actual, expected);

        let actual = format!("result is {}{u}", format_value!(v, "{:<8.3}"), u = unit);
        let expected = "result is 0.000    F";
        assert_eq!(actual, expected);
    }
}

// macro_rules! format_scale {
//     ($name:ident $fmtstr:literal groupings $groupings:expr) => {
//         pub fn $name(value: Value) -> String {
//             match value.prefix {
//                 Some(prefix) => format!(concat!($fmtstr, " {}"), value.mantissa, prefix),
//                 None => format!($fmtstr, value.mantissa),
//             }
//         }
//     };
// }
//
// format_scale!(scafmt1 "{:<8.3}" groupings "_");
