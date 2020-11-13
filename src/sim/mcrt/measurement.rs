//! Peel-off function.

use super::{Attributes, ClearEvent, Local, Photon, Universe};
use crate::{
    geom::Trace,
    math::{Dir3, Pos3},
    phys::Crossing,
};

/// Minimum weight before discarding.
const THRESHOLD: f64 = 1.0e-6;

/// Calculate the change in weight over a give flight towards a given point.
/// Perform a peel off event.
#[inline]
#[must_use]
pub fn peel_off(env: &Local, uni: &Universe, mut phot: Photon, pos: Pos3) -> Option<f64> {
    let g = env.asym();
    let g_sq = g * g;

    let dir = Dir3::new_normalize(pos - phot.ray().pos());

    let cos_ang = phot.ray().dir().dot(&dir);
    let mut prob = 0.5 * ((1.0 - g_sq) / (1.0 + g_sq - (2.0 * g * cos_ang)).powf(1.5));
    if prob < THRESHOLD {
        return None;
    }

    // let dist = distance(pos, phot.ray().pos());
    // prob *= (-dist * env.inter_coeff()).exp();

    *phot.ray_mut().dir_mut() = dir;

    let bump_dist = uni.sett.bump_dist();
    let mut inter_coeff = env.inter_coeff();

    while let Some((_index, voxel)) = uni.grid.gen_index_voxel(phot.ray().pos()) {
        if prob < THRESHOLD {
            return None;
        }

        let voxel_dist = voxel
            .dist(phot.ray())
            .expect("Could not determine voxel distance.");
        let surf_hit = uni.tree.observe(phot.ray().clone(), bump_dist, voxel_dist);

        match ClearEvent::new(voxel_dist, surf_hit, bump_dist) {
            ClearEvent::Voxel(dist) => {
                let x = dist + bump_dist;
                phot.ray_mut().travel(x);
                prob *= (-x * inter_coeff).exp();
            }
            ClearEvent::Surface(hit) => {
                // Move to the collision point.
                let x = hit.dist();
                phot.ray_mut().travel(x);
                prob *= (-x * inter_coeff).exp();

                // Do something at the collision point.
                if let Some(attr) = uni.attrs.map().get(*hit.tag()) {
                    match *attr {
                        Attributes::Spectrometer => {
                            return None;
                        }
                        Attributes::Mirror => {
                            *phot.ray_mut().dir_mut() =
                                Crossing::calc_ref_dir(phot.ray().dir(), hit.side().norm());
                        }
                        Attributes::Refractive {
                            ref inside,
                            ref outside,
                        } => {
                            // Determine far side material.
                            let next_mat = if hit.side().is_inside() {
                                outside
                            } else {
                                inside
                            };

                            inter_coeff = uni.mats.map()[next_mat]
                                .env(phot.wavelength())
                                .inter_coeff();
                        }
                    }
                } else {
                    panic!("Unknown attribute tag: {}", hit.tag());
                }

                // Move forward from the collision point.
                phot.ray_mut().travel(bump_dist);
                prob *= (-bump_dist * inter_coeff).exp();
            }
        }
    }

    Some(prob)
}
