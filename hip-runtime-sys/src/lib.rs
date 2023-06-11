mod bindings;
pub use bindings::*;

use std::{env::VarError, path::PathBuf};

pub const DEFAULT_HIP_PATH: &str = "/opt/rocm/hip";

/// Attempt to identify the HIP installation path from the `HIP_PATH`
/// environment variable, or if that isn't available or doesn't exist, use the
/// [`DEFAULT_HIP_PATH`].
pub fn get_hip_path() -> PathBuf {
    match std::env::var("HIP_PATH") {
        Ok(hip_path) => {
            let pb = PathBuf::from(hip_path);
            // Check that the path exists.
            if !pb.exists() {
                // Use the default.
                PathBuf::from(DEFAULT_HIP_PATH)
            } else {
                pb
            }
        }
        Err(VarError::NotPresent) => PathBuf::from(DEFAULT_HIP_PATH),
        Err(e @ VarError::NotUnicode(_)) => panic!("{}: HIP_PATH: {}", env!("CARGO_PKG_NAME"), e),
    }
}
