use deny_toml_parser::parse as parse_deny_toml;
use g3rs_deny_config_checks_types::G3RsDenyConfigChecksInput;
use g3rs_deny_types::G3RsDenyRustPolicyState;
use guardrail3_rs_toml_parser::RustProfile;
use guardrail3_check_types::G3CheckResult;

pub(crate) fn input(deny_toml: &str, profile_name: Option<&str>, policy_context_valid: bool) -> G3RsDenyConfigChecksInput {
    G3RsDenyConfigChecksInput {
        deny_rel_path: "deny.toml".to_owned(),
        deny: parse_deny_toml(deny_toml).expect("deny fixture should parse"),
        rust_policy: rust_policy(profile_name, policy_context_valid),
    }
}

pub(crate) fn run(
    deny_toml: &str,
    profile_name: Option<&str>,
    policy_context_valid: bool,
    rule: fn(&G3RsDenyConfigChecksInput, &mut Vec<G3CheckResult>),
) -> Vec<G3CheckResult> {
    let input = input(deny_toml, profile_name, policy_context_valid);
    let mut results = Vec::new();
    rule(&input, &mut results);
    results
}

pub(crate) fn run_with_rust_policy(
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

pub(crate) fn canonical_bans_toml(profile_name: &str) -> String {
    match profile_name {
        "service" => SERVICE_BANS_TOML.to_owned(),
        "library" => LIBRARY_BANS_TOML.to_owned(),
        other => panic!("unexpected canonical bans profile `{other}`"),
    }
}

fn rust_policy(profile_name: Option<&str>, policy_context_valid: bool) -> G3RsDenyRustPolicyState {
    if !policy_context_valid {
        return G3RsDenyRustPolicyState::ParseError {
            rel_path: "guardrail3-rs.toml".to_owned(),
            reason: "invalid Rust policy".to_owned(),
        };
    }

    match profile_name {
        Some("library") => G3RsDenyRustPolicyState::Parsed {
            rel_path: "guardrail3-rs.toml".to_owned(),
            profile: Some(RustProfile::Library),
        },
        Some("service") => G3RsDenyRustPolicyState::Parsed {
            rel_path: "guardrail3-rs.toml".to_owned(),
            profile: Some(RustProfile::Service),
        },
        Some(other) => panic!("unexpected test profile `{other}`"),
        None => G3RsDenyRustPolicyState::Missing,
    }
}

const SERVICE_BANS_TOML: &str = r#"[bans]
deny = [
  "actix-web",
  "anyhow",
  "async-std",
  "bincode",
  "chrono",
  "diesel",
  "env_logger",
  "fancy-regex",
  "fern",
  "flatbuffers",
  "grep-cli",
  "grep-matcher",
  "grep-regex",
  "isahc",
  "json5",
  "lazy_static",
  "log4rs",
  "onig",
  "openssl",
  "openssl-sys",
  "pcre2",
  "poem",
  "prost",
  "regex",
  "rmp-serde",
  "rocket",
  "sea-orm",
  "simd-json",
  "simple_logger",
  "smol",
  "sonic-rs",
  "surf",
  "ureq",
  "warp"
]
"#;

const LIBRARY_BANS_TOML: &str = r#"[bans]
deny = [
  "actix-web",
  "anyhow",
  "async-std",
  "axum",
  "bincode",
  "chrono",
  "diesel",
  "env_logger",
  "fancy-regex",
  "fern",
  "flatbuffers",
  "grep-cli",
  "grep-matcher",
  "grep-regex",
  "hyper",
  "isahc",
  "json5",
  "lazy_static",
  "log4rs",
  "onig",
  "openssl",
  "openssl-sys",
  "pcre2",
  "poem",
  "prost",
  "regex",
  "reqwest",
  "rmp-serde",
  "rocket",
  "sea-orm",
  "simd-json",
  "simple_logger",
  "smol",
  "sonic-rs",
  "sqlx",
  "surf",
  "tokio",
  "ureq",
  "warp"
]
"#;
