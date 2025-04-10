use log::{debug, error, info, log, trace, warn};
pub fn log_examples() {
    trace!("This is a trace level message demonstrating detailed logging.");
    debug!("This is a debug level message for debugging purposes.");
    info!("This is an info level message, providing general operational information.");
    warn!("This is a warning, indicating a potential issue.");
    error!("This is an error message, signaling a failure that occurred.");
    log!(
        log::Level::Info,
        "This is a generic log! macro message at the Info level."
    );
}

macro_rules! impl_T {
    (for $($t:ty),+) => {
        $(impl T for $t {
            fn double(&self) -> u32 {
                self.x * 2
            }
        })*
    }
}
