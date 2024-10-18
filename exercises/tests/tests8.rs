// tests8.rs
//
// This execrise shares `build.rs` with the previous exercise.
// You need to add some code to `build.rs` to make both this exercise and
// the previous one work.
//
// Execute `rustlings hint tests8` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

// build.rs
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // Get the current timestamp in seconds since the Unix epoch
    let current_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    // Set the TEST_FOO environment variable to the current timestamp
    println!("cargo:TEST_FOO={}", current_timestamp);

    // Enable the `pass` feature if the tests are being run
    if let Ok(_) = env::var("CARGO_CFG_TEST") {
        println!("cargo:rustc-cfg=feature=\"pass\"");
    }

    // Ensure the build script runs again if this file changes
    println!("cargo:rerun-if-changed=build.rs");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        #[cfg(feature = "pass")]
        return;

        panic!("no cfg set");
    }
}
