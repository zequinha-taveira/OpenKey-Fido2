//! Applets de produto do dispositivo (`openkey-device-firmware`).
//!
//! Aplicações Yubico (OATH, Management, OpenPGP, PIV) como applets ISO 7816-4
//! e o dispatcher multi-protocolo sobre `CardRouter` (ADR-0024). Camada de
//! composição do produto sobre `core` + `transports`; o FIDO2 puro vive em
//! `openkey-authenticator`.
//!
//! Compila tanto em host (`std`, padrão) quanto em alvos bare-metal
//! (`no_std` + `alloc`) via a feature `std`.

extern crate alloc;

/// Aplicação Yubico OATH (YKOATH) como applet ISO 7816-4.
pub mod yubico_oath;

/// Aplicação Yubico Management como applet ISO 7816-4.
pub mod yubico_management;

pub mod yubico_openpgp;
/// Applets stub multi-protocolo (PIV / OpenPGP).
pub mod yubico_piv;

/// Dispatcher multi-protocolo sobre CardRouter (ADR-0024).
pub mod multiprotocol;

pub use multiprotocol::{
    register_multiprotocol_applets, MULTIPROTOCOL_APPLET_COUNT,
    MULTIPROTOCOL_SUPPORTED_CAPABILITIES,
};
pub use yubico_management::{register_yubico_applets, ManagementApplet, AID_YUBICO_MANAGEMENT};
pub use yubico_oath::{OathAlgorithm, OathApplet, OathType, AID_YUBICO_OATH, MAX_CREDENTIALS};
pub use yubico_openpgp::{OpenPgpApplet, AID_OPENPGP, AID_OPENPGP_FULL};
pub use yubico_piv::{PivApplet, AID_PIV};
