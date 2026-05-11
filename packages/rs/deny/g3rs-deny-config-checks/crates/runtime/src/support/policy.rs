use g3rs_deny_types::{G3RsDenyConfigChecksInput, G3RsDenyRustPolicyState};
use g3rs_toml_parser::types::RustProfile;

/// Implements `rust policy valid`.
pub(crate) const fn rust_policy_valid(input: &G3RsDenyConfigChecksInput) -> bool {
    !matches!(
        input.rust_policy,
        G3RsDenyRustPolicyState::Unreadable { .. } | G3RsDenyRustPolicyState::ParseError { .. }
    )
}

/// Implements `managed profile name`.
pub(crate) const fn managed_profile_name(
    input: &G3RsDenyConfigChecksInput,
) -> Option<&'static str> {
    match input.rust_policy {
        G3RsDenyRustPolicyState::Parsed {
            profile: Some(RustProfile::Library),
            ..
        } => Some("library"),
        G3RsDenyRustPolicyState::Parsed {
            profile: Some(RustProfile::Service) | None,
            ..
        }
        | G3RsDenyRustPolicyState::Missing
        | G3RsDenyRustPolicyState::Unreadable { .. }
        | G3RsDenyRustPolicyState::ParseError { .. } => None,
    }
}
