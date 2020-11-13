//! Simulation control functions.

use super::{Data, Landscape};
use crate::{
    err::Error,
    tools::{linear_to_three_dim, ProgressBar},
};
use rand::thread_rng;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Run a multi-threaded cartography simulation.
/// # Errors
/// if the progress bad can not be locked.
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::expect_used)]
#[inline]
pub fn multi_thread(land: &Landscape) -> Result<Data, Error> {
    let pb = ProgressBar::new("Multi-threaded", land.grid.total_cells() as u64);
    let pb = Arc::new(Mutex::new(pb));

    let threads: Vec<_> = (0..num_cpus::get()).collect();
    let mut out: Vec<_> = threads
        .par_iter()
        .map(|_id| thread(&Arc::clone(&pb), land))
        .collect();
    pb.lock()?.finish_with_message("Mapping complete.");

    let mut data = out.pop().expect("No data received.");
    while let Some(o) = out.pop() {
        data += &o;
    }

    Ok(data)
}

/// Run a cartography simulation using a single thread.
#[allow(clippy::module_name_repetitions)]
#[inline]
#[must_use]
pub fn single_thread(land: &Landscape) -> Data {
    let pb = ProgressBar::new("Single-threaded", land.grid.total_cells() as u64);
    let pb = Arc::new(Mutex::new(pb));

    thread(&pb, land)
}

/// Thread control function.
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::expect_used)]
#[inline]
#[must_use]
fn thread(pb: &Arc<Mutex<ProgressBar>>, land: &Landscape) -> Data {
    let res = *land.grid.res();
    let mut data = Data::new(land.inters, res);
    let mut rng = thread_rng();

    while let Some((start, end)) = {
        let mut pb = pb.lock().expect("Could not lock progress bar.");
        let b = pb.block(land.sett.block_size());
        std::mem::drop(pb);
        b
    } {
        for i in start as usize..end as usize {
            let index = linear_to_three_dim(i, &res);
            super::engine::basic::sample(land, &mut data, index, &mut rng);
        }
    }

    data
}
