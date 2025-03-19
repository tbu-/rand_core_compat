#[cfg(feature = "rand_core_0_5")]
pub use v0_5::*;
#[cfg(feature = "rand_core_0_6")]
pub use v0_6::*;
#[cfg(feature = "rand_core_0_9")]
pub use v0_9::*;

#[cfg(feature = "rand_core_0_5")]
mod v0_5;
#[cfg(feature = "rand_core_0_6")]
mod v0_6;
#[cfg(feature = "rand_core_0_9")]
mod v0_9;
