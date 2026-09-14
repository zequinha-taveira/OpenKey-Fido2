//! USB-HID (CTAPHID) transport.
//!
//! [`UsbHidTransport`] stub mais framing [`ctaphid`] (CTAP 2.1 §8.2) via
//! [`FramedUsbHidTransport`] (feature `embedded`) sobre qualquer
//! [`transport_core::embedded::UsbHidDevice`](transport_core::embedded::UsbHidDevice).

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[path = "../ctaphid/mod.rs"]
pub mod ctaphid;
pub mod usb_hid;

#[cfg(feature = "embedded")]
pub mod framed_hid;

pub use ctaphid::{
    ctaphid_capabilities, ChannelManager, CtaphidAssembler, CtaphidCommand, CtaphidErrorCode,
    CtaphidFragmenter, CtaphidKeepaliveStatus, CtaphidMessage, CtaphidPacket,
};
#[cfg(feature = "embedded")]
pub use framed_hid::FramedUsbHidTransport;
pub use usb_hid::UsbHidTransport;
