//! MCRT settings.

use crate::{
    ord::{Register, Setup},
    sim::mcrt::Settings,
};
use arctk_attr::load;

/// General settings structure.
#[load]
pub struct SettingsSetup {
    /// Number of photons to simulate.
    num_phot: u64,
    /// Number of photons to simulate in each thread block.
    block_size: u64,
    /// Bump distance [m].
    bump_dist: f64,
    /// Emission material.
    init_mat: String,
}

impl Setup for SettingsSetup {
    type Inst = Settings;

    #[inline]
    fn setup(self, reg: &Register) -> Self::Inst {
        let init_mat_index = reg.index_of(&self.init_mat);

        Self::Inst::new(
            self.num_phot,
            self.block_size,
            self.bump_dist,
            init_mat_index,
        )
    }
}
