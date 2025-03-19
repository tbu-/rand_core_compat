use core::fmt;

/// Wrapper struct for a `rand_core 0.9`/`rand 0.9` RNG.
///
/// It implements traits from the other selected `rand_core`/`rand` versions.
#[derive(Clone, Debug)]
pub struct Rng09<T: rand_core_0_9::RngCore>(pub T);

/// Wrapper struct for a `rand_core 0.9`/`rand 0.9` fallible RNG.
///
/// It implements traits from the other selected `rand_core`/`rand` versions.
#[derive(Clone, Debug)]
pub struct TryRng09<T: rand_core_0_9::TryRngCore>(pub T);

/// Wrapper struct for a `rand_core 0.9`/`rand 0.9` error type.
///
/// It can be converted to errors from the other selected `rand_core`/`rand`
/// versions.
#[derive(Debug)]
pub struct Error09<T: fmt::Debug + fmt::Display + Send + Sync + 'static>(pub T);

#[cfg(feature = "std")]
impl<T: fmt::Debug + fmt::Display + Send + Sync + 'static> std::error::Error for Error09<T> {}

impl<T: fmt::Debug + fmt::Display + Send + Sync + 'static> fmt::Display for Error09<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

#[cfg(feature = "rand_core_0_5")]
mod v0_5 {
    use super::Error09;
    use super::Rng09;
    use super::TryRng09;
    use core::fmt;

    /// Implement the `rand_core 0.5`/`rand 0.7` RNG trait.
    ///
    /// Since the newer trait is infallible, this simply creates a
    /// `try_fill_bytes` function that never returns an error.
    impl<T: rand_core_0_9::RngCore> rand_core_0_5::RngCore for Rng09<T> {
        fn next_u32(&mut self) -> u32 {
            self.0.next_u32()
        }
        fn next_u64(&mut self) -> u64 {
            self.0.next_u64()
        }
        fn fill_bytes(&mut self, dst: &mut [u8]) {
            self.0.fill_bytes(dst)
        }
        fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), rand_core_0_5::Error> {
            self.0.fill_bytes(dst);
            Ok(())
        }
    }

    /// Implement the `rand_core 0.5`/`rand 0.7` crypto RNG trait.
    ///
    /// Since the trait has the same shape, it forwards perfectly.
    impl<T: rand_core_0_9::CryptoRng> rand_core_0_5::CryptoRng for Rng09<T> {}

    /// Implement the `rand_core 0.5`/`rand 0.7` RNG trait.
    ///
    /// Since the older trait cannot express fallible RNGs perfectly, it'll
    /// panic on error if `next_u32`, `next_u64` or `fill_bytes` is called.
    impl<T: rand_core_0_9::TryRngCore> rand_core_0_5::RngCore for TryRng09<T>
    where
        T::Error: Send + Sync + 'static,
    {
        fn next_u32(&mut self) -> u32 {
            rand_core_0_5::impls::next_u32_via_fill(self)
        }
        fn next_u64(&mut self) -> u64 {
            rand_core_0_5::impls::next_u64_via_fill(self)
        }
        fn fill_bytes(&mut self, dst: &mut [u8]) {
            self.0.try_fill_bytes(dst).unwrap();
        }
        fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), rand_core_0_5::Error> {
            Ok(self.0.try_fill_bytes(dst).map_err(Error09)?)
        }
    }

    /// Implement the `rand_core 0.5`/`rand 0.7` crypto RNG trait.
    ///
    /// Since the trait has the same shape, it forwards perfectly.
    impl<T: rand_core_0_9::TryCryptoRng> rand_core_0_5::CryptoRng for TryRng09<T> {}

    /// Convert to an `rand_core 0.5`/`rand 0.7` error.
    ///
    /// If the `std` feature is enabled, then this conversion is lossless by
    /// wrapping this `Error09` struct using [`rand_core_0_5::Error::new`].
    /// Otherwise, it simply returns an error code [`crate::error::UNKNOWN`].
    impl<T: fmt::Debug + fmt::Display + Send + Sync + 'static> From<Error09<T>>
        for rand_core_0_5::Error
    {
        fn from(error: Error09<T>) -> rand_core_0_5::Error {
            #[cfg(feature = "std")]
            {
                rand_core_0_5::Error::new(error)
            }
            #[cfg(not(feature = "std"))]
            {
                use crate::error;

                let _ = error;
                error::UNKNOWN.into()
            }
        }
    }
}

#[cfg(feature = "rand_core_0_6")]
mod v0_6 {
    use super::Error09;
    use super::Rng09;
    use super::TryRng09;
    use core::fmt;

    /// Implement the `rand_core 0.6`/`rand 0.8` RNG trait.
    ///
    /// Since the newer trait is infallible, this simply creates a
    /// `try_fill_bytes` function that never returns an error.
    impl<T: rand_core_0_9::RngCore> rand_core_0_6::RngCore for Rng09<T> {
        fn next_u32(&mut self) -> u32 {
            self.0.next_u32()
        }
        fn next_u64(&mut self) -> u64 {
            self.0.next_u64()
        }
        fn fill_bytes(&mut self, dst: &mut [u8]) {
            self.0.fill_bytes(dst)
        }
        fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), rand_core_0_6::Error> {
            self.0.fill_bytes(dst);
            Ok(())
        }
    }

    /// Implement the `rand_core 0.6`/`rand 0.8` crypto RNG trait.
    ///
    /// Since the trait has the same shape, it forwards perfectly.
    impl<T: rand_core_0_9::CryptoRng> rand_core_0_6::CryptoRng for Rng09<T> {}

    /// Implement the `rand_core 0.6`/`rand 0.8` RNG trait.
    ///
    /// Since the older trait cannot express fallible RNGs perfectly, it'll
    /// panic on error if `next_u32`, `next_u64` or `fill_bytes` is called.
    impl<T: rand_core_0_9::TryRngCore> rand_core_0_6::RngCore for TryRng09<T>
    where
        T::Error: Send + Sync + 'static,
    {
        fn next_u32(&mut self) -> u32 {
            rand_core_0_6::impls::next_u32_via_fill(self)
        }
        fn next_u64(&mut self) -> u64 {
            rand_core_0_6::impls::next_u64_via_fill(self)
        }
        fn fill_bytes(&mut self, dst: &mut [u8]) {
            self.0.try_fill_bytes(dst).unwrap();
        }
        fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), rand_core_0_6::Error> {
            Ok(self.0.try_fill_bytes(dst).map_err(Error09)?)
        }
    }

    /// Implement the `rand_core 0.6`/`rand 0.8` crypto RNG trait.
    ///
    /// Since the trait has the same shape, it forwards perfectly.
    impl<T: rand_core_0_9::TryCryptoRng> rand_core_0_6::CryptoRng for TryRng09<T> {}

    /// Convert to an `rand_core 0.6`/`rand 0.8` error.
    ///
    /// If the `std` feature is enabled, then this conversion is lossless by
    /// wrapping this `Error09` struct using [`rand_core_0_5::Error::new`].
    /// Otherwise, it simply returns an error code [`crate::error::UNKNOWN`].
    impl<T: fmt::Debug + fmt::Display + Send + Sync + 'static> From<Error09<T>>
        for rand_core_0_6::Error
    {
        fn from(error: Error09<T>) -> rand_core_0_6::Error {
            #[cfg(feature = "std")]
            {
                rand_core_0_6::Error::new(error)
            }
            #[cfg(not(feature = "std"))]
            {
                use crate::error;

                let _ = error;
                error::UNKNOWN.into()
            }
        }
    }
}
