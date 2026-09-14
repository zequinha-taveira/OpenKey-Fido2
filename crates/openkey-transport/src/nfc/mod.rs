//! NFC (ISO/IEC 14443) transport.
//!
//! [`NfcTransport`] stub mais [`FramedNfcTransport`] (feature `embedded`) com
//! roteamento APDU via
//! [`transport_core::iso7816::CardRouter`](transport_core::iso7816::CardRouter).

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod nfc;

#[cfg(feature = "embedded")]
pub mod framed_nfc;

#[cfg(feature = "embedded")]
pub use framed_nfc::FramedNfcTransport;
pub use nfc::NfcTransport;
