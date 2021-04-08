//! Photo imaging photon-lifetime engine function.

use crate::{
    geom::{ Trace},
    img::Colour,
    math::{ Mat4, Pos3, Vec3},
    ord::{X, Y},
    phys::Photon,
    sim::mcrt::{
        peel_off::peel_off, scatter::scatter, surface::surface, travel::travel, Event, Input,
        Output,
    },
};
use nalgebra::Perspective3;
use rand::{rngs::ThreadRng, Rng};

// Transform the point position into pixel coordinates.
pub fn project(pos: &Pos3, view: &Mat4, proj: &Perspective3<f64>, res: [usize; 2]) -> [usize; 2] {
    // let p = proj.project_point(pos);
    // let p = view * p.to_homogeneous();
    let p = view * pos.to_homogeneous();
    let p = proj.as_matrix() * p;

    let x = (res[X] as f64 * (p.x + 1.0) * 0.5) as usize;
    let y = (res[Y] as f64 * (p.y + 1.0) * 0.5) as usize;

    [x, y]
}

/// Simulate the life of a single photon
/// with the potential to absorbed by a fluorophore species.
#[allow(clippy::expect_used)]
#[inline]
pub fn photo(input: &Input, mut rng: &mut ThreadRng, mut phot: Photon, mut data: &mut Output) {
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

    // Camera data.
    let res = [
        data.photos[0].pixels().raw_dim()[X],
        data.photos[0].pixels().raw_dim()[Y],
    ];
    let cam_pos = input.cam_pos.unwrap();
    let aspect = res[X] as f64 / res[Y] as f64;
    let view = Mat4::look_at_rh(&cam_pos, &input.grid.boundary().centre(), &Vec3::z_axis());
    let fovy = 30.0_f64.to_radians();
    let proj = Perspective3::new(
        aspect,
        fovy,
        0.1,
        100.0
    );

    // Initialisation.
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

                {
                    // Capture.
                    let [x, y] = project(phot.ray().pos(), &view, &proj, res);
                    if x < res[X] && y < res[Y] {
                        if let Some(weight) = peel_off(&input, phot.clone(), &env, cam_pos) {
                            data.photos[0].pixels_mut()[[x, y]] +=
                                Colour::new(1.0, 1.0, 1.0, (phot.weight() * weight) as f32);
                        }
                    }
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
