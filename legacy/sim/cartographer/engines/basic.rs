//! Ray observation engine function.

use crate::sim::cartographer::{Input, Output};
use rand::rngs::ThreadRng;

/// Determine the material contents of a given voxel.
#[allow(clippy::expect_used)]
#[inline]
pub fn basic(_input: &Input, _rng: &mut ThreadRng, _index: [usize; 3], _data: &mut Output) {
    // let super_sampling = input.sett.super_sampling();
    // let super_samples = super_sampling.num_samples();
    // let weight = 1.0 / super_samples as f64;

    // let voxel = input.grid.gen_voxel(&index);
    // for n in 0..super_samples {
    //     let pos = super_sampling.sample(&voxel, n, rng);

    //     if let Some(mat) = find_mat(input, &pos) {
    //         let mat_index = input.mat_reg.index_of(&mat);
    //         data.mats[mat_index][index] += weight;
    //     } else {
    //         println!(
    //             "[WARN] Could not determine key at index: {} : {} : {}",
    //             index[X], index[Y], index[Z],
    //         );

    //         data.void[index] += weight;
    //     }
    // }
}

// /// Determine the material at this point.
// #[allow(clippy::expect_used)]
// #[inline]
// fn find_mat(input: &Input, pos: &Pos3) -> Option<Name> {
//     let bump_dist = input.sett.bump_dist();
//     let loop_limit = input.sett.loop_limit();

//     let caster = input.sett.caster();
//     let num_casts = caster.num_casts();

//     let grid = input.grid.boundary();

//     for m in 0..num_casts {
//         let mut ray = caster.gen_ray(*pos, m);

//         let mut num_loops = 0;

//         while grid.contains(ray.pos()) {
//             // Loop limit check.
//             if num_loops >= loop_limit {
//                 // println!("[WARN] : Terminating photon: loop limit reached.");
//                 break;
//             }
//             num_loops += 1;

//             // Interaction distances.
//             let (grid_dist, grid_side) = grid
//                 .dist_side(&ray)
//                 .expect("Failed to determine grid distance.");
//             let surf_hit = input.tree.scan(
//                 ray.clone(),
//                 bump_dist,
//                 grid.dist(&ray).expect("Failed to determine grid distance."),
//             );

//             // Event handling.
//             match Event::new(grid_dist, surf_hit, bump_dist) {
//                 Event::Grid(dist) => {
//                     ray.travel(dist);
//                     *ray.dir_mut() = Crossing::calc_ref_dir(ray.dir(), grid_side.norm());
//                     ray.travel(bump_dist);
//                 }
//                 Event::Surface(ref hit) => match *hit.tag() {
//                     AttributeLinker::Interface(ref inside, ref outside) => {
//                         return Some(if hit.side().is_inside() {
//                             inside.clone()
//                         } else {
//                             outside.clone()
//                         });
//                     }
//                     AttributeLinker::Mirror(..) => {
//                         ray.travel(hit.dist());
//                         *ray.dir_mut() = Crossing::calc_ref_dir(ray.dir(), hit.side().norm());
//                         ray.travel(bump_dist);
//                     }
//                 },
//             }
//         }
//     }

//     None
// }
