//! Live-scheme rendering function.

use crate::render::{Input, Scene};

/// Render an image with a live preview.
/// # Errors
/// if window could not be constructed or
/// a mutex unwrapping failed or
/// an arc unwrapping failed.
#[inline]
pub fn run(_input: &Input, _scene: &Scene)
// -> Result<Output, Error>
{
    // let num_pixels = scene.cam().sensor().num_pixels();
    // let width = scene.cam().sensor().res().0 as usize;
    // let height = scene.cam().sensor().res().1 as usize;

    // let order: Vec<u64> = match input.sett.order() {
    //     Order::Forward => (0..num_pixels).collect(),
    //     Order::Backward => {
    //         let mut o: Vec<u64> = (0..num_pixels).collect();
    //         o.reverse();
    //         o
    //     }
    //     Order::Shuffle => {
    //         let mut o: Vec<u64> = (0..num_pixels).collect();
    //         o.shuffle(&mut thread_rng());
    //         o
    //     }
    // };

    // let img_buffer: Vec<u32> = vec![0; num_pixels as usize];
    // let img_buffer = Arc::new(Mutex::new(img_buffer));
    // let mut img_win = Window::new(
    //     "Rendering in Space",
    //     width,
    //     height,
    //     WindowOptions {
    //         resize: true,
    //         scale: Scale::FitScreen,
    //         scale_mode: ScaleMode::AspectRatioStretch,
    //         ..WindowOptions::default()
    //     },
    // )?;
    // img_win.set_cursor_style(CursorStyle::Crosshair);
    // img_win.update_with_buffer(&img_buffer.lock()?, width, height)?;

    // let mut main_bar = Bar::new("Rendering", num_pixels as u64);

    // let data = Output::new([width, height]);
    // let data = Arc::new(Mutex::new(data));

    // let threads: Vec<usize> = (0..num_cpus::get()).collect();
    // while let Some((start, end)) = main_bar.block(input.sett.block_size()) {
    //     let pb = SilentBar::new(end - start);
    //     let pb = Arc::new(Mutex::new(pb));

    //     while !pb.lock()?.is_done() {
    //         let _out: Vec<()> = threads
    //             .par_iter()
    //             .map(|_id| {
    //                 render_pix(
    //                     engine,
    //                     &order,
    //                     start,
    //                     &Arc::clone(&pb),
    //                     input,
    //                     scene,
    //                     &Arc::clone(&data),
    //                     [&Arc::clone(&img_buffer), &Arc::clone(&time_buffer)],
    //                 )
    //             })
    //             .collect();
    //     }

    //     img_win.update_with_buffer(&img_buffer.lock()?, width, height)?;
    //     time_win.update_with_buffer(&time_buffer.lock()?, width, height)?;
    // }
    // main_bar.finish_with_message("Render complete.");

    // if let Ok(d) = Arc::try_unwrap(data) {
    //     return Ok(d.into_inner()?);
    // }

    // unreachable!("Failed to unwrap data.");
}
