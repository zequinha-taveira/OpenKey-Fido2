//! Hardware Flash/Non-volatile Storage interface.

/// Raw non-volatile storage flash operations.
pub trait FlashStorage {
    type Error: core::fmt::Debug;

    /// Read bytes from raw storage address.
    fn read(&mut self, offset: u32, buf: &mut [u8]) -> Result<(), Self::Error>;

    /// Write bytes to storage at given offset.
    fn write(&mut self, offset: u32, data: &[u8]) -> Result<(), Self::Error>;

    /// Erase sector/page at given offset.
    fn erase(&mut self, offset: u32, len: u32) -> Result<(), Self::Error>;
}
