//! MCRT settings.

use crate::{
    err::Error,
    img::Gradient,
    ord::{Link, Set},
    sim::render::Settings,
};
use arctk_attr::load;

/// General settings structure.
#[load]
pub struct SettingsLinker {
    /// Number of photons to simulate in each thread block.
    block_size: usize,
    /// Bump distance [m].
    bump_dist: f64,
    /// Loop limit.
    loop_limit: u64,
    /// Minimum statistical weight to consider.
    min_weight: f64,
    /// Sky gradient.
    sky_grad: String,
}

impl<'a> Link<'a, Gradient> for SettingsLinker {
    type Inst = Settings<'a>;

    #[inline]
    fn link(self, grads: &'a Set<Gradient>) -> Result<Self::Inst, Error> {
        Ok(Settings::new(
            self.block_size,
            self.bump_dist,
            self.loop_limit,
            self.min_weight,
            &grads[&self.sky_grad],
        ))
    }
}
