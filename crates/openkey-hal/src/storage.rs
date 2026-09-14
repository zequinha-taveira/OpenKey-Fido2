/// Minimal flash contract for target-specific persistent storage.
pub trait Storage {
    type Error;

    fn read(&mut self, offset: usize, output: &mut [u8]) -> Result<(), Self::Error>;
    fn write(&mut self, offset: usize, data: &[u8]) -> Result<(), Self::Error>;
    fn erase(&mut self, sector: usize) -> Result<(), Self::Error>;
}
