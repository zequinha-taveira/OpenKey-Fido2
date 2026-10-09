//! Credential domain types shared with the persistence implementation.

#[allow(clippy::module_inception)]
pub mod credential;
pub mod resident;

pub use credential::{Credential, StoredCredential};
