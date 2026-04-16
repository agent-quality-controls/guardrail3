use deny_toml_parser::parse as parse_deny_toml;
use g3rs_deny_types::G3RsDenyConfigChecksInput;
use g3rs_deny_types::G3RsDenyRustPolicyState;
use guardrail3_check_types::G3CheckResult;
use guardrail3_rs_toml_parser::RustProfile;

pub fn input(
    deny_toml: &str,
    profile: Option<RustProfile>,
    policy_context_valid: bool,
) -> G3RsDenyConfigChecksInput {
    G3RsDenyConfigChecksInput {
        deny_rel_path: "deny.toml".to_owned(),
        deny: parse_deny_toml(deny_toml).expect("deny fixture should parse"),
        rust_policy: rust_policy(profile, policy_context_valid),
    }
}

pub fn run(
    deny_toml: &str,
    profile: Option<RustProfile>,
    policy_context_valid: bool,
    rule: fn(&G3RsDenyConfigChecksInput, &mut Vec<G3CheckResult>),
) -> Vec<G3CheckResult> {
    let input = input(deny_toml, profile, policy_context_valid);
    let mut results = Vec::new();
    rule(&input, &mut results);
    results
}

pub fn run_with_rust_policy(
    deny_toml: &str,
    rust_policy: G3RsDenyRustPolicyState,
    rule: fn(&G3RsDenyConfigChecksInput, &mut Vec<G3CheckResult>),
) -> Vec<G3CheckResult> {
    let input = G3RsDenyConfigChecksInput {
        deny_rel_path: "deny.toml".to_owned(),
        deny: parse_deny_toml(deny_toml).expect("deny fixture should parse"),
        rust_policy,
    };
    let mut results = Vec::new();
    rule(&input, &mut results);
    results
}

fn rust_policy(profile: Option<RustProfile>, policy_context_valid: bool) -> G3RsDenyRustPolicyState {
    if !policy_context_valid {
        return G3RsDenyRustPolicyState::ParseError {
            rel_path: "guardrail3-rs.toml".to_owned(),
            reason: "invalid Rust policy".to_owned(),
        };
    }

    match profile {
        Some(RustProfile::Library) => G3RsDenyRustPolicyState::Parsed {
            rel_path: "guardrail3-rs.toml".to_owned(),
            profile: Some(RustProfile::Library),
        },
        Some(RustProfile::Service) => G3RsDenyRustPolicyState::Parsed {
            rel_path: "guardrail3-rs.toml".to_owned(),
            profile: Some(RustProfile::Service),
        },
        None => G3RsDenyRustPolicyState::Missing,
    }
}
