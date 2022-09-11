//! Run control.

use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{
    fs,
    path::Path,
    sync::{Arc, Mutex},
};

use crate::{
    render::{Input, Output, Parameters},
    rt::{Camera, Ray},
    util::ProgressBar,
};

/// Run the simulation with the given parameterisation.
#[inline]
pub fn run<
    T: Fn(&Input<'_>, &Camera, Ray, f64, [usize; 2], &mut Output, &mut ThreadRng) + Send + Sync + Copy,
>(
    parameters: &Parameters,
    output_dir: &Path,
    sample: T,
) {
    // Setup.
    let settings = parameters.build_settings();
    let meshes = parameters.load_meshes();
    let gradients = parameters.load_gradients();
    let attributes = parameters.load_attributes(&gradients);
    let surfaces = parameters.load_surfaces(&meshes, &attributes);
    let tree = parameters.build_tree(&surfaces);
    let shader = parameters.build_shader(&gradients);
    let camera = parameters.build_camera();

    // Create runtime object.
    let runtime = Input::new(settings, shader, tree);

    // Run
    let tiles_output_dir = output_dir.join("tiles");
    if tiles_output_dir.exists() {
        fs::remove_dir_all(&tiles_output_dir).expect("Failed to initialise output directory.");
    }
    fs::create_dir_all(&tiles_output_dir).expect("Failed to create output directory.");
    render(&tiles_output_dir, &runtime, &camera, sample);
}

/// Perform the rendering.
#[allow(clippy::integer_division)]
#[inline]
fn render<
    T: Fn(&Input<'_>, &Camera, Ray, f64, [usize; 2], &mut Output, &mut ThreadRng)
        + Send
        + Sync
        + Clone,
>(
    output_dir: &Path,
    input: &Input,
    camera: &Camera,
    sample: T,
) {
    let tiles = input.settings.tiles;
    let tile_res = [camera.res[0] / tiles[0], camera.res[1] / tiles[1]];

    let mut tile_order = Vec::with_capacity(tiles[0] * tiles[1]);
    for iy in 0..tiles[1] {
        for ix in 0..tiles[0] {
            tile_order.push((ix, iy));
        }
    }
    tile_order.shuffle(&mut thread_rng());

    let pb = Arc::new(Mutex::new(ProgressBar::new(
        "Rendering image",
        tiles[0] * tiles[1],
    )));
    let print_width = ((tiles[0].max(tiles[1])) as f64).log10() as usize + 1;
    tile_order.par_iter().for_each(|&(ix, iy)| {
        let offset = [tile_res[0] * ix, tile_res[1] * iy];
        let data = render_tile(input, camera, offset, tile_res, sample.clone());
        data.save(
            &input.shader,
            output_dir,
            &format!(
                "{:0>width$}_{:0>width$}",
                ix,
                tiles[1] - iy - 1,
                width = print_width
            ),
        );
        pb.lock().expect("Could not lock progress bar.").tick();
    });
    pb.lock()
        .expect("Could not lock progress bar.")
        .finish_with_message("Rendering complete");
}

/// Render a sub-tile.
#[inline]
#[must_use]
fn render_tile<T: Fn(&Input<'_>, &Camera, Ray, f64, [usize; 2], &mut Output, &mut ThreadRng)>(
    input: &Input,
    camera: &Camera,
    offset: [usize; 2],
    sub_res: [usize; 2],
    sample: T,
) -> Output {
    let mut data = Output::new(sub_res);

    let weight = 1.0 / (camera.ss_power * camera.ss_power) as f64;
    let mut rng = thread_rng();

    for px in 0..sub_res[0] {
        for py in 0..sub_res[1] {
            let rx = offset[0] + px;
            let ry = offset[1] + py;

            for ssx in 0..camera.ss_power {
                for ssy in 0..camera.ss_power {
                    let ray = camera.emit([rx, ry], [ssx, ssy]);
                    sample(input, camera, ray, weight, [px, py], &mut data, &mut rng);
                }
            }
        }
    }

    data
}
