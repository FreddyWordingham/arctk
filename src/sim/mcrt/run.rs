//! Simulation control functions.

use crate::{
    err::Error,
    sim::mcrt::{Engine, Input, Output},
    tools::ProgressBar,
};
use rand::thread_rng;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Run a multi-threaded MCRT simulation.
/// # Errors
/// if the progress bar can not be locked.
#[allow(clippy::expect_used)]
#[inline]
pub fn multi_thread(engine: Engine, input: &Input) -> Result<Output, Error> {
    let pb = ProgressBar::new("Multi-threaded", input.sett.num_phot());
    let pb = Arc::new(Mutex::new(pb));

    let threads: Vec<_> = (0..num_cpus::get()).collect();
    let mut out: Vec<_> = threads
        .par_iter()
        .map(|_id| thread(engine, input, &Arc::clone(&pb)))
        .collect();
    pb.lock()?.finish_with_message("Simulation complete.");

    let mut data = out.pop().expect("No data received.");
    while let Some(o) = out.pop() {
        data += &o;
    }

    Ok(data)
}

/// Run a MCRT simulation using a single thread.
/// # Errors
/// if the progress bar can not be locked.
#[inline]
pub fn single_thread(engine: Engine, input: &Input) -> Result<Output, Error> {
    let pb = ProgressBar::new("Single-threaded", input.sett.num_phot());
    let pb = Arc::new(Mutex::new(pb));

    Ok(thread(engine, input, &pb))
}

/// Thread control function.
#[allow(clippy::expect_used)]
#[inline]
#[must_use]
fn thread(engine: Engine, input: &Input, pb: &Arc<Mutex<ProgressBar>>) -> Output {
    let res = *input.grid.res();
    let mut data = Output::new(input.grid.boundary().clone(), res);

    let mut rng = thread_rng();

    let phot_energy = input.light.power() / input.sett.num_phot() as f64;

    let block_size = input.sett.block_size();
    while let Some((start, end)) = {
        let mut pb = pb.lock().expect("Could not lock progress bar.");
        let b = pb.block(block_size);
        std::mem::drop(pb);
        b
    } {
        for _ in start..end {
            let phot = input.light.emit(&mut rng, phot_energy);
            engine(input, &mut rng, phot, &mut data);
        }
    }

    data
}
