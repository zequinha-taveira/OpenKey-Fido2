//! USB-CCID (smartcard) transport.
//!
//! [`UsbCcidTransport`] stub mais [`FramedCcidTransport`] (feature `embedded`)
//! sobre qualquer
//! [`transport_core::embedded::UsbCcidDevice`](transport_core::embedded::UsbCcidDevice).

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod usb_ccid;

#[cfg(feature = "embedded")]
pub mod framed_ccid;

#[cfg(feature = "embedded")]
pub use framed_ccid::FramedCcidTransport;
pub use usb_ccid::UsbCcidTransport;
