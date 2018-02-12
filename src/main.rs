//! This program tries to lock a file, sleeps for N seconds, and then unlocks the file.

// cargo-deps: fs2
extern crate fs2;

use fs2::FileExt;
use std::io::Result;
use std::io::ErrorKind;
use std::env::args;
use std::time::Duration;
use std::thread::sleep;
use std::fs::remove_file;
use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;

fn main() {
    run().unwrap();
}

fn run() -> Result<()> {
    let sleep_seconds = args().nth(1).and_then(|arg| arg.parse().ok()).unwrap_or(0);
    let sleep_duration = Duration::from_secs(sleep_seconds);

    println!("{}: Creating lock file.", sleep_seconds);

    const LOCK_FILENAME: &str = "file.lock";

    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(LOCK_FILENAME)?;

    println!("{}: Preparing to lock file.", sleep_seconds);
    file.lock_exclusive()?; // block until this process can lock the file
    println!("{}: Obtained lock.", sleep_seconds);

    sleep(sleep_duration);

    println!("{}: Sleep completed", sleep_seconds);
    file.unlock()?;
    println!("{}: Released lock, returning", sleep_seconds);

    match remove_file(LOCK_FILENAME) {
        Err(e) => if e.kind() == ErrorKind::NotFound {
            println!("{}: Lock file not found, but OK", sleep_seconds);
            Ok(())
        } else {
            Err(e)
        },
        Ok(()) => Ok(()),
    }
}
