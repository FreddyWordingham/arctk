//! Photon scattering function.

use crate::{
    geom::Hit,
    phys::{Crossing, Local, Photon},
    sim::mcrt::{Attribute, Output},
};
use rand::{rngs::ThreadRng, Rng};

/// Handle a surface collision.
#[allow(clippy::expect_used)]
#[inline]
pub fn surface(
    rng: &mut ThreadRng,
    hit: &Hit<Attribute>,
    phot: &mut Photon,
    env: &mut Local,
    data: &mut Output,
) {
    match hit.tag() {
        Attribute::Interface(inside, outside) => {
            // Reference materials.
            let (curr_mat, next_mat) = if hit.side().is_inside() {
                (inside, outside)
            } else {
                (outside, inside)
            };

            // Find local optical environments.
            let curr_env = curr_mat.sample_environment(phot.wavelength());
            let next_env = next_mat.sample_environment(phot.wavelength());

            // Get the near, and far side refractive indices.
            let curr_ref_index = curr_env.ref_index();
            let next_ref_index = next_env.ref_index();

            // Calculate the crossing normal vectors.
            let crossing = Crossing::new(
                phot.ray().dir(),
                hit.side().norm(),
                curr_ref_index,
                next_ref_index,
            );

            // Determine if a reflection or transmission occurs.
            let r = rng.gen::<f64>();
            if r <= crossing.ref_prob() {
                // Reflect.
                *phot.ray_mut().dir_mut() = *crossing.ref_dir();
            } else {
                // Refract.
                *phot.ray_mut().dir_mut() = crossing.trans_dir().expect("Invalid refraction.");
                *env = next_env;
            }
        }
        Attribute::Mirror(abs) => {
            *phot.weight_mut() *= abs;
            *phot.ray_mut().dir_mut() = Crossing::calc_ref_dir(phot.ray().dir(), hit.side().norm());
        }
        Attribute::Spectrometer(id) => {
            // det.detect(phot);
        }
    }
}
