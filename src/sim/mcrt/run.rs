//! MCRT run functions.

use crate::{
    mcrt::{Input, Output},
    Bar, Error, X, Y, Z,
};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Run an MCRT simulation.
#[inline]
pub fn run(input: &Input) -> Result<Output, Error> {
    let res = *input.grid.res();
    let num_cells = res[X] * res[Y] * res[Z];
    let br = Bar::new("Mapping", num_cells as u64);
    let br = Arc::new(Mutex::new(br));

    let threads: Vec<usize> = (0..num_cpus::get()).collect();
    let mut out: Vec<Output> = threads
        .par_iter()
        .map(|_id| run_thread(&Arc::clone(&br), input))
        .collect();
    br.lock()?.finish_with_message("Render complete.");

    let mut data = out.pop().expect("No data received.");
    while let Some(o) = out.pop() {
        data += &o;
    }

    Ok(data)
}

/// Run and MCRT simulation using a single thread.
#[inline]
#[must_use]
pub fn run_thread(pb: &Arc<Mutex<Bar>>, input: &Input) -> Output {
    let res = *input.grid.res();
    let mut data = Output::new(res);

    while let Some((start, end)) = {
        let mut pb = pb.lock().expect("Could not lock progress bar.");
        let b = pb.block(input.sett.block_size());
        std::mem::drop(pb);
        b
    } {
        for i in start..end {}
    }

    data
}
