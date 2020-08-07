//! Photon-lifetime engine function.

use super::Event;
use crate::{
    mcrt::{Data, Local, Photon, Sample, Scene},
    Trace,
};
use rand::{rngs::ThreadRng, Rng};

/// Simulate the life of a within the photon.
#[inline]
#[must_use]
pub fn simulate_photon(
    scene: &Scene,
    rng: &mut ThreadRng,
    data: &mut Data,
    mut phot: Photon,
) -> Sample {
    // Check photon is within the grid.
    if let Some(index) = scene.grid.gen_index(phot.ray().pos()) {
        data.emission_power[index] += phot.power() * phot.weight();
    } else {
        panic!("Photon was not emitted within the grid.");
    }

    // Common constants.
    let bump_dist = scene.sett.bump_dist();
    let loop_limit = scene.sett.loop_limit();
    let roulette_weight = scene.sett.roulette_weight();
    let roulette_barrels = scene.sett.roulette_barrels() as f64;
    let roulette_survive_prob = 1.0 / roulette_barrels;

    // Initialisation.
    let mat = &scene.mats.map()["air"];
    let env = mat.env(phot.wavelength());

    // Main loop.
    let mut loops = 0;
    while let Some((index, voxel)) = scene.grid.gen_index_voxel(phot.ray().pos()) {
        // Loop limit check.
        if loops >= loop_limit {
            println!("Warning! Terminating photon: loop limit reached.");
            break;
        }
        loops += 1;

        // Roulette.
        if phot.weight() <= roulette_weight {
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
        let surf_hit = scene
            .tree
            .observe(phot.ray().clone(), bump_dist, voxel_dist.min(scat_dist));

        // Event handling.
        match Event::new(voxel_dist, scat_dist, surf_hit, bump_dist) {
            Event::Voxel(dist) => travel(data, index, &env, &mut phot, dist + bump_dist),
            Event::Scattering(dist) => {}
            Event::Surface(hit) => {}
        }
    }

    Sample::new(0.0)
}

/// Move the photon forward and record the flight.
#[inline]
fn travel(data: &mut Data, index: [usize; 3], env: &Local, phot: &mut Photon, dist: f64) {
    debug_assert!(dist > 0.0);

    let weight_power_dist = phot.weight() * phot.power() * dist;
    // data.energy[index] += weight_power_dist * (env.ref_index() / SPEED_OF_LIGHT_IN_VACUUM);
    // data.absorptions[index] += weight_power_dist * env.abs_coeff();
    // data.shifts[index] += weight_power_dist * env.shift_coeff();

    // data.dist_travelled[index] += dist;

    phot.ray_mut().travel(dist);
}
