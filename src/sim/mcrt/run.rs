//! MCRT run functions.

use crate::{
    mcrt::{Input, Light, Output},
    Bar, Error, X, Y, Z,
};
use rand::thread_rng;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Run a multi-threaded MCRT simulation.
/// # Errors
/// if the progress bad can not be locked.
#[inline]
pub fn multi(input: &Input, light: &Light) -> Result<Output, Error> {
    let res = *input.grid.res();
    let num_cells = res[X] * res[Y] * res[Z];
    let br = Bar::new("Mapping", num_cells as u64);
    let br = Arc::new(Mutex::new(br));

    let threads: Vec<usize> = (0..num_cpus::get()).collect();
    let mut out: Vec<Output> = threads
        .par_iter()
        .map(|_id| run_thread(&Arc::clone(&br), input, light))
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
pub fn run_thread(pb: &Arc<Mutex<Bar>>, input: &Input, light: &Light) -> Output {
    let res = *input.grid.res();
    let data = Output::new(res);

    let mut rng = thread_rng();

    while let Some((start, end)) = {
        let mut pb = pb.lock().expect("Could not lock progress bar.");
        let b = pb.block(input.sett.block_size());
        std::mem::drop(pb);
        b
    } {
        for _i in start..end {
            let _phot = light.emit(&mut rng);
            println!("TODO: Engine!");
        }
    }

    data
}
