//! Secure Element (SE) / Hardware Security Module interface.

/// Hardware Secure Element (e.g. ATECC608A, SE050, Optiga Trust M).
pub trait SecureElement {
    type Error: core::fmt::Debug;

    /// Read public key for a slot index.
    fn get_public_key(&mut self, slot: u8, out_key: &mut [u8]) -> Result<usize, Self::Error>;

    /// Sign digest with key in hardware slot.
    fn sign_digest(&mut self, slot: u8, digest: &[u8; 32], signature: &mut [u8]) -> Result<usize, Self::Error>;

    /// Hardware true random generator from SE.
    fn random_bytes(&mut self, dest: &mut [u8]) -> Result<(), Self::Error>;
}
