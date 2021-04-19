//! Photography photon-lifetime engine function.

use crate::{
    geom::Trace,
    img::Colour,
    ord::Set,
    phys::Photon,
    sim::mcrt::{
        peel_off::peel_off, scatter::scatter, surface::surface, travel::travel, Event, Frame,
        Input, Output,
    },
};
use rand::{rngs::ThreadRng, Rng};

/// Photograph the life of a single photon.
#[allow(clippy::expect_used)]
#[inline]
pub fn photo(
    frames: &Set<Frame>,
    input: &Input,
    mut data: &mut Output,
    mut rng: &mut ThreadRng,
    mut phot: Photon,
) {
    // Check photon is within the grid.
    if let Some(index) = input.grid.gen_index(phot.ray().pos()) {
        data.emission[index] += phot.power() * phot.weight();
    } else {
        panic!("Photon was not emitted within the grid.");
    }

    // Common constants.
    let bump_dist = input.sett.bump_dist();
    let loop_limit = input.sett.loop_limit();
    let min_weight = input.sett.min_weight();
    let roulette_barrels = input.sett.roulette_barrels() as f64;
    let roulette_survive_prob = 1.0 / roulette_barrels;

    // Initialisation.
    let phot_col = wavelength_to_rbg(phot.wavelength());
    let mat = input.light.mat();
    let mut env = mat.sample_environment(phot.wavelength());

    // Main event loop.
    let mut num_loops = 0;
    while let Some((index, voxel)) = input.grid.gen_index_voxel(phot.ray().pos()) {
        // Loop limit check.
        if num_loops >= loop_limit {
            println!("[WARN] : Terminating photon: loop limit reached.");
            break;
        }
        num_loops += 1;

        // Roulette.
        if phot.weight() < min_weight {
            let r = rng.gen::<f64>();
            if r > roulette_survive_prob {
                break;
            }
            *phot.weight_mut() *= roulette_barrels;
        }

        // Interaction distances.
        let voxel_dist = voxel
            .dist(phot.ray())
            .expect("Could not determine voxel distance.");
        let scat_dist = -(rng.gen::<f64>()).ln() / env.inter_coeff();
        let surf_hit = input
            .tree
            .scan(phot.ray().clone(), bump_dist, voxel_dist.min(scat_dist));

        // Event handling.
        match Event::new(voxel_dist, scat_dist, surf_hit, bump_dist) {
            Event::Voxel(dist) => travel(&mut data, &mut phot, &env, index, dist + bump_dist),
            Event::Scattering(dist) => {
                travel(&mut data, &mut phot, &env, index, dist);

                // Capture.
                for (name, frame) in frames.map() {
                    if let Some([x, y]) = frame.transform(phot.ray().pos()) {
                        if let Some(weight) = peel_off(input, phot.clone(), &env, *frame.pos()) {
                            let idx = data.img_reg().set().map()[name];
                            data.imgs[idx].pixels_mut()[[x, y]] += Colour::new(
                                phot_col[0] as f32,
                                phot_col[1] as f32,
                                phot_col[2] as f32,
                                1.0,
                            ) * (phot.weight() * weight)
                                as f32;
                        }
                    };
                }

                scatter(&mut rng, &mut phot, &env)
            }
            Event::Surface(hit) => {
                travel(&mut data, &mut phot, &env, index, hit.dist());
                surface(&mut rng, &hit, &mut phot, &mut env, &mut data);
                travel(&mut data, &mut phot, &env, index, bump_dist);
            }
        }

        if phot.weight() <= 0.0 {
            break;
        }
    }
}

/// Generate the RGB components of a given wavelength.
#[inline]
#[must_use]
pub fn wavelength_to_rbg(mut wavelength: f64) -> [f64; 3] {
    let gamma = 0.8;
    wavelength *= 1.0e9;

    if (380.0..440.0).contains(&wavelength) {
        let a = 0.3 + (0.7 * (wavelength - 380.0) / (440.0 - 380.0));
        let r = ((-(wavelength - 440.0) / (440.0 - 380.0)) * a).powf(gamma);
        let g = 0.0;
        let b = a.powf(gamma);
        return [r, g, b];
    } else if (440.0..490.0).contains(&wavelength) {
        let r = 0.0;
        let g = ((wavelength - 440.0) / (490.0 - 440.0)).powf(gamma);
        let b = 1.0;
        return [r, g, b];
    } else if (490.0..510.0).contains(&wavelength) {
        let r = 0.0;
        let g = 1.0;
        let b = (-(wavelength - 510.0) / (510.0 - 490.0)).powf(gamma);
        return [r, g, b];
    } else if (510.0..580.0).contains(&wavelength) {
        let r = ((wavelength - 510.0) / (580.0 - 510.0)).powf(gamma);
        let g = 1.0;
        let b = 0.0;
        return [r, g, b];
    } else if (580.0..645.0).contains(&wavelength) {
        let r = 1.0;
        let g = (-(wavelength - 645.0) / (645.0 - 580.0)).powf(gamma);
        let b = 0.0;
        return [r, g, b];
    } else if (645.0..750.0).contains(&wavelength) {
        let a = 0.3 + (0.7 * (750.0 - wavelength) / (750.0 - 645.0));
        let r = a.powf(gamma);
        let g = 0.0;
        let b = 0.0;
        return [r, g, b];
    }

    [1.0, 0.0, 1.0]
}
