/// Represents the base for units Prefix.
///
#[derive(Debug, PartialEq)]
pub enum Base {
    B1000,
    B1024,
}

impl Base {
    pub(crate) fn integral_exponent_for(&self, x: f64) -> i32 {
        match self {
            Self::B1000 => (x.abs().log10() / 3f64).floor() as i32,
            Self::B1024 => (x.abs().log2() / 10f64).floor() as i32,
        }
    }

    pub(crate) fn pow(&self, exponent: i32) -> f64 {
        match self {
            Self::B1000 => 1000f64.powf(exponent as f64),
            Self::B1024 => 1024f64.powf(exponent as f64),
        }
    }
}
