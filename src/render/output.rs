//! Output data.

use ndarray::Array2;
use ndarray_stats::QuantileExt;
use palette::LinSrgba;
use std::path::Path;

use crate::{parse::png, render::Shader};

/// Saveable output data.
pub struct Output {
    /// Colour data.
    pub colour: Array2<LinSrgba>,
    /// Time data.
    pub time: Array2<f64>,
    // /// Thread data.
    // pub thread: Array2<usize>,
}

impl Output {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(res: [usize; 2]) -> Self {
        let colour = Array2::from_elem(res, LinSrgba::new(0.0, 0.0, 0.0, 0.0));
        let time = Array2::zeros(res);
        // let thread = Array2::zeros(res);

        Self {
            colour,
            time,
            // thread,
        }
    }

    /// Save the output, in it's current state, to the given output directory.
    #[inline]
    pub fn save(&self, shader: &Shader, output_dir: &Path, tag: &str) {
        png::save(
            self.colour.view(),
            &output_dir
                .join(&format!("colour_{}", tag))
                .with_extension("png"),
        );

        let max_time = self.time.max().expect("Failed to resolve time data.");
        png::save(
            self.time
                .map(|t| shader.data_grad.get((t / max_time) as f32))
                .view(),
            &output_dir
                .join(&format!("time_{}", tag))
                .with_extension("png"),
        );

        // let max_thread_id = self.thread.max().expect("Failed to resolve thread data.") + 1;
        // png::save(
        //     self.thread
        //         .map(|id| shader.data_grad.get((id / max_thread_id) as f32))
        //         .view(),
        //     &output_dir
        //         .join(&format!("thread_{}", tag))
        //         .with_extension("png"),
        // );
    }
}
