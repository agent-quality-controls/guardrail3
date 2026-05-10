use cargo_config_toml_parser::parse as parse_cargo_config_toml;
use cargo_toml_parser::parse_document as parse_cargo_document;
use clippy_toml_parser::parse_document as parse_clippy_document;
use g3rs_clippy_types::{
    G3RsClippyCargoConfigState, G3RsClippyCargoMemberState, G3RsClippyCargoRootState,
    G3RsClippyConfigChecksInput, G3RsClippyConfigState, G3RsClippyRustPolicyState,
    G3RsClippyWaiver,
};
use guardrail3_rs_toml_parser::types::RustProfile;

#[must_use]
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

#[must_use]
pub fn parse_error_rust_policy(rel_path: &str, reason: &str) -> G3RsClippyRustPolicyState {
    G3RsClippyRustPolicyState::ParseError {
        rel_path: rel_path.to_owned(),
        reason: reason.to_owned(),
    }
}

#[must_use]
pub fn unreadable_cargo_config(rel_path: &str, reason: &str) -> G3RsClippyCargoConfigState {
    G3RsClippyCargoConfigState::Unreadable {
        rel_path: rel_path.to_owned(),
        reason: reason.to_owned(),
    }
}

#[must_use]
pub fn parse_error_cargo_config(rel_path: &str, reason: &str) -> G3RsClippyCargoConfigState {
    G3RsClippyCargoConfigState::ParseError {
        rel_path: rel_path.to_owned(),
        reason: reason.to_owned(),
    }
}

/// Build a `Parsed` `cargo/config.toml` state at `rel_path` from raw TOML text.
///
/// # Panics
///
/// Panics when `raw` does not parse as `cargo/config.toml`; intended for fixture-only use.
#[must_use]
#[expect(
    clippy::expect_used,
    reason = "test-support fixture builder: malformed `raw` is a fixture bug, not a runtime error; surfacing it via panic gives a clear test failure"
)]
pub fn cargo_config(rel_path: &str, raw: &str) -> G3RsClippyCargoConfigState {
    G3RsClippyCargoConfigState::Parsed {
        rel_path: rel_path.to_owned(),
        cargo_config: Box::new(
            parse_cargo_config_toml(raw).expect("cargo config fixture should parse"),
        ),
    }
}

#[must_use]
pub const fn missing_cargo_root() -> G3RsClippyCargoRootState {
    G3RsClippyCargoRootState::Missing
}

#[must_use]
pub fn unreadable_cargo_root(rel_path: &str, reason: &str) -> G3RsClippyCargoRootState {
    G3RsClippyCargoRootState::Unreadable {
        rel_path: rel_path.to_owned(),
        reason: reason.to_owned(),
    }
}

#[must_use]
pub fn parse_error_cargo_root(rel_path: &str, reason: &str) -> G3RsClippyCargoRootState {
    G3RsClippyCargoRootState::ParseError {
        rel_path: rel_path.to_owned(),
        reason: reason.to_owned(),
    }
}

/// Build a `Parsed` workspace root `Cargo.toml` state at `rel_path` from raw TOML text.
///
/// # Panics
///
/// Panics when `raw` does not parse as a Cargo.toml document; intended for fixture-only use.
#[must_use]
#[expect(
    clippy::expect_used,
    reason = "test-support fixture builder: malformed `raw` is a fixture bug, not a runtime error; surfacing it via panic gives a clear test failure"
)]
pub fn cargo_root(rel_path: &str, raw: &str) -> G3RsClippyCargoRootState {
    G3RsClippyCargoRootState::Parsed {
        rel_path: rel_path.to_owned(),
        cargo: Box::new(parse_cargo_document(raw).expect("cargo root fixture should parse")),
    }
}

/// Build a `Parsed` workspace-member `Cargo.toml` state at `rel_path` from raw TOML text.
///
/// # Panics
///
/// Panics when `raw` does not parse as a Cargo.toml document; intended for fixture-only use.
#[must_use]
#[expect(
    clippy::expect_used,
    reason = "test-support fixture builder: malformed `raw` is a fixture bug, not a runtime error; surfacing it via panic gives a clear test failure"
)]
pub fn cargo_member(member_rel: &str, rel_path: &str, raw: &str) -> G3RsClippyCargoMemberState {
    G3RsClippyCargoMemberState::Parsed {
        member_rel: member_rel.to_owned(),
        rel_path: rel_path.to_owned(),
        cargo: Box::new(parse_cargo_document(raw).expect("cargo member fixture should parse")),
    }
}

#[must_use]
pub fn input_from_raw(rel_path: &str, raw: &str) -> G3RsClippyConfigChecksInput {
    input_with_raw_and_waivers(
        rel_path,
        raw,
        G3RsClippyRustPolicyState::Missing,
        G3RsClippyCargoRootState::Missing,
        Vec::new(),
        Vec::new(),
        Vec::new(),
    )
}

#[must_use]
pub fn input_with_raw(
    rel_path: &str,
    raw: &str,
    rust_policy: G3RsClippyRustPolicyState,
    cargo_root: G3RsClippyCargoRootState,
    cargo_workspace_members: Vec<G3RsClippyCargoMemberState>,
    cargo_configs: Vec<G3RsClippyCargoConfigState>,
) -> G3RsClippyConfigChecksInput {
    input_with_raw_and_waivers(
        rel_path,
        raw,
        rust_policy,
        cargo_root,
        cargo_workspace_members,
        cargo_configs,
        Vec::new(),
    )
}

#[must_use]
pub fn waiver(rule: &str, file: &str, selector: &str, reason: &str) -> G3RsClippyWaiver {
    G3RsClippyWaiver {
        rule: rule.to_owned(),
        file: file.to_owned(),
        selector: selector.to_owned(),
        reason: reason.to_owned(),
    }
}

#[must_use]
pub fn input_with_raw_and_waivers(
    rel_path: &str,
    raw: &str,
    rust_policy: G3RsClippyRustPolicyState,
    cargo_root: G3RsClippyCargoRootState,
    cargo_workspace_members: Vec<G3RsClippyCargoMemberState>,
    cargo_configs: Vec<G3RsClippyCargoConfigState>,
    waivers: Vec<G3RsClippyWaiver>,
) -> G3RsClippyConfigChecksInput {
    let clippy = match parse_clippy_document(raw) {
        Ok(document) => G3RsClippyConfigState::Parsed(Box::new(document)),
        Err(err) => G3RsClippyConfigState::ParseError {
            reason: err.to_string(),
        },
    };

    G3RsClippyConfigChecksInput {
        clippy_rel_path: rel_path.to_owned(),
        clippy,
        rust_policy,
        cargo_root,
        cargo_workspace_members,
        cargo_configs,
        waivers,
    }
}
