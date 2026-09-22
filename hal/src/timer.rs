//! Hardware Timer and Delay interface.

/// Hardware timer and monotonic clock interface.
pub trait HardwareTimer {
    /// Return current time in milliseconds since system boot.
    fn now_ms(&self) -> u64;

    /// Busy wait / delay for specified milliseconds.
    fn delay_ms(&mut self, ms: u32);
}
