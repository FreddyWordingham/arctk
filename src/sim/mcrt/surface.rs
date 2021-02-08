//! Photon scattering function.

use crate::{
    geom::Hit,
    img::Colour,
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
    match *hit.tag() {
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
            data.specs[id].try_collect_weight(phot.wavelength(), phot.weight());
            phot.kill();
        }
        Attribute::Imager(id, _width, ref _orient) => {
            let pixel = [10, 20];
            data.imgs[id].pixels_mut()[pixel] +=
                wavelength_to_col(phot.wavelength()) * phot.weight() as f32;
            phot.kill();
        }
    }
}

fn wavelength_to_col(wavelength: f64) -> Colour {
    debug_assert!(wavelength > 0.0);

    let gamma = 0.8;

    let (r, g, b) = if wavelength >= 380.0e-9 && wavelength <= 440.0e-9 {
        let attenuation = 0.3 + (0.7 * ((wavelength - 380.0e-9) / (440.0e-9 - 380.0e-9)));
        (
            ((-(wavelength - 440.0e-9) / (440.0e-9 - 380.0e-9)) * attenuation).powf(gamma),
            0.0,
            attenuation.powf(gamma),
        )
    } else if wavelength >= 440.0e-9 && wavelength <= 490.0e-9 {
        (
            0.0,
            ((wavelength - 440.0e-9) / (490.0e-9 - 440.0e-9)).powf(gamma),
            1.0,
        )
    } else if wavelength >= 490.0e-9 && wavelength <= 510.0e-9 {
        (
            0.0,
            1.0,
            (-(wavelength - 510.0e-9) / (510.0e-9 - 490.0e-9)).powf(gamma),
        )
    } else if wavelength >= 510.0e-9 && wavelength <= 580.0e-9 {
        (
            ((wavelength - 510.0e-9) / (580.0e-9 - 510.0e-9)).powf(gamma),
            1.0,
            0.0,
        )
    } else if wavelength >= 580.0e-9 && wavelength <= 645.0e-9 {
        (
            1.0,
            (-(wavelength - 645.0e-9) / (645.0e-9 - 580.0e-9)).powf(gamma),
            0.0,
        )
    } else if wavelength >= 645.0e-9 && wavelength <= 750.0e-9 {
        let attenuation = 0.3 + (0.7 * ((750.0e-9 - wavelength) / (750.0e-9 - 645.0e-9)));
        (attenuation.powf(gamma), 0.0, 0.0)
    } else {
        (0.0, 0.0, 0.0)
    };

    Colour::new(r as f32, g as f32, b as f32, 1.0)
}
