//! CTAP2 command definitions and handlers.

pub mod make_credential;
pub mod get_assertion;
pub mod get_info;
pub mod client_pin;
pub mod reset;

/// Standard CTAP2 command byte codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctap2Command {
    MakeCredential = 0x01,
    GetAssertion = 0x02,
    GetNextAssertion = 0x08,
    GetInfo = 0x04,
    ClientPin = 0x06,
    Reset = 0x07,
    Selection = 0x0B,
    LargeBlobs = 0x0C,
    Config = 0x0D,
}
