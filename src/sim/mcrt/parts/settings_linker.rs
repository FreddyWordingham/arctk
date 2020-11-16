//! MCRT settings.

use crate::{
    err::Error,
    opt::Material,
    ord::{Link, Set},
    sim::mcrt::Settings,
};
use arctk_attr::load;

/// General settings structure.
#[load]
pub struct SettingsLinker {
    /// Number of photons to simulate.
    num_phot: u64,
    /// Number of photons to simulate in each thread block.
    block_size: u64,
    /// Bump distance [m].
    bump_dist: f64,
    /// Loop limit.
    loop_limit: u64,
    /// Minimum statistical weight to consider.
    min_weight: f64,
    /// Number of roulette barrels.
    roulette_barrels: u64,
    /// Emission material.
    init_mat: String,
}

impl<'a> Link<'a, Material> for SettingsLinker {
    type Inst = Settings<'a>;

    #[inline]
    fn link(self, mats: &'a Set<Material>) -> Result<Self::Inst, Error> {
        Ok(Settings::new(
            self.num_phot,
            self.block_size,
            self.bump_dist,
            self.loop_limit,
            self.min_weight,
            self.roulette_barrels,
            &mats[&self.init_mat],
        ))
    }
}
