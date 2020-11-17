//! Ray observation engine function.

use crate::{
    geom::Trace,
    math::Pos3,
    ord::{X, Y, Z},
    phys::Crossing,
    sim::cartographer::{Input, Output},
    sim::mcrt::AttributeLinker,
};
use rand::rngs::ThreadRng;

/// Determine what a single ray observes.
#[allow(clippy::expect_used)]
#[inline]
pub fn basic(input: &Input, rng: &mut ThreadRng, index: [usize; 3], data: &mut Output) {
    let super_sampling = input.sett.super_sampling();
    let super_samples = super_sampling.num_samples();
    let weight = 1.0 / super_samples as f64;

    let grid = input.grid.boundary();

    for n in 0..super_samples {
        let pos = super_sampling.sample(&grid, n, rng);

        if let Some(mat) = search(input, &pos) {
            let index = input.mat_reg.index_of(&mat);
            data.mats[index] += weight;
        } else {
            println!(
                "[WARN] Could not determine key at index: {} : {} : {}",
                index[X], index[Y], index[Z],
            );

            data.void[index] += weight;
        }
    }
}

/// Determine the material at this point.
#[allow(clippy::expect_used)]
#[inline]
fn search(input: &Input, pos: &Pos3) -> Option<String> {
    let bump_dist = input.sett.bump_dist();

    let caster = input.sett.caster();
    let num_casts = caster.num_casts();

    let grid = input.grid.boundary();

    for m in 0..num_casts {
        let mut ray = caster.gen_ray(*pos, m);

        let grid_dist = grid.dist(&ray).expect("Failed to determine grid distance.");
        let surf_hit = input.tree.scan(ray.clone(), bump_dist, grid_dist);

        if let Some(hit) = surf_hit {
            ray.travel(hit.dist());

            match *hit.tag() {
                AttributeLinker::Interface(ref inside, ref outside) => {
                    return Some(if hit.side().is_inside() {
                        inside.clone()
                    } else {
                        outside.clone()
                    });
                }
                AttributeLinker::Mirror(..) => {
                    *ray.dir_mut() = Crossing::calc_ref_dir(ray.dir(), hit.side().norm());
                }
            }

            ray.travel(bump_dist);
        }
    }

    None
}
