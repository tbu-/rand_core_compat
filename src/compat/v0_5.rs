/// Wrapper struct for a `rand_core 0.5`/`rand 0.7` RNG.
///
/// It implements traits from the other selected `rand_core`/`rand` versions.
#[derive(Clone, Debug)]
pub struct Rng05<T: rand_core_0_5::RngCore>(pub T);

/// Wrapper struct for a `rand_core 0.5`/`rand 0.7` fallible RNG.
///
/// It implements traits from the other selected `rand_core`/`rand` versions.
#[derive(Clone, Debug)]
pub struct TryRng05<T: rand_core_0_5::RngCore>(pub T);

/// Wrapper struct for a `rand_core 0.5`/`rand 0.7` error type.
///
/// It can be converted to errors from the other selected `rand_core`/`rand`
/// versions.
#[derive(Debug)]
pub struct Error05(pub rand_core_0_5::Error);

#[cfg(feature = "rand_core_0_6")]
mod v0_6 {
    use super::Error05;
    use super::Rng05;
    use super::TryRng05;

    /// Implement the `rand_core 0.6`/`rand 0.8` RNG trait.
    ///
    /// Since the trait has the same shape, it forwards perfectly.
    impl<T: rand_core_0_5::RngCore> rand_core_0_6::RngCore for Rng05<T> {
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
            Ok(self.0.try_fill_bytes(dst).map_err(Error05)?)
        }
    }

    /// Implement the `rand_core 0.6`/`rand 0.8` crypto RNG trait.
    ///
    /// Since the trait has the same shape, it forwards perfectly.
    impl<T: rand_core_0_5::RngCore + rand_core_0_5::CryptoRng> rand_core_0_6::CryptoRng for Rng05<T> {}

    /// Implement the `rand_core 0.6`/`rand 0.8` RNG trait.
    ///
    /// Since the trait has the same shape, it forwards perfectly.
    impl<T: rand_core_0_5::RngCore> rand_core_0_6::RngCore for TryRng05<T> {
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
            Ok(self.0.try_fill_bytes(dst).map_err(Error05)?)
        }
    }

    /// Implement the `rand_core 0.6`/`rand 0.8` crypto RNG trait.
    ///
    /// Since the trait has the same shape, it forwards perfectly.
    impl<T: rand_core_0_5::RngCore + rand_core_0_5::CryptoRng> rand_core_0_6::CryptoRng
        for TryRng05<T>
    {
    }

    /// Convert to an `rand_core 0.6`/`rand 0.8` error.
    ///
    /// If the `std` feature is enabled, then this conversion is lossless.
    /// Otherwise, extracting and duplicating the error code is tried, and
    /// if not successful, the error code is set to [`crate::error::UNKNOWN`].
    impl From<Error05> for rand_core_0_6::Error {
        fn from(error: Error05) -> rand_core_0_6::Error {
            #[cfg(feature = "std")]
            {
                rand_core_0_6::Error::new(error.0.take_inner())
            }
            #[cfg(not(feature = "std"))]
            {
                use crate::error;
                use core::num::NonZeroU32;

                if let Some(code) = error.0.code() {
                    return code.into();
                }
                if let Some(code) = error.0.raw_os_error() {
                    if let Some(code) = NonZeroU32::new(code as u32) {
                        return code.into();
                    } else {
                        return error::OS_ERROR_0.into();
                    }
                }
                error::UNKNOWN.into()
            }
        }
    }
}

#[cfg(feature = "rand_core_0_9")]
mod v0_9 {
    use super::Rng05;
    use super::TryRng05;

    /// Implement the `rand_core 0.9`/`rand 0.9` RNG trait.
    ///
    /// This trait cannot fail and as such will convert failures in the
    /// [`rand_core_0_5::RngCore::try_fill_bytes`] method to panics (by
    /// directly calling the [`rand_core_0_5::RngCore::fill_bytes`] function.
    impl<T: rand_core_0_5::RngCore> rand_core_0_9::RngCore for Rng05<T> {
        fn next_u32(&mut self) -> u32 {
            self.0.next_u32()
        }
        fn next_u64(&mut self) -> u64 {
            self.0.next_u64()
        }
        fn fill_bytes(&mut self, dst: &mut [u8]) {
            self.0.fill_bytes(dst)
        }
    }

    /// Implement the `rand_core 0.9`/`rand 0.9` crypto RNG trait.
    ///
    /// Since the trait has the same shape, it forwards perfectly.
    impl<T: rand_core_0_5::RngCore + rand_core_0_5::CryptoRng> rand_core_0_9::CryptoRng for Rng05<T> {}

    /// Implement the `rand_core 0.9`/`rand 0.9` fallible RNG trait.
    ///
    /// `try_next_u32`/`try_next_u64` are implemented in terms of
    /// `try_fill_bytes` because the old version of the trait lacked the
    /// fallible methods for `u32` and `u64`.
    impl<T: rand_core_0_5::RngCore> rand_core_0_9::TryRngCore for TryRng05<T> {
        type Error = rand_core_0_5::Error;
        fn try_next_u32(&mut self) -> Result<u32, rand_core_0_5::Error> {
            let mut buf = [0; 4];
            self.try_fill_bytes(&mut buf)?;
            Ok(u32::from_le_bytes(buf))
        }
        fn try_next_u64(&mut self) -> Result<u64, rand_core_0_5::Error> {
            let mut buf = [0; 8];
            self.try_fill_bytes(&mut buf)?;
            Ok(u64::from_le_bytes(buf))
        }
        fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), rand_core_0_5::Error> {
            self.0.try_fill_bytes(dst)
        }
    }

    /// Implement the `rand_core 0.9`/`rand 0.9` fallible crypto RNG trait.
    ///
    /// Since the trait has the same shape, it forwards perfectly.
    impl<T: rand_core_0_5::RngCore + rand_core_0_5::CryptoRng> rand_core_0_9::TryCryptoRng
        for TryRng05<T>
    {
    }
}
