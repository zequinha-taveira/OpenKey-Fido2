//! CBOR parsing and serialization utilities for CTAP2.

use ciborium::value::Value;
use serde::{de::DeserializeOwned, Serialize};

/// Decodes CBOR bytes into a typed structure.
pub fn decode_cbor<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, ciborium::de::Error<std::io::Error>> {
    ciborium::from_reader(bytes)
}

/// Encodes a typed structure into CBOR bytes.
pub fn encode_cbor<T: Serialize>(val: &T) -> Result<Vec<u8>, ciborium::ser::Error<std::io::Error>> {
    let mut buf = Vec::new();
    ciborium::into_writer(val, &mut buf)?;
    Ok(buf)
}

/// Parses raw bytes into generic CBOR Value.
pub fn parse_cbor_value(bytes: &[u8]) -> Result<Value, ciborium::de::Error<std::io::Error>> {
    ciborium::from_reader(bytes)
}
