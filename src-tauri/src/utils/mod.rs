//! Example Utilities Module
pub use example::Data;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
mod example;

pub struct DataStore(pub Mutex<example::Data>);

/// Get unix timestamp in seconds
pub fn get_unix_time() -> f64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    println!("{:?}", since_the_epoch);
    since_the_epoch.as_secs_f64()
}
