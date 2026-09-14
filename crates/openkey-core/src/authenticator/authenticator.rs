use super::{AuthenticatorCapabilities, AuthenticatorState};

/// Domain-only authenticator state used by device integrations.
#[derive(Debug, Default)]
pub struct AuthenticatorDomain {
    pub state: AuthenticatorState,
    pub capabilities: AuthenticatorCapabilities,
}
