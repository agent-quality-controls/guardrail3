use clippy_toml_parser::parse as parse_clippy_toml;
use g3rs_clippy_types::{
    G3RsClippyCargoConfigOverride, G3RsClippyConfigChecksInput, G3RsClippyConfigState,
    G3RsClippyRustPolicyState,
};
use guardrail3_rs_toml_parser::RustProfile;

pub fn parsed_rust_policy(
    rel_path: &str,
    profile: Option<RustProfile>,
    garde_enabled: bool,
) -> G3RsClippyRustPolicyState {
    G3RsClippyRustPolicyState::Parsed {
        rel_path: rel_path.to_owned(),
        profile,
        garde_enabled,
    }
}

pub fn parse_error_rust_policy(rel_path: &str, reason: &str) -> G3RsClippyRustPolicyState {
    G3RsClippyRustPolicyState::ParseError {
        rel_path: rel_path.to_owned(),
        reason: reason.to_owned(),
    }
}

pub fn override_facts(rel_path: &str, parse_error: Option<&str>) -> G3RsClippyCargoConfigOverride {
    G3RsClippyCargoConfigOverride {
        rel_path: rel_path.to_owned(),
        parse_error: parse_error.map(str::to_owned),
    }
}

pub fn input_from_raw(rel_path: &str, raw: &str) -> G3RsClippyConfigChecksInput {
    input_with_raw(
        rel_path,
        raw,
        G3RsClippyRustPolicyState::Missing,
        false,
        Vec::new(),
    )
}

pub fn input_with_raw(
    rel_path: &str,
    raw: &str,
    rust_policy: G3RsClippyRustPolicyState,
    published_library_policy: bool,
    cargo_config_overrides: Vec<G3RsClippyCargoConfigOverride>,
) -> G3RsClippyConfigChecksInput {
    let clippy = match toml::from_str::<toml::Value>(raw) {
        Ok(raw_value) => G3RsClippyConfigState::Parsed {
            raw: raw_value,
            typed: parse_clippy_toml(raw).map_err(|err| err.to_string()),
        },
        Err(err) => G3RsClippyConfigState::ParseError {
            reason: err.to_string(),
        },
    };

    G3RsClippyConfigChecksInput {
        clippy_rel_path: rel_path.to_owned(),
        clippy,
        rust_policy,
        published_library_policy,
        cargo_config_overrides,
    }
}
