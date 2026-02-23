#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(missing_docs)]

/// `rand_core 0.5` (corresponding to `rand 0.7`).
#[cfg(feature = "rand_core_0_5")]
pub extern crate rand_core_0_5;

/// `rand_core 0.6` (corresponding to `rand 0.8`).
#[cfg(feature = "rand_core_0_6")]
pub extern crate rand_core_0_6;

/// `rand_core 0.9` (corresponding to `rand 0.9`).
#[cfg(feature = "rand_core_0_9")]
pub extern crate rand_core_0_9;

/// `rand_core 0.10` (corresponding to `rand 0.10`).
#[cfg(feature = "rand_core_0_10")]
pub extern crate rand_core_0_10;

pub use compat::*;

/// Custom error codes used when compiled without `std` support.
#[cfg(any(feature = "rand_core_0_5", feature = "rand_core_0_6"))]
pub mod error {
    use core::num::NonZeroU32;

    /// Error code when we couldn't extract an error code from the original error.
    pub const UNKNOWN: NonZeroU32 = match NonZeroU32::new(3222222222) {
        Some(n) => n,
        None => unreachable!(),
    };
    /// Error code when the original error code was 0.
    pub const OS_ERROR_0: NonZeroU32 = match NonZeroU32::new(3222222223) {
        Some(n) => n,
        None => unreachable!(),
    };
}

#[cfg(not(any(
    feature = "rand_core_0_5",
    feature = "rand_core_0_6",
    feature = "rand_core_0_9",
    feature = "rand_core_0_10",
)))]
compile_error!(concat!(
    "rand_core_compat: activate the versions you need compatibility between:\n",
    "`rand_core_0_5`, `rand_core_0_6`, `rand_core_0_9`, `rand_core_0_10`",
));

mod compat;
