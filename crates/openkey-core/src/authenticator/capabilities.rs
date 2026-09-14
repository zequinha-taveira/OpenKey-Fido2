/// Protocol capabilities exposed by the domain layer.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct AuthenticatorCapabilities {
    pub resident_keys: bool,
    pub user_verification: bool,
    pub client_pin: bool,
}
