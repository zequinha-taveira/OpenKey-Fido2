/// Hardware random source supplied by a target.
pub trait RandomSource {
    type Error;

    fn fill_bytes(&mut self, output: &mut [u8]) -> Result<(), Self::Error>;
}
