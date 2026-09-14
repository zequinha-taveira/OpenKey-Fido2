//! MCU-independent authenticator domain state.

pub mod authenticator;
pub mod capabilities;
pub mod state;

pub use authenticator::AuthenticatorDomain;
pub use capabilities::AuthenticatorCapabilities;
pub use state::AuthenticatorState;
