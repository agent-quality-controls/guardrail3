use crate::rs_clippy_config_20_forbid_clippy_conf_dir_override::check;
use g3rs_clippy_config_checks_assertions::rs_clippy_config_20_forbid_clippy_conf_dir_override as assertions;
use g3rs_clippy_types::G3RsClippyRustPolicyState;
use test_support::{input_with_raw, override_facts};

#[test]
fn inventories_clean_state_when_no_overrides_exist() {
    let input = input_with_raw(
        "clippy.toml",
        "",
        G3RsClippyRustPolicyState::Missing,
        false,
        Vec::new(),
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assertions::assert_no_overrides_inventory(&results);
}

#[test]
fn errors_on_override_surface() {
    let input = input_with_raw(
        "clippy.toml",
        "",
        G3RsClippyRustPolicyState::Missing,
        false,
        vec![override_facts(".cargo/config.toml", None)],
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assert_eq!(
        assertions::findings(&results),
        vec![assertions::error(
            "clippy config dir override is forbidden",
            "`.cargo/config.toml` sets `CLIPPY_CONF_DIR`, which bypasses the routed clippy policy-root model. Remove the `CLIPPY_CONF_DIR` setting from `.cargo/config.toml`.",
            ".cargo/config.toml",
            false,
        )]
    );
}

#[test]
fn errors_on_malformed_override_surface() {
    let input = input_with_raw(
        "clippy.toml",
        "",
        G3RsClippyRustPolicyState::Missing,
        false,
        vec![override_facts(".cargo/config", Some("bad env"))],
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assert_eq!(
        assertions::findings(&results),
        vec![assertions::error(
            "cargo config override surface is not parseable",
            "Failed to parse `.cargo/config` while checking for forbidden `CLIPPY_CONF_DIR` overrides: bad env",
            ".cargo/config",
            false,
        )]
    );
}
