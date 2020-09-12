//! World settings structure.

use crate::display_field;
use arctk_attr::load;
use std::fmt::{Display, Formatter, Result};

/// Loadable world settings structure.
#[load]
pub struct World {
    /// Optional world seed.
    seed: Option<String>,
}

impl World {
    /// Get the world seed.
    pub fn seed(&self) -> u64 {
        if let Some(_seed) = &self.seed {
            1
        } else {
            0
        }
    }
}

impl Display for World {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field!(fmt, "seed", self.seed())
    }
}
