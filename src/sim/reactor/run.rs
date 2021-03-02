//! Simulation control functions.

use crate::{
    chem::Reactor,
    err::Error,
    fs::Save,
    math::Vec3,
    ord::{X, Y, Z},
    sim::reactor::{stencil, Input},
    tools::ProgressBar,
};
use ndarray::{Array1, Array2, Array4, Axis};
use ndarray_stats::QuantileExt;
use rayon::prelude::*;
use std::{
    f64::MIN_POSITIVE,
    path::PathBuf,
    sync::{Arc, Mutex},
};

/// Run a single-threaded reaction-diffusion simulation.
/// # Errors
/// if the progress bar can not be locked.
#[allow(clippy::expect_used)]
#[inline]
pub fn multi_thread(
    input: &Input,
    mut values: Array4<f64>,
    out_dir: &PathBuf,
) -> Result<Array4<f64>, Error> {
    Ok(values)
}
