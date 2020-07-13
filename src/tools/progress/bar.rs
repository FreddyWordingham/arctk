//! Progress-Bar implementation.

/// Progress-bar structure.
pub struct Bar {
    /// Graphics.
    pb: indicatif::ProgressBar,
    /// Current value.
    count: u64,
    /// Total target value.
    total: u64,
}

impl Bar {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(msg: &str, total: u64) -> Self {
        debug_assert!(total > 0);

        let pb = indicatif::ProgressBar::new(total);

        pb.set_style(
            indicatif::ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.green/red}] [{pos}/{len}] {percent}% ({eta}) {msg}")
            .progress_chars("\\/")
        );
        pb.set_message(msg);

        Self {
            pb,
            count: 0,
            total,
        }
    }

    /// Request a block of values to work on.
    /// Return the requested block if available.
    /// If there is not enough, return the remaining block.
    /// If there are none at all, return None.
    #[inline]
    pub fn block(&mut self, size: u64) -> Option<(u64, u64)> {
        debug_assert!(size > 0);

        if self.count >= self.total {
            None
        } else {
            let remaining = self.total - self.count;
            let alloc = size.min(remaining);

            let start = self.count;
            let end = start + alloc;

            self.count += alloc;
            self.pb.inc(alloc);

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
