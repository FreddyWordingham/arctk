//! Pause macro.

/// Pause the thread for the given number of milliseconds.
#[macro_export]
macro_rules! pause {
    ($val:expr) => {
        std::thread::sleep(std::time::Duration::from_millis($val));
    };
}
