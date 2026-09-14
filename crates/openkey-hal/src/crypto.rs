/// Optional hardware acceleration hooks for target integrations.
pub trait CryptoAccelerator {
    type Error;

    fn sha256(&mut self, input: &[u8], output: &mut [u8; 32]) -> Result<(), Self::Error>;
}
