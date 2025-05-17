#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Size {
    Xs,
    Sm,
    Md,
    Lg,
    Xl
}

impl Size {
    /// Returns the diameter of this size in meters.
    pub fn get_diameter(&self) -> f64 {
        match self {
            Self::Xs => 0.3,
            Self::Sm => 1.25,
            Self::Md => todo!(),
            Self::Lg => todo!(),
            Self::Xl => todo!(),
        }
    }
}