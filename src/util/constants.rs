use std::{env, ffi::OsString};

/// The `$HOME` environment variable is used to determine the user's home directory.
pub static HOME: std::sync::LazyLock<OsString> = std::sync::LazyLock::new(|| {
    env::var_os("HOME")
        .ok_or("The HOME environment variable is not set")
        .unwrap()
});
