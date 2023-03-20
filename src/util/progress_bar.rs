//! Progress-Bar implementation.

use atty::Stream;

/// Progress-bar structure.
pub struct ProgressBar {
    /// Graphics.
    pb: indicatif::ProgressBar,
    /// Current value.
    count: usize,
    /// Total target value.
    total: usize,
}

impl ProgressBar {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(msg: &'static str, total: usize) -> Self {
        debug_assert!(total > 0);

        let pb = indicatif::ProgressBar::new(total as u64);
        pb.set_style(
            indicatif::ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.green/red}] [{pos}/{len}] {percent}% ({eta}) {msg}")
                .expect("Failed to set progress-bar style.")
                .progress_chars("\\/"),
        );
        pb.set_message(msg);

        Self {
            pb,
            count: 0,
            total,
        }
    }

    /// Tick the bar forward a single increment.
    #[allow(clippy::print_stdout)]
    #[inline]
    pub fn tick(&mut self) {
        self.count += 1;
        self.pb.inc(1);

        if !atty::is(Stream::Stdout) {
            println!("{:.2}", self.count as f64 / self.total as f64 * 100.0);
        }
    }

    /// Request a block of values to work on.
    /// Return the requested block if available.
    /// If there is not enough, return the remaining block.
    /// If there are none at all, return None.
    #[inline]
    #[allow(clippy::print_stdout)]
    pub fn block(&mut self, size: usize) -> Option<(usize, usize)> {
        debug_assert!(size > 0);

        if self.count >= self.total {
            None
        } else {
            let remaining = self.total - self.count;
            let alloc = size.min(remaining);

            let start = self.count;
            let end = start + alloc;

            self.count += alloc;
            self.pb.inc(alloc as u64);

            if !atty::is(Stream::Stdout) {
                println!("{:.2}", self.count as f64 / self.total as f64 * 100.0);
            }

            Some((start, end))
        }
    }

    /// Check if the progress bar is complete.
    #[inline]
    #[must_use]
    pub const fn is_done(&self) -> bool {
        self.count >= self.total
    }

    /// Finish with a message.
    #[inline]
    pub fn finish_with_message(&mut self, msg: &'static str) {
        self.pb.finish_with_message(msg);
    }
}
