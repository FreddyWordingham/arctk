//! Simulation running functions.

use crate::{
    err::Error,
    sim::cartographer::{engine, Input, Output},
    tools::{linear_to_three_dim, ProgressBar},
};
use rand::thread_rng;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Run a multi-threaded Cartography simulation.
/// # Errors
/// if the progress bar can not be locked.
#[allow(clippy::expect_used)]
#[inline]
pub fn multi_thread<'a>(input: &'a Input) -> Result<Output<'a>, Error> {
    let pb = ProgressBar::new("Multi-threaded", input.grid.num_cells());
    let pb = Arc::new(Mutex::new(pb));

    let num_threads = input
        .sett
        .num_threads()
        .unwrap_or(std::usize::MAX)
        .min(num_cpus::get());
    let threads: Vec<_> = (0..num_threads).collect();
    let mut out: Vec<_> = threads
        .par_iter()
        .map(|_id| thread(input, &Arc::clone(&pb)))
        .collect();
    pb.lock()?.finish_with_message("Mapping complete.");

    let mut data = out.pop().expect("No data received.");
    while let Some(o) = out.pop() {
        data += &o;
    }

    Ok(data)
}

/// Run a Cartography simulation using a single thread.
/// # Errors
/// if the progress bar can not be locked.
#[inline]
pub fn single_thread<'a>(input: &'a Input) -> Result<Output<'a>, Error> {
    let pb = ProgressBar::new("Single-threaded", input.grid.num_cells());
    let pb = Arc::new(Mutex::new(pb));

    Ok(thread(input, &pb))
}

/// Thread control function.
#[allow(clippy::expect_used)]
#[inline]
#[must_use]
fn thread<'a>(input: &'a Input, pb: &Arc<Mutex<ProgressBar>>) -> Output<'a> {
    let res = *input.grid.res();
    let mut data = Output::new(input.mat_reg, res);

    let mut rng = thread_rng();

    let block_size = input.sett.block_size();
    while let Some((start, end)) = {
        let mut pb = pb.lock().expect("Could not lock progress bar.");
        let b = pb.block(block_size);
        std::mem::drop(pb);
        b
    } {
        for n in start..end {
            let index = linear_to_three_dim(n, &res);
            engine(input, &mut rng, index, &mut data);
        }
    }

    data
}
