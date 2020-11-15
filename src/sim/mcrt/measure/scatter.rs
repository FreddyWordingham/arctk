//! Photon scattering function.

use crate::{
    math::sample_henyey_greenstein,
    opt::{Local, Photon},
};
use rand::{rngs::ThreadRng, Rng};
use std::f64::consts::PI;

/// Perform a photon scattering event.
#[inline]
pub fn scatter(rng: &mut ThreadRng, phot: &mut Photon, env: &Local) {
    // Part of the weight is absorbed.
    *phot.weight_mut() *= env.albedo();

    // The remaining weight is scattered.
    let phi = sample_henyey_greenstein(rng, env.asym());
    let theta = rng.gen_range(0.0, PI * 2.0);
    phot.ray_mut().rotate(phi, theta);
}

/// Perform a photon scattering event with a probability of shifting wavelength.
#[inline]
pub fn shift_scatter(rng: &mut ThreadRng, phot: &mut Photon, env: &Local) {
    // Part of the weight is absorbed.
    *phot.weight_mut() *= env.albedo();

    // The remaining weight may be shifted in a Raman/fluorescence event.
    let r = rng.gen::<f64>();
    if r <= env.shift_prob() {
        // Shift occurs.
        // Fluorescence event removes photons from optical range of interest.
        *phot.weight_mut() = 0.0;
        return;
    }

    // The remaining weight is scattered.
    let phi = sample_henyey_greenstein(rng, env.asym());
    let theta = rng.gen_range(0.0, PI * 2.0);
    phot.ray_mut().rotate(phi, theta);
}
