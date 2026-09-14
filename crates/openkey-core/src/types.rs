//! Small domain types that are independent of a concrete MCU or transport.

extern crate alloc;

/// A relying-party identifier represented as UTF-8 text.
pub type RpId = alloc::string::String;
