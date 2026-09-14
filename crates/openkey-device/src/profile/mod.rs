//! Configuração de produto e descoberta de capabilities.
//!
//! Separa o que é hardware (`board-generic`) do que é decisão de produto
//! (nome, versão, política de PIN, extensões), evitando recompilar o firmware
//! inteiro para variar apenas o perfil comercial.
//!
//! Compila tanto em host (`std`, padrão) quanto em alvos bare-metal
//! (`no_std` + `alloc`) via a feature `std`.

/// Relato de capabilities em runtime.
pub mod capability;
/// Perfil de produto e seu builder.
pub mod profile;

pub use capability::{Capabilities, CapabilityDiscovery};
// Tipos de configuração (device/config), re-exportados por compatibilidade.
pub use crate::config::{
    AttestationType, Extension, PinPolicy, Protocol, Transport, TransportConfig, TransportType,
    UsbIdentity, UsbVendorPreset,
};
pub use profile::{DeviceProfile, DeviceProfileBuilder};

// Re-export attestation types for convenience
pub use openkey_core::ctap2::{AttestationCertificate, AttestationFormat};
