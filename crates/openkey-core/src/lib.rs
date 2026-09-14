//! MCU-independent FIDO2 domain and protocol implementation.
//!
//! Hardware access, transport composition, and product policy belong to the
//! sibling `openkey-hal`, `openkey-transport`, and `openkey-device` crates.

#![cfg_attr(not(feature = "std"), no_std)]

pub mod authenticator;
pub mod credential;
pub mod ctap2;
pub mod error;
pub mod types;
pub mod webauthn;

pub use ctap2::*;
pub use webauthn::{WebAuthnAuthenticator, WebAuthnError};
