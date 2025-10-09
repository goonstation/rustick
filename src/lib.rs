pub mod realtimers;

#[cfg(all(not(target_pointer_width = "32"), not(feature = "allow_non_32bit")))]
compile_error!(
    "Compiling for non-32bit is not allowed without enabling the `allow_non_32bit` feature."
);
