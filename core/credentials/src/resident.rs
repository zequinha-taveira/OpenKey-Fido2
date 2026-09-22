//! Resident key storage structures.

use serde::{Deserialize, Serialize};

/// Resident credential key record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResidentKey {
    pub credential_id: Vec<u8>,
    pub rp_id_hash: [u8; 32],
    pub user_id: Vec<u8>,
    pub private_key: Vec<u8>,
    pub sign_count: u32,
}
