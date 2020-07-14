//! Live-scheme rendering function.

use super::Output;
use crate::{
    render::{Input, Scene},
    Bar, Error, Ray, SilentBar, BLUE, GREEN, RED,
};
use minifb::{CursorStyle, Scale, ScaleMode, Window, WindowOptions};
use palette::{LinSrgba, Pixel};
use rand::{rngs::ThreadRng, thread_rng, Rng};
use rayon::prelude::*;
use std::{
    f64::consts::PI,
    sync::{Arc, Mutex},
};

/// Render an image with a live preview.
/// # Errors
/// if window could not be constructed or
/// a mutex unwrapping failed or
/// an arc unwrapping failed.
#[allow(clippy::result_expect_used)]
#[inline]
pub fn run(input: &Input, scene: &Scene) -> Result<Output, Error> {
    let update_size = input.sett.live().unwrap();
    let width = scene.cam().sensor().res().0 as usize;
    let height = scene.cam().sensor().res().1 as usize;

    let num_pixels = scene.cam().sensor().num_pixels();
    let order = input.sett.order().list(num_pixels);

    let buffer: Vec<u32> = vec![0; num_pixels as usize];
    let buffer = Arc::new(Mutex::new(buffer));
    let mut img_win = Window::new(
        "ARCTK - Rendering",
        width,
        height,
        WindowOptions {
            resize: true,
            scale: Scale::FitScreen,
            scale_mode: ScaleMode::AspectRatioStretch,
            ..WindowOptions::default()
        },
    )
    .expect("Could not create live window.");
    img_win.set_cursor_style(CursorStyle::Crosshair);
    img_win
        .update_with_buffer(&buffer.lock()?, width, height)
        .expect("Could not update window buffer.");

    let mut main_bar = Bar::new("Rendering", num_pixels as u64);

    let data = Output::new([width, height]);
    let data = Arc::new(Mutex::new(data));

    let threads: Vec<usize> = (0..num_cpus::get()).collect();
    while let Some((start, end)) = main_bar.block(update_size) {
        let pb = SilentBar::new(end - start);
        let pb = Arc::new(Mutex::new(pb));

        while !pb.lock()?.is_done() {
            let _out: Vec<()> = threads
                .par_iter()
                .map(|_id| {
                    render_pix(
                        &order,
                        start,
                        &Arc::clone(&pb),
                        input,
                        scene,
                        &Arc::clone(&data),
                        &Arc::clone(&buffer),
                    )
                })
                .collect();
        }

        img_win
            .update_with_buffer(&buffer.lock()?, width, height)
            .expect("Could not update window buffer.");
    }
    main_bar.finish_with_message("Render complete.");

    if let Ok(d) = Arc::try_unwrap(data) {
        return Ok(d.into_inner()?);
    }

    unreachable!("Failed to unwrap data.");
}

/// Render a range of pixels using a single thread.
#[allow(clippy::result_expect_used)]
#[inline]
fn render_pix(
    order: &[u64],
    buffer_start: u64,
    pb: &Arc<Mutex<SilentBar>>,
    input: &Input,
    scene: &Scene,
    data: &Arc<Mutex<Output>>,
    buffer: &Arc<Mutex<Vec<u32>>>,
) {
    let mut rng = thread_rng();
    let super_samples = scene.cam().sensor().super_samples();
    let dof_samples = scene.cam().focus().dof_samples();
    let h_res = scene.cam().sensor().res().0;
    let total_pixels = scene.cam().sensor().num_pixels();

    let weight = 1.0 / f64::from(super_samples * dof_samples);

    if let Some((start, end)) = {
        let mut pb = pb.lock().expect("Could not lock progress bar.");
        let b = pb.block(input.sett.block_size());
        std::mem::drop(pb);
        b
    } {
        for q in start..end {
            let now = std::time::Instant::now();

            let p = order[(q + buffer_start) as usize];
            let pixel = [(p % h_res) as usize, (p / h_res) as usize];
            let mut col = palette::LinSrgba::default();

            for sub_sample in 0..super_samples {
                let offset = rng.gen_range(0.0, 2.0 * PI);
                for depth_sample in 0..dof_samples {
                    let ray = scene.cam().gen_ray(pixel, offset, sub_sample, depth_sample);
                    col += paint(&mut rng, input, scene, ray, 1.0) * weight as f32;
                }
            }

            let time = std::time::Instant::now().duration_since(now).as_nanos();
            let t = time_scaler(weight * time as f64);
            let time_col = input.cols.map()["time"].get(t as f32);
            data.lock().expect("Could not lock data.").time[pixel] = time_col;
            let raw_time: [u8; 4] = time_col.into_format().into_raw();

            data.lock().expect("Could not lock data.").image[pixel] = col;
            let _raw_col: [u8; 4] = col.into_format().into_raw();
            buffer.lock().expect("Could not lock window buffer.")
                [(total_pixels - (p + 1)) as usize] =
                // from_u8_rgb(raw_col[RED], raw_col[GREEN], raw_col[BLUE]);
                from_u8_rgb(raw_time[RED], raw_time[GREEN], raw_time[BLUE]);
        }
    }
}

/// Naboo rendering engine function.
#[allow(clippy::never_loop)]
#[allow(clippy::option_expect_used)]
#[allow(clippy::single_match_else)]
#[allow(clippy::too_many_lines)]
#[inline]
#[must_use]
pub fn paint(
    mut _rng: &mut ThreadRng,
    _input: &Input,
    _scene: &Scene,
    mut _ray: Ray,
    mut _weight: f64,
) -> LinSrgba {
    LinSrgba::default()
}

/// Create a 32 bit colour representation from 8 bit components.
#[inline]
#[must_use]
const fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

/// Scale a time in nano seconds to between zero and unity.
#[inline]
#[must_use]
fn time_scaler(time: f64) -> f64 {
    (time.log10() * 0.1).min(1.0)
}
