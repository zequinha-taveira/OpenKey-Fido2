//! Contratos compartilhados de transporte do autenticador FIDO2.
//!
//! [`Transport`] (trait object-safe + [`DummyTransport`]), contratos HAL
//! [`embedded`], roteamento smart-card [`iso7816`] e agregador
//! [`MultiTransport`]. Consumido pelas crates irmãs (`openkey-transport-hid`,
//! `openkey-transport-ccid`, `openkey-transport-nfc`,
//! `openkey-transport-bluetooth`) e re-exportado pela facade
//! `openkey-transports`.
//!
//! When the `embedded` feature is enabled, the [`embedded`] module provides
//! `no_std` trait contracts and reference implementations for targets that
//! implement [`embedded-hal`].

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "embedded")]
pub mod embedded;

pub mod iso7816;
pub mod multitransport;
pub mod transport;

#[cfg(all(feature = "embedded", feature = "usb-device"))]
pub use embedded::usb_ccid_backend::{CcidClass, UsbCcidBackend, UsbCcidBackendDevice};
#[cfg(all(feature = "embedded", feature = "usb-device"))]
pub use embedded::usb_hid_backend::{CtapHidClass, UsbHidBackend};
pub use iso7816::{Applet, CardRouter, ResponseData};
pub use multitransport::MultiTransport;
pub use transport::{DummyTransport, Transport, TransportError};
