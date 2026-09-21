//! Status LED indicator control interface.

/// Status LED color and animation controller.
pub trait LedIndicator {
    /// Turn LED on.
    fn on(&mut self);

    /// Turn LED off.
    fn off(&mut self);

    /// Blink LED pattern (e.g. for user presence waiting).
    fn blink(&mut self, count: u8, interval_ms: u16);
}
