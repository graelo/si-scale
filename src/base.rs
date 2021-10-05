/// Represents the base for units Prefix.
///
#[derive(Debug, PartialEq)]
pub enum Base {
    B1000,
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
    /// use pretty_units::base::Base;
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
        match self {
            Self::B1000 => (x.abs().log10() / 3f64).floor() as i32 * 3,
            Self::B1024 => (x.abs().log2() / 10f64).floor() as i32 * 3,
        }
    }

    /// Raises self to the power of the provided `base_exponent`.
    ///
    /// # Example
    ///
    /// ```
    /// use pretty_units::base::Base;
    ///
    /// assert_eq!(Base::B1000.pow(9), 1e9);
    /// ```
    ///
    pub fn pow(&self, base_exponent: i32) -> f64 {
        match self {
            Self::B1000 => 1000f64.powf(base_exponent as f64 / 3f64),
            Self::B1024 => 1024f64.powf(base_exponent as f64 / 3f64),
        }
    }
}
