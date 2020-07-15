//! Live-scheme rendering function.

use super::{paint, Output};
use crate::{
    fmt::rgb::components_to_u32,
    render::{Input, Scene},
    Bar, Error, SilentBar, BLUE, GREEN, RED,
};
use minifb::{CursorStyle, Scale, ScaleMode, Window, WindowOptions};
use palette::Pixel;
use rand::{thread_rng, Rng};
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

    let buffer: Vec<u32> = vec![0; (num_pixels * 4) as usize];
    let buffer = Arc::new(Mutex::new(buffer));
    let mut window = Window::new(
        "ARCTK - Rendering",
        width * 2,
        height * 2,
        WindowOptions {
            resize: true,
            scale: Scale::FitScreen,
            scale_mode: ScaleMode::AspectRatioStretch,
            ..WindowOptions::default()
        },
    )
    .expect("Could not create live window.");
    window.set_cursor_style(CursorStyle::Crosshair);
    window
        .update_with_buffer(&buffer.lock()?, width * 2, height * 2)
        .expect("Could not update window buffer.");

    let mut main_bar = Bar::new("Rendering", num_pixels as u64);

    let data = Output::new([width, height]);
    let data = Arc::new(Mutex::new(data));

    let threads: Vec<usize> = (0..num_cpus::get()).collect();
    while let Some((start, end)) = main_bar.block(update_size) {
        let pb = SilentBar::new(end - start);
        let pb = Arc::new(Mutex::new(pb));

        while !pb.lock()?.is_done() {
            threads
                .par_iter()
                .map(|_id| {
                    render_range(
                        &order,
                        start,
                        &Arc::clone(&pb),
                        input,
                        scene,
                        &Arc::clone(&data),
                        &Arc::clone(&buffer),
                    )
                })
                .collect::<()>();
        }

        window
            .update_with_buffer(&buffer.lock()?, width * 2, height * 2)
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
fn render_range(
    order: &[u64],
    offset: u64,
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
        let block = pb.block(input.sett.block_size());
        std::mem::drop(pb);
        block
    } {
        for i in start..end {
            // Timing.
            let now = std::time::Instant::now();

            // Pixel index.
            let p = order[(i + offset) as usize];
            let pixel = [(p % h_res) as usize, (p / h_res) as usize];

            // Image colour.
            let mut col = palette::LinSrgba::default();
            for sub_sample in 0..super_samples {
                let offset = rng.gen_range(0.0, 2.0 * PI);
                for depth_sample in 0..dof_samples {
                    let ray = scene.cam().gen_ray(pixel, offset, sub_sample, depth_sample);
                    col += paint(&mut rng, input, scene, ray, 1.0) * weight as f32;
                }
            }
            data.lock().expect("Could not lock data.").image[pixel] = col;
            let _raw_col: [u8; 4] = col.into_format().into_raw();

            // Time colour.
            let time = std::time::Instant::now().duration_since(now).as_nanos();
            let t = time_scaler(weight * time as f64);
            let time_col = input.cols.map()["time"].get(t as f32);
            data.lock().expect("Could not lock data.").time[pixel] = time_col;
            let raw_time: [u8; 4] = time_col.into_format().into_raw();

            // Window writing.
            let b = p + ((p / h_res) * h_res);
            // println!("{} -> {}", b, p);
            buffer.lock().expect("Could not lock window buffer.")
                // [(total_pixels - (p + 1)) as usize] =
                [b as usize] = components_to_u32(raw_time[RED], raw_time[GREEN], raw_time[BLUE]);
            buffer.lock().expect("Could not lock window buffer.")[(b + h_res) as usize] =
                components_to_u32(raw_time[RED], raw_time[GREEN], raw_time[BLUE]);

            buffer.lock().expect("Could not lock window buffer.")
                [(b + (2 * total_pixels)) as usize] =
                components_to_u32(raw_time[RED], raw_time[GREEN], raw_time[BLUE]);

            buffer.lock().expect("Could not lock window buffer.")
                [(b + (2 * total_pixels) + h_res) as usize] =
                components_to_u32(raw_time[RED], raw_time[GREEN], raw_time[BLUE]);
        }
    }
}

/// Scale a time in nano seconds to between zero and unity.
#[inline]
#[must_use]
fn time_scaler(time: f64) -> f64 {
    time.log10().max(0.0).min(10.0) * 0.1
}
