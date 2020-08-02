//! Distribution functions.

use crate::{Dir3, Vec3};
use rand::{rngs::ThreadRng, Rng};
use std::f64::consts::PI;

/// Sample the Henyey-Greenstein phase function with a given asymmetry parameter.
#[inline]
#[must_use]
pub fn henyey_greenstein(rng: &mut ThreadRng, asym: f64) -> f64 {
    debug_assert!(asym.abs() <= 1.0);

    if asym.abs() < 1.0e-6 {
        return rng.gen_range(-1.0_f64, 1.0).acos();
    }

    ((1.0 + asym.powi(2)
        - ((1.0 - asym.powi(2)) / asym.mul_add(rng.gen_range(-1.0, 1.0), 1.0)).powi(2))
        / (2.0 * asym))
        .acos()
}

/// Sample the normal distribution.
#[inline]
#[must_use]
pub fn normal(rng: &mut ThreadRng) -> f64 {
    let a = (-2.0 * rng.gen_range(0.0_f64, 1.0).ln()).sqrt();
    let theta = rng.gen_range(0.0, 2.0 * PI);

    // Z = Some(a * theta.sin()); // Using mutable static will lead to data race :(.

    a * theta.cos()
}

/// Sample a gaussian distribution.
#[inline]
#[must_use]
pub fn gaussian(rng: &mut ThreadRng, mu: f64, sigma: f64) -> f64 {
    debug_assert!(sigma > 0.0);

    normal(rng).mul_add(sigma, mu)
}

/// Create a random unit vector.
#[inline]
#[must_use]
pub fn isotropic(rng: &mut ThreadRng) -> Dir3 {
    let theta = rng.gen_range(0.0, 2.0 * PI);
    let z: f64 = rng.gen_range(-1.0, 1.0);

    let v = (1.0 - z.powi(2)).sqrt();

    let x = v * theta.cos();
    let y = v * theta.sin();

    Dir3::new_normalize(Vec3::new(x, y, z))
}
