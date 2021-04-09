//! Photo imaging photon-lifetime engine function.

use crate::{
    geom::Trace,
    img::Colour,
    math::{Mat4, Pos3, Vec3},
    ord::{X, Y},
    phys::Photon,
    sim::mcrt::{
        peel_off::peel_off, scatter::scatter, surface::surface, travel::travel, Event, Input,
        Output,
    },
};
use rand::{rngs::ThreadRng, Rng};

/// Generate the RGB components of a given wavelength.
pub fn wavelength_to_rbg(mut wavelength: f64) -> [f64; 3] {
    let gamma = 0.8;
    wavelength *= 1.0e9;

    if wavelength >= 380.0 && wavelength <= 440.0 {
        let a = 0.3 + (0.7 * (wavelength - 380.0) / (440.0 - 380.0));
        let r = ((-(wavelength - 440.0) / (440.0 - 380.0)) * a).powf(gamma);
        let g = 0.0;
        let b = a.powf(gamma);
        return [r, g, b];
    } else if wavelength >= 440.0 && wavelength <= 490.0 {
        let r = 0.0;
        let g = ((wavelength - 440.0) / (490.0 - 440.0)).powf(gamma);
        let b = 1.0;
        return [r, g, b];
    } else if wavelength >= 490.0 && wavelength <= 510.0 {
        let r = 0.0;
        let g = 1.0;
        let b = (-(wavelength - 510.0) / (510.0 - 490.0)).powf(gamma);
        return [r, g, b];
    } else if wavelength >= 510.0 && wavelength <= 580.0 {
        let r = ((wavelength - 510.0) / (580.0 - 510.0)).powf(gamma);
        let g = 1.0;
        let b = 0.0;
        return [r, g, b];
    } else if wavelength >= 580.0 && wavelength <= 645.0 {
        let r = 1.0;
        let g = (-(wavelength - 645.0) / (645.0 - 580.0)).powf(gamma);
        let b = 0.0;
        return [r, g, b];
    } else if wavelength >= 645.0 && wavelength <= 750.0 {
        let a = 0.3 + (0.7 * (750.0 - wavelength) / (750.0 - 645.0));
        let r = a.powf(gamma);
        let g = 0.0;
        let b = 0.0;
        return [r, g, b];
    } else {
        return [1.0, 0.0, 1.0];
    }
}

/// Transform the point position into pixel coordinates.
pub fn project(pos: &Pos3, mvp: &Mat4, res: [usize; 2]) -> [usize; 2] {
    // let p = mvp * pos.to_homogeneous();
    let p = mvp * pos.to_homogeneous();

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
    let fov = 120.0_f64.to_radians();

    let model = crate::math::Mat4::identity();
    // let model = nalgebra::Translation3::new(cam_pos.x, cam_pos.y, cam_pos.z).to_homogeneous(); //* self.rotation.matrix();
    // let model = model.try_inverse().unwrap();
    let view = Mat4::look_at_rh(&cam_pos, &input.grid.boundary().centre(), &Vec3::z_axis());
    let proj = Mat4::new_perspective(
        aspect,
        fov,
        0.1,
        10.0,
    );
    let mvp = proj * view * model;
    let phot_col = wavelength_to_rbg(phot.wavelength());

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
                    let [x, y] = project(phot.ray().pos(), &mvp, res);
                    if x < res[X] && y < res[Y] {
                        if let Some(weight) = peel_off(&input, phot.clone(), &env, cam_pos) {
                            if weight > 0.0 {
                                data.photos[0].pixels_mut()[[x, y]] += Colour::new(
                                    phot_col[0] as f32,
                                    phot_col[1] as f32,
                                    phot_col[2] as f32,
                                    1.0,
                                ) * (phot.weight() * weight)
                                    as f32;
                            }
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
