//! CTAP2 Response encoding and status codes.

use serde::{Deserialize, Serialize};

/// CTAP2 status return codes (CTAP2.1 §8.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum CtapStatusCode {
    Ok = 0x00,
    InvalidCommand = 0x01,
    InvalidParameter = 0x02,
    InvalidLength = 0x03,
    InvalidSeq = 0x04,
    Timeout = 0x05,
    ChannelBusy = 0x06,
    LockRequired = 0x0A,
    InvalidChannel = 0x0B,
    CborUnexpectedType = 0x11,
    InvalidCbor = 0x12,
    MissingParameter = 0x14,
    LimitExceeded = 0x15,
    UnsupportedExtension = 0x16,
    CredentialExcluded = 0x19,
    Processing = 0x21,
    InvalidCredential = 0x22,
    UserActionPending = 0x23,
    OperationPending = 0x24,
    PinInvalid = 0x31,
    PinBlocked = 0x32,
    PinAuthInvalid = 0x33,
    PinAuthBlocked = 0x34,
    PinNotSet = 0x35,
    PinRequired = 0x36,
    PinPolicyViolation = 0x37,
    PinTokenExpired = 0x38,
    RequestAndPinNotAllowed = 0x39,
    KeyStoreFull = 0x2C,
    NoCredentials = 0x2E,
    UserActionTimeout = 0x2F,
    NotAllowed = 0x30,
    Other = 0x7F,
}

/// CTAP2 outgoing response structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ctap2Response {
    pub status: CtapStatusCode,
    pub data: Option<Vec<u8>>,
}
