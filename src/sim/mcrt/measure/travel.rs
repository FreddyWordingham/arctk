//! Photon movement function.

use crate::{
    opt::{Local, Photon},
    sim::mcrt::Output,
};
use physical_constants::SPEED_OF_LIGHT_IN_VACUUM;

/// Move the photon forward and record the flight.
#[inline]
pub fn travel(data: &mut Output, phot: &mut Photon, env: &Local, index: [usize; 3], dist: f64) {
    debug_assert!(dist > 0.0);

    let weight_power_dist = phot.weight() * phot.power() * dist;
    data.energy[index] += weight_power_dist * env.ref_index() / SPEED_OF_LIGHT_IN_VACUUM;
    data.absorptions[index] += weight_power_dist * env.abs_coeff();
    data.shifts[index] += weight_power_dist * env.shift_coeff();

    phot.ray_mut().travel(dist);
}
