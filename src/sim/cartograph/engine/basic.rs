//! Sampling engine function.

use super::super::{Data, Landscape};
use crate::{
    geom::{Ray, Trace},
    ord::{Key, X, Y, Z},
};
use rand::{prelude::SliceRandom, rngs::ThreadRng};

/// Determine what a single ray will observe.
#[inline]
pub fn sample(land: &Landscape, data: &mut Data, index: [usize; 3], mut rng: &mut ThreadRng) {
    let bump_dist = land.sett.bump_dist();
    let num_pos_samples = land.sett.super_sampling().num_samples();
    let num_cast_samples = land.sett.caster().num_samples();
    let weight = 1.0 / num_pos_samples as f64;
    let voxel = land.grid.gen_voxel(&index);
    let sampler = land.sett.super_sampling();
    let caster = land.sett.caster();

    for n in 0..num_pos_samples {
        let pos = sampler.sample(&voxel, n, rng);

        let mut found = false;

        let mut order = (0..num_cast_samples).collect::<Vec<i32>>();
        order.as_mut_slice().shuffle(&mut rng);
        for m in order {
            let ray = caster.gen_ray(pos, m);

            if let Some(key) = scan(land, ray, bump_dist) {
                *data
                    .maps
                    .mut_map()
                    .get_mut(key)
                    .unwrap()
                    .get_mut(index)
                    .unwrap() += weight;

                found = true;
                break;
            }
        }

        if !found {
            println!(
                "WARNING! Could not determine key at index: {} : {} : {}",
                index[X], index[Y], index[Z],
            );

            *data.undetermined.get_mut(index).unwrap() += weight;
        }
    }
}

/// Determine what a single ray will observe.
#[allow(clippy::expect_used)]
#[inline]
#[must_use]
pub fn scan<'a>(land: &'a Landscape, ray: Ray, bump_dist: f64) -> Option<&'a Key> {
    debug_assert!(bump_dist > 0.0);

    let bound_dist = land
        .grid
        .boundary()
        .dist(&ray)
        .expect("Failed to determine distance to grid boundary.");

    land.tree.observe(ray, bump_dist, bound_dist).map(|hit| {
        let interface: &Key = hit.tag();
        let &(ref inside, ref outside) = land
            .inters
            .map()
            .get(interface)
            .unwrap_or_else(|| panic!("Entry '{}' not found in interface map.", interface));

        if hit.side().is_inside() {
            inside
        } else {
            outside
        }
    })
}
