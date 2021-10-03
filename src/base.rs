/// Represents the base for units Prefix.
///
#[derive(Debug, PartialEq)]
pub enum Base {
    B1000,
    B1024,
    Custom(f64),
}

impl Base {
    pub(crate) fn exponent_for(&self, x: f64) -> f64 {
        match self {
            Self::B1000 => (x.abs().log10() / 3f64).floor(),
            Self::B1024 => (x.abs().log2() / 10f64).floor(),
            Self::Custom(base) => (x.abs().log10() / base.log10()).floor(),
        }
    }

    pub(crate) fn powf(&self, exponent: f64) -> f64 {
        match self {
            Self::B1000 => 1000f64.powf(exponent),
            Self::B1024 => 1024f64.powf(exponent),
            Self::Custom(base) => base.powf(exponent),
        }
    }

    pub(crate) fn powi(&self, exponent: i32) -> f64 {
        match self {
            Self::B1000 => 1000f64.powi(exponent),
            Self::B1024 => 1024f64.powi(exponent),
            Self::Custom(base) => base.powi(exponent),
        }
    }
}
