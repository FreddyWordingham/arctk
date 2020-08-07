//! MCRT thread control functions.

use crate::{
    mcrt::{Data, Light, Scene},
    Bar, Error,
};
use rand::thread_rng;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Run a multi-threaded MCRT simulation.
/// # Errors
/// if the progress bad can not be locked.
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::option_expect_used)]
#[inline]
pub fn multi_thread(scene: &Scene, light: &Light) -> Result<Data, Error> {
    let pb = Bar::new("Multi-threaded", scene.sett.num_phot());
    let pb = Arc::new(Mutex::new(pb));

    let threads: Vec<_> = (0..num_cpus::get()).collect();
    let mut out: Vec<_> = threads
        .par_iter()
        .map(|_id| thread(&Arc::clone(&pb), scene, light))
        .collect();
    pb.lock()?.finish_with_message("Render complete.");

    let mut data = out.pop().expect("No data received.");
    while let Some(o) = out.pop() {
        data += &o;
    }

    Ok(data)
}

/// Run and MCRT simulation using a single thread.
#[allow(clippy::module_name_repetitions)]
#[inline]
#[must_use]
pub fn single_thread(scene: &Scene, light: &Light) -> Data {
    let pb = Bar::new("Single-threaded", scene.sett.num_phot());
    let pb = Arc::new(Mutex::new(pb));

    thread(&pb, scene, light)
}

/// Run and MCRT simulation using a single thread.
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::result_expect_used)]
#[inline]
#[must_use]
pub fn thread(pb: &Arc<Mutex<Bar>>, scene: &Scene, light: &Light) -> Data {
    let res = *scene.grid.res();
    let mut data = Data::new(
        scene.grid.boundary().clone(),
        res,
        scene.sett.range(),
        scene.sett.hist_bins(),
    );

    let mut rng = thread_rng();

    while let Some((start, end)) = {
        let mut pb = pb.lock().expect("Could not lock progress bar.");
        let b = pb.block(scene.sett.block_size());
        std::mem::drop(pb);
        b
    } {
        for _i in start..end {
            let phot = light.emit(&mut rng, light.power() / scene.sett.num_phot() as f64);
            // let mat = &scene.mats.map()["air"];

            let sample = super::simulate_photon(scene, &mut rng, &mut data, phot);
            data.escaped_weight += sample.remaining_weight;
        }
    }

    data
}
