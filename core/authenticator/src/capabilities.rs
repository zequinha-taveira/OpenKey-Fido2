//! Authenticator capabilities and supported protocol features.

use serde::{Deserialize, Serialize};

/// Supported capabilities of the authenticator device.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticatorCapabilities {
    pub rk: bool,
    pub up: bool,
    pub uv: bool,
    pub plat: bool,
    pub pin_uv_auth_token: bool,
    pub cred_mgmt: bool,
    pub large_blobs: bool,
}

impl Default for AuthenticatorCapabilities {
    fn default() -> Self {
        Self {
            rk: true,
            up: true,
            uv: false,
            plat: false,
            pin_uv_auth_token: true,
            cred_mgmt: true,
            large_blobs: false,
        }
    }
}
