//! Core Authenticator Module for OpenKey FIDO2.

pub mod authenticator;
pub mod capabilities;
pub mod state;
pub mod user_presence;

pub use authenticator::Authenticator;
pub use capabilities::AuthenticatorCapabilities;
pub use state::AuthenticatorState;
pub use user_presence::UserPresenceVerifier;
