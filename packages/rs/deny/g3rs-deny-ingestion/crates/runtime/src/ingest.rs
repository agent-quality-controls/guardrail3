/// Assemble the checks input from selected and parsed data.
use deny_toml_parser::types::DenyToml;
use g3rs_deny_types::{
    G3RsDenyConfigChecksInput, G3RsDenyFileTreeChecksInput, G3RsDenyInputFailure,
    G3RsDenyRustPolicyState,
};

/// Build the checks input from the parsed deny config and its relative path.
pub(crate) fn assemble(
    deny_rel_path: String,
    deny: DenyToml,
    rust_policy: &G3RsDenyRustPolicyState,
) -> G3RsDenyConfigChecksInput {
    G3RsDenyConfigChecksInput {
        deny_rel_path,
        deny,
        rust_policy: rust_policy.clone(),
    }
}

pub(crate) fn input_failure(
    title: impl Into<String>,
    rel_path: impl Into<String>,
    message: impl Into<String>,
) -> G3RsDenyInputFailure {
    G3RsDenyInputFailure {
        title: title.into(),
        rel_path: rel_path.into(),
        message: message.into(),
    }
}

pub(crate) fn filetree_input(
    selected_deny_rel_path: Option<String>,
    candidate_deny_rel_paths: Vec<String>,
    input_failures: Vec<G3RsDenyInputFailure>,
) -> G3RsDenyFileTreeChecksInput {
    G3RsDenyFileTreeChecksInput {
        selected_deny_rel_path,
        candidate_deny_rel_paths,
        input_failures,
    }
}
