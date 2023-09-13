//! Accurate intermittent sleeping and returning the control flow inbetween.
//! Sleeping for the duration of `len` or up to `self.total`.
//! Won't sleep longer than `len` within accuracy of the operation system.
//!
//! For higher accuracy try the feature `accuracy` and `accurate_snooze` which
//! utilizes [spin_sleep](https://crates.io/crates/spin_sleep).
//!
//! # Examples
//! ```
//! use std::time::{Duration, Instant};
//! let total = Duration::from_secs(1);  // Sleeping for a total of 1 s
//! let len = Duration::from_millis(300);  // Interrupting the sleep after 100 ms
//! let int = isleep::IntermittentSleeping::new(total);
//! let start = Instant::now();
//! let mut counter = 0;
//! while int.snooze(len) {
//!     counter += 1;  // Doing something while "sleeping"
//! }
//! // Sleeps approximately 300, 300, 300 and 100 ms ≈ `total`
//! assert!(start.elapsed().lt(&Duration::from_millis(1200)));
//! assert_eq!(counter, 4);
//! ```
use std::time::{Duration, Instant};

/// Intermittent sleeping for a specified duration since struct instantiation
pub struct IntermittentSleeping {
    /// Total time that is supposed to be spent sleeping
    total: Duration,
    /// Point in time when the instance has been created.
    start: Instant,
}

impl IntermittentSleeping {
    /// Sleeps for `len` or up to `self.total` within accuracy of the OS.
    pub fn snooze(&self, len: Duration) -> bool {
        match self.total.checked_sub(self.start.elapsed()) {
            Some(dt) => {
                std::thread::sleep(len.min(dt));
                true
            }
            None => false,
        }
    }

    pub fn new(total: Duration) -> Self {
        Self {
            total,
            start: Instant::now(),
        }
    }

    #[cfg(feature = "accuracy")]
    /// Sleeps for `len` or up to `self.total` with high accuracy utilizing spin locks.
    pub fn accurate_snooze(&self, len: Duration) -> bool {
        match self.total.checked_sub(self.start.elapsed()) {
            Some(dt) => {
                spin_sleep::sleep(len.min(dt));
                true
            }
            None => false,
        }
    }
}

#[cfg(feature = "accuracy")]
#[test]
fn test_accuracy() {
    let responsiveness = Duration::from_micros(1);
    let mut counter_1 = 0;
    let mut counter_2 = 0;
    let int = IntermittentSleeping::new(Duration::from_secs(1));
    while int.snooze(responsiveness) {
        counter_1 += 1;
    }
    let int = IntermittentSleeping::new(Duration::from_secs(1));
    while int.accurate_snooze(responsiveness) {
        counter_2 += 1;
    }
    assert!(counter_1 <= counter_2);
}

#[rustfmt::skip]
#[test]
fn test_readme_example() {
    let total = std::time::Duration::from_secs(1); // Sleeping for a total of 1 s
    let len = std::time::Duration::from_millis(100); // Interrupting the sleep after 100 ms
    let snoozy = IntermittentSleeping::new(total); // `total` start at class initialization
    // while snoozy.accurate_snooze(len):  // <- higher accuracy with the `accuracy` feature
    while snoozy.snooze(len) {
        println!("Checking if the user pressed CTRL+C...");
    }
}
