# Intermittent Sleeping
Sleeps for a while and returns the control flow afterwards.
Can be used to check for user input (eg: CTRL+C) while sleeping or updating a status bar.

```rust
let total = std::time::Duration::from_secs(1);  // Sleeping for a total of 1 s
let len = std::time::Duration::from_millis(100);  // Interrupting the sleep after 100 ms
let snoozy = isleep::IntermittentSleeping::new(total);  // `total` start at class initialization
// while snoozy.accurate_snooze(len):  // <- higher accuracy with the `accuracy` feature
while snoozy.snooze(len) {
    println!("Checking if the user pressed CTRL+C...");
}
```

## Accuracy
> cargo add isleep --features=accuracy

The accuracy of sleep duration is depending on the operating system. 
Sleeping for less than 15 ms won't work reliably (eg: on Windows).  
For more accurate sleeping the `accuracy` feature and the `accurate_snooze` function can be used.  
This relies on the [spin-sleep](https://github.com/alexheretic/spin-sleep) crate for improved accuracy.