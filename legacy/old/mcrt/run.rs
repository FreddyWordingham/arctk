//! Simulation control functions.

use super::{engine::Engine, Data, Light, Universe};
use crate::{
    err::Error,
    tools::{ProgressBar, Range},
};
use rand::thread_rng;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Run a multi-threaded MCRT simulation.
/// # Errors
/// if the progress bad can not be locked.
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::expect_used)]
#[inline]
pub fn multi_thread(engine: Engine, uni: &Universe, light: &Light) -> Result<Data, Error> {
    let pb = ProgressBar::new("Multi-threaded", uni.sett.num_phot());
    let pb = Arc::new(Mutex::new(pb));

    let threads: Vec<_> = (0..num_cpus::get()).collect();
    let mut out: Vec<_> = threads
        .par_iter()
        .map(|_id| thread(engine, &Arc::clone(&pb), uni, light))
        .collect();
    pb.lock()?.finish_with_message("Simulation complete.");

    let mut data = out.pop().expect("No data received.");
    while let Some(o) = out.pop() {
        data += &o;
    }

    Ok(data)
}

/// Run a MCRT simulation using a single thread.
#[allow(clippy::module_name_repetitions)]
#[inline]
#[must_use]
pub fn single_thread(engine: Engine, uni: &Universe, light: &Light) -> Data {
    let pb = ProgressBar::new("Single-threaded", uni.sett.num_phot());
    let pb = Arc::new(Mutex::new(pb));

    thread(engine, &pb, uni, light)
}

/// Thread control function.
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::expect_used)]
#[inline]
#[must_use]
pub fn thread(engine: Engine, pb: &Arc<Mutex<ProgressBar>>, uni: &Universe, light: &Light) -> Data {
    let res = *uni.grid.res();
    let mut data = Data::new(
        uni.grid.boundary().clone(),
        res,
        Range::new(0.0, 1000.0e-9),
        1000,
    );

    let mut rng = thread_rng();

    let phot_energy = light.power() / uni.sett.num_phot() as f64;
    let block_size = uni.sett.block_size();
    while let Some((start, end)) = {
        let mut pb = pb.lock().expect("Could not lock progress bar.");
        let b = pb.block(block_size);
        std::mem::drop(pb);
        b
    } {
        for _i in start..end {
            let phot = light.emit(&mut rng, phot_energy);

            let sample = engine(&mut rng, uni, &mut data, phot);
            data.escaped_weight += sample.remaining_weight;
        }
    }

    data
}
