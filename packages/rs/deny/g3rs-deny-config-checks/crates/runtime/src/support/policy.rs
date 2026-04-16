use g3rs_deny_types::{G3RsDenyConfigChecksInput, G3RsDenyRustPolicyState};
use guardrail3_rs_toml_parser::RustProfile;

pub(crate) fn rust_policy_valid(input: &G3RsDenyConfigChecksInput) -> bool {
    !matches!(
        input.rust_policy,
        G3RsDenyRustPolicyState::Unreadable { .. } | G3RsDenyRustPolicyState::ParseError { .. }
    )
}

pub(crate) fn managed_profile_name(input: &G3RsDenyConfigChecksInput) -> Option<&'static str> {
    match input.rust_policy {
        G3RsDenyRustPolicyState::Parsed {
            profile: Some(RustProfile::Library),
            ..
        } => Some("library"),
        _ => None,
    }
}
