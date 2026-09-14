//! Product composition layer for OpenKey.
//!
//! This crate is intentionally above the MCU-independent core. It combines
//! product configuration with HAL and transport implementations, while board
//! crates under `targets/` provide the concrete hardware.

#![cfg_attr(not(feature = "std"), no_std)]

pub mod authenticator;
pub mod config;
pub mod firmware;
pub mod profile;

pub use authenticator::EmbeddedAuthenticator;
#[cfg(feature = "std")]
pub use authenticator::InsecureHostStorage;
pub use config::*;
pub use profile::{Capabilities, CapabilityDiscovery, DeviceProfile, DeviceProfileBuilder};
