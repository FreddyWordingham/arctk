//! Peel-off function.

use crate::{
    math::{Dir3, Pos3},
    phys::{Local, Photon},
    sim::mcrt::{Attribute, Input},
};
use nalgebra::distance;

/// Minimum weight before discarding during peel-off.
const THRESHOLD: f64 = 1.0e-6;

/// Perform a peel-off event.
/// Calculate the change in weight over a give flight towards a given point.
#[inline]
#[must_use]
pub fn peel_off(input: &Input, mut phot: Photon, env: &Local, pos: Pos3) -> Option<f64> {
    let g = env.asym();
    let g_sq = g * g;

    let dir = Dir3::new_normalize(pos - phot.ray().pos());

    let cos_ang = phot.ray().dir().dot(&dir);
    let mut prob = 0.5 * ((1.0 - g_sq) / (1.0 + g_sq - (2.0 * g * cos_ang)).powf(1.5));

    if prob < THRESHOLD {
        return None;
    }

    *phot.ray_mut().dir_mut() = dir;

    let loop_limit = input.sett.loop_limit();
    let bump_dist = input.sett.bump_dist();
    let mut inter_coeff = env.inter_coeff();

    // Main trace loop.
    let mut num_loops = 0;
    let mut tar_dist;
    loop {
        // Loop limit check.
        if num_loops >= loop_limit {
            println!("[WARN] : Terminating peel-off: loop limit reached.");
            break;
        }
        num_loops += 1;

        // Interaction distances.
        tar_dist = distance(&pos, phot.ray().pos());
        let surf_hit = input.tree.scan(phot.ray().clone(), bump_dist, tar_dist);

        // Event handling.
        if let Some(hit) = surf_hit {
            // Move to the collision point.
            let x = hit.dist();
            phot.ray_mut().travel(x);
            prob *= (-x * inter_coeff).exp();

            // Do something at the collision point.
            match *hit.tag() {
                Attribute::Interface(inside, outside) => {
                    // Determine far side material.
                    inter_coeff = if hit.side().is_inside() {
                        outside.sample_environment(phot.wavelength()).inter_coeff()
                    } else {
                        inside.sample_environment(phot.wavelength()).inter_coeff()
                    };
                    phot.ray_mut().travel(bump_dist);
                    prob *= (-bump_dist * inter_coeff).exp();
                }
                Attribute::Mirror(..) | Attribute::Spectrometer(..) | Attribute::Imager(..) => {
                    return None
                }
            }
        } else {
            prob *= (-tar_dist * inter_coeff).exp();
            return Some(prob);
        }

        if prob <= THRESHOLD {
            return None;
        }
    }

    Some(prob)
}
