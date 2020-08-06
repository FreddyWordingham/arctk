//! MCRT sampling information structure.

/// Scene sampling return information.
pub struct Sample {
    /// Remaining photon weight.
    pub remaining_weight: f64,
}

impl Sample {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(remaining_weight: f64) -> Self {
        debug_assert!(remaining_weight >= 0.0);

        Self { remaining_weight }
    }
}
