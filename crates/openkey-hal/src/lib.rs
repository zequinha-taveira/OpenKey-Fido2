//! Camada de abstração de hardware (HAL): traits independentes de MCU.
//!
//! [`BoardDefinition`] descreve pinagem, transportes e features de segurança
//! de forma `const`. Perfis concretos de board vivem nas crates irmãs
//! (`openkey-profile-generic`, `openkey-profile-nrf52840`,
//! `openkey-profile-stm32`, `openkey-profile-esp32`, `openkey-profile-rp2350`
//! em `targets/profile-*/`).

#![no_std]

/// Definição de board, HAL e traits de periféricos.
pub mod board_generic;
/// Detecção de user presence reaproveitando o botão BOOTSEL (RP2350).
pub mod bootsel;
pub mod crypto;
pub mod rng;
pub mod storage;
pub mod timer;
pub mod user_presence;

pub use board_generic::{
    BoardDefinition, BoardHAL, BoardTrait, GpioPin, I2cBus, SecurityFeatures, UserPresenceSource,
    UserVerificationDevice, UserVerificationError, TRANSPORT_BLE, TRANSPORT_NFC,
    TRANSPORT_USB_CCID, TRANSPORT_USB_HID,
};
pub use bootsel::{BootselButton, Rp2350Qspi, UserPresenceButton};
