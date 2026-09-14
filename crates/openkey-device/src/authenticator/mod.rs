//! API do autenticador FIDO2 embarcado.
//!
//! [`EmbeddedAuthenticator`] costura as camadas (device profile, transporte,
//! WebAuthn, CTAP2, crypto, storage) para que integradores dependam de um
//! único tipo. Applets de produto (OATH/Management/OpenPGP/PIV) vivem em
//! `openkey-device-firmware`. Ver `docs/architecture.md` para o diagrama completo.
//!
//! Compila tanto em host (`std`, padrão — inclui o storage inseguro de
//! arquivo) quanto em alvos bare-metal (`no_std` + `alloc`) via a feature
//! `std`.

/// Coordenação das camadas do firmware.
pub mod authenticator;

pub use authenticator::EmbeddedAuthenticator;
#[cfg(feature = "std")]
pub use authenticator::InsecureHostStorage;
