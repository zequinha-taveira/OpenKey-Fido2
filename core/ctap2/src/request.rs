//! CTAP2 Request decoding and model.

use serde::{Deserialize, Serialize};

/// CTAP2 incoming request structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ctap2Request {
    pub cmd: u8,
    pub payload: Vec<u8>,
}
