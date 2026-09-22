//! WebAuthn Relying Party (RP) entity model.

use serde::{Deserialize, Serialize};

/// Relying Party entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelyingParty {
    pub id: String,
    pub name: Option<String>,
}
