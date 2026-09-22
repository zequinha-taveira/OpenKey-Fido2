//! WebAuthn credential data representation.

use serde::{Deserialize, Serialize};

/// WebAuthn Credential descriptor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebAuthnCredential {
    pub id: Vec<u8>,
    pub cred_type: String,
    pub transports: Vec<String>,
}
