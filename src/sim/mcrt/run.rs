//! Simulation control functions.

use crate::{
    data::Histogram,
    err::Error,
    sim::mcrt::{Engine, Input, Output},
    tools::ProgressBar,
};
use rand::thread_rng;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Spectrometer minimum range value.
const SPECTROMETER_MIN: f64 = 400e-9;
/// Spectrometer maximum range value.
const SPECTROMETER_MAX: f64 = 800e-9;
/// Spectrometer resolution.
const SPECTROMETER_BINS: u64 = 40;

/// Run a multi-threaded MCRT simulation.
/// # Errors
/// if the progress bar can not be locked.
#[allow(clippy::expect_used)]
#[inline]
pub fn multi_thread<'a>(engine: Engine, input: &'a Input) -> Result<Output<'a>, Error> {
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
pub fn single_thread<'a>(engine: Engine, input: &'a Input) -> Result<Output<'a>, Error> {
    let pb = ProgressBar::new("Single-threaded", input.sett.num_phot());
    let pb = Arc::new(Mutex::new(pb));

    Ok(thread(engine, input, &pb))
}

/// Thread control function.
#[allow(clippy::expect_used)]
#[inline]
#[must_use]
fn thread<'a>(engine: Engine, input: &'a Input, pb: &Arc<Mutex<ProgressBar>>) -> Output<'a> {
    let res = *input.grid.res();

    let mut spectrometers = Vec::with_capacity(input.spec_reg.len());
    for _ in 0..input.spec_reg.len() {
        spectrometers.push(Histogram::new(
            SPECTROMETER_MIN,
            SPECTROMETER_MAX,
            SPECTROMETER_BINS,
        ));
    }

    let mut data = Output::new(
        input.spec_reg,
        input.grid.boundary().clone(),
        res,
        spectrometers,
    );

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
