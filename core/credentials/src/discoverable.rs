//! Discoverable credential management.

use serde::{Deserialize, Serialize};

/// Discoverable credential metadata and descriptor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoverableCredential {
    pub credential_id: Vec<u8>,
    pub rp_id: String,
    pub user_name: Option<String>,
    pub user_display_name: Option<String>,
}
