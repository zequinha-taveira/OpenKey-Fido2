//! Authenticator main abstraction and execution logic.

use crate::capabilities::AuthenticatorCapabilities;
use crate::state::AuthenticatorState;

/// OpenKey FIDO2 Authenticator core engine.
#[derive(Debug)]
pub struct Authenticator {
    state: AuthenticatorState,
    capabilities: AuthenticatorCapabilities,
}

impl Authenticator {
    /// Creates a new Authenticator instance.
    pub fn new(capabilities: AuthenticatorCapabilities) -> Self {
        Self {
            state: AuthenticatorState::default(),
            capabilities,
        }
    }

    /// Returns the current state of the authenticator.
    pub fn state(&self) -> &AuthenticatorState {
        &self.state
    }

    /// Returns the capabilities supported by this authenticator.
    pub fn capabilities(&self) -> &AuthenticatorCapabilities {
        &self.capabilities
    }
}
