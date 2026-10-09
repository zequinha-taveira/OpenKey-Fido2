//! MCU-independent authenticator domain state.

#[allow(clippy::module_inception)]
pub mod authenticator;
pub mod capabilities;
pub mod state;

pub use authenticator::AuthenticatorDomain;
pub use capabilities::AuthenticatorCapabilities;
pub use state::AuthenticatorState;
