use std::time::{Duration, Instant};

/// Intermittent sleeping for a specified duration since struct instantiation
pub struct IntermittentSleeping {
    /// Total time that is supposed to be spent sleeping
    total: Duration,
    /// Point in time when the instance has been created.
    start: Instant,
}

impl IntermittentSleeping {
    /// Intermittent sleeping and returning the control flow inbetween.
    /// Sleeping for the duration of `len` or up to `self.total`.
    /// Won't sleep longer than `len` within the accuracy of the operating system.
    ///
    /// # Examples
    /// ```
    /// use std::time::{Duration, Instant};
    /// let total = Duration::from_secs(1);  // Sleeping for a total of 1 s
    /// let len = Duration::from_millis(300);  // Interrupting the sleep after 100 ms
    /// let int = isleep::IntermittentSleeping::new(total);
    /// let start = Instant::now();
    /// let mut counter = 0;
    /// while int.snooze(len) {
    ///     counter += 1;  // Doing something while "sleeping"
    /// }
    /// // Sleeps approximately 300, 300, 300 and 100 ms ≈ `total`
    /// assert!(start.elapsed().lt(&Duration::from_millis(1200)));
    /// assert_eq!(counter, 4);
    /// ```
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
}
