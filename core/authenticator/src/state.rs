//! Authenticator lifecycle and operational state.

use serde::{Deserialize, Serialize};

/// Current state and runtime counters of the authenticator.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthenticatorState {
    pub initialized: bool,
    pub sign_count: u32,
    pub pin_set: bool,
    pub pin_retries: u8,
}
