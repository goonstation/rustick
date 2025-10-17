#![forbid(unsafe_op_in_unsafe_fn)]

use meowtonin::byond_fn;

pub mod byondtimers;
pub mod core;
pub mod realtimers;
pub mod timer;

#[cfg(all(not(target_pointer_width = "32"), not(feature = "allow_non_32bit")))]
compile_error!(
    "Compiling for non-32bit is not allowed without enabling the `allow_non_32bit` feature."
);

/// Returns the current version of the rustick library.
///
/// # Returns
/// * The version string from Cargo.toml
#[byond_fn]
pub fn get_version() -> Option<&'static str> {
    Some(env!("CARGO_PKG_VERSION"))
}
