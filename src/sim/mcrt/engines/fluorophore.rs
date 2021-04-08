//! Fluorophore supporting engine function.

use crate::{
    geom::Trace,
    phys::{Local, Photon},
    sim::mcrt::{scatter::scatter, surface::surface, travel::travel, Event, Input, Output},
};
use rand::{rngs::ThreadRng, Rng};

/// Simulate the life of a single photon
/// with the potential to absorbed by a fluorophore species.
#[allow(clippy::expect_used)]
#[inline]
pub fn fluorophore(
    input: &Input,
    mut rng: &mut ThreadRng,
    mut phot: Photon,
    mut data: &mut Output,
) {
    // Fluorophore data.
    let flu = input.shifts_conc_spec.as_ref();
    let (flu_concs, flu_spec) = flu.unwrap();
    let mu_shift = flu_spec.y(phot.wavelength());

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
    let mat = input.light.mat();
    let mut local = mat.sample_environment(phot.wavelength());
    let mut env;

    // Main event loop.
    let mut num_loops = 0;
    while let Some((index, voxel)) = input.grid.gen_index_voxel(phot.ray().pos()) {
        env = Local::new(
            local.ref_index(),
            local.scat_coeff(),
            local.abs_coeff(),
            local.shift_coeff() + (mu_shift * flu_concs[index]),
            local.asym(),
        );

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
                scatter(&mut rng, &mut phot, &env)
            }
            Event::Surface(hit) => {
                travel(&mut data, &mut phot, &env, index, hit.dist());
                surface(&mut rng, &hit, &mut phot, &mut local, &mut data);
                travel(&mut data, &mut phot, &env, index, bump_dist);
            }
        }

        if phot.weight() <= 0.0 {
            break;
        }
    }
}
