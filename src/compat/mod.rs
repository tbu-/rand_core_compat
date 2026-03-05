#[cfg(feature = "rand_core_0_5")]
pub use v0_5::*;
#[cfg(feature = "rand_core_0_6")]
pub use v0_6::*;
#[cfg(feature = "rand_core_0_9")]
pub use v0_9::*;
#[cfg(feature = "rand_core_0_10")]
pub use v0_10::*;

#[cfg(feature = "rand_core_0_5")]
mod v0_5;
#[cfg(feature = "rand_core_0_6")]
mod v0_6;
#[cfg(feature = "rand_core_0_9")]
mod v0_9;
// FIXME(https://github.com/rust-lang/rustfmt/issues/6820): This comment fixes sort ordering.
#[cfg(feature = "rand_core_0_10")]
mod v0_10;
