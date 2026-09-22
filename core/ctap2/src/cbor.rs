//! CBOR parsing and serialization utilities for CTAP2.

use ciborium::value::Value;
use serde::{de::DeserializeOwned, Serialize};

/// Decodes CBOR bytes into a typed structure.
pub fn decode_cbor<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, ciborium::de::Error<std::io::Error>> {
    let mut reader = bytes;
    let res: T = ciborium::from_reader(&mut reader)?;
    if !reader.is_empty() {
        return Err(ciborium::de::Error::Semantic(
            None,
            "trailing data after CBOR item".to_string(),
        ));
    }
    Ok(res)
}

/// Encodes a typed structure into CBOR bytes.
pub fn encode_cbor<T: Serialize>(val: &T) -> Result<Vec<u8>, ciborium::ser::Error<std::io::Error>> {
    let mut buf = Vec::new();
    ciborium::into_writer(val, &mut buf)?;
    Ok(buf)
}

/// Parses raw bytes into generic CBOR Value.
pub fn parse_cbor_value(bytes: &[u8]) -> Result<Value, ciborium::de::Error<std::io::Error>> {
    let mut reader = bytes;
    let res: Value = ciborium::from_reader(&mut reader)?;
    if !reader.is_empty() {
        return Err(ciborium::de::Error::Semantic(
            None,
            "trailing data after CBOR item".to_string(),
        ));
    }
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_cbor_exact() {
        let original = 42u32;
        let encoded = encode_cbor(&original).expect("encode succeeds");
        let decoded: u32 = decode_cbor(&encoded).expect("decode succeeds");
        assert_eq!(decoded, original);
    }

    #[test]
    fn test_decode_cbor_rejects_trailing_bytes() {
        let original = 42u32;
        let mut encoded = encode_cbor(&original).expect("encode succeeds");
        encoded.push(0xFF); // extra trailing byte
        let result: Result<u32, _> = decode_cbor(&encoded);
        assert!(result.is_err(), "should reject extra trailing bytes");
    }
}

