//! User Presence (UP) verification traits and helpers.

/// User presence verification interface.
pub trait UserPresenceVerifier {
    /// Check whether user presence is confirmed (e.g. physical touch / button press).
    fn check_presence(&mut self, timeout_ms: u32) -> Result<bool, &'static str>;
}
