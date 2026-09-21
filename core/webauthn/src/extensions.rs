//! WebAuthn / CTAP2 extensions processing.

use serde::{Deserialize, Serialize};

/// Supported WebAuthn extensions.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebAuthnExtensions {
    pub hmac_secret: bool,
    pub cred_protect: Option<u8>,
    pub min_pin_length: bool,
    pub large_blob_key: bool,
}
