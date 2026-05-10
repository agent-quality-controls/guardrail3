#![expect(
    clippy::expect_used,
    clippy::disallowed_methods,
    reason = "test-only fixture helpers fail-fast on bad fixtures and need raw toml::Value to enumerate explicit top-level rustfmt keys, mirroring the production parse path"
)]

use cargo_toml_parser::parse as parse_cargo_toml;
use g3rs_fmt_types::{
    G3RsFmtCargoState, G3RsFmtConfigChecksInput, G3RsFmtRustPolicyState, G3RsFmtRustfmtConfigState,
    G3RsFmtToolchainState, G3RsFmtWaiver,
};
use rust_toolchain_toml_parser::parse as parse_toolchain_toml;
use rustfmt_toml_parser::parse as parse_rustfmt_toml;

#[must_use]
/// Implements this item.
///
/// # Panics
/// Panics on assertion failure or unexpected input.
pub fn parsed_rustfmt(rustfmt_toml: &str) -> G3RsFmtRustfmtConfigState {
    G3RsFmtRustfmtConfigState::Parsed(Box::new(
        parse_rustfmt_toml(rustfmt_toml).expect("rustfmt test fixture should parse"),
    ))
}

#[must_use]
/// Implements this item.
///
/// # Panics
/// Panics on assertion failure or unexpected input.
pub fn parsed_cargo(cargo_toml: &str) -> G3RsFmtCargoState {
    G3RsFmtCargoState::Parsed(Box::new(
        parse_cargo_toml(cargo_toml).expect("cargo test fixture should parse"),
    ))
}

#[must_use]
/// Implements this item.
///
/// # Panics
/// Panics on assertion failure or unexpected input.
pub fn parsed_toolchain(toolchain_toml: &str) -> G3RsFmtToolchainState {
    G3RsFmtToolchainState::Parsed(Box::new(
        parse_toolchain_toml(toolchain_toml).expect("toolchain test fixture should parse"),
    ))
}

#[must_use]
/// Implements this item.
///
/// # Panics
/// Panics on assertion failure or unexpected input.
pub fn explicit_keys(rustfmt_toml: &str) -> Vec<String> {
    toml::from_str::<toml::Value>(rustfmt_toml)
        .expect("rustfmt test fixture should parse as toml")
        .as_table()
        .expect("rustfmt test fixture should be a toml table")
        .keys()
        .cloned()
        .collect()
}

#[must_use]
pub fn rustfmt_input(
    rustfmt_state: G3RsFmtRustfmtConfigState,
    rustfmt_explicit_keys: Vec<String>,
    cargo_state: G3RsFmtCargoState,
    toolchain_state: G3RsFmtToolchainState,
    rust_policy: G3RsFmtRustPolicyState,
) -> G3RsFmtConfigChecksInput {
    G3RsFmtConfigChecksInput {
        rustfmt_rel_path: "rustfmt.toml".to_owned(),
        rustfmt_state,
        rustfmt_explicit_keys,
        cargo_rel_path: "Cargo.toml".to_owned(),
        cargo_state,
        toolchain_rel_path: "rust-toolchain.toml".to_owned(),
        toolchain_state,
        rust_policy,
    }
}

#[must_use]
pub fn waiver(reason: &str) -> G3RsFmtWaiver {
    G3RsFmtWaiver {
        rule: "g3rs-fmt/ignore-escape-hatch".to_owned(),
        file: "rustfmt.toml".to_owned(),
        selector: "ignore".to_owned(),
        reason: reason.to_owned(),
    }
}
