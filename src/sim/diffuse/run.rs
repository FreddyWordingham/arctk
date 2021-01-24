// //! Simulation control functions.

// use crate::{
//     err::Error,
//     sim::mcrt::{Engine, Input, Output},
//     tools::SilentProgressBar,
// };
// use ndarray::Array3;
// use rand::thread_rng;
// use rayon::prelude::*;
// use std::sync::{Arc, Mutex};

// /// Run a multi-threaded Diffuse simulation.
// /// # Errors
// /// if the progress bar can not be locked.
// #[allow(clippy::expect_used)]
// #[inline]
// pub fn multi_thread(input: &Input, concs: Array3<f64>) -> Result<Array3, Error> {
//     let pb = SilentProgressBar::new("Multi-threaded", input.sett.num_phot());
//     let pb = Arc::new(Mutex::new(pb));

//     let threads: Vec<_> = (0..num_cpus::get()).collect();
//     let mut out: Vec<_> = threads
//         .par_iter()
//         .map(|_id| thread(input, &concs, &Arc::clone(&pb)))
//         .collect();
//     pb.lock()?.finish_with_message("Simulation complete.");

//     let mut data = out.pop().expect("No data received.");
//     while let Some(o) = out.pop() {
//         data += &o;
//     }

//     Ok(data)
// }

// /// Run a MCRT simulation using a single thread.
// /// # Errors
// /// if the progress bar can not be locked.
// #[inline]
// pub fn single_thread(input: &Input, concs: Array3<f64>) -> Result<Array3, Error> {
//     let pb = SilentProgressBar::new("Single-threaded", input.grid.num_cells() as u64);
//     let pb = Arc::new(Mutex::new(pb));

//     Ok(thread(input, &concs, &pb))
// }

// /// Thread control function.
// #[allow(clippy::expect_used)]
// #[inline]
// #[must_use]
// fn thread(input: &Input, concs: &Array3<f64>, pb: &Arc<Mutex<SilentProgressBar>>) -> Array3 {
//     let res = *input.grid.res();
//     let data = Array3::zeros(*grid.res());

//     let mut rng = thread_rng();

//     let phot_energy = input.light.power() / input.sett.num_phot() as f64;

//     let block_size = input.sett.block_size();
//     while let Some((start, end)) = {
//         let mut pb = pb.lock().expect("Could not lock progress bar.");
//         let b = pb.block(block_size);
//         std::mem::drop(pb);
//         b
//     } {
//         for _ in start..end {}
//     }

//     data
// }
