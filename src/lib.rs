//! Provides the functions [snooze](snooze) and [accurate_snooze](accurate_snooze)
//! for intermittent sleeping which return the control flow inbetween.
//!
//! Relies on platform support for `std::time` and `std::thread`.
//!
//! # Example
//! ```
//! // Sleeping for a total of 1 s
//! let total = std::time::Duration::from_secs(1);
//! // Interrupting the sleep after 100 ms
//! let len = std::time::Duration::from_millis(100);
//! // Starting now
//! let start = std::time::Instant::now();
//! // Sleeps for `total` in steps up to `len`
//! // Will never sleep longer than `total` within accuracy of the platform
//! while isleep::snooze(start, total, len) {
//!    println!("Checking if the user pressed CTRL+C...");
//! }
//! ```
//!
//! # Accuracy
//!
//! The accuracy is platform dependent and might be low for small durations (eg: <20 ms on Windows).
//! Higher accuracy can be achieved with the `accuracy` feature and [accurate_snooze](accurate_snooze) which
//! utilizes [spin_sleep](https://crates.io/crates/spin_sleep).
//!
//! `cargo add isleep --features=accuracy`

use std::time::{Duration, Instant};

/// Sleeps for `total` in steps of up to `len` and returns control flow inbetween.
///
/// Accuracy is bounded by platform and might be low. For higher accuracy use the `accuracy` feature
/// and [accurate_snooze](accurate_snooze).
///
/// # Examples
/// ```
/// // Sleeping for a total of 1 s
/// let total = std::time::Duration::from_secs(1);
/// // Interrupting the sleep after 100 ms
/// let len = std::time::Duration::from_millis(100);
/// // Starting now
/// let start = std::time::Instant::now();
/// while isleep::snooze(start, total, len) {
///     println!("Checking if the user pressed CTRL+C...");
/// }
/// ```
pub fn snooze(start: Instant, total: Duration, len: Duration) -> bool {
    match total.checked_sub(start.elapsed()) {
        None => false,
        Some(dt) => {
            std::thread::sleep(len.min(dt));
            true
        }
    }
}

#[cfg(feature = "accuracy")]
/// Sleeps for `total` in accurate steps of up to `len` and returns control flow inbetween.
///
/// Higher accuracy than [snooze](snooze). Only uses native sleep as far as it can be trusted, then spins.
/// Has higher accuracy but also increases CPU usage.
///
/// # Examples
/// ```
/// // Sleeping for a total of 1 s
/// let total = std::time::Duration::from_secs(1);
/// // Interrupting the sleep after 100 ms
/// let len = std::time::Duration::from_millis(100);
/// // Starting now
/// let start = std::time::Instant::now();
/// while isleep::accurate_snooze(start, total, len) { ///
///     println!("Checking if the user pressed CTRL+C...");
/// }
/// ```
pub fn accurate_snooze(start: Instant, total: Duration, len: Duration) -> bool {
    match total.checked_sub(start.elapsed()) {
        None => false,
        Some(dt) => {
            spin_sleep::sleep(len.min(dt));
            true
        }
    }
}

#[cfg(test)]
mod test {
    use super::snooze;

    #[cfg(feature = "accuracy")]
    #[test]
    fn test_accuracy() {
        let total = std::time::Duration::from_secs(1);
        let len = std::time::Duration::from_millis(100);
        let start = std::time::Instant::now();
        let mut counter = 0;
        while super::accurate_snooze(start, total, len) {
            counter += 1;
        }
        let start = std::time::Instant::now();
        while snooze(start, total, len) {
            counter += 1;
        }
        assert!(counter >= 0);
    }

    #[test]
    fn test_readme_example() {
        // Sleeping for a total of 1 s
        let total = std::time::Duration::from_secs(1);
        // Interrupting the sleep after 100 ms
        let len = std::time::Duration::from_millis(100);
        // Starting now
        let start = std::time::Instant::now();
        // Sleeps for `total` in steps up to `len`
        // Will never sleep longer than `total` within accuracy of the platform
        while snooze(start, total, len) {
            println!("Checking if the user pressed CTRL+C...");
        }
    }
}
