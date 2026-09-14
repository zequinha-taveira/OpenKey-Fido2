//! Unified transport crate for OpenKey.
//!
//! CTAPHID, USB-HID, CCID, NFC, and BLE implementations live in one crate so
//! targets can select a single transport dependency while keeping hardware
//! drivers behind the `embedded` contracts.

#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;

// The historical transport modules use this name internally. Keeping the
// alias local avoids coupling the merged implementation to another crate.
extern crate self as transport_core;

#[path = "core.rs"]
mod transport_core_impl;

#[cfg(feature = "embedded")]
pub use transport_core_impl::embedded;
pub use transport_core_impl::{iso7816, multitransport, transport};

pub mod bluetooth;
pub mod ccid;
pub mod hid;
pub mod nfc;
pub use hid::ctaphid;

pub mod channel;
pub mod framing;

#[cfg(feature = "embedded")]
pub use bluetooth::{ble_fragment, FramedBleGattTransport};
pub use bluetooth::{
    BleGattTransport, BLE_CMD_CANCEL, BLE_CMD_ERROR, BLE_CMD_KEEPALIVE, BLE_CMD_MSG, BLE_CMD_PING,
    BLE_DEFAULT_MTU, BLE_HEADER_LEN, BLE_MAX_MESSAGE_LEN, BLE_MAX_NOTIFICATION_LEN,
    FIDO_CONTROL_POINT_UUID16, FIDO_SERVICE_REVISION_UUID16, FIDO_SERVICE_UUID16,
    FIDO_STATUS_UUID16,
};
#[cfg(feature = "embedded")]
pub use ccid::FramedCcidTransport;
pub use ccid::UsbCcidTransport;
#[cfg(feature = "embedded")]
pub use hid::FramedUsbHidTransport;
pub use hid::{
    ctaphid_capabilities, ChannelManager, CtaphidAssembler, CtaphidCommand, CtaphidErrorCode,
    CtaphidFragmenter, CtaphidKeepaliveStatus, CtaphidMessage, CtaphidPacket, UsbHidTransport,
};
#[cfg(feature = "embedded")]
pub use nfc::FramedNfcTransport;
pub use nfc::NfcTransport;
pub use transport_core_impl::{Applet, CardRouter, DummyTransport, MultiTransport, ResponseData};
#[cfg(all(feature = "embedded", feature = "usb-device"))]
pub use transport_core_impl::{
    CcidClass, CtapHidClass, UsbCcidBackend, UsbCcidBackendDevice, UsbHidBackend,
};
pub use transport_core_impl::{Transport, TransportError};
