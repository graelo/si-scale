use std::convert::TryFrom;

use super::Prefix;

#[derive(Debug, PartialEq, Eq)]
pub enum Allowed {
    /// Allows all prefixes, from `Yocto` to `Yotta`.
    All,
    /// Allows prefixes from `Unit` to `Yotta`.
    UnitAndAbove,
    /// Allows prefixes from `Yocto` to `Unit`.
    UnitAndBelow,
    /// Custom prefixes (should be sorted ascending).
    Custom(Vec<Prefix>),
}

impl Allowed {
    /// Determines the closest allowed prefix corresponding to the provided
    /// exponent.
    ///
    /// In any case, the available prefixes correspond to `-24, -21, ..., -3,
    /// 0, 3, ..., 24`.
    ///
    /// If no correspondance is found , the function returns `None`. This may
    /// happen in the following cases:
    ///
    /// - the provided exponent is negative and the allowed prefixes is
    /// `Allowed::UnitAndAbove`
    /// - the provided exponent is positive and the allowed prefixes is
    /// `Allowed::UnitAndBelow`
    /// - the provided exponent is not a multiple of 3 (should never happen by
    /// design)
    ///
    /// # Example
    ///
    /// If the allowed prefixes are `Allowed::All`, the function returns
    /// the corresponding prefix if the exponent is a multiple of 3 and
    /// between `-24` and `24` (incl.), otherwise it returns `None`.
    ///
    /// ```
    /// use pretty_units::prelude::{Allowed, Prefix};
    ///
    /// let allowed_prefixes = Allowed::All;
    ///
    /// let exponent = -24;
    /// let actual = allowed_prefixes.closest_prefix_below(exponent);
    /// let expected = Some(Prefix::Yocto);
    /// assert_eq!(actual, expected);
    ///
    /// let exponent = 0;
    /// let actual = allowed_prefixes.closest_prefix_below(exponent);
    /// let expected = Some(Prefix::Unit);
    /// assert_eq!(actual, expected);
    ///
    /// let exponent = 24;
    /// let actual = allowed_prefixes.closest_prefix_below(exponent);
    /// let expected = Some(Prefix::Yotta);
    /// assert_eq!(actual, expected);
    ///
    /// let exponent = 30;
    /// let actual = allowed_prefixes.closest_prefix_below(exponent);
    /// let expected = Some(Prefix::Yotta);
    /// assert_eq!(actual, expected);
    ///
    /// let exponent = 1; // should never happen
    /// let actual = allowed_prefixes.closest_prefix_below(exponent);
    /// let expected = None;
    /// assert_eq!(actual, expected);
    /// ```
    ///
    /// If the allowed prefixes are `Allowed::UnitAndAbove`, the function
    /// returns the corresponding prefix if the exponent is greater or equal
    /// than `0`.
    ///
    /// ```
    /// use pretty_units::prelude::{Allowed, Prefix};
    ///
    /// let allowed_prefixes = Allowed::UnitAndAbove;
    ///
    /// let exponent = -24;
    /// let actual = allowed_prefixes.closest_prefix_below(exponent);
    /// let expected = Some(Prefix::Unit);
    /// assert_eq!(actual, expected);
    ///
    /// let exponent = 0;
    /// let actual = allowed_prefixes.closest_prefix_below(exponent);
    /// let expected = Some(Prefix::Unit);
    /// assert_eq!(actual, expected);
    ///
    /// let exponent = 24;
    /// let actual = allowed_prefixes.closest_prefix_below(exponent);
    /// let expected = Some(Prefix::Yotta);
    /// assert_eq!(actual, expected);
    ///
    /// let exponent = 30;
    /// let actual = allowed_prefixes.closest_prefix_below(exponent);
    /// let expected = Some(Prefix::Yotta);
    /// assert_eq!(actual, expected);
    ///
    /// let exponent = 1; // should never happen
    /// let actual = allowed_prefixes.closest_prefix_below(exponent);
    /// let expected = None;
    /// assert_eq!(actual, expected);
    /// ```
    ///
    /// If the allowed prefixes are `Allowed::UnitAndBelow`, the function
    /// returns the corresponding prefix if the exponent is smaller or equal
    /// than `0`.
    ///
    /// ```
    /// use pretty_units::prelude::{Allowed, Prefix};
    ///
    /// let allowed_prefixes = Allowed::UnitAndBelow;
    ///
    /// let exponent = -24;
    /// let actual = allowed_prefixes.closest_prefix_below(exponent);
    /// let expected = Some(Prefix::Yocto);
    /// assert_eq!(actual, expected);
    ///
    /// let exponent = 0;
    /// let actual = allowed_prefixes.closest_prefix_below(exponent);
    /// let expected = Some(Prefix::Unit);
    /// assert_eq!(actual, expected);
    ///
    /// let exponent = 24;
    /// let actual = allowed_prefixes.closest_prefix_below(exponent);
    /// let expected = Some(Prefix::Unit);
    /// assert_eq!(actual, expected);
    ///
    /// let exponent = -30;
    /// let actual = allowed_prefixes.closest_prefix_below(exponent);
    /// let expected = Some(Prefix::Yocto);
    /// assert_eq!(actual, expected);
    ///
    /// let exponent = -1; // should never happen
    /// let actual = allowed_prefixes.closest_prefix_below(exponent);
    /// let expected = None;
    /// assert_eq!(actual, expected);
    /// ```
    ///
    /// If the allowed prefixes are `Allowed::Custom(...)`, the function
    /// returns the corresponding prefix if the exponent matches one of them.
    ///
    /// ```
    /// use pretty_units::prelude::{Allowed, Prefix};
    ///
    /// let allowed_prefixes = Allowed::Custom(vec![
    ///     Prefix::Milli, Prefix::Unit, Prefix::Kilo
    /// ]);
    ///
    /// let exponent = -24;
    /// let actual = allowed_prefixes.closest_prefix_below(exponent);
    /// let expected = Some(Prefix::Milli);
    /// assert_eq!(actual, expected);
    ///
    /// let exponent = -3;
    /// let actual = allowed_prefixes.closest_prefix_below(exponent);
    /// let expected = Some(Prefix::Milli);
    /// assert_eq!(actual, expected);
    ///
    /// let exponent = 0;
    /// let actual = allowed_prefixes.closest_prefix_below(exponent);
    /// let expected = Some(Prefix::Unit);
    /// assert_eq!(actual, expected);
    ///
    /// let exponent = 3;
    /// let actual = allowed_prefixes.closest_prefix_below(exponent);
    /// let expected = Some(Prefix::Kilo);
    /// assert_eq!(actual, expected);
    ///
    /// let exponent = 24;
    /// let actual = allowed_prefixes.closest_prefix_below(exponent);
    /// let expected = Some(Prefix::Kilo);
    /// assert_eq!(actual, expected);
    ///
    /// let exponent = -30;
    /// let actual = allowed_prefixes.closest_prefix_below(exponent);
    /// let expected = Some(Prefix::Milli);
    /// assert_eq!(actual, expected);
    ///
    /// let exponent = -1; // should never happen
    /// let actual = allowed_prefixes.closest_prefix_below(exponent);
    /// let expected = Some(Prefix::Milli);
    /// assert_eq!(actual, expected);
    ///
    ///
    /// let allowed_prefixes = Allowed::Custom(vec![]);
    ///
    /// let exponent = 3;
    /// let actual = allowed_prefixes.closest_prefix_below(exponent);
    /// let expected = None;
    /// assert_eq!(actual, expected);
    /// ```
    ///
    pub fn closest_prefix_below(&self, exponent: i32) -> Option<Prefix> {
        match self {
            Self::All => {
                Prefix::try_from(exponent.clamp(Prefix::Yocto as i32, Prefix::Yotta as i32)).ok()
            }
            Self::UnitAndAbove => {
                Prefix::try_from(exponent.clamp(Prefix::Unit as i32, Prefix::Yotta as i32)).ok()
            }
            Self::UnitAndBelow => {
                Prefix::try_from(exponent.clamp(Prefix::Yocto as i32, Prefix::Unit as i32)).ok()
            }
            Self::Custom(allowed) => {
                if allowed.is_empty() {
                    return None;
                }
                let smallest_prefix = *allowed.first().unwrap();
                if exponent < smallest_prefix as i32 {
                    return Some(smallest_prefix);
                }
                allowed
                    .iter()
                    .take_while(|&&prefix| prefix as i32 <= exponent)
                    .cloned()
                    .last()
            }
        }
    }
}
