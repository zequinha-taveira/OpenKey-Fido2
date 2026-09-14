//! BLE GATT (FIDO Bluetooth Service) transport.
//!
//! [`BleGattTransport`] stub mais fragmentação [`FramedBleGattTransport`]
//! (feature `embedded`) sobre qualquer
//! [`transport_core::embedded::BleGattDevice`](transport_core::embedded::BleGattDevice).

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod ble_gatt;

#[cfg(feature = "embedded")]
pub mod framed_ble;

pub use ble_gatt::{
    BleGattTransport, BLE_CMD_CANCEL, BLE_CMD_ERROR, BLE_CMD_KEEPALIVE, BLE_CMD_MSG, BLE_CMD_PING,
    BLE_DEFAULT_MTU, BLE_HEADER_LEN, BLE_MAX_MESSAGE_LEN, BLE_MAX_NOTIFICATION_LEN,
    FIDO_CONTROL_POINT_UUID16, FIDO_SERVICE_REVISION_UUID16, FIDO_SERVICE_UUID16,
    FIDO_STATUS_UUID16,
};
#[cfg(feature = "embedded")]
pub use framed_ble::{ble_fragment, FramedBleGattTransport};
