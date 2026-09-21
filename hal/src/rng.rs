//! Hardware Random Number Generator (TRNG) interface.

/// Hardware RNG trait for cryptographically secure random bytes.
pub trait HardwareRng {
    type Error: core::fmt::Debug;

    /// Fill destination buffer with hardware-generated random bytes.
    fn fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Self::Error>;
}
