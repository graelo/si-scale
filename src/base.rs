/// Represents the base for units [Prefix](crate::prefix::Prefix).
///
#[derive(Debug, PartialEq)]
pub enum Base {
    /// The most common base, where 1 k means `1000,` 1 M means `1000^2`, ...
    B1000,
    /// A very common base for bibytes, where 1 kiB means `1024`, 1 MiB means
    /// `1024 * 1024`, ...
    B1024,
}

impl Base {
    /// Using `floor()`, returns the closest integer exponent to represent the
    /// provided value `x` in the self `Base`.
    ///
    /// The returned integer exponent is a multiple of 3 in order to match the
    /// prefixes' exponents.
    ///
    /// # Example
    ///
    /// ```
    /// use si_scale::base::Base;
    ///
    /// let x: f32  = 5.4e4;
    /// let actual = Base::B1000.integral_exponent_for(x);
    /// assert_eq!(actual, 3);  // 1e3
    ///
    /// let x: f64  = -5.4e-4;
    /// let actual = Base::B1000.integral_exponent_for(x);
    /// assert_eq!(actual, -6);  // 1e-6
    /// ```
    ///
    pub fn integral_exponent_for<F>(&self, x: F) -> i32
    where
        F: Into<f64>,
    {
        let x: f64 = x.into();
        if x == 0.0 {
            return 0;
        }
        match self {
            Self::B1000 => (x.abs().log10() / 3f64).floor() as i32 * 3,
            Self::B1024 => (x.abs().log2() / 10f64).floor() as i32 * 3,
        }
    }

    /// This helper function returns a `f64` scaling factor for the mantissa,
    /// obtained by raising self to the power of the provided `exponent`
    /// divided by 3.
    ///
    /// # Example
    ///
    /// ```
    /// use si_scale::base::Base;
    ///
    /// assert_eq!(Base::B1000.pow(9), 1e9);
    /// assert_eq!(Base::B1024.pow(3), 1024f64)
    /// ```
    ///
    pub fn pow(&self, exponent: i32) -> f64 {
        match self {
            Self::B1000 => 1000f64.powf(exponent as f64 / 3f64),
            Self::B1024 => 1024f64.powf(exponent as f64 / 3f64),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exponent_of_zero_is_zero() {
        assert_eq!(0, Base::B1000.integral_exponent_for(0.0));
        assert_eq!(0, Base::B1000.integral_exponent_for(-0.0));
        
        assert_eq!(0, Base::B1024.integral_exponent_for(0.0));
        assert_eq!(0, Base::B1024.integral_exponent_for(-0.0));
    }
}