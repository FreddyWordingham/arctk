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
pub fn multi_thread<'a>(
    engine: &Engine,
    input: &'a Input,
    output: &Output<'a>,
) -> Result<Output<'a>, Error> {
    let pb = ProgressBar::new("MCRT", input.sett.num_phot());
    let pb = Arc::new(Mutex::new(pb));

    let threads: Vec<_> = (0..num_cpus::get()).collect();
    let mut out: Vec<_> = threads
        .par_iter()
        .map(|_id| thread(engine, input, output.clone(), &Arc::clone(&pb)))
        .collect();
    pb.lock()?.finish_with_message("Simulation complete.");

    let mut data = out.pop().expect("No data received.");
    while let Some(o) = out.pop() {
        data += &o;
    }

    Ok(data)
}

/// Thread control function.
#[allow(clippy::expect_used)]
#[inline]
#[must_use]
fn thread<'a>(
    engine: &Engine,
    input: &'a Input,
    mut output: Output<'a>,
    pb: &Arc<Mutex<ProgressBar>>,
) -> Output<'a> {
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
            engine.run(input, &mut output, &mut rng, phot);
        }
    }

    output
}
