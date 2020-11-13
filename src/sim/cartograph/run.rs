//! Simulation control functions.

use super::{engine::Engine, Data, Landscape};
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
pub fn multi_thread(engine: Engine, land: &Landscape) -> Result<Data, Error> {
    let pb = ProgressBar::new("Multi-threaded", land.grid.total_cells() as u64);
    let pb = Arc::new(Mutex::new(pb));

    let threads: Vec<_> = (0..num_cpus::get()).collect();
    let mut out: Vec<_> = threads
        .par_iter()
        .map(|_id| thread(engine, &Arc::clone(&pb), land))
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
pub fn single_thread(engine: Engine, land: &Landscape) -> Result<Data, Error> {
    let pb = ProgressBar::new("Single-threaded", land.grid.total_cells() as u64);
    let pb = Arc::new(Mutex::new(pb));

    Ok(thread(engine, &pb, land))
}

/// Thread control function.
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::expect_used)]
#[inline]
#[must_use]
fn thread(engine: Engine, pb: &Arc<Mutex<ProgressBar>>, land: &Landscape) -> Data {
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
            engine(land, &mut data, index, &mut rng);
        }
    }

    data
}
