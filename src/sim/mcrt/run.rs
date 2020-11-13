//! Simulation control functions.

use super::{Data, Key, Light, Universe};
use crate::{err::Error, tools::ProgressBar};
use rand::thread_rng;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Run a multi-threaded MCRT simulation.
/// # Errors
/// if the progress bad can not be locked.
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::expect_used)]
#[inline]
pub fn multi_thread(uni: &Universe<Key>, light: &Light) -> Result<Data, Error> {
    let pb = ProgressBar::new("Multi-threaded", uni.sett.num_phot);
    let pb = Arc::new(Mutex::new(pb));

    let threads: Vec<_> = (0..num_cpus::get()).collect();
    let mut out: Vec<_> = threads
        .par_iter()
        .map(|_id| thread(&Arc::clone(&pb), uni, light))
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
pub fn single_thread(uni: &Universe<Key>, light: &Light) -> Data {
    let pb = ProgressBar::new("Single-threaded", uni.sett.num_phot);
    let pb = Arc::new(Mutex::new(pb));

    thread(&pb, uni, light)
}

/// Thread control function.
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::expect_used)]
#[inline]
#[must_use]
pub fn thread(pb: &Arc<Mutex<ProgressBar>>, uni: &Universe<Key>, light: &Light) -> Data {
    let res = *uni.grid.res();
    let mut data = Data::new();

    let mut rng = thread_rng();

    while let Some((start, end)) = {
        let mut pb = pb.lock().expect("Could not lock progress bar.");
        let b = pb.block(uni.sett.block_size);
        std::mem::drop(pb);
        b
    } {
        for _i in start..end {
            let phot = light.emit(&mut rng, light.power() / uni.sett.num_phot as f64);

            let sample = super::engine::basic::sample(uni, &mut rng, &mut data, phot);
            data.escaped_weight += sample.remaining_weight;
        }
    }

    data
}
