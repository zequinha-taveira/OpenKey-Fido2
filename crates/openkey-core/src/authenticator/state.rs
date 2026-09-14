/// Mutable state that does not depend on a board or transport.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct AuthenticatorState {
    pub user_presence_required: bool,
    pub user_verification_required: bool,
}
