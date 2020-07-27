//! Mapping function.

use crate::{
    cartographer::{Input, Output},
    Bar, Error, X, Y, Z,
};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Map a volume of surfaces.
/// #Errors
/// if a mutex unwrapping failed or
/// an arc unwrapping failed.
#[inline]
#[must_use]
pub fn map(input: &Input) -> Result<Output, Error> {
    let res = *input.grid.res();
    let num_cells = res[X] * res[Y] * res[Z];
    let bar = Bar::new("Mapping", num_cells as u64);
    let bar = Arc::new(Mutex::new(bar));

    let threads: Vec<usize> = (0..num_cpus::get()).collect();
    let mut out: Vec<Output> = threads
        .par_iter()
        .map(|_id| run_thread(&Arc::clone(&bar), input))
        .collect();
    bar.lock()?.finish_with_message("Render complete.");

    let mut data = out.pop().expect("No data received.");
    while let Some(o) = out.pop() {
        data += &o;
    }

    Ok(data)
}

/// Map a volume of surfaces using a single thread.
#[inline]
#[must_use]
pub fn run_thread(pb: &Arc<Mutex<Bar>>, input: &Input) -> Output {
    let data = Output::new(*input.grid.res());

    while let Some((start, end)) = {
        let mut pb = pb.lock().expect("Could not lock progress bar.");
        let b = pb.block(input.sett.block_size());
        std::mem::drop(pb);
        b
    } {
        for _i in start..end {}
    }

    data
}
