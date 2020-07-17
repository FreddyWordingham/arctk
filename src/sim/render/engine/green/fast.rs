//! Live-scheme rendering function.

// use super::{paint, Output};
use super::Output;
use crate::{
    render::{Input, Scene},
    Error,
};
// use minifb::{CursorStyle, Scale, ScaleMode, Window, WindowOptions};
// use palette::Pixel;
// use rand::{thread_rng, Rng};
// use rayon::prelude::*;
// use std::{
//     f64::consts::PI,
//     sync::{Arc, Mutex},
// };

/// Render an image fast.
/// # Errors
/// if a mutex unwrapping failed or
/// an arc unwrapping failed.
#[allow(clippy::result_expect_used)]
#[inline]
pub fn run(_input: &Input, scene: &Scene) -> Result<Output, Error> {
    let width = scene.cam().sensor().res().0 as usize;
    let height = scene.cam().sensor().res().1 as usize;

    println!("Hello!");

    let data = Output::new([width, height]);
    Ok(data)
}
