//! Credentials storage and management abstractions.

pub mod discoverable;
pub mod resident;

pub use discoverable::DiscoverableCredential;
pub use resident::ResidentKey;
