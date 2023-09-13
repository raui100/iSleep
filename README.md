# isleep
Provides the functions `snooze` and `accurate_snooze` for intermittent sleeping which return the control flow inbetween.  

Relies on platform support for `std::time` and `std::thread`.

# Example
```rust
// Sleeping for a total of 1 s
let total = std::time::Duration::from_secs(1);
// Interrupting the sleep after 100 ms
let len = std::time::Duration::from_millis(100);
// Starting now
let start = std::time::Instant::now();
// Sleeps for `total` in steps up to `len`
// Wont sleep longer than `total` within accuracy of the platform
while snooze(start, total, len) {
   println!("Checking if the user pressed CTRL+C...");
}
```

# Accuracy
The accuracy is platform dependent and might be low for small durations (eg: <20 ms on Windows).
Higher accuracy can be achieved with the `accuracy` feature and `accurate_snooze` which
utilizes [spin_sleep](https://crates.io/crates/spin_sleep).

`cargo add isleep --features=accuracy`